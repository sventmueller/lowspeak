//! Names and technical terms the user wants spelled exactly, in the `vocabulary` default.

use objc2_foundation::{NSString, NSUserDefaults};

const KEY: &str = "vocabulary";
const MAX_TERMS: usize = 100;
const MAX_TERM_CHARS: usize = 60;
/// Whisper reads at most 224 prompt tokens; 500 characters stay below that for most terms.
const MAX_PROMPT_CHARS: usize = 500;
/// Includes the commas and semicolons that Arabic, Chinese and Japanese keyboards type.
const SEPARATORS: [char; 9] = [',', ';', '\n', '\r', '،', '؛', '，', '；', '、'];

/// OpenAI rejects the whole request if a keyword contains `<` or `>`, so those are dropped.
pub fn parse(input: &str) -> Vec<String> {
    let mut terms: Vec<String> = Vec::new();
    for raw in input.split(SEPARATORS) {
        let cleaned: String = raw.chars().filter(|c| !matches!(c, '<' | '>')).collect();
        let term = cleaned.split_whitespace().collect::<Vec<_>>().join(" ");
        if term.is_empty()
            || term.chars().count() > MAX_TERM_CHARS
            || terms
                .iter()
                .any(|known| known.to_lowercase() == term.to_lowercase())
        {
            continue;
        }
        terms.push(term);
        if terms.len() == MAX_TERMS {
            break;
        }
    }
    terms
}

/// The terms as one finished sentence: Whisper takes the prompt for the text before the
/// recording, and after a list without a full stop it drops repeated sentences of the dictation
/// (measured in docs/development.md, Vocabulary).
pub fn prompt(terms: &[String]) -> Option<String> {
    let mut prompt = String::new();
    let mut chars = 0;
    for term in terms {
        let separator = if prompt.is_empty() { "" } else { ", " };
        let added = separator.chars().count() + term.chars().count();
        if chars + added + 1 > MAX_PROMPT_CHARS {
            break;
        }
        prompt.push_str(separator);
        prompt.push_str(term);
        chars += added;
    }
    if prompt.is_empty() {
        return None;
    }
    if !prompt.ends_with(['.', '!', '?']) {
        prompt.push('.');
    }
    Some(prompt)
}

pub fn load() -> Vec<String> {
    load_from(&NSUserDefaults::standardUserDefaults())
}

pub fn save(terms: &[String]) {
    save_to(&NSUserDefaults::standardUserDefaults(), terms);
}

fn load_from(defaults: &NSUserDefaults) -> Vec<String> {
    defaults
        .stringForKey(&NSString::from_str(KEY))
        .map(|value| parse(&value.to_string()))
        .unwrap_or_default()
}

fn save_to(defaults: &NSUserDefaults, terms: &[String]) {
    let key = NSString::from_str(KEY);
    if terms.is_empty() {
        defaults.removeObjectForKey(&key);
    } else {
        let value = NSString::from_str(&terms.join(", "));
        // SAFETY: NSString is a property-list type.
        unsafe { defaults.setObject_forKey(Some(&value), &key) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_is_split_cleaned_and_deduplicated() {
        assert_eq!(
            parse(" Postgres,  Traefik ;\nKubernetes\r\npostgres, <insurware>, ,"),
            ["Postgres", "Traefik", "Kubernetes", "insurware"]
        );
        assert_eq!(
            parse("Postgres، Traefik，Kubernetes、Redis"),
            ["Postgres", "Traefik", "Kubernetes", "Redis"]
        );
        assert_eq!(parse(&"x".repeat(MAX_TERM_CHARS + 1)), Vec::<String>::new());
        let many = (0..150).map(|i| format!("term{i}")).collect::<Vec<_>>();
        assert_eq!(parse(&many.join(",")).len(), MAX_TERMS);
    }

    #[test]
    fn prompt_lists_terms_within_the_limit() {
        assert_eq!(prompt(&[]), None);
        assert_eq!(
            prompt(&["Postgres".into(), "Traefik".into()]).as_deref(),
            Some("Postgres, Traefik.")
        );
        assert_eq!(
            prompt(&["Muster GmbH".into(), "ACME Inc.".into()]).as_deref(),
            Some("Muster GmbH, ACME Inc.")
        );
        let long: Vec<String> = (0..100).map(|i| format!("Fachbegriff{i:03}")).collect();
        let cut = prompt(&long).unwrap();
        assert!(cut.len() <= MAX_PROMPT_CHARS, "{}", cut.len());
        let last_term = cut.strip_suffix('.').unwrap();
        assert!(last_term.ends_with(|c: char| c.is_ascii_digit()), "{cut}");
    }

    #[test]
    fn the_prompt_limit_counts_characters_not_bytes() {
        let terms: Vec<String> = (0..100).map(|i| format!("東京都千代田区{i:03}")).collect();
        let cut = prompt(&terms).unwrap();
        let chars = cut.chars().count();
        assert!(
            chars <= MAX_PROMPT_CHARS && chars > MAX_PROMPT_CHARS - 12,
            "{chars}"
        );
        assert!(cut.len() > MAX_PROMPT_CHARS);
    }

    #[test]
    fn saved_terms_load_again_and_none_removes_the_entry() {
        let test = crate::config::settings::TestDefaults::new("vocabulary");
        let terms = parse("Postgres, Traefik");
        save_to(&test.defaults, &terms);
        assert_eq!(load_from(&test.defaults), terms);
        save_to(&test.defaults, &[]);
        assert!(
            test.defaults
                .objectForKey(&NSString::from_str(KEY))
                .is_none()
        );
        assert!(load_from(&test.defaults).is_empty());
    }
}
