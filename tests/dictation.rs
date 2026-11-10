//! `dictation::Dictation` with a recorder and a transcriber in place of the microphone and the models: the order of
//! events and results, cancelling, the one-recording rule, failures and the recording limit. No microphone, model,
//! key or network.

use std::sync::mpsc::{self, Receiver};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use lowspeak::audio::recorder::{OnLive, StartError};
use lowspeak::config::api_keys::KeyStore;
use lowspeak::config::settings::Settings;
use lowspeak::dictation::{
    Busy, Choices, Dictation, Event, Failure, Nothing, Parts, Recorder, Samples, Target,
    Transcriber, texts_for_locale,
};
use lowspeak::i18n::texts;
use lowspeak::models::{
    ApiProvider, Language, LocalModel, OnlineModel, PostProcessingMode, TranscriptionMode,
};
use lowspeak::transcription::error::TranscriptionError;
use lowspeak::transcription::postprocess::Status;
use lowspeak::transcription::service::{Outcome, Transcript};

const WAIT: Duration = Duration::from_secs(5);

type Started = Box<dyn FnOnce(Result<(), StartError>) + Send>;

/// A microphone that opens when the test says so and records what the test put in.
#[derive(Default)]
struct FakeMic {
    pending: Mutex<Option<(OnLive, Started)>>,
    /// The samples of the next stop: first the transcription's delay in ms, then the number of its text.
    next: Mutex<Vec<f32>>,
    stops: Mutex<usize>,
}

impl FakeMic {
    fn open(&self, result: Result<(), StartError>) {
        let (on_live, started) = self.pending.lock().unwrap().take().expect("a start waits");
        let ok = result.is_ok();
        started(result);
        if ok {
            on_live();
        }
    }

    /// Sound arrives before the microphone reports that it opened, as the recorder's watcher thread allows.
    fn open_heard_first(&self) {
        let (on_live, started) = self.pending.lock().unwrap().take().expect("a start waits");
        on_live();
        started(Ok(()));
    }

    fn record(&self, delay_ms: f32, number: f32) {
        *self.next.lock().unwrap() = vec![delay_ms, number];
    }
}

struct Mic(Arc<FakeMic>);

impl Recorder for Mic {
    fn start(&self, _device: Option<String>, on_live: OnLive, started: Started) {
        *self.0.pending.lock().unwrap() = Some((on_live, started));
    }

    fn stop(&self, stopped: Box<dyn FnOnce(Samples) + Send>) {
        *self.0.stops.lock().unwrap() += 1;
        let samples = std::mem::take(&mut *self.0.next.lock().unwrap());
        stopped(Box::new(move || Ok(samples)));
    }
}

/// Waits the delay the samples name and returns their number as text; an empty recording is too short.
struct FakeEngine;

impl Transcriber for FakeEngine {
    fn transcribe(
        &self,
        samples: Vec<f32>,
        _choices: &Choices,
    ) -> Result<Outcome, TranscriptionError> {
        let &[delay, number] = samples.as_slice() else {
            return Ok(Outcome::TooShort);
        };
        std::thread::sleep(Duration::from_secs_f32(delay / 1000.0));
        if number < 0.0 {
            return Err(TranscriptionError::ModelNotAvailable);
        }
        Ok(Outcome::Text(Transcript {
            text: format!("text {number} "),
            local_fallback: None,
            post_processing: Status::Skipped,
        }))
    }
}

struct Keys(Option<&'static str>);

impl KeyStore for Keys {
    fn load(&self, _provider: ApiProvider) -> Option<String> {
        self.0.map(str::to_owned)
    }
}

fn choices(mode: TranscriptionMode, post_processing: PostProcessingMode) -> Choices {
    Choices {
        settings: Settings {
            mode,
            local_model: LocalModel::Tiny,
            online_model: OnlineModel::GptTranscribe,
            language: Language::Auto,
            post_processing,
            preprocessing: true,
            start_sound: false,
        },
        microphone: None,
        vocabulary: Vec::new(),
    }
}

fn dictation(limit: Duration, keys: Keys) -> (Dictation, Arc<FakeMic>, Receiver<String>) {
    with_choices(limit, keys, || {
        choices(TranscriptionMode::Online, PostProcessingMode::Off)
    })
}

fn with_choices(
    limit: Duration,
    keys: Keys,
    choices: impl Fn() -> Choices + Send + Sync + 'static,
) -> (Dictation, Arc<FakeMic>, Receiver<String>) {
    let mic = Arc::new(FakeMic::default());
    let (sender, events) = mpsc::channel();
    let sender = Mutex::new(sender);
    let parts = Parts {
        recorder: Box::new(Mic(Arc::clone(&mic))),
        transcriber: Box::new(FakeEngine),
        choices: Box::new(choices),
        keys: Box::new(keys),
        limit,
    };
    let dictation = Dictation::with_parts(parts, move |event| {
        let _ = sender.lock().unwrap().send(describe(&event));
    });
    (dictation, mic, events)
}

fn describe(event: &Event) -> String {
    match event {
        Event::Started(t) => format!("started {t}"),
        Event::Live(t) => format!("live {t}"),
        Event::LimitReached(t) => format!("limit {t}"),
        Event::Transcribing(t) => format!("transcribing {t}"),
        Event::Text(t, text) => format!("text {t}: {text}"),
        Event::Nothing(t, Nothing::TooShort) => format!("too short {t}"),
        Event::Nothing(t, Nothing::NoSpeech) => format!("no speech {t}"),
        Event::Failed(t, Failure::Microphone(_)) => format!("microphone failed {t}"),
        Event::Failed(t, Failure::Transcription(_)) => format!("transcription failed {t}"),
    }
}

fn next(events: &Receiver<String>) -> String {
    events.recv_timeout(WAIT).expect("an event in time")
}

fn none_within(events: &Receiver<String>, wait: Duration) -> bool {
    events.recv_timeout(wait).is_err()
}

const A: Target = 1;
const B: Target = 2;

#[test]
fn a_dictation_reports_each_step_and_ends_with_its_text() {
    let (dictation, mic, events) = dictation(Duration::from_secs(60), Keys(None));
    dictation.start(A).unwrap();
    assert_eq!(dictation.recording(), Some(A));
    mic.open(Ok(()));
    assert_eq!(next(&events), "started 1");
    assert_eq!(next(&events), "live 1");
    mic.record(0.0, 7.0);
    dictation.stop(A);
    assert_eq!(next(&events), "transcribing 1");
    assert_eq!(
        next(&events),
        "text 1: text 7",
        "without the trailing space"
    );
    assert_eq!(dictation.recording(), None);
}

#[test]
fn one_recording_runs_at_a_time() {
    let (dictation, mic, events) = dictation(Duration::from_secs(60), Keys(None));
    dictation.start(A).unwrap();
    assert_eq!(dictation.start(B), Err(Busy));
    mic.open(Ok(()));
    let _ = (next(&events), next(&events));
    mic.record(0.0, 1.0);
    dictation.stop(A);
    let _ = next(&events);
    dictation.start(B).unwrap();
}

#[test]
fn texts_come_in_the_order_they_were_recorded() {
    let (dictation, mic, events) = dictation(Duration::from_secs(60), Keys(None));
    dictation.start(A).unwrap();
    mic.open(Ok(()));
    mic.record(400.0, 1.0);
    dictation.stop(A);
    dictation.start(B).unwrap();
    mic.open(Ok(()));
    mic.record(0.0, 2.0);
    dictation.stop(B);
    let mut texts: Vec<String> = Vec::new();
    while texts.len() < 2 {
        let event = next(&events);
        if event.starts_with("text") {
            texts.push(event);
        }
    }
    assert_eq!(texts, ["text 1: text 1", "text 2: text 2"]);
}

#[test]
fn cancelling_drops_the_recording_and_a_transcription_still_running() {
    let (dictation, mic, events) = dictation(Duration::from_secs(60), Keys(None));
    dictation.start(A).unwrap();
    mic.open(Ok(()));
    let _ = (next(&events), next(&events));
    dictation.cancel(A);
    assert_eq!(*mic.stops.lock().unwrap(), 1);
    assert!(
        none_within(&events, Duration::from_millis(200)),
        "a cancelled recording is not transcribed"
    );
    dictation.start(A).unwrap();
    mic.open(Ok(()));
    let _ = (next(&events), next(&events));
    mic.record(300.0, 3.0);
    dictation.stop(A);
    assert_eq!(next(&events), "transcribing 1");
    dictation.start(B).unwrap();
    mic.open(Ok(()));
    let _ = (next(&events), next(&events));
    mic.record(0.0, 4.0);
    dictation.stop(B);
    assert_eq!(next(&events), "transcribing 2");
    dictation.cancel(A);
    assert_eq!(
        next(&events),
        "text 2: text 4",
        "the later dictation does not wait for the dropped one"
    );
    assert!(none_within(&events, Duration::from_millis(500)));
}

#[test]
fn a_stop_or_cancel_before_the_microphone_opened_waits_for_it() {
    let (dictation, mic, events) = dictation(Duration::from_secs(60), Keys(None));
    dictation.start(A).unwrap();
    mic.record(0.0, 5.0);
    dictation.stop(A);
    mic.open(Ok(()));
    assert_eq!(next(&events), "started 1");
    assert_eq!(next(&events), "transcribing 1");
    assert_eq!(
        next(&events),
        "text 1: text 5",
        "no live event once the recording ended"
    );
    dictation.start(B).unwrap();
    dictation.cancel(B);
    mic.open(Ok(()));
    assert_eq!(
        *mic.stops.lock().unwrap(),
        2,
        "the microphone closes once it opened"
    );
    assert!(none_within(&events, Duration::from_millis(200)));
    assert_eq!(dictation.recording(), None);
}

#[test]
fn failures_end_the_dictation_and_say_why_in_the_interface_language() {
    let (dictation, mic, events) = dictation(Duration::from_secs(60), Keys(None));
    dictation.start(A).unwrap();
    mic.open(Err(StartError::PermissionDenied));
    assert_eq!(next(&events), "microphone failed 1");
    assert_eq!(dictation.recording(), None);
    dictation.start(A).unwrap();
    mic.open(Ok(()));
    let _ = (next(&events), next(&events));
    mic.record(0.0, -1.0);
    dictation.stop(A);
    let _ = next(&events);
    assert_eq!(next(&events), "transcription failed 1");
    dictation.start(A).unwrap();
    mic.open(Ok(()));
    let _ = (next(&events), next(&events));
    dictation.stop(A);
    let _ = next(&events);
    assert_eq!(next(&events), "too short 1");
    let failure = Failure::Microphone(StartError::PermissionDenied);
    assert_eq!(
        failure.describe(texts_for_locale("de-CH")),
        failure.describe(texts(Language::De))
    );
}

#[test]
fn a_recording_stops_at_its_limit_and_is_transcribed() {
    let (dictation, mic, events) = dictation(Duration::from_millis(200), Keys(None));
    dictation.start(A).unwrap();
    mic.record(0.0, 6.0);
    mic.open(Ok(()));
    let _ = (next(&events), next(&events));
    assert_eq!(next(&events), "limit 1");
    assert_eq!(next(&events), "transcribing 1");
    assert_eq!(next(&events), "text 1: text 6");
}

#[test]
fn an_online_dictation_without_the_hosts_key_names_the_provider() {
    let (without, _, _) = dictation(Duration::from_secs(60), Keys(None));
    assert_eq!(without.missing_key(), Some(ApiProvider::OpenAi));
    let (with, _, _) = dictation(Duration::from_secs(60), Keys(Some("sk-test")));
    assert_eq!(with.missing_key(), None);
}

#[test]
fn post_processing_without_any_of_its_keys_names_the_provider_it_prefers() {
    let local = |post_processing| move || choices(TranscriptionMode::Local, post_processing);
    let (off, _, _) = with_choices(WAIT, Keys(None), local(PostProcessingMode::Off));
    assert_eq!(
        off.missing_key(),
        None,
        "local mode without post-processing needs no key"
    );
    let (on, _, _) = with_choices(WAIT, Keys(None), local(PostProcessingMode::Format));
    assert_eq!(on.missing_key(), Some(ApiProvider::Groq));
    let (keyed, _, _) = with_choices(
        WAIT,
        Keys(Some("gsk_test")),
        local(PostProcessingMode::Format),
    );
    assert_eq!(keyed.missing_key(), None);
}

#[test]
fn sound_before_the_microphone_reported_open_is_announced_after_the_start() {
    let (dictation, mic, events) = dictation(Duration::from_secs(60), Keys(None));
    dictation.start(A).unwrap();
    mic.open_heard_first();
    assert_eq!(next(&events), "started 1");
    assert_eq!(next(&events), "live 1");
}

#[test]
fn a_microphone_failing_after_a_cancel_reports_nothing() {
    let (dictation, mic, events) = dictation(Duration::from_secs(60), Keys(None));
    dictation.start(A).unwrap();
    dictation.cancel(A);
    mic.open(Err(StartError::NoMicrophone));
    assert!(none_within(&events, Duration::from_millis(200)));
    assert_eq!(dictation.recording(), None);
}

#[test]
fn the_limit_of_an_ended_recording_does_not_stop_the_next_one() {
    let limit = Duration::from_millis(600);
    let (dictation, mic, events) = dictation(limit, Keys(None));
    dictation.start(A).unwrap();
    mic.open(Ok(()));
    mic.record(0.0, 1.0);
    dictation.stop(A);
    for _ in 0..4 {
        let _ = next(&events);
    }
    std::thread::sleep(Duration::from_millis(300));
    dictation.start(A).unwrap();
    mic.open(Ok(()));
    let _ = (next(&events), next(&events));
    assert!(
        none_within(&events, Duration::from_millis(450)),
        "the first recording's limit passed without effect"
    );
    assert_eq!(dictation.recording(), Some(A));
    assert_eq!(next(&events), "limit 1");
}

#[test]
fn texts_are_found_by_language_tag() {
    assert!(std::ptr::eq(texts_for_locale("de-CH"), texts(Language::De)));
    assert!(std::ptr::eq(
        texts_for_locale("zh-Hant-TW"),
        texts(Language::ZhHant)
    ));
    assert!(std::ptr::eq(texts_for_locale("nb"), texts(Language::No)));
    assert!(std::ptr::eq(texts_for_locale("tlh"), texts(Language::En)));
}
