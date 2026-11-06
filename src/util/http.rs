use std::time::Duration;

use ureq::Agent;
use ureq::tls::{RootCerts, TlsConfig};

const CONNECT_TIMEOUT: Duration = Duration::from_secs(15);

/// Agent that trusts the system keychain's certificates and returns error statuses as responses.
/// `per_read` bounds every wait for data, so a stalled transfer fails instead of blocking forever.
pub fn agent(global: Option<Duration>, per_read: Duration) -> Agent {
    Agent::config_builder()
        .tls_config(
            TlsConfig::builder()
                .root_certs(RootCerts::PlatformVerifier)
                .build(),
        )
        .timeout_global(global)
        .timeout_connect(Some(CONNECT_TIMEOUT))
        .timeout_per_read(Some(per_read))
        .http_status_as_error(false)
        .user_agent(format!("Lowspeak/{}", super::version::app_version()))
        .build()
        .into()
}

const KEY_PREFIXES: [&str; 2] = ["gsk_", "sk-"];

fn is_key_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_' || c == '-'
}

/// Random key material: real keys are long and practically always contain a digit, while words
/// such as "sk-SK" or "sk-learn" have none.
fn looks_like_key(body: &str) -> bool {
    body.len() >= 20 || body.bytes().any(|byte| byte.is_ascii_digit())
}

/// Masks OpenAI (`sk-…`) and Groq (`gsk_…`) keys that API error bodies may echo back.
pub fn redact_api_keys(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut copied = 0;
    let mut position = 0;
    while position < text.len() {
        let rest = &text[position..];
        let at_word_start = !text[..position].ends_with(is_key_char);
        let prefix = KEY_PREFIXES
            .into_iter()
            .find(|prefix| at_word_start && rest.starts_with(prefix));
        let Some(prefix) = prefix else {
            position += rest.chars().next().map_or(1, char::len_utf8);
            continue;
        };
        let body_start = position + prefix.len();
        let body_len = text[body_start..]
            .find(|c: char| !is_key_char(c))
            .unwrap_or(text.len() - body_start);
        if looks_like_key(&text[body_start..body_start + body_len]) {
            out.push_str(&text[copied..body_start]);
            out.push_str("***");
            copied = body_start + body_len;
        }
        position = body_start + body_len;
    }
    out.push_str(&text[copied..]);
    out
}

#[cfg(test)]
mod tests {
    use super::redact_api_keys;

    #[test]
    fn api_keys_are_masked() {
        assert_eq!(
            redact_api_keys("bad key sk-abc_12-3, also gsk_XYZ9 here"),
            "bad key sk-***, also gsk_*** here"
        );
        assert_eq!(redact_api_keys("nothing to hide"), "nothing to hide");
    }

    #[test]
    fn realistic_keys_are_masked_wherever_they_stand() {
        let openai = "sk-proj-AbCdEfGhIjKlMnOpQrStUvWxYz";
        let groq = "gsk_AbCdEfGhIjKlMnOpQrStUvWxYzAbCdEfGhIjKlMnOpQrSt";
        assert_eq!(
            redact_api_keys(&format!("Incorrect API key provided: {openai}.")),
            "Incorrect API key provided: sk-***."
        );
        assert_eq!(redact_api_keys(&format!("\"{groq}\"")), "\"gsk_***\"");
        assert_eq!(redact_api_keys(&format!("key={groq}")), "key=gsk_***");
    }

    #[test]
    fn words_that_merely_contain_a_prefix_stay() {
        for text in [
            "task-x failed",
            "a risk-based check",
            "disk-cache full",
            "locale sk-SK is unsupported",
            "sk- alone",
            "über gsk_ ende",
        ] {
            assert_eq!(redact_api_keys(text), text);
        }
    }
}
