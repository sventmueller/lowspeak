//! AAC/M4A encoding with the system encoder, to cut upload size for the online APIs.

use std::ffi::c_void;
use std::mem::ManuallyDrop;
use std::ptr::NonNull;
use std::sync::atomic::{AtomicU64, Ordering};

use anyhow::{Context as _, Result, bail};
use objc2_audio_toolbox::{
    AudioConverterRef, AudioConverterSetProperty, AudioFileFlags, ExtAudioFileCreateWithURL,
    ExtAudioFileDispose, ExtAudioFileGetProperty, ExtAudioFileRef, ExtAudioFileSetProperty,
    ExtAudioFileWrite, kAudioConverterEncodeBitRate, kAudioFileM4AType,
    kExtAudioFileProperty_AudioConverter, kExtAudioFileProperty_ClientDataFormat,
    kExtAudioFileProperty_ConverterConfig,
};
use objc2_core_audio_types::{
    AudioBuffer, AudioBufferList, AudioStreamBasicDescription, kAudioFormatFlagIsPacked,
    kAudioFormatFlagIsSignedInteger, kAudioFormatLinearPCM, kAudioFormatMPEG4AAC,
};
use objc2_core_foundation::CFURL;

use super::mp4;
use super::wav::to_i16;
use crate::util::filelog;

/// Highest rate the system AAC encoder accepts for 16 kHz mono.
const BIT_RATE: u32 = 48_000;
const CHUNK_FRAMES: usize = 4_096;

/// Byte size of a property value, as AudioToolbox takes it.
fn property_size<T>() -> u32 {
    u32::try_from(size_of::<T>()).expect("property values are a few bytes")
}

fn check(status: i32, what: &str) -> Result<()> {
    if status != 0 {
        bail!("{what} failed (OSStatus {status})");
    }
    Ok(())
}

struct ExtAudioFile(ExtAudioFileRef);

impl ExtAudioFile {
    /// Disposing writes the rest of the M4A (its `moov` atom); a failure leaves the file unplayable.
    fn close(self) -> Result<()> {
        let file = ManuallyDrop::new(self);
        // SAFETY: the reference came from ExtAudioFileCreateWithURL or ExtAudioFileOpenURL and,
        // with `Drop` suppressed, is disposed exactly once.
        check(
            unsafe { ExtAudioFileDispose(file.0) },
            "finishing the M4A file",
        )
    }
}

/// Only error paths drop an open file, and they discard it, so the status does not matter here.
impl Drop for ExtAudioFile {
    fn drop(&mut self) {
        // SAFETY: the reference came from ExtAudioFileCreateWithURL or ExtAudioFileOpenURL and is
        // disposed once.
        unsafe { ExtAudioFileDispose(self.0) };
    }
}

fn encode_to(path: &std::path::Path, samples: &[f32], sample_rate: f64) -> Result<()> {
    let url = CFURL::from_file_path(path).context("invalid temporary path")?;
    let mut file_format = AudioStreamBasicDescription {
        mSampleRate: sample_rate,
        mFormatID: kAudioFormatMPEG4AAC,
        mFormatFlags: 0,
        mBytesPerPacket: 0,
        mFramesPerPacket: 1024,
        mBytesPerFrame: 0,
        mChannelsPerFrame: 1,
        mBitsPerChannel: 0,
        mReserved: 0,
    };
    let mut client_format = AudioStreamBasicDescription {
        mSampleRate: sample_rate,
        mFormatID: kAudioFormatLinearPCM,
        mFormatFlags: kAudioFormatFlagIsSignedInteger | kAudioFormatFlagIsPacked,
        mBytesPerPacket: 2,
        mFramesPerPacket: 1,
        mBytesPerFrame: 2,
        mChannelsPerFrame: 1,
        mBitsPerChannel: 16,
        mReserved: 0,
    };

    let mut raw: ExtAudioFileRef = std::ptr::null_mut();
    check(
        // SAFETY: all pointers refer to live locals for the duration of the call.
        unsafe {
            ExtAudioFileCreateWithURL(
                &url,
                kAudioFileM4AType,
                NonNull::from(&mut file_format),
                std::ptr::null(),
                AudioFileFlags::EraseFile.0,
                NonNull::from(&mut raw),
            )
        },
        "creating the M4A file",
    )?;
    let file = ExtAudioFile(raw);

    check(
        // SAFETY: the property value is a correctly sized AudioStreamBasicDescription.
        unsafe {
            ExtAudioFileSetProperty(
                file.0,
                kExtAudioFileProperty_ClientDataFormat,
                property_size::<AudioStreamBasicDescription>(),
                NonNull::from(&mut client_format).cast(),
            )
        },
        "setting the client format",
    )?;

    let mut converter: AudioConverterRef = std::ptr::null_mut();
    let mut size = property_size::<AudioConverterRef>();
    // SAFETY: reading the converter reference into a correctly sized local.
    let status = unsafe {
        ExtAudioFileGetProperty(
            file.0,
            kExtAudioFileProperty_AudioConverter,
            NonNull::from(&mut size),
            NonNull::from(&mut converter).cast(),
        )
    };
    if status != 0 || converter.is_null() {
        flog!(
            filelog::AUDIO,
            "AAC converter unavailable (OSStatus {status}), encoding at the default bit rate"
        );
    } else {
        let mut bit_rate = BIT_RATE;
        // SAFETY: the converter belongs to `file`; the property value is a u32.
        let status = unsafe {
            AudioConverterSetProperty(
                converter,
                kAudioConverterEncodeBitRate,
                property_size::<u32>(),
                NonNull::from(&mut bit_rate).cast(),
            )
        };
        if status == 0 {
            // Changing the converter only takes effect once the file re-reads its configuration.
            let mut config: *const c_void = std::ptr::null();
            // SAFETY: a null configuration tells ExtAudioFile to apply the converter's settings.
            let status = unsafe {
                ExtAudioFileSetProperty(
                    file.0,
                    kExtAudioFileProperty_ConverterConfig,
                    property_size::<*const c_void>(),
                    NonNull::from(&mut config).cast(),
                )
            };
            if status != 0 {
                flog!(
                    filelog::AUDIO,
                    "Applying the AAC bit rate failed (OSStatus {status})"
                );
            }
        } else {
            flog!(
                filelog::AUDIO,
                "Setting the AAC bit rate failed (OSStatus {status})"
            );
        }
    }

    for chunk in samples.chunks(CHUNK_FRAMES) {
        let mut pcm: Vec<i16> = chunk.iter().map(|&sample| to_i16(sample)).collect();
        let frames = u32::try_from(pcm.len())?;
        let mut buffers = AudioBufferList {
            mNumberBuffers: 1,
            mBuffers: [AudioBuffer {
                mNumberChannels: 1,
                mDataByteSize: frames * property_size::<i16>(),
                mData: pcm.as_mut_ptr().cast(),
            }],
        };
        check(
            // SAFETY: the buffer list points at `pcm`, which outlives the call.
            unsafe { ExtAudioFileWrite(file.0, frames, NonNull::from(&mut buffers)) },
            "writing audio",
        )?;
    }
    file.close()
}

/// Fails rather than return a file that may not play; the caller then sends WAV.
pub fn compress_to_m4a(samples: &[f32], sample_rate: u32) -> Result<Vec<u8>> {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let path = std::env::temp_dir().join(format!(
        "lowspeak-{}-{}.m4a",
        std::process::id(),
        COUNTER.fetch_add(1, Ordering::Relaxed)
    ));
    let result =
        encode_to(&path, samples, f64::from(sample_rate)).and_then(|()| Ok(std::fs::read(&path)?));
    let _ = std::fs::remove_file(&path);
    let raw = result?;
    // The reserved space is ~23 KB, which costs noticeable upload time on slow links. A file whose
    // atoms cannot be walked is damaged, so it is not sent with its padding either.
    let data = mp4::without_padding(&raw).context("checking the M4A file")?;
    let wav_size = samples.len() * 2;
    if wav_size > 0 {
        let reduction = (1.0 - data.len() as f64 / wav_size as f64) * 100.0;
        flog!(
            filelog::AUDIO,
            "Compressed: {} KB → {} KB ({reduction:.0}% smaller)",
            wav_size / 1024,
            data.len() / 1024
        );
    }
    Ok(data)
}

/// The first encode loads the codec; doing it at launch keeps it off the first dictation.
pub fn warm_up() {
    std::thread::spawn(|| {
        let _ = compress_to_m4a(&[0.1; 1_600], super::SAMPLE_RATE);
    });
}

#[cfg(test)]
mod tests {
    use objc2_audio_toolbox::{ExtAudioFileOpenURL, kExtAudioFileProperty_FileLengthFrames};

    use super::*;

    fn frames_in(data: &[u8]) -> i64 {
        let path =
            std::env::temp_dir().join(format!("lowspeak-aac-test-{}.m4a", std::process::id()));
        std::fs::write(&path, data).unwrap();
        let url = CFURL::from_file_path(&path).unwrap();
        let mut raw: ExtAudioFileRef = std::ptr::null_mut();
        check(
            // SAFETY: opening a file we just wrote; the reference is disposed by `ExtAudioFile`.
            unsafe { ExtAudioFileOpenURL(&url, NonNull::from(&mut raw)) },
            "opening",
        )
        .unwrap();
        let file = ExtAudioFile(raw);
        let mut frames = 0i64;
        let mut size = property_size::<i64>();
        check(
            // SAFETY: reading an i64 property into a correctly sized local.
            unsafe {
                ExtAudioFileGetProperty(
                    file.0,
                    kExtAudioFileProperty_FileLengthFrames,
                    NonNull::from(&mut size),
                    NonNull::from(&mut frames).cast(),
                )
            },
            "reading the length",
        )
        .unwrap();
        file.close().unwrap();
        let _ = std::fs::remove_file(path);
        frames
    }

    #[test]
    fn produces_a_compact_playable_mp4() {
        let samples: Vec<f32> = (0..32_000).map(|i| 0.3 * (i as f32 * 0.07).sin()).collect();
        let data = compress_to_m4a(&samples, crate::audio::SAMPLE_RATE).unwrap();
        assert_eq!(&data[4..8], b"ftyp");
        assert!(
            !data.windows(4).any(|window| window == b"free"),
            "padding left in the file"
        );
        assert!(data.len() < 20_000, "{} bytes for 2 s", data.len());
        assert_eq!(frames_in(&data), i64::try_from(samples.len()).unwrap());
    }
}
