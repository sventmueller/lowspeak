use std::fmt;
use std::time::Duration;

use crate::i18n::Texts;
use crate::models::{ApiProvider, OnlineModel};
use crate::util::retry::Retryable;
use crate::util::url_session::RequestError;

#[derive(Debug)]
pub enum TranscriptionError {
    NoApiKey(ApiProvider),
    InvalidResponse,
    Api {
        status: u16,
        message: String,
        retry_after: Option<Duration>,
    },
    /// The account has no credit left; asking again does not help until the user adds some.
    QuotaExhausted(ApiProvider),
    Network(RequestError),
    /// The provider answers 404 for the model, so it no longer offers it under this name.
    ModelRetired(OnlineModel),
    ModelNotAvailable,
    /// The recording could not be converted to 16 kHz.
    Audio(anyhow::Error),
    /// Whisper or Parakeet failed, the thread that runs them is gone, or the transcription panicked.
    Engine(anyhow::Error),
}

/// What an error status from OpenAI or Groq means, for transcription and post-processing alike.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatusClass {
    /// The key is wrong, revoked, or has no access to the model.
    KeyRejected,
    ModelRetired,
    /// The provider refuses this request as it is.
    RequestRejected,
    TimedOut,
    RateLimited,
    ServerError,
    Unexpected,
}

impl StatusClass {
    pub fn of(status: u16) -> Self {
        match status {
            401 | 403 => Self::KeyRejected,
            404 => Self::ModelRetired,
            408 => Self::TimedOut,
            429 => Self::RateLimited,
            400..=499 => Self::RequestRejected,
            500..=599 => Self::ServerError,
            _ => Self::Unexpected,
        }
    }

    /// The provider keeps answering like this until the user changes the key or the app.
    pub fn is_permanent(self) -> bool {
        matches!(
            self,
            Self::KeyRejected | Self::ModelRetired | Self::RequestRejected
        )
    }
}

/// OpenAI answers 429 for a rate limit and for an account without credit alike; only the body tells
/// them apart.
pub fn quota_exhausted(body: &str) -> bool {
    let Ok(json) = serde_json::from_str::<serde_json::Value>(body) else {
        return false;
    };
    let error = &json["error"];
    ["type", "code"].iter().any(|field| {
        error[*field]
            .as_str()
            .is_some_and(|value| matches!(value, "insufficient_quota" | "credit_balance_exhausted"))
    })
}

impl fmt::Display for TranscriptionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoApiKey(provider) => {
                write!(f, "no {} API key configured", provider.display_name())
            }
            Self::InvalidResponse => f.write_str("invalid response from the API"),
            Self::Api {
                status, message, ..
            } => write!(f, "API error ({status}): {message}"),
            Self::QuotaExhausted(provider) => write!(
                f,
                "the {} account has no credit left",
                provider.display_name()
            ),
            Self::Network(error) => write!(f, "network error: {error}"),
            Self::ModelRetired(model) => write!(
                f,
                "{} no longer offers {} (HTTP 404)",
                model.provider().display_name(),
                model.raw_value()
            ),
            Self::ModelNotAvailable => f.write_str("local model not downloaded"),
            Self::Audio(error) => write!(f, "audio conversion failed: {error:#}"),
            Self::Engine(error) => write!(f, "local model failed: {error:#}"),
        }
    }
}

impl std::error::Error for TranscriptionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Network(error) => Some(error),
            Self::Audio(error) | Self::Engine(error) => Some(&**error),
            Self::NoApiKey(_)
            | Self::InvalidResponse
            | Self::Api { .. }
            | Self::QuotaExhausted(_)
            | Self::ModelRetired(_)
            | Self::ModelNotAvailable => None,
        }
    }
}

impl TranscriptionError {
    /// Message for notifications: never contains raw API response bodies.
    pub fn user_facing_description(&self, t: &Texts) -> String {
        match self {
            Self::NoApiKey(provider) => (t.no_api_key_error)(provider.display_name()),
            Self::InvalidResponse => t.invalid_response.to_owned(),
            Self::Api { status, .. } => match StatusClass::of(*status) {
                StatusClass::KeyRejected => t.authentication_failed.to_owned(),
                StatusClass::TimedOut => t.request_timed_out.to_owned(),
                StatusClass::RateLimited => t.rate_limited.to_owned(),
                StatusClass::ServerError => (t.server_error)(*status),
                StatusClass::ModelRetired
                | StatusClass::RequestRejected
                | StatusClass::Unexpected => (t.api_error)(*status),
            },
            Self::QuotaExhausted(provider) => (t.quota_exhausted)(provider.display_name()),
            Self::Network(RequestError::TimedOut) => t.request_timed_out.to_owned(),
            Self::Network(error) => (t.network_error)(&error.to_string()),
            Self::ModelRetired(model) => {
                (t.online_model_retired)(model.provider().display_name(), model.name())
            }
            Self::ModelNotAvailable => (t.model_not_available)(t.manage_models),
            Self::Audio(_) => t.audio_failed.to_owned(),
            Self::Engine(_) => t.local_model_failed.to_owned(),
        }
    }
}

impl Retryable for TranscriptionError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::Api { status, .. } => !StatusClass::of(*status).is_permanent(),
            Self::InvalidResponse | Self::Network(_) => true,
            Self::NoApiKey(_)
            | Self::QuotaExhausted(_)
            | Self::ModelRetired(_)
            | Self::ModelNotAvailable
            | Self::Audio(_)
            | Self::Engine(_) => false,
        }
    }

    fn retry_after(&self) -> Option<Duration> {
        match self {
            Self::Api { retry_after, .. } => *retry_after,
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error as _;

    use super::*;
    use crate::i18n;
    use crate::models::Language;

    fn api(status: u16) -> TranscriptionError {
        TranscriptionError::Api {
            status,
            message: String::new(),
            retry_after: None,
        }
    }

    #[test]
    fn client_errors_are_final_except_timeouts_and_rate_limits() {
        for permanent in [400, 401, 403, 404, 413, 422] {
            assert!(!api(permanent).is_retryable(), "{permanent}");
        }
        for temporary in [408, 429, 500, 503] {
            assert!(api(temporary).is_retryable(), "{temporary}");
        }
        assert!(TranscriptionError::Network(RequestError::TimedOut).is_retryable());
    }

    #[test]
    fn statuses_are_classified_once_for_every_request() {
        assert_eq!(StatusClass::of(401), StatusClass::KeyRejected);
        assert_eq!(StatusClass::of(403), StatusClass::KeyRejected);
        assert_eq!(StatusClass::of(404), StatusClass::ModelRetired);
        assert_eq!(StatusClass::of(422), StatusClass::RequestRejected);
        assert_eq!(StatusClass::of(408), StatusClass::TimedOut);
        assert_eq!(StatusClass::of(502), StatusClass::ServerError);
        assert_eq!(StatusClass::of(302), StatusClass::Unexpected);
        let t = i18n::texts(Language::En);
        assert_eq!(api(403).user_facing_description(t), t.authentication_failed);
        assert_eq!(api(408).user_facing_description(t), t.request_timed_out);
    }

    #[test]
    fn notifications_show_translated_texts_instead_of_error_chains() {
        let t = i18n::texts(Language::De);
        let engine = TranscriptionError::Engine(anyhow::anyhow!("MLModel prediction failed"));
        assert_eq!(engine.user_facing_description(t), t.local_model_failed);
        let timeout = TranscriptionError::Network(RequestError::TimedOut);
        assert_eq!(timeout.user_facing_description(t), t.request_timed_out);
        let body = TranscriptionError::Api {
            status: 400,
            message: "sk-secret echoed".into(),
            retry_after: None,
        };
        assert!(!body.user_facing_description(t).contains("secret"));
    }

    #[test]
    fn an_account_without_credit_is_told_apart_from_a_rate_limit() {
        let exhausted = r#"{"error": {"message": "You have no credits remaining.", "type": "insufficient_quota", "code": "credit_balance_exhausted"}}"#;
        assert!(quota_exhausted(exhausted));
        assert!(quota_exhausted(
            r#"{"error": {"type": "invalid_request_error", "code": "insufficient_quota"}}"#
        ));
        assert!(!quota_exhausted(
            r#"{"error": {"message": "Rate limit reached", "type": "requests", "code": "rate_limit_exceeded"}}"#
        ));
        assert!(!quota_exhausted("slow down"));
        let error = TranscriptionError::QuotaExhausted(ApiProvider::OpenAi);
        assert!(!error.is_retryable());
        assert_eq!(
            error.user_facing_description(i18n::texts(Language::En)),
            "OpenAI reports that the account has no credit left. Add credit in your OpenAI account."
        );
    }

    #[test]
    fn a_retired_model_is_named_with_its_provider() {
        let retired = TranscriptionError::ModelRetired(OnlineModel::GroqWhisperLargeV3);
        assert!(!retired.is_retryable());
        assert_eq!(
            retired.user_facing_description(i18n::texts(Language::En)),
            "Groq no longer offers Whisper Large V3. Choose another model in the menu."
        );
        assert_eq!(
            retired.to_string(),
            "Groq no longer offers whisper-large-v3 (HTTP 404)"
        );
    }

    #[test]
    fn the_cause_is_available_as_the_source() {
        let engine = TranscriptionError::Engine(anyhow::anyhow!("Metal device lost"));
        assert_eq!(engine.source().unwrap().to_string(), "Metal device lost");
        let network = TranscriptionError::Network(RequestError::TimedOut);
        assert!(network.source().unwrap().is::<RequestError>());
        assert!(api(500).source().is_none());
    }
}
