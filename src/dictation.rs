//! Dictation for a host app other than Lowspeak: it records into one of the host's text fields, transcribes as Lowspeak
//! would, with Lowspeak's models and choices, and hands the text back for the host to insert. The host keeps its own
//! API keys, since reading Lowspeak's keychain items makes macOS ask each time, and its own log directory.
//! `Lowspeak.app` does not use this module.

use std::collections::{HashMap, VecDeque};
use std::path::PathBuf;
use std::sync::mpsc::{self, RecvTimeoutError, Sender};
use std::sync::{Arc, Mutex, MutexGuard, PoisonError, Weak};
use std::time::Duration;

use objc2::AllocAnyThread as _;
use objc2_foundation::{NSString, NSUserDefaults};

use crate::app::capture::Capture;
use crate::app::policy::MAX_HANDS_FREE_RECORDING;
use crate::audio::recorder::{OnLive, StartError};
use crate::config::api_keys::{self, KeyStore};
use crate::config::settings::{self, ChosenMicrophone, Settings};
use crate::config::vocabulary;
use crate::i18n::Texts;
use crate::models::{ApiProvider, PostProcessingMode, TranscriptionMode};
use crate::transcription::error::TranscriptionError;
use crate::transcription::local::LocalTranscriber;
use crate::transcription::model_store::{Disk, ModelStore as _};
use crate::transcription::postprocess::ActiveApp;
use crate::transcription::service::{self, Job, Outcome, Services};
use crate::transcription::spelling::SpellChecker;
use crate::transcription::{Api, online};
use crate::util::retry::CancelToken;
use crate::util::url_session::UrlSession;
use crate::util::{filelog, paths};

pub use crate::i18n::texts_for_locale;

/// The defaults suite Lowspeak keeps its choices in; a host reads it and never writes it.
pub const LOWSPEAK_SUITE: &str = "com.lowspeak.app";

/// Where a dictation goes: the host's own number for one of its text fields.
pub type Target = u64;

/// What a host app provides.
pub struct Host {
    /// The API keys: the host's own keychain items, never Lowspeak's.
    pub keys: Box<dyn KeyStore + Send + Sync>,
    /// Where lowspeak's log files go instead of Lowspeak's folder.
    pub log_dir: Option<PathBuf>,
    /// The app the text goes into, for the formatting hint of post-processing.
    pub app: ActiveApp,
}

/// What happens to a dictation, in the order it happens; every event names the field it belongs to.
#[derive(Debug)]
pub enum Event {
    /// The microphone is open.
    Started(Target),
    /// The first real sound arrived: the dictation listens.
    Live(Target),
    /// The recording reached its longest length and is transcribed as it is.
    LimitReached(Target),
    /// The recording ended and is transcribed.
    Transcribing(Target),
    /// The text to insert, without the space Lowspeak adds after a paste.
    Text(Target, String),
    /// The dictation ended without text.
    Nothing(Target, Nothing),
    /// The recording or its transcription failed.
    Failed(Target, Failure),
}

/// Why a dictation brought no text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Nothing {
    TooShort,
    NoSpeech,
}

/// Why a dictation failed.
#[derive(Debug)]
pub enum Failure {
    Microphone(StartError),
    Transcription(TranscriptionError),
}

impl Failure {
    /// The reason in the language of `texts`, for the user; details go to the log.
    pub fn describe(&self, texts: &Texts) -> String {
        match self {
            Self::Microphone(error) => error.user_facing_description(texts),
            Self::Transcription(error) => error.user_facing_description(texts),
        }
    }
}

/// A recording runs already; there is one microphone.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Busy;

/// Lowspeak's choices for a dictation, read from its suite when the dictation starts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Choices {
    pub settings: Settings,
    pub microphone: Option<ChosenMicrophone>,
    pub vocabulary: Vec<String>,
}

impl Choices {
    /// Lowspeak's choices as its menu left them; the defaults where Lowspeak never ran.
    pub fn of_lowspeak() -> Self {
        let defaults = NSUserDefaults::initWithSuiteName(
            NSUserDefaults::alloc(),
            Some(&NSString::from_str(LOWSPEAK_SUITE)),
        )
        .unwrap_or_else(NSUserDefaults::standardUserDefaults);
        Self {
            settings: Settings::load_from(&defaults),
            microphone: settings::chosen_microphone_in(&defaults),
            vocabulary: vocabulary::load_from(&defaults),
        }
    }
}

/// Whether Lowspeak keeps an API key for `provider`; asked without reading the key, so macOS does not ask the user.
pub fn lowspeak_has_key(provider: ApiProvider) -> bool {
    api_keys::exists(provider)
}

/// Lowspeak's API key for `provider`. macOS asks the user before handing it out, so call it only once the user agreed
/// to take the key over into the host's own keychain item; an error is a refusal or a keychain failure.
pub fn read_lowspeak_key(provider: ApiProvider) -> anyhow::Result<Option<String>> {
    api_keys::read(provider)
}

/// The recording side of a dictation; `Microphone` is the real one, tests bring their own.
pub trait Recorder: Send + Sync + 'static {
    /// Opens `device`, the system default for `None`, and reports whether it opened.
    fn start(
        &self,
        device: Option<String>,
        on_live: OnLive,
        started: Box<dyn FnOnce(Result<(), StartError>) + Send>,
    );
    /// Ends the recording and hands over a way to its 16 kHz samples, which takes a while and runs on the
    /// dictation's own thread.
    fn stop(&self, stopped: Box<dyn FnOnce(Samples) + Send>);
}

/// A finished recording's samples at 16 kHz, once resampled.
pub type Samples = Box<dyn FnOnce() -> anyhow::Result<Vec<f32>> + Send>;

/// The transcription side; `Pipeline` is Lowspeak's, tests bring their own.
pub trait Transcriber: Send + Sync + 'static {
    /// Blocks until the text is there; runs on a thread of the dictation's own.
    fn transcribe(
        &self,
        samples: Vec<f32>,
        choices: &Choices,
    ) -> Result<Outcome, TranscriptionError>;
    /// Loads the local model of `choices` while the user speaks, so the transcription does not wait for it.
    fn prepare(&self, _choices: &Choices) {}
    /// Releases what it holds within `timeout`; whether it let go.
    fn shutdown(&self, _timeout: Duration) -> bool {
        true
    }
}

/// The microphone on Lowspeak's recorder thread.
pub struct Microphone {
    capture: Capture,
}

impl Microphone {
    pub fn new() -> Self {
        Self {
            capture: Capture::spawn(MAX_HANDS_FREE_RECORDING + Duration::from_secs(10)),
        }
    }
}

impl Default for Microphone {
    fn default() -> Self {
        Self::new()
    }
}

impl Recorder for Microphone {
    fn start(
        &self,
        device: Option<String>,
        on_live: OnLive,
        started: Box<dyn FnOnce(Result<(), StartError>) + Send>,
    ) {
        self.capture.start(device, on_live, started);
    }

    fn stop(&self, stopped: Box<dyn FnOnce(Samples) + Send>) {
        self.capture
            .stop(move |captured| stopped(Box::new(move || captured.into_16k())));
    }
}

/// Lowspeak's pipeline with the host's keys: local models on their own thread, online providers through the shared
/// URLSession.
pub struct Pipeline {
    local: LocalTranscriber,
    keys: Box<dyn KeyStore + Send + Sync>,
    app: ActiveApp,
    cancel: CancelToken,
}

impl Pipeline {
    pub fn new(keys: Box<dyn KeyStore + Send + Sync>, app: ActiveApp) -> Self {
        Self {
            local: LocalTranscriber::spawn(),
            keys,
            app,
            cancel: CancelToken::new(),
        }
    }
}

impl Transcriber for Pipeline {
    fn transcribe(
        &self,
        samples: Vec<f32>,
        choices: &Choices,
    ) -> Result<Outcome, TranscriptionError> {
        let services = Services {
            local: &self.local,
            models: &Disk,
            api: Api {
                http: &UrlSession,
                keys: self.keys.as_ref(),
                cancel: &self.cancel,
                retry_delay: online::RETRY_DELAY,
            },
            lexicon: &SpellChecker,
        };
        let settings = choices.settings;
        service::run(
            Job {
                samples: Arc::new(samples),
                settings,
                active_app: self.app.clone(),
                raw_parakeet_first: crate::app::policy::raw_parakeet_first(&settings),
                vocabulary: choices.vocabulary.clone(),
            },
            &services,
        )
    }

    fn prepare(&self, choices: &Choices) {
        let model = choices.settings.local_model;
        if choices.settings.mode == TranscriptionMode::Local && Disk.installed().contains(&model) {
            self.local.preload(model);
        }
    }

    fn shutdown(&self, timeout: Duration) -> bool {
        self.cancel.cancel();
        self.local.unload_within(timeout)
    }
}

/// The parts of a dictation; tests replace each one.
pub struct Parts {
    pub recorder: Box<dyn Recorder>,
    pub transcriber: Box<dyn Transcriber>,
    /// Reads Lowspeak's choices at the start of each dictation.
    pub choices: Box<dyn Fn() -> Choices + Send + Sync>,
    /// Where the keys come from, for `missing_key`.
    pub keys: Box<dyn KeyStore + Send + Sync>,
    /// The longest recording.
    pub limit: Duration,
}

type Sink = Arc<dyn Fn(Event) + Send + Sync>;

/// One host's dictations: one recording at a time, transcriptions in the order they were recorded.
pub struct Dictation {
    inner: Arc<Inner>,
}

struct Inner {
    recorder: Box<dyn Recorder>,
    transcriber: Arc<dyn Transcriber>,
    choices: Box<dyn Fn() -> Choices + Send + Sync>,
    keys: Box<dyn KeyStore + Send + Sync>,
    limit: Duration,
    events: Sink,
    state: Mutex<State>,
}

/// The recording that runs.
struct Running {
    target: Target,
    number: u64,
    choices: Choices,
    /// The microphone has opened; until then a stop or cancel waits for it.
    open: bool,
    stop_asked: bool,
    cancel_asked: bool,
    /// Sound arrived before the microphone reported that it opened; `Live` follows `Started`.
    heard: bool,
    /// Dropped with the recording, which wakes its limit watcher.
    limit: Option<Sender<()>>,
}

#[derive(Default)]
struct State {
    recording: Option<Running>,
    next: u64,
    /// The dictations being transcribed, oldest first, with the field each belongs to; a cancelled one leaves it.
    queue: VecDeque<(u64, Target)>,
    /// The finished ones waiting for those before them.
    done: HashMap<u64, Event>,
}

impl Dictation {
    /// Dictation with Lowspeak's recorder and pipeline, its logs moved to `host.log_dir`; `events` is called on
    /// background threads.
    pub fn new(host: Host, events: impl Fn(Event) + Send + Sync + 'static) -> Self {
        if let Some(dir) = &host.log_dir {
            paths::use_log_dir(dir.clone());
        }
        let keys = KeyCopy(Arc::new(host.keys));
        let parts = Parts {
            recorder: Box::new(Microphone::new()),
            transcriber: Box::new(Pipeline::new(Box::new(keys.clone()), host.app)),
            choices: Box::new(Choices::of_lowspeak),
            keys: Box::new(keys),
            limit: MAX_HANDS_FREE_RECORDING,
        };
        Self::with_parts(parts, events)
    }

    /// Dictation with other parts, for tests.
    pub fn with_parts(parts: Parts, events: impl Fn(Event) + Send + Sync + 'static) -> Self {
        Self {
            inner: Arc::new(Inner {
                recorder: parts.recorder,
                transcriber: Arc::from(parts.transcriber),
                choices: parts.choices,
                keys: parts.keys,
                limit: parts.limit,
                events: Arc::new(events),
                state: Mutex::new(State::default()),
            }),
        }
    }

    /// Lowspeak's choices as they are now.
    pub fn choices(&self) -> Choices {
        (self.inner.choices)()
    }

    /// The provider whose key the next dictation needs and the host does not have: the online model's, or with
    /// post-processing on and no key for it, the provider post-processing prefers.
    pub fn missing_key(&self) -> Option<ApiProvider> {
        let settings = self.choices().settings;
        let keys = &self.inner.keys;
        let online = settings.online_model.provider();
        if settings.mode == TranscriptionMode::Online && keys.load(online).is_none() {
            return Some(online);
        }
        let preferred = ApiProvider::POST_PROCESSING_PREFERENCE;
        (settings.post_processing != PostProcessingMode::Off
            && preferred
                .iter()
                .all(|&provider| keys.load(provider).is_none()))
        .then_some(preferred[0])
    }

    /// Opens the microphone for `target`; refused while another recording runs.
    pub fn start(&self, target: Target) -> Result<(), Busy> {
        let choices = self.choices();
        let number = {
            let mut state = self.inner.lock();
            if state.recording.is_some() {
                return Err(Busy);
            }
            state.next += 1;
            let number = state.next;
            state.recording = Some(Running {
                target,
                number,
                choices: choices.clone(),
                open: false,
                stop_asked: false,
                cancel_asked: false,
                heard: false,
                limit: None,
            });
            number
        };
        flog!(filelog::APP, "Dictation {number} starts");
        self.inner.transcriber.prepare(&choices);
        let live = Arc::downgrade(&self.inner);
        let on_live: OnLive = Box::new(move || {
            if let Some(inner) = live.upgrade() {
                Inner::heard(&inner, number, target);
            }
        });
        let started = Arc::downgrade(&self.inner);
        self.inner.recorder.start(
            choices.microphone.map(|m| m.id),
            on_live,
            Box::new(move |result| {
                if let Some(inner) = started.upgrade() {
                    Inner::opened(&inner, number, result);
                }
            }),
        );
        Ok(())
    }

    /// Ends the recording of `target` and transcribes it.
    pub fn stop(&self, target: Target) {
        Inner::stop(&self.inner, target, None);
    }

    /// Ends the recording of `target` without transcribing it, and drops a transcription of it that still runs.
    pub fn cancel(&self, target: Target) {
        let stop_now = {
            let mut state = self.inner.lock();
            state.queue.retain(|(_, t)| *t != target);
            let queued: Vec<u64> = state.queue.iter().map(|(n, _)| *n).collect();
            state.done.retain(|n, _| queued.contains(n));
            match state.recording.as_mut() {
                Some(running) if running.target == target && running.open => {
                    state.recording = None;
                    true
                }
                Some(running) if running.target == target => {
                    running.cancel_asked = true;
                    false
                }
                _ => false,
            }
        };
        if stop_now {
            self.inner.recorder.stop(Box::new(drop));
        }
        Inner::hand_out(&self.inner);
    }

    /// Whether a recording runs, and for which field.
    pub fn recording(&self) -> Option<Target> {
        self.inner.lock().recording.as_ref().map(|r| r.target)
    }

    /// Releases the loaded model within `timeout` and ends the retries of online requests; call it when the host
    /// quits, since whisper.cpp fails in its exit-time destructors while a model is loaded.
    pub fn shutdown(&self, timeout: Duration) -> bool {
        self.inner.transcriber.shutdown(timeout)
    }
}

impl Inner {
    fn lock(&self) -> MutexGuard<'_, State> {
        self.state.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// The first real sound of dictation `number`.
    fn heard(inner: &Arc<Inner>, number: u64, target: Target) {
        let live = {
            let mut state = inner.lock();
            match state.recording.as_mut().filter(|r| r.number == number) {
                Some(running) if running.open => true,
                Some(running) => {
                    running.heard = true;
                    false
                }
                None => false,
            }
        };
        if live {
            (inner.events)(Event::Live(target));
        }
    }

    /// The microphone of dictation `number` opened, or failed to.
    fn opened(inner: &Arc<Inner>, number: u64, result: Result<(), StartError>) {
        let (target, stop, cancel, heard) = {
            let mut state = inner.lock();
            let Some(running) = state.recording.as_mut().filter(|r| r.number == number) else {
                return;
            };
            let target = running.target;
            if let Err(error) = result {
                let cancelled = running.cancel_asked;
                state.recording = None;
                drop(state);
                flog!(
                    filelog::APP,
                    "Dictation {number}: the microphone failed: {error}"
                );
                if !cancelled {
                    (inner.events)(Event::Failed(target, Failure::Microphone(error)));
                }
                return;
            }
            running.open = true;
            let (stop, cancel, heard) = (running.stop_asked, running.cancel_asked, running.heard);
            if cancel {
                state.recording = None;
            } else if !stop {
                running.limit = Some(Inner::watch_limit(inner, number, target));
            }
            (target, stop, cancel, heard)
        };
        if cancel {
            inner.recorder.stop(Box::new(drop));
            return;
        }
        (inner.events)(Event::Started(target));
        if stop {
            Inner::stop(inner, target, None);
        } else if heard {
            Inner::heard(inner, number, target);
        }
    }

    /// Stops dictation `number` once it reaches the limit; the watcher ends early when the returned sender is
    /// dropped with the recording.
    fn watch_limit(inner: &Arc<Inner>, number: u64, target: Target) -> Sender<()> {
        let (sender, ended) = mpsc::channel::<()>();
        let weak: Weak<Inner> = Arc::downgrade(inner);
        let limit = inner.limit;
        let spawned = std::thread::Builder::new()
            .name("lowspeak.dictation-limit".into())
            .spawn(move || {
                if ended.recv_timeout(limit) != Err(RecvTimeoutError::Timeout) {
                    return;
                }
                if let Some(inner) = weak.upgrade() {
                    flog!(filelog::APP, "Dictation {number} reached the limit");
                    Inner::stop(&inner, target, Some(number));
                }
            });
        if let Err(error) = spawned {
            flog!(filelog::APP, "Watching the dictation limit: {error}");
        }
        sender
    }

    /// Ends the recording of `target`; `limit` names the dictation that reached its limit.
    fn stop(inner: &Arc<Inner>, target: Target, limit: Option<u64>) {
        let (number, choices) = {
            let mut state = inner.lock();
            let Some(running) = state
                .recording
                .as_mut()
                .filter(|r| r.target == target && limit.is_none_or(|n| n == r.number))
            else {
                return;
            };
            if !running.open {
                running.stop_asked = true;
                return;
            }
            let Some(running) = state.recording.take() else {
                return;
            };
            state.queue.push_back((running.number, target));
            (running.number, running.choices)
        };
        if limit.is_some() {
            (inner.events)(Event::LimitReached(target));
        }
        (inner.events)(Event::Transcribing(target));
        let weak = Arc::downgrade(inner);
        inner.recorder.stop(Box::new(move |samples| {
            let Some(inner) = weak.upgrade() else { return };
            let transcriber = Arc::clone(&inner.transcriber);
            drop(inner);
            let weak = weak.clone();
            let spawned = std::thread::Builder::new()
                .name("lowspeak.dictation".into())
                .spawn(move || {
                    let event = transcribe(transcriber.as_ref(), samples, &choices, target);
                    if let Some(inner) = weak.upgrade() {
                        {
                            let mut state = inner.lock();
                            if state.queue.iter().any(|(n, _)| *n == number) {
                                state.done.insert(number, event);
                            }
                        }
                        Inner::hand_out(&inner);
                    }
                });
            if let Err(error) = spawned {
                flog!(
                    filelog::APP,
                    "Starting the transcription of dictation {number}: {error}"
                );
            }
        }));
    }

    /// Hands out the finished dictations whose turn it is, in the order they were recorded.
    fn hand_out(inner: &Arc<Inner>) {
        let ready = {
            let mut state = inner.lock();
            let mut ready = Vec::new();
            while let Some(&(number, _)) = state.queue.front() {
                let Some(event) = state.done.remove(&number) else {
                    break;
                };
                state.queue.pop_front();
                ready.push(event);
            }
            ready
        };
        for event in ready {
            (inner.events)(event);
        }
    }
}

/// Transcribes one recording; a panic of the engine becomes a failure, as in the app.
fn transcribe(
    transcriber: &dyn Transcriber,
    samples: Samples,
    choices: &Choices,
    target: Target,
) -> Event {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let samples = samples().map_err(TranscriptionError::Audio)?;
        transcriber.transcribe(samples, choices)
    }))
    .unwrap_or_else(|_| {
        Err(TranscriptionError::Engine(anyhow::anyhow!(
            "the transcription panicked"
        )))
    });
    match result {
        Ok(Outcome::Text(transcript)) => Event::Text(target, transcript.text.trim_end().to_owned()),
        Ok(Outcome::NoSpeech) => Event::Nothing(target, Nothing::NoSpeech),
        Ok(Outcome::TooShort) => Event::Nothing(target, Nothing::TooShort),
        Err(error) => {
            flog!(filelog::APP, "Dictation failed: {error}");
            Event::Failed(target, Failure::Transcription(error))
        }
    }
}

/// One key store shared by the pipeline and `missing_key`.
#[derive(Clone)]
struct KeyCopy(Arc<Box<dyn KeyStore + Send + Sync>>);

impl KeyStore for KeyCopy {
    fn load(&self, provider: ApiProvider) -> Option<String> {
        self.0.load(provider)
    }
}
