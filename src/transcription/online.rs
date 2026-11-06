//! Transcription through the OpenAI-compatible APIs of OpenAI and Groq.

use std::collections::BTreeMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use super::Api;
use super::error::TranscriptionError;
use crate::config::vocabulary;
use crate::models::{ApiProvider, Language, OnlineModel};
use crate::util::retry::{self, Backoff};
use crate::util::{filelog, http, url_session};

/// A connection that carried a request this recently is still open.
const WARM_UP_INTERVAL: Duration = Duration::from_secs(20);
/// Covers uploading the longest recording (300 s, about 1.8 MB of AAC at 48 kbit/s) over a slow
/// uplink and waiting for its transcript.
const FULL_TIMEOUT: Duration = Duration::from_secs(60);
/// With [`RETRY_DELAY`] the retries wait 2 s and 4 s, which rides out a brief overload (429, 5xx)
/// without keeping the user waiting for minutes.
const FULL_ATTEMPTS: u32 = 3;
/// A local model stands by, so a slow answer is given up sooner; 20 s still carries the longest
/// recording over a 1 Mbit/s uplink.
const BRIEF_TIMEOUT: Duration = Duration::from_secs(20);
pub const RETRY_DELAY: Duration = Duration::from_secs(2);

pub struct Upload<'a> {
    pub data: &'a [u8],
    pub file_name: &'a str,
    pub mime_type: &'a str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Patience {
    /// Nothing can take over: retry and wait for slow answers.
    Full,
    /// A local model can take over, so the user should not wait for retries.
    Brief,
}

pub fn transcribe(
    api: Api,
    upload: &Upload,
    model: OnlineModel,
    language: Language,
    vocabulary: &[String],
    patience: Patience,
) -> Result<String, TranscriptionError> {
    let provider = model.provider();
    let api_key = api
        .keys
        .load(provider)
        .ok_or(TranscriptionError::NoApiKey(provider))?;
    let fields = form_fields(model, language, vocabulary);
    let (max_attempts, timeout) = match patience {
        Patience::Full => (FULL_ATTEMPTS, FULL_TIMEOUT),
        Patience::Brief => (1, BRIEF_TIMEOUT),
    };
    let backoff = Backoff {
        max_attempts,
        first_delay: api.retry_delay,
    };
    retry::with_exponential_backoff(backoff, api.cancel, || {
        send(api, upload, model, &fields, &api_key, timeout)
    })
}

static LAST_CONTACT: Mutex<BTreeMap<ApiProvider, Instant>> = Mutex::new(BTreeMap::new());

/// Records that a request to `provider` just finished, so its connection is open.
pub(super) fn contacted(provider: ApiProvider) {
    if let Ok(mut last) = LAST_CONTACT.lock() {
        last.insert(provider, Instant::now());
    }
}

/// Whether `provider` needs a warm-up at `now`; if so, the warm-up counts as contact.
fn claim_warm_up(
    last: &mut BTreeMap<ApiProvider, Instant>,
    provider: ApiProvider,
    now: Instant,
) -> bool {
    if last
        .get(&provider)
        .is_some_and(|at| now.duration_since(*at) < WARM_UP_INTERVAL)
    {
        return false;
    }
    last.insert(provider, now);
    true
}

/// Opens the connection to the provider while the user is still speaking, unless a recent request
/// or warm-up left one open.
pub fn warm_up(provider: ApiProvider) {
    let due = LAST_CONTACT
        .lock()
        .is_ok_and(|mut last| claim_warm_up(&mut last, provider, Instant::now()));
    if due {
        url_session::warm_up(provider.transcription_url());
    }
}

fn form_fields(
    model: OnlineModel,
    language: Language,
    vocabulary: &[String],
) -> Vec<(&'static str, String)> {
    let language = language.code_for(model);
    let mut fields = vec![("model", model.raw_value().to_owned())];
    if model == OnlineModel::GptTranscribe {
        // gpt-transcribe takes `languages` instead of `language` and rejects requests with both.
        fields.extend(language.map(|code| ("languages[]", code.to_owned())));
        fields.extend(vocabulary.iter().map(|term| ("keywords[]", term.clone())));
    } else {
        fields.extend(language.map(|code| ("language", code.to_owned())));
        fields.extend(vocabulary::prompt(vocabulary).map(|prompt| ("prompt", prompt)));
    }
    fields.push(("response_format", "json".to_owned()));
    fields
}

fn multipart(boundary: &str, fields: &[(&str, String)], upload: &Upload) -> Vec<u8> {
    let mut body = Vec::with_capacity(upload.data.len() + 1024);
    for (name, value) in fields {
        body.extend_from_slice(
            format!(
                "--{boundary}\r\nContent-Disposition: form-data; name=\"{name}\"\r\n\r\n{value}\r\n"
            )
            .as_bytes(),
        );
    }
    body.extend_from_slice(
        format!(
            "--{boundary}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"{}\"\r\nContent-Type: {}\r\n\r\n",
            upload.file_name, upload.mime_type
        )
        .as_bytes(),
    );
    body.extend_from_slice(upload.data);
    body.extend_from_slice(format!("\r\n--{boundary}--\r\n").as_bytes());
    body
}

fn send(
    api: Api,
    upload: &Upload,
    model: OnlineModel,
    fields: &[(&str, String)],
    api_key: &str,
    timeout: Duration,
) -> Result<String, TranscriptionError> {
    let boundary = format!(
        "lowspeak-{:x}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
    );
    let body = multipart(&boundary, fields, upload);

    flog!(
        filelog::APP,
        "Sending {} KB to {}",
        upload.data.len() / 1024,
        model.raw_value()
    );
    let authorization = format!("Bearer {api_key}");
    let content_type = format!("multipart/form-data; boundary={boundary}");
    let response = api
        .http
        .post(
            model.provider().transcription_url(),
            &[
                ("Authorization", &authorization),
                ("Content-Type", &content_type),
            ],
            &body,
            timeout,
        )
        .map_err(TranscriptionError::Network)?;
    contacted(model.provider());
    match response.status {
        None => Err(TranscriptionError::InvalidResponse),
        Some(200) => transcript(&response.body),
        Some(status) => {
            let message = http::redact_api_keys(&response.text());
            flog!(filelog::APP, "API error {status}: {message}");
            Err(TranscriptionError::Api { status, message })
        }
    }
}

fn transcript(body: &[u8]) -> Result<String, TranscriptionError> {
    let json: serde_json::Value =
        serde_json::from_slice(body).map_err(|_| TranscriptionError::InvalidResponse)?;
    json.get("text")
        .and_then(|text| text.as_str())
        .map(|text| text.trim().to_owned())
        .ok_or(TranscriptionError::InvalidResponse)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multipart_body_has_fields_file_and_terminator() {
        let upload = Upload {
            data: b"AUDIO",
            file_name: "audio.m4a",
            mime_type: "audio/mp4",
        };
        let body =
            String::from_utf8(multipart("B", &[("model", "whisper-1".into())], &upload)).unwrap();
        assert!(body.starts_with(
            "--B\r\nContent-Disposition: form-data; name=\"model\"\r\n\r\nwhisper-1\r\n"
        ));
        assert!(body.contains(
            "filename=\"audio.m4a\"\r\nContent-Type: audio/mp4\r\n\r\nAUDIO\r\n--B--\r\n"
        ));
    }

    #[test]
    fn vocabulary_goes_into_keywords_or_the_whisper_prompt() {
        let terms = ["Postgres".to_owned(), "Traefik".to_owned()];
        assert_eq!(
            form_fields(OnlineModel::GptTranscribe, Language::De, &terms),
            [
                ("model", "gpt-transcribe".to_owned()),
                ("languages[]", "de".to_owned()),
                ("keywords[]", "Postgres".to_owned()),
                ("keywords[]", "Traefik".to_owned()),
                ("response_format", "json".to_owned()),
            ]
        );
        assert_eq!(
            form_fields(OnlineModel::GroqWhisperLargeV3, Language::De, &terms),
            [
                ("model", "whisper-large-v3".to_owned()),
                ("language", "de".to_owned()),
                ("prompt", "Postgres, Traefik".to_owned()),
                ("response_format", "json".to_owned()),
            ]
        );
        assert_eq!(
            form_fields(OnlineModel::Whisper1, Language::Auto, &[]),
            [
                ("model", "whisper-1".to_owned()),
                ("response_format", "json".to_owned()),
            ]
        );
    }

    #[test]
    fn a_transcript_needs_a_text_field() {
        assert_eq!(transcript(br#"{"text": " Hallo "}"#).unwrap(), "Hallo");
        for body in [
            &b"<html>Bad Gateway</html>"[..],
            br#"{"text": 42}"#,
            br#"{"error": "none"}"#,
            b"",
        ] {
            assert!(matches!(
                transcript(body),
                Err(TranscriptionError::InvalidResponse)
            ));
        }
    }

    #[test]
    fn warm_ups_wait_for_the_connection_to_go_idle() {
        let mut last = BTreeMap::new();
        let start = Instant::now();
        assert!(claim_warm_up(&mut last, ApiProvider::Groq, start));
        assert!(!claim_warm_up(
            &mut last,
            ApiProvider::Groq,
            start + Duration::from_secs(5)
        ));
        assert!(claim_warm_up(&mut last, ApiProvider::OpenAi, start));
        assert!(claim_warm_up(
            &mut last,
            ApiProvider::Groq,
            start + WARM_UP_INTERVAL
        ));
    }
}
