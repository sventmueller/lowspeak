//! The vocabulary for Parakeet, which hears none: a word it wrote close to one of the user's terms
//! takes the term's spelling, unless the word is one of the language's own. Without the acoustic
//! check of FluidAudio's boosting the thresholds are much stricter than its 0.52 similarity.

use std::collections::HashSet;
use std::ops::Range;
use std::sync::mpsc;
use std::time::Duration;

use objc2_app_kit::NSSpellChecker;
use objc2_foundation::{NSLocale, NSString};

use crate::util::main_thread;

/// Tells real words from what a model made of a name it did not know.
pub trait Lexicon {
    /// Whether `word` is spelled right in `language`, the user's macOS languages or English.
    fn is_word(&self, word: &str, language: Option<&str>) -> bool;
}

/// Asks the macOS spell checker on the main thread, since AppKit does not promise more; a busy
/// main thread means no answer in time, and the word counts as real, so nothing is replaced.
pub struct SpellChecker;

const SPELL_CHECK_TIMEOUT: Duration = Duration::from_millis(500);

impl Lexicon for SpellChecker {
    fn is_word(&self, word: &str, language: Option<&str>) -> bool {
        let (sender, answer) = mpsc::channel();
        let word = word.to_owned();
        let language = language.map(str::to_owned);
        main_thread::run(move || {
            let _ = sender.send(known_to_macos(&word, language.as_deref()));
        });
        answer.recv_timeout(SPELL_CHECK_TIMEOUT).unwrap_or(true)
    }
}

/// Whether the macOS spell checker accepts `word` in `language`, the user's macOS languages or
/// English. Its own list of spelling languages is no help: it holds every language set to
/// "Automatic by Language", and those without a dictionary accept any word.
pub fn known_to_macos(word: &str, language: Option<&str>) -> bool {
    let checker = NSSpellChecker::sharedSpellChecker();
    let accepts = |word: &str, language: &str| {
        // SAFETY: a null word count is allowed; the strings live through the call.
        let misspelled = unsafe {
            checker.checkSpellingOfString_startingAt_language_wrap_inSpellDocumentWithTag_wordCount(
                &NSString::from_str(word),
                0,
                Some(&NSString::from_str(language)),
                false,
                0,
                std::ptr::null_mut(),
            )
        };
        misspelled.length == 0
    };
    let mut seen = HashSet::new();
    language
        .map(str::to_owned)
        .into_iter()
        .chain(
            NSLocale::preferredLanguages()
                .iter()
                .filter_map(|tag| tag.to_string().split(['-', '_']).next().map(str::to_owned)),
        )
        .chain(["en".to_owned()])
        .filter(|language| seen.insert(language.clone()))
        .filter(|language| !accepts(NONSENSE, language))
        .any(|language| accepts(word, &language))
}

/// No language has this word, so a dictionary that accepts it is none.
const NONSENSE: &str = "qxzvjkwqh";

/// Shorter terms would match too many ordinary words.
const MIN_TERM_CHARS: usize = 5;
/// From this length a term may differ in two letters instead of one.
const LONG_TERM_CHARS: usize = 9;
/// From this length a name that is no word itself is taken to be what the model heard, even where
/// it wrote a real word one letter off, such as "Trafik" for Traefik; it is also the length from
/// which a name the model split into several words is joined back.
const DISTINCT_TERM_CHARS: usize = 7;
/// "Lowspeaks" and "Traefiks" are the term with an ending of its language, not misspellings.
const MAX_ENDING_CHARS: usize = 2;

/// `text` with the spelling of `terms` where it holds something very close.
pub fn with_vocabulary(
    text: &str,
    terms: &[String],
    language: Option<&str>,
    lexicon: &dyn Lexicon,
) -> String {
    let words = word_ranges(text);
    let mut terms: Vec<(&str, String, usize)> = terms
        .iter()
        .map(|term| (term.as_str(), folded(term), words_in(term)))
        .filter(|(_, folded, _)| folded.chars().count() >= MIN_TERM_CHARS)
        .collect();
    terms.sort_by_key(|(_, folded, _)| std::cmp::Reverse(folded.chars().count()));

    let mut taken = vec![false; words.len()];
    let mut replacements: Vec<(Range<usize>, &str)> = Vec::new();
    for (term, term_folded, term_words) in &terms {
        let length = term_folded.chars().count();
        let mut term_is_word = None;
        for start in 0..words.len() {
            for span in [*term_words, term_words + 1] {
                let end = start + span;
                if end > words.len() || taken[start..end].iter().any(|&t| t) {
                    continue;
                }
                if !joined_by_spaces(text, &words[start..end])
                    || in_code(text, words[start].start..words[end - 1].end)
                {
                    continue;
                }
                let spoken: String = words[start..end]
                    .iter()
                    .map(|word| folded(&text[word.clone()]))
                    .collect();
                let distance = distance(&spoken, term_folded);
                let range = words[start].start..words[end - 1].end;
                let original = &text[range.clone()];
                let inflected = spoken
                    .strip_prefix(term_folded.as_str())
                    .is_some_and(|ending| {
                        !ending.is_empty() && ending.chars().count() <= MAX_ENDING_CHARS
                    });
                if original == *term || inflected {
                    taken[start..end].fill(true);
                    break;
                }
                let close = match distance {
                    0 => true,
                    1 if span > 1 => length >= DISTINCT_TERM_CHARS,
                    _ if span > 1 => false,
                    1 if length >= DISTINCT_TERM_CHARS
                        && !*term_is_word.get_or_insert_with(|| {
                            word_ranges(term)
                                .into_iter()
                                .all(|word| lexicon.is_word(&term[word], language))
                        }) =>
                    {
                        true
                    }
                    1 => !lexicon.is_word(original, language),
                    2 => length >= LONG_TERM_CHARS && !lexicon.is_word(original, language),
                    _ => false,
                };
                if close {
                    taken[start..end].fill(true);
                    replacements.push((range, term));
                    break;
                }
            }
        }
    }
    replacements.sort_by_key(|(range, _)| std::cmp::Reverse(range.start));
    let mut spelled = text.to_owned();
    for (range, term) in replacements {
        spelled.replace_range(range, term);
    }
    spelled
}

/// The byte ranges of the words: runs of letters and digits.
fn word_ranges(text: &str) -> Vec<Range<usize>> {
    let mut words = Vec::new();
    let mut start = None;
    for (index, c) in text.char_indices() {
        match (c.is_alphanumeric(), start) {
            (true, None) => start = Some(index),
            (false, Some(begin)) => {
                words.push(begin..index);
                start = None;
            }
            _ => {}
        }
    }
    if let Some(begin) = start {
        words.push(begin..text.len());
    }
    words
}

fn words_in(term: &str) -> usize {
    word_ranges(term).len().max(1)
}

/// Words split by a space or hyphen may be one name; a comma or full stop ends it.
fn joined_by_spaces(text: &str, words: &[Range<usize>]) -> bool {
    words.windows(2).all(|pair| {
        text[pair[0].end..pair[1].start]
            .chars()
            .all(|c| c == ' ' || c == '-')
    })
}

/// Whether `range` lies in an address, a file name or an identifier such as `com.lowspeak.app`,
/// where the spelling must stay as written.
fn in_code(text: &str, range: Range<usize>) -> bool {
    let start = text[..range.start]
        .rfind(char::is_whitespace)
        .map_or(0, |index| index + 1);
    let end = text[range.end..]
        .find(char::is_whitespace)
        .map_or(text.len(), |index| range.end + index);
    let token: Vec<char> = text[start..end].chars().collect();
    token
        .iter()
        .any(|c| matches!(c, '/' | '\\' | '_' | '@' | '`' | '=' | '~'))
        || token
            .windows(3)
            .any(|w| w[1] == '.' && w[0].is_alphanumeric() && w[2].is_alphanumeric())
}

/// Lower case, umlauts as two letters, letters and digits only: "Träfik" and "Traefik" meet.
fn folded(text: &str) -> String {
    let mut folded = String::with_capacity(text.len());
    for c in text.chars().flat_map(char::to_lowercase) {
        match c {
            'ä' => folded.push_str("ae"),
            'ö' => folded.push_str("oe"),
            'ü' => folded.push_str("ue"),
            'ß' => folded.push_str("ss"),
            c if c.is_alphanumeric() => folded.push(c),
            _ => {}
        }
    }
    folded
}

/// Edits between two strings, a swap of neighbouring letters counting as one.
fn distance(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let mut rows = vec![vec![0usize; b.len() + 1]; a.len() + 1];
    for (i, row) in rows.iter_mut().enumerate() {
        row[0] = i;
    }
    for (j, cell) in rows[0].iter_mut().enumerate() {
        *cell = j;
    }
    for i in 1..=a.len() {
        for j in 1..=b.len() {
            let cost = usize::from(a[i - 1] != b[j - 1]);
            let mut best = (rows[i - 1][j] + 1)
                .min(rows[i][j - 1] + 1)
                .min(rows[i - 1][j - 1] + cost);
            if i > 1 && j > 1 && a[i - 1] == b[j - 2] && a[i - 2] == b[j - 1] {
                best = best.min(rows[i - 2][j - 2] + 1);
            }
            rows[i][j] = best;
        }
    }
    rows[a.len()][b.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Knows the words listed, in every language.
    struct Known(&'static [&'static str]);

    impl Lexicon for Known {
        fn is_word(&self, word: &str, _language: Option<&str>) -> bool {
            self.0.iter().any(|known| known.eq_ignore_ascii_case(word))
        }
    }

    const ORDINARY: Known = Known(&[
        "traffic",
        "Los",
        "Peak",
        "Post",
        "Grafik",
        "logs",
        "Lux",
        "Kuchen",
        "Postgasse",
        "Trafik",
        "Projekt",
        "Projekte",
        "Meier",
        "Meyer",
        "Muster",
    ]);

    fn vocabulary() -> Vec<String> {
        [
            "Traefik",
            "Lowspeak",
            "Postgres",
            "Kubernetes",
            "Grafana",
            "Muster GmbH",
            "AWS",
            "Projekt",
            "Meier",
        ]
        .iter()
        .map(|&term| term.to_owned())
        .collect()
    }

    fn spelled(text: &str) -> String {
        with_vocabulary(text, &vocabulary(), Some("de"), &ORDINARY)
    }

    #[test]
    fn names_the_model_did_not_know_take_the_vocabulary_spelling() {
        assert_eq!(
            spelled("Danach prüfen wir die Lux von Träfig."),
            "Danach prüfen wir die Lux von Traefik."
        );
        assert_eq!(
            spelled("Kannst du bitte Los Peak neu starten?"),
            "Kannst du bitte Lowspeak neu starten?"
        );
        assert_eq!(spelled("Wir nutzen Kubernetis."), "Wir nutzen Kubernetes.");
        assert_eq!(spelled("Die postgres Datenbank"), "Die Postgres Datenbank");
        assert_eq!(
            spelled("auf Post-Gres umgestellt"),
            "auf Postgres umgestellt"
        );
        assert_eq!(spelled("die Muster Gmbh"), "die Muster GmbH");
        assert_eq!(spelled("Kubernetis und Gravana"), "Kubernetes und Grafana");
        assert_eq!(
            spelled("die Logs von Trafik"),
            "die Logs von Traefik",
            "a real word one letter off a name"
        );
    }

    #[test]
    fn real_words_and_distant_ones_stay() {
        for text in [
            "Check the traffic logs.",
            "Die Grafik ist fertig.",
            "Die Post kommt morgen.",
            "Wir wohnen in der Postgasse.",
            "Los, Peak ist erreicht.",
            "Der Kuchen ist gut.",
            "Das war aws.",
            "Grafiken und Graphen",
            "Zwei Projekte sind fertig.",
            "Frau Meyer kommt.",
            "Lowspeaks Menü und Traefiks Logs",
            "Klone github.com/sventmueller/lowspeak neu.",
            "Die Kennung ist com.lowspeak.app.",
            "Starte postgres_dev und kubernetis-test.sh",
        ] {
            assert_eq!(spelled(text), text);
        }
    }

    #[test]
    fn without_terms_or_with_the_right_spelling_nothing_changes() {
        let text = "Traefik und Lowspeak laufen.";
        assert_eq!(spelled(text), text);
        assert_eq!(with_vocabulary("Träfig", &[], None, &ORDINARY), "Träfig");
    }

    #[test]
    fn macos_knows_words_and_not_what_a_model_made_of_a_name() {
        assert!(known_to_macos("traffic", Some("en")));
        assert!(known_to_macos("house", None));
        assert!(!known_to_macos("Kubernetis", Some("en")));
        assert!(!known_to_macos(NONSENSE, None));
    }

    #[test]
    fn distances_count_a_swap_as_one_edit() {
        assert_eq!(distance("traefik", "traefig"), 1);
        assert_eq!(distance("traefik", "traefki"), 1);
        assert_eq!(distance("traffic", "traefik"), 2);
        assert_eq!(distance("", "abc"), 3);
        assert_eq!(folded("Träfik-Grüße"), "traefikgruesse");
    }
}
