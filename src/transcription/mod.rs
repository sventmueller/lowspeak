pub mod catalog;
pub mod error;
pub mod hf;
pub mod local;
pub mod model_store;
pub mod online;
pub mod parakeet;
pub mod postprocess;
pub mod service;
#[cfg(test)]
mod test_server;
pub mod whisper;

use std::time::Duration;

use crate::config::api_keys::{KeyFile, KeyStore};
use crate::util::retry::CancelToken;
use crate::util::url_session::{HttpClient, UrlSession};

/// How requests reach OpenAI and Groq.
#[derive(Clone, Copy)]
pub struct Api<'a> {
    pub http: &'a dyn HttpClient,
    pub keys: &'a dyn KeyStore,
    /// Ends the retries of running requests.
    pub cancel: &'a CancelToken,
    /// The wait before the first retry; it doubles before each further one.
    pub retry_delay: Duration,
}

static SHUTDOWN: CancelToken = CancelToken::new();

impl Api<'static> {
    /// The shared URLSession and the key file; [`shutdown`] ends its retries.
    pub const SYSTEM: Self = Self {
        http: &UrlSession,
        keys: &KeyFile,
        cancel: &SHUTDOWN,
        retry_delay: online::RETRY_DELAY,
    };
}

/// Stops the retries of online requests made through [`Api::SYSTEM`], for good; call it when the
/// app quits, so no transcription thread waits for its next attempt.
pub fn shutdown() {
    SHUTDOWN.cancel();
}
