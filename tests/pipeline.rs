//! The transcription pipeline with substitutes for the network, the key file, the installed models
//! and the local engine, so it runs without models, keys or a connection.

use std::cell::{Cell, RefCell};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;
use std::time::Duration;

use lowspeak::config::api_keys::KeyStore;
use lowspeak::config::settings::Settings;
use lowspeak::models::{
    ApiProvider, Language, LocalModel, OnlineModel, PostProcessingMode, TranscriptionMode,
};
use lowspeak::transcription::Api;
use lowspeak::transcription::error::TranscriptionError;
use lowspeak::transcription::local::LocalEngine;
use lowspeak::transcription::model_store::ModelStore;
use lowspeak::transcription::postprocess::{ActiveApp, Rejection, Status};
use lowspeak::transcription::service::{self, Job, Outcome, Services, Transcript};
use lowspeak::util::numeric::floor_usize;
use lowspeak::util::retry::CancelToken;
use lowspeak::util::url_session::{HttpClient, RequestError, Response};

struct Request {
    url: String,
    headers: Vec<(String, String)>,
    body: Vec<u8>,
}

impl Request {
    fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(key, _)| key == name)
            .map(|(_, value)| value.as_str())
    }

    fn body_contains(&self, part: &str) -> bool {
        self.body
            .windows(part.len())
            .any(|window| window == part.as_bytes())
    }
}

type Answer = Box<dyn Fn(&str) -> Result<Response, RequestError>>;

struct FakeHttp {
    answer: Answer,
    requests: RefCell<Vec<Request>>,
}

impl FakeHttp {
    fn new(answer: impl Fn(&str) -> Result<Response, RequestError> + 'static) -> Self {
        Self {
            answer: Box::new(answer),
            requests: RefCell::new(Vec::new()),
        }
    }

    /// Answers the requests in turn with `statuses`, the last one for every further request.
    fn in_turn(statuses: &'static [(u16, &'static str)]) -> Self {
        let next = Cell::new(0);
        Self::new(move |_| {
            let index = next.get().min(statuses.len() - 1);
            next.set(index + 1);
            let (status, body) = statuses[index];
            answer(status, body)
        })
    }

    fn urls(&self) -> Vec<String> {
        self.requests
            .borrow()
            .iter()
            .map(|request| request.url.clone())
            .collect()
    }
}

impl HttpClient for FakeHttp {
    fn post(
        &self,
        url: &str,
        headers: &[(&str, &str)],
        body: &[u8],
        _timeout: Duration,
    ) -> Result<Response, RequestError> {
        self.requests.borrow_mut().push(Request {
            url: url.to_owned(),
            headers: headers
                .iter()
                .map(|(name, value)| ((*name).to_owned(), (*value).to_owned()))
                .collect(),
            body: body.to_vec(),
        });
        (self.answer)(url)
    }
}

fn answer(status: u16, body: &str) -> Result<Response, RequestError> {
    Ok(Response {
        status: Some(status),
        body: body.as_bytes().to_vec(),
    })
}

fn chat_answer(content: &str) -> Result<Response, RequestError> {
    chat_answer_ending(content, "stop")
}

fn chat_answer_ending(content: &str, finish_reason: &str) -> Result<Response, RequestError> {
    answer(
        200,
        &serde_json::json!({
            "choices": [{"finish_reason": finish_reason, "message": {"content": content}}]
        })
        .to_string(),
    )
}

struct Keys(HashMap<ApiProvider, String>);

impl Keys {
    fn of(providers: &[ApiProvider]) -> Self {
        Self(
            providers
                .iter()
                .map(|&provider| {
                    let key = match provider {
                        ApiProvider::OpenAi => "sk-test",
                        ApiProvider::Groq => "gsk_test",
                    };
                    (provider, key.to_owned())
                })
                .collect(),
        )
    }
}

impl KeyStore for Keys {
    fn load(&self, provider: ApiProvider) -> Option<String> {
        self.0.get(&provider).cloned()
    }
}

struct Installed(HashSet<LocalModel>);

impl ModelStore for Installed {
    fn installed(&self) -> HashSet<LocalModel> {
        self.0.clone()
    }
}

struct LocalCall {
    samples: usize,
    model: LocalModel,
    prompt: Option<String>,
}

#[derive(Default)]
struct FakeLocal {
    answers: RefCell<VecDeque<Result<String, TranscriptionError>>>,
    calls: RefCell<Vec<LocalCall>>,
}

impl FakeLocal {
    fn answering(answers: Vec<Result<String, TranscriptionError>>) -> Self {
        Self {
            answers: RefCell::new(answers.into()),
            calls: RefCell::default(),
        }
    }
}

impl LocalEngine for FakeLocal {
    fn transcribe_shared(
        &self,
        samples: Arc<Vec<f32>>,
        model: LocalModel,
        _language: Option<&'static str>,
        prompt: Option<String>,
    ) -> Result<String, TranscriptionError> {
        self.calls.borrow_mut().push(LocalCall {
            samples: samples.len(),
            model,
            prompt,
        });
        self.answers
            .borrow_mut()
            .pop_front()
            .expect("no answer left for the local engine")
    }
}

/// A tone the voice activity detection takes for speech.
fn speech(seconds: f32) -> Vec<f32> {
    (0..floor_usize(f64::from(seconds * 16_000.0)))
        .map(|i| 0.12 * (2.0 * std::f32::consts::PI * 220.0 * i as f32 / 16_000.0).sin())
        .collect()
}

fn settings(mode: TranscriptionMode) -> Settings {
    Settings {
        mode,
        local_model: LocalModel::LargeV3Turbo,
        online_model: OnlineModel::GroqWhisperLargeV3Turbo,
        language: Language::De,
        post_processing: PostProcessingMode::Off,
        preprocessing: true,
        start_sound: false,
    }
}

fn job(settings: Settings, samples: Vec<f32>) -> Job {
    Job {
        samples,
        settings,
        active_app: ActiveApp::default(),
        raw_parakeet_first: false,
        vocabulary: Vec::new(),
    }
}

struct World {
    http: FakeHttp,
    keys: Keys,
    installed: Installed,
    local: FakeLocal,
    cancel: CancelToken,
}

impl World {
    fn new(
        http: FakeHttp,
        keys: &[ApiProvider],
        installed: &[LocalModel],
        local: FakeLocal,
    ) -> Self {
        Self {
            http,
            keys: Keys::of(keys),
            installed: Installed(installed.iter().copied().collect()),
            local,
            cancel: CancelToken::new(),
        }
    }

    fn run(&self, job: Job) -> Result<Outcome, TranscriptionError> {
        service::run(
            job,
            &Services {
                local: &self.local,
                models: &self.installed,
                api: Api {
                    http: &self.http,
                    keys: &self.keys,
                    cancel: &self.cancel,
                    retry_delay: Duration::ZERO,
                },
            },
        )
    }
}

fn text(outcome: Result<Outcome, TranscriptionError>) -> Transcript {
    match outcome {
        Ok(Outcome::Text(transcript)) => transcript,
        Ok(Outcome::NoSpeech) => panic!("no speech"),
        Ok(Outcome::TooShort) => panic!("too short"),
        Err(error) => panic!("failed: {error}"),
    }
}

fn no_requests() -> FakeHttp {
    FakeHttp::new(|url| panic!("unexpected request to {url}"))
}

#[test]
fn online_transcription_uploads_compressed_audio_with_language_and_vocabulary() {
    let world = World::new(
        FakeHttp::new(|_| answer(200, r#"{"text": " Hallo Welt "}"#)),
        &[ApiProvider::Groq],
        &[],
        FakeLocal::default(),
    );
    let mut job = job(settings(TranscriptionMode::Online), speech(1.5));
    job.vocabulary = vec!["Postgres".into(), "Traefik".into()];

    let transcript = text(world.run(job));

    assert_eq!(transcript.text, "Hallo Welt");
    assert!(transcript.local_fallback.is_none());
    assert_eq!(transcript.post_processing, Status::Skipped);
    let requests = world.http.requests.borrow();
    let [request] = &requests[..] else {
        panic!("expected one request, got {}", requests.len());
    };
    assert_eq!(request.url, ApiProvider::Groq.transcription_url());
    assert_eq!(request.header("Authorization"), Some("Bearer gsk_test"));
    assert!(request.body_contains("filename=\"audio.m4a\""));
    assert!(request.body_contains("name=\"language\"\r\n\r\nde\r\n"));
    assert!(request.body_contains("name=\"prompt\"\r\n\r\nPostgres, Traefik\r\n"));
    assert!(world.local.calls.borrow().is_empty());
}

#[test]
fn a_failed_online_request_is_tried_once_and_then_transcribed_locally() {
    let world = World::new(
        FakeHttp::new(|_| answer(503, "overloaded")),
        &[ApiProvider::Groq],
        &[LocalModel::Medium, LocalModel::ParakeetUltra],
        FakeLocal::answering(vec![Ok("lokal".into())]),
    );
    let mut job = job(settings(TranscriptionMode::Online), speech(1.5));
    job.vocabulary = vec!["Postgres".into()];

    let transcript = text(world.run(job));

    assert_eq!(transcript.text, "lokal");
    assert!(matches!(
        transcript.local_fallback,
        Some(TranscriptionError::Api { status: 503, .. })
    ));
    assert_eq!(world.http.requests.borrow().len(), 1);
    let calls = world.local.calls.borrow();
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].model, LocalModel::ParakeetUltra);
    assert_eq!(calls[0].prompt.as_deref(), Some("Postgres"));
}

#[test]
fn without_a_local_model_the_online_error_reaches_the_user() {
    let world = World::new(
        FakeHttp::new(|_| answer(401, "invalid key gsk_test")),
        &[ApiProvider::Groq],
        &[],
        FakeLocal::default(),
    );
    let error = world
        .run(job(settings(TranscriptionMode::Online), speech(1.5)))
        .err()
        .expect("expected an error");
    assert!(matches!(error, TranscriptionError::Api { status: 401, .. }));
    assert_eq!(world.http.requests.borrow().len(), 1, "401 is not retried");
}

#[test]
fn a_model_the_provider_no_longer_offers_is_named() {
    let world = World::new(
        FakeHttp::new(|_| answer(404, "model not found")),
        &[ApiProvider::Groq],
        &[],
        FakeLocal::default(),
    );
    let error = world
        .run(job(settings(TranscriptionMode::Online), speech(1.5)))
        .err()
        .expect("expected an error");
    assert!(
        matches!(error, TranscriptionError::ModelRetired(model) if model.provider() == ApiProvider::Groq),
        "{error}"
    );
    assert_eq!(world.http.requests.borrow().len(), 1, "404 is not retried");
}

#[test]
fn a_missing_key_fails_before_any_request() {
    let world = World::new(
        no_requests(),
        &[ApiProvider::OpenAi],
        &[],
        FakeLocal::default(),
    );
    let error = world
        .run(job(settings(TranscriptionMode::Online), speech(1.5)))
        .err()
        .expect("expected an error");
    assert!(matches!(
        error,
        TranscriptionError::NoApiKey(ApiProvider::Groq)
    ));
}

#[test]
fn parakeet_gets_the_raw_recording_first_and_the_trimmed_one_when_that_finds_nothing() {
    let world = World::new(
        no_requests(),
        &[],
        &[LocalModel::ParakeetUltra],
        FakeLocal::answering(vec![Ok(String::new()), Ok("Text".into())]),
    );
    let mut settings = settings(TranscriptionMode::Local);
    settings.local_model = LocalModel::ParakeetUltra;
    let silence = vec![0.0; 16_000];
    let samples = [silence.clone(), speech(1.5), silence].concat();
    let recorded = samples.len();
    let mut job = job(settings, samples);
    job.raw_parakeet_first = true;

    assert_eq!(text(world.run(job)).text, "Text");

    let calls = world.local.calls.borrow();
    assert_eq!(calls.len(), 2);
    assert_eq!(calls[0].samples, recorded);
    assert!(calls[1].samples < recorded, "silence was not trimmed");
}

#[test]
fn audio_too_short_to_transcribe_reaches_no_backend() {
    let world = World::new(no_requests(), &[], &[], FakeLocal::default());
    let outcome = world.run(job(settings(TranscriptionMode::Local), speech(0.1)));
    assert!(matches!(outcome, Ok(Outcome::TooShort)));
    assert!(world.local.calls.borrow().is_empty());
}

#[test]
fn a_local_engine_failure_is_reported() {
    let world = World::new(
        no_requests(),
        &[],
        &[LocalModel::LargeV3Turbo],
        FakeLocal::answering(vec![Err(TranscriptionError::Engine(anyhow::anyhow!(
            "Metal device lost"
        )))]),
    );
    let outcome = world.run(job(settings(TranscriptionMode::Local), speech(1.5)));
    assert!(matches!(outcome, Err(TranscriptionError::Engine(_))));
}

fn post_processing_world(chat: impl Fn() -> Result<Response, RequestError> + 'static) -> World {
    World::new(
        FakeHttp::new(move |url| {
            if url == ApiProvider::OpenAi.transcription_url() {
                answer(200, r#"{"text": "ähm hallo"}"#)
            } else {
                chat()
            }
        }),
        &[ApiProvider::OpenAi, ApiProvider::Groq],
        &[],
        FakeLocal::default(),
    )
}

fn post_processing_job() -> Job {
    let mut settings = settings(TranscriptionMode::Online);
    settings.online_model = OnlineModel::GptTranscribe;
    settings.post_processing = PostProcessingMode::Cleanup;
    job(settings, speech(1.5))
}

#[test]
fn post_processing_prefers_groq_and_replaces_the_text() {
    let world = post_processing_world(|| chat_answer("Hallo."));

    let transcript = text(world.run(post_processing_job()));

    assert_eq!(transcript.text, "Hallo.");
    assert_eq!(transcript.post_processing, Status::Applied);
    assert_eq!(
        world.http.urls(),
        [
            ApiProvider::OpenAi.transcription_url(),
            ApiProvider::Groq.chat_url()
        ]
    );
    let requests = world.http.requests.borrow();
    let chat: serde_json::Value = serde_json::from_slice(&requests[1].body).unwrap();
    assert_eq!(chat["model"], ApiProvider::Groq.post_processing_model());
    assert_eq!(
        chat["messages"][1]["content"],
        "<transcript>\nähm hallo\n</transcript>"
    );
}

#[test]
fn a_refused_post_processing_keeps_the_raw_text_and_names_the_reason() {
    let world = post_processing_world(|| answer(401, "bad key"));
    let transcript = text(world.run(post_processing_job()));
    assert_eq!(transcript.text, "ähm hallo");
    assert_eq!(
        transcript.post_processing,
        Status::Rejected(Rejection::ApiKey(ApiProvider::Groq))
    );
}

#[test]
fn a_post_processing_timeout_keeps_the_raw_text_as_a_temporary_failure() {
    let world = post_processing_world(|| Err(RequestError::TimedOut));
    let transcript = text(world.run(post_processing_job()));
    assert_eq!(transcript.text, "ähm hallo");
    assert_eq!(transcript.post_processing, Status::Failed);
}

#[test]
fn without_a_fallback_a_temporary_failure_is_retried_until_it_succeeds() {
    let world = World::new(
        FakeHttp::in_turn(&[
            (503, "overloaded"),
            (429, "slow down"),
            (200, r#"{"text": "Hallo"}"#),
        ]),
        &[ApiProvider::Groq],
        &[],
        FakeLocal::default(),
    );
    let transcript = text(world.run(job(settings(TranscriptionMode::Online), speech(1.5))));
    assert_eq!(transcript.text, "Hallo");
    assert!(transcript.local_fallback.is_none());
    assert_eq!(world.http.requests.borrow().len(), 3);
}

#[test]
fn without_a_fallback_the_request_gives_up_after_three_attempts() {
    let world = World::new(
        FakeHttp::in_turn(&[(500, "down")]),
        &[ApiProvider::Groq],
        &[],
        FakeLocal::default(),
    );
    let outcome = world.run(job(settings(TranscriptionMode::Online), speech(1.5)));
    assert!(matches!(
        outcome,
        Err(TranscriptionError::Api { status: 500, .. })
    ));
    assert_eq!(world.http.requests.borrow().len(), 3);
}

#[test]
fn a_cancelled_api_makes_no_further_attempts() {
    let world = World::new(
        FakeHttp::in_turn(&[(503, "overloaded")]),
        &[ApiProvider::Groq],
        &[],
        FakeLocal::default(),
    );
    world.cancel.cancel();
    let outcome = world.run(job(settings(TranscriptionMode::Online), speech(1.5)));
    assert!(matches!(
        outcome,
        Err(TranscriptionError::Api { status: 503, .. })
    ));
    assert_eq!(world.http.requests.borrow().len(), 1);
}

#[test]
fn answers_that_are_no_transcript_are_invalid_responses() {
    for body in ["<html>Bad Gateway</html>", r#"{"error": 1}"#] {
        let world = World::new(
            FakeHttp::new(move |_| answer(200, body)),
            &[ApiProvider::Groq],
            &[],
            FakeLocal::default(),
        );
        let outcome = world.run(job(settings(TranscriptionMode::Online), speech(1.5)));
        assert!(matches!(outcome, Err(TranscriptionError::InvalidResponse)));
        assert_eq!(
            world.http.requests.borrow().len(),
            3,
            "invalid answers are retried"
        );
    }
    let world = World::new(
        FakeHttp::new(|_| {
            Ok(Response {
                status: None,
                body: Vec::new(),
            })
        }),
        &[ApiProvider::Groq],
        &[LocalModel::ParakeetUltra],
        FakeLocal::answering(vec![Ok("lokal".into())]),
    );
    let transcript = text(world.run(job(settings(TranscriptionMode::Online), speech(1.5))));
    assert!(matches!(
        transcript.local_fallback,
        Some(TranscriptionError::InvalidResponse)
    ));
}

#[test]
fn when_the_local_fallback_fails_too_the_online_error_is_reported() {
    let world = World::new(
        FakeHttp::new(|_| answer(503, "overloaded")),
        &[ApiProvider::Groq],
        &[LocalModel::ParakeetUltra],
        FakeLocal::answering(vec![Err(TranscriptionError::Engine(anyhow::anyhow!(
            "MLModel prediction failed"
        )))]),
    );
    let outcome = world.run(job(settings(TranscriptionMode::Online), speech(1.5)));
    assert!(matches!(
        outcome,
        Err(TranscriptionError::Api { status: 503, .. })
    ));
    assert_eq!(world.local.calls.borrow().len(), 1);
}

#[test]
fn an_empty_transcript_is_no_speech() {
    let world = World::new(
        no_requests(),
        &[],
        &[LocalModel::LargeV3Turbo],
        FakeLocal::answering(vec![Ok(" ".into())]),
    );
    assert!(matches!(
        world.run(job(settings(TranscriptionMode::Local), speech(1.5))),
        Ok(Outcome::NoSpeech)
    ));
    let world = World::new(
        FakeHttp::new(|_| answer(200, r#"{"text": ""}"#)),
        &[ApiProvider::Groq],
        &[],
        FakeLocal::default(),
    );
    assert!(matches!(
        world.run(job(settings(TranscriptionMode::Online), speech(1.5))),
        Ok(Outcome::NoSpeech)
    ));
}

fn raw_parakeet_job(samples: Vec<f32>) -> Job {
    let mut settings = settings(TranscriptionMode::Local);
    settings.local_model = LocalModel::ParakeetUltra;
    let mut job = job(settings, samples);
    job.raw_parakeet_first = true;
    job
}

#[test]
fn text_from_the_raw_recording_is_used_as_it_is() {
    let world = World::new(
        no_requests(),
        &[],
        &[LocalModel::ParakeetUltra],
        FakeLocal::answering(vec![Ok("Roh".into())]),
    );
    let samples = [vec![0.0; 16_000], speech(1.5)].concat();
    let recorded = samples.len();
    assert_eq!(text(world.run(raw_parakeet_job(samples))).text, "Roh");
    let calls = world.local.calls.borrow();
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].samples, recorded);
    assert!(calls[0].prompt.is_none(), "Parakeet takes no prompt");
}

#[test]
fn a_failure_on_the_raw_recording_falls_back_to_the_preprocessed_one() {
    let world = World::new(
        no_requests(),
        &[],
        &[LocalModel::ParakeetUltra],
        FakeLocal::answering(vec![
            Err(TranscriptionError::Engine(anyhow::anyhow!("ANE busy"))),
            Ok("Text".into()),
        ]),
    );
    let samples = [vec![0.0; 16_000], speech(1.5), vec![0.0; 16_000]].concat();
    let recorded = samples.len();
    assert_eq!(text(world.run(raw_parakeet_job(samples))).text, "Text");
    let calls = world.local.calls.borrow();
    assert_eq!(calls.len(), 2);
    assert_eq!(calls[0].samples, recorded);
    assert!(calls[1].samples < recorded);
}

#[test]
fn pasted_text_never_ends_in_a_line_break() {
    let world = World::new(
        no_requests(),
        &[],
        &[LocalModel::LargeV3Turbo],
        FakeLocal::answering(vec![Ok("rm -rf build\n".into())]),
    );
    let transcript = text(world.run(job(settings(TranscriptionMode::Local), speech(1.5))));
    assert_eq!(transcript.text, "rm -rf build");
}

#[test]
fn a_cut_off_post_processing_answer_keeps_the_raw_text() {
    let world = post_processing_world(|| chat_answer_ending("Hallo, ich", "length"));
    let transcript = text(world.run(post_processing_job()));
    assert_eq!(transcript.text, "ähm hallo");
    assert_eq!(transcript.post_processing, Status::Failed);
}

#[test]
fn a_runaway_post_processing_answer_keeps_the_raw_text() {
    let world = post_processing_world(|| chat_answer(&"Sure! Here is a poem. ".repeat(10)));
    let transcript = text(world.run(post_processing_job()));
    assert_eq!(transcript.text, "ähm hallo");
    assert_eq!(transcript.post_processing, Status::Failed);
}

#[test]
fn post_processing_without_any_key_is_reported_as_lasting() {
    let world = World::new(
        no_requests(),
        &[],
        &[LocalModel::LargeV3Turbo],
        FakeLocal::answering(vec![Ok("ähm hallo".into())]),
    );
    let mut settings = settings(TranscriptionMode::Local);
    settings.post_processing = PostProcessingMode::Cleanup;
    let transcript = text(world.run(job(settings, speech(1.5))));
    assert_eq!(transcript.text, "ähm hallo");
    assert_eq!(
        transcript.post_processing,
        Status::Rejected(Rejection::NoApiKey)
    );
}
