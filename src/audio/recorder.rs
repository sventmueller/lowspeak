//! Microphone capture at the device's native rate, resampled to 16 kHz mono when recording stops.

use std::fmt;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicUsize, Ordering};
use std::time::Duration;

use anyhow::Result;
use block2::RcBlock;
use cpal::traits::{DeviceTrait as _, HostTrait as _, StreamTrait as _};
use cpal::{FromSample, SampleFormat, SizedSample};
use objc2::runtime::Bool;
use objc2_av_foundation::{
    AVAuthorizationStatus, AVCaptureDevice, AVCaptureMicrophoneMode, AVMediaTypeAudio,
};

use super::SAMPLE_RATE;
use super::preprocess::{resample, rms};
use crate::i18n::Texts;
use crate::util::filelog;

static MIC_PERMISSION: AtomicBool = AtomicBool::new(false);

/// How soon after the first real sound the [`OnLive`] callback runs at the latest.
const LIVE_POLL_INTERVAL: Duration = Duration::from_millis(10);

/// Asks for microphone access up front so the first recording is not lost to the prompt.
pub fn request_mic_permission() {
    // SAFETY: AVMediaTypeAudio is an immutable constant that AVFoundation sets when it loads.
    let Some(audio) = (unsafe { AVMediaTypeAudio }) else {
        return;
    };
    // SAFETY: `audio` is AVFoundation's own media type constant, the only argument; the class
    // method has no other precondition and may be called from any thread.
    let status = unsafe { AVCaptureDevice::authorizationStatusForMediaType(audio) };
    if status == AVAuthorizationStatus::Authorized {
        MIC_PERMISSION.store(true, Ordering::Relaxed);
        flog!(filelog::AUDIO, "Microphone permission already granted");
    } else if status == AVAuthorizationStatus::NotDetermined {
        let handler = RcBlock::new(|granted: Bool| {
            MIC_PERMISSION.store(granted.as_bool(), Ordering::Relaxed);
            flog!(
                filelog::AUDIO,
                "Microphone permission {}",
                if granted.as_bool() {
                    "granted"
                } else {
                    "denied"
                }
            );
        });
        // SAFETY: `audio` is AVFoundation's media type constant, and the handler, which AVFoundation
        // calls on an arbitrary queue, only touches an atomic and the thread-safe file log.
        unsafe { AVCaptureDevice::requestAccessForMediaType_completionHandler(audio, &handler) };
    } else {
        flog!(filelog::AUDIO, "Microphone permission denied");
    }
}

fn mic_permission_granted() -> bool {
    if MIC_PERMISSION.load(Ordering::Relaxed) {
        return true;
    }
    // Access may have been granted in System Settings since launch.
    // SAFETY: AVMediaTypeAudio is an immutable constant that AVFoundation sets when it loads.
    let granted = unsafe { AVMediaTypeAudio }.is_some_and(|audio| {
        // SAFETY: `audio` is AVFoundation's own media type constant, the only argument; the class
        // method has no other precondition and may be called from any thread.
        let status = unsafe { AVCaptureDevice::authorizationStatusForMediaType(audio) };
        status == AVAuthorizationStatus::Authorized
    });
    MIC_PERMISSION.store(granted, Ordering::Relaxed);
    granted
}

fn mode_name(mode: AVCaptureMicrophoneMode) -> &'static str {
    match mode {
        AVCaptureMicrophoneMode::Standard => "standard",
        AVCaptureMicrophoneMode::WideSpectrum => "wide spectrum",
        AVCaptureMicrophoneMode::VoiceIsolation => "voice isolation",
        _ => "unknown",
    }
}

/// Why the microphone did not start.
#[derive(Debug)]
pub enum StartError {
    PermissionDenied,
    NoMicrophone,
    /// CoreAudio refused the device or its format.
    Device(String),
}

impl fmt::Display for StartError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PermissionDenied => f.write_str("microphone permission not granted"),
            Self::NoMicrophone => f.write_str("no microphone available"),
            Self::Device(detail) => write!(f, "microphone failed: {detail}"),
        }
    }
}

impl std::error::Error for StartError {}

impl StartError {
    pub fn user_facing_description(&self, t: &Texts) -> String {
        match self {
            Self::PermissionDenied => t.microphone_permission_denied.to_owned(),
            Self::NoMicrophone => t.no_microphone.to_owned(),
            Self::Device(detail) => (t.microphone_error)(detail),
        }
    }
}

/// Samples written by the audio thread and read elsewhere without locks: its capacity is
/// allocated once per recording, so the audio thread never allocates or waits.
struct SampleBuffer {
    samples: Box<[AtomicU32]>,
    len: AtomicUsize,
    overflowed: AtomicBool,
}

impl SampleBuffer {
    fn with_capacity(capacity: usize) -> Self {
        // Zeroed allocations this large are mapped lazily, so memory is used only as it fills.
        let zeroed = vec![0u32; capacity].into_boxed_slice();
        // SAFETY: AtomicU32 has the same size, alignment and bit validity as u32, and the slice
        // keeps its length.
        let samples = unsafe { Box::from_raw(Box::into_raw(zeroed) as *mut [AtomicU32]) };
        Self {
            samples,
            len: AtomicUsize::new(0),
            overflowed: AtomicBool::new(false),
        }
    }

    /// Appends samples; only the audio thread calls this.
    fn append(&self, samples: impl Iterator<Item = f32>) {
        let mut end = self.len.load(Ordering::Relaxed);
        for sample in samples {
            let Some(slot) = self.samples.get(end) else {
                self.overflowed.store(true, Ordering::Relaxed);
                break;
            };
            slot.store(sample.to_bits(), Ordering::Relaxed);
            end += 1;
        }
        self.len.store(end, Ordering::Release);
    }

    fn samples(&self) -> impl Iterator<Item = f32> + '_ {
        let len = self.len.load(Ordering::Acquire);
        self.samples[..len]
            .iter()
            .map(|sample| f32::from_bits(sample.load(Ordering::Relaxed)))
    }

    fn len(&self) -> usize {
        self.len.load(Ordering::Acquire)
    }
}

pub struct Recorder {
    stream: Option<cpal::Stream>,
    buffer: Option<Arc<SampleBuffer>>,
    live_watch: Option<LiveWatch>,
    device_rate: u32,
}

/// Called at most once per recording, from a background thread, soon after the first real sound
/// arrives; not called if the recording ends first.
pub type OnLive = Box<dyn Fn() + Send + Sync>;

/// The audio thread only raises `live`; a watcher thread runs the [`OnLive`] callback, whose work
/// (the app dispatches to the main thread) would allocate on the real-time thread.
#[derive(Default)]
struct LiveSignal {
    live: AtomicBool,
    stopped: AtomicBool,
}

/// Owns the watcher of one recording and ends it when dropped.
struct LiveWatch(Arc<LiveSignal>);

impl LiveWatch {
    fn signal(&self) -> Arc<LiveSignal> {
        Arc::clone(&self.0)
    }
}

impl Drop for LiveWatch {
    fn drop(&mut self) {
        self.0.stopped.store(true, Ordering::Release);
    }
}

fn watch_live(on_live: OnLive) -> std::io::Result<LiveWatch> {
    let signal = Arc::new(LiveSignal::default());
    let watched = Arc::clone(&signal);
    std::thread::Builder::new()
        .name("microphone-live".to_owned())
        .spawn(move || {
            while !watched.live.load(Ordering::Acquire) {
                if watched.stopped.load(Ordering::Acquire) {
                    return;
                }
                std::thread::sleep(LIVE_POLL_INTERVAL);
            }
            if !watched.stopped.load(Ordering::Acquire) {
                on_live();
            }
        })?;
    Ok(LiveWatch(signal))
}

fn build_stream<T>(
    device: &cpal::Device,
    config: cpal::StreamConfig,
    buffer: Arc<SampleBuffer>,
    signal: Arc<LiveSignal>,
) -> Result<cpal::Stream, cpal::Error>
where
    T: SizedSample,
    f32: FromSample<T>,
{
    let channels = usize::from(config.channels.max(1));
    let scale = 1.0 / f32::from(config.channels.max(1));
    let mut heard = false;
    device.build_input_stream::<T, _, _>(
        config,
        move |data: &[T], _| {
            let mono = data.chunks_exact(channels).map(|frame| {
                frame
                    .iter()
                    .map(|&sample| sample.to_sample::<f32>())
                    .sum::<f32>()
                    * scale
            });
            // Bluetooth headsets deliver exact zeros while they switch to their headset profile.
            buffer.append(mono.inspect(|sample| {
                if !heard && sample.abs() > 1e-6 {
                    heard = true;
                    signal.live.store(true, Ordering::Release);
                }
            }));
        },
        |error| flog!(filelog::AUDIO, "Stream error: {error}"),
        None,
    )
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Microphone {
    /// CoreAudio device UID, stable across reconnects.
    pub id: String,
    pub name: String,
}

pub fn microphones() -> Vec<Microphone> {
    let Ok(devices) = cpal::default_host().input_devices() else {
        return Vec::new();
    };
    devices
        .filter_map(|device| {
            Some(Microphone {
                id: device.id().ok()?.to_string(),
                name: device.description().ok()?.name().to_owned(),
            })
        })
        .collect()
}

fn input_device(microphone: Option<&str>) -> Option<cpal::Device> {
    let host = cpal::default_host();
    if let Some(id) = microphone {
        match id.parse().ok().and_then(|id| host.device_by_id(&id)) {
            Some(device) => return Some(device),
            None => flog!(
                filelog::AUDIO,
                "Microphone {id} not connected, using the system default"
            ),
        }
    }
    host.default_input_device()
}

fn device_error(what: &str, error: impl fmt::Display) -> StartError {
    flog!(filelog::AUDIO, "{what} failed: {error}");
    StartError::Device(error.to_string())
}

impl Recorder {
    pub fn new() -> Self {
        Self {
            stream: None,
            buffer: None,
            live_watch: None,
            device_rate: SAMPLE_RATE,
        }
    }

    /// Starts capture from `microphone` (a device id, `None` for the system default) with room
    /// for `max_duration` of audio; anything beyond is dropped.
    pub fn start(
        &mut self,
        microphone: Option<&str>,
        max_duration: Duration,
        on_live: OnLive,
    ) -> Result<(), StartError> {
        let permitted = mic_permission_granted();
        flog!(
            filelog::AUDIO,
            "start recording, mic permission={permitted}"
        );
        if !permitted {
            return Err(StartError::PermissionDenied);
        }
        if self.stream.take().is_some() {
            flog!(filelog::AUDIO, "Stopping previous stream before restart");
        }
        self.live_watch = None;

        let device = input_device(microphone).ok_or(StartError::NoMicrophone)?;
        let supported = device
            .default_input_config()
            .map_err(|error| device_error("Reading the microphone format", error))?;
        let config = supported.config();
        flog!(
            filelog::AUDIO,
            "Input device: {}, {} Hz, {} ch, {:?}",
            device
                .description()
                .map(|description| description.to_string())
                .unwrap_or_default(),
            config.sample_rate,
            config.channels,
            supported.sample_format()
        );
        let seconds = usize::try_from(max_duration.as_secs()).unwrap_or(usize::MAX);
        let capacity = seconds.saturating_mul(config.sample_rate as usize);
        let buffer = Arc::new(SampleBuffer::with_capacity(capacity));
        let shared = Arc::clone(&buffer);
        let live_watch =
            watch_live(on_live).map_err(|error| device_error("Watching the microphone", error))?;
        let signal = live_watch.signal();
        let stream = match supported.sample_format() {
            SampleFormat::F32 => build_stream::<f32>(&device, config, shared, signal),
            SampleFormat::I16 => build_stream::<i16>(&device, config, shared, signal),
            SampleFormat::I32 => build_stream::<i32>(&device, config, shared, signal),
            SampleFormat::F64 => build_stream::<f64>(&device, config, shared, signal),
            format => {
                return Err(device_error(
                    "Opening the microphone",
                    format!("unsupported sample format {format:?}"),
                ));
            }
        }
        .map_err(|error| device_error("Opening the microphone", error))?;
        stream
            .play()
            .map_err(|error| device_error("Starting the microphone", error))?;
        self.device_rate = config.sample_rate;
        self.buffer = Some(buffer);
        self.live_watch = Some(live_watch);
        self.stream = Some(stream);
        flog!(filelog::AUDIO, "Recording started");
        Ok(())
    }

    /// RMS of everything captured so far, for the silent-microphone warning.
    pub fn level(&self) -> Option<f32> {
        let buffer = self.buffer.as_ref()?;
        let count = buffer.len();
        (count > 0).then(|| {
            let sum = buffer
                .samples()
                .fold(0.0f32, |sum, sample| sum + sample * sample);
            (sum / count as f32).sqrt()
        })
    }

    /// Stops capture; the recording is still at the device's sample rate.
    pub fn stop(&mut self) -> Captured {
        // SAFETY: class properties without arguments or preconditions, readable from any thread;
        // they report Control Center's Mic Mode for this app.
        let (active_mode, chosen_mode) = unsafe {
            (
                AVCaptureDevice::activeMicrophoneMode(),
                AVCaptureDevice::preferredMicrophoneMode(),
            )
        };
        drop(self.stream.take());
        self.live_watch = None;
        Captured {
            buffer: self.buffer.take(),
            sample_rate: self.device_rate,
            mic_modes: (mode_name(active_mode), mode_name(chosen_mode)),
        }
    }
}

/// A finished recording; reading and resampling it happens off the main thread.
pub struct Captured {
    buffer: Option<Arc<SampleBuffer>>,
    sample_rate: u32,
    /// Active and chosen Mic Mode, for the log.
    mic_modes: (&'static str, &'static str),
}

impl Captured {
    /// 16 kHz mono, as every transcription backend expects.
    pub fn into_16k(self) -> Result<Vec<f32>> {
        let samples: Vec<f32> = self
            .buffer
            .as_ref()
            .map(|buffer| buffer.samples().collect())
            .unwrap_or_default();
        if self
            .buffer
            .as_ref()
            .is_some_and(|buffer| buffer.overflowed.load(Ordering::Relaxed))
        {
            flog!(
                filelog::AUDIO,
                "Recording exceeded its buffer; later audio was dropped"
            );
        }
        let (active, chosen) = self.mic_modes;
        flog!(
            filelog::AUDIO,
            "Recording stopped ({} samples, {:.1}s at {} Hz, RMS {:.4}, peak {:.3}, mic mode {active} (chosen: {chosen}))",
            samples.len(),
            samples.len() as f64 / f64::from(self.sample_rate),
            self.sample_rate,
            rms(&samples),
            samples
                .iter()
                .fold(0.0f32, |peak, sample| peak.max(sample.abs())),
        );
        resample(&samples, self.sample_rate)
    }
}

impl Default for Recorder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc::{self, RecvTimeoutError};

    use super::*;

    #[test]
    fn microphones_have_ids_that_resolve_back_to_a_device() {
        let microphones = microphones();
        if microphones.is_empty() {
            eprintln!("skipped: this machine has no microphone");
            return;
        }
        for microphone in microphones {
            assert!(!microphone.name.is_empty());
            assert!(
                input_device(Some(&microphone.id)).is_some(),
                "{}",
                microphone.id
            );
        }
    }

    #[test]
    fn sample_buffer_keeps_what_fits_and_flags_the_rest() {
        let buffer = SampleBuffer::with_capacity(5);
        buffer.append([0.1, -0.2, 0.3].into_iter());
        assert_eq!(buffer.samples().collect::<Vec<_>>(), [0.1, -0.2, 0.3]);
        assert!(!buffer.overflowed.load(Ordering::Relaxed));
        buffer.append([0.4, 0.5, 0.6].into_iter());
        assert_eq!(buffer.samples().count(), 5);
        assert!(buffer.overflowed.load(Ordering::Relaxed));
    }

    #[test]
    fn sample_buffer_is_read_while_the_audio_thread_writes() {
        let buffer = Arc::new(SampleBuffer::with_capacity(100_000));
        let writer = Arc::clone(&buffer);
        let thread = std::thread::spawn(move || {
            for chunk in 0..100u32 {
                writer.append((0..1_000).map(|i| (chunk * 1_000 + i) as f32));
            }
        });
        while buffer.len() < 100_000 {
            let seen: Vec<f32> = buffer.samples().collect();
            assert!(seen.iter().enumerate().all(|(i, &s)| s == i as f32));
        }
        thread.join().unwrap();
    }

    #[test]
    fn live_callback_runs_on_the_watcher_once_sound_arrives() {
        let (sender, receiver) = mpsc::channel();
        let watch = watch_live(Box::new(move || {
            let thread = std::thread::current().name().map(str::to_owned);
            sender.send(thread).unwrap();
        }))
        .unwrap();
        assert_eq!(
            receiver.recv_timeout(LIVE_POLL_INTERVAL * 5),
            Err(RecvTimeoutError::Timeout)
        );
        watch.signal().live.store(true, Ordering::Release);
        let thread = receiver.recv_timeout(Duration::from_secs(5)).unwrap();
        assert_eq!(thread.as_deref(), Some("microphone-live"));
        assert_eq!(
            receiver.recv_timeout(LIVE_POLL_INTERVAL * 5),
            Err(RecvTimeoutError::Disconnected),
            "called more than once"
        );
    }

    #[test]
    fn live_callback_is_skipped_when_the_recording_ends_first() {
        let (sender, receiver) = mpsc::channel();
        let watch = watch_live(Box::new(move || sender.send(()).unwrap())).unwrap();
        let signal = watch.signal();
        drop(watch);
        signal.live.store(true, Ordering::Release);
        // The watcher returns without calling back and drops the sender with the callback.
        assert_eq!(
            receiver.recv_timeout(Duration::from_secs(5)),
            Err(RecvTimeoutError::Disconnected)
        );
    }
}
