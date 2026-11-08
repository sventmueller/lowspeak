//! Text that Whisper writes although nobody said it: the credits of subtitled videos it was trained
//! on, and the vocabulary it was handed as its prompt.

/// From the closing credits of subtitled videos in Whisper's training data; it writes them into
/// silence and noise, and nobody dictates them.
const SUBTITLE_CREDITS: [&str; 8] = [
    "im auftrag des zdf",
    "untertitelung des zdf",
    "amara.org",
    "stephanie geiges",
    "copyright wdr",
    "dimatorzok",
    "sous-titrage société radio-canada",
    "sous-titrage st' 501",
];
/// With a longer vocabulary, Whisper echoes at least this many of its terms.
const MIN_ECHOED_TERMS: usize = 3;
const SENTENCE_ENDS: [char; 4] = ['.', '!', '?', '…'];
/// Chinese and Japanese put no space after these.
const FULL_WIDTH_SENTENCE_ENDS: [char; 3] = ['。', '！', '？'];

/// `text` without subtitle credits, or empty if it only repeats the vocabulary.
pub fn without_phantoms(text: &str, vocabulary: &[String]) -> String {
    if echoes_vocabulary(text, vocabulary) {
        return String::new();
    }
    sentences(text)
        .filter(|sentence| {
            let sentence = sentence.to_lowercase();
            !SUBTITLE_CREDITS
                .iter()
                .any(|credit| sentence.contains(credit))
        })
        .collect::<String>()
        .trim()
        .to_owned()
}

/// The sentences of `text`, each with the whitespace before it, so that together they are `text`.
fn sentences(text: &str) -> impl Iterator<Item = &str> {
    let mut rest = text;
    std::iter::from_fn(move || {
        if rest.is_empty() {
            return None;
        }
        let mut chars = rest.char_indices().peekable();
        let mut end = rest.len();
        while let Some((index, c)) = chars.next() {
            let ends = FULL_WIDTH_SENTENCE_ENDS.contains(&c)
                || SENTENCE_ENDS.contains(&c)
                    && chars.peek().is_none_or(|&(_, next)| next.is_whitespace());
            if ends && !rest[..index].trim().is_empty() {
                end = index + c.len_utf8();
                break;
            }
        }
        let (sentence, after) = rest.split_at(end);
        rest = after;
        Some(sentence)
    })
}

/// Whisper answers silence with its prompt at times; dictating the start of one's own vocabulary
/// in its order is unlikely enough to take as that.
fn echoes_vocabulary(text: &str, vocabulary: &[String]) -> bool {
    if vocabulary.len() < 2 {
        return false;
    }
    let spoken = words(text);
    if spoken.is_empty() {
        return false;
    }
    let needed = MIN_ECHOED_TERMS.min(vocabulary.len());
    let mut prompt = Vec::new();
    for (index, term) in vocabulary.iter().enumerate() {
        prompt.extend(words(term));
        if prompt.len() > spoken.len() {
            return false;
        }
        if index + 1 >= needed && prompt == spoken {
            return true;
        }
    }
    false
}

fn words(text: &str) -> Vec<String> {
    text.split(|c: char| !c.is_alphanumeric())
        .filter(|word| !word.is_empty())
        .map(str::to_lowercase)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn terms(list: &[&str]) -> Vec<String> {
        list.iter().map(|&term| term.to_owned()).collect()
    }

    #[test]
    fn subtitle_credits_are_removed_and_the_rest_kept() {
        assert_eq!(
            without_phantoms(
                "Wir sehen uns morgen. Untertitel im Auftrag des ZDF für funk, 2017",
                &[]
            ),
            "Wir sehen uns morgen."
        );
        assert_eq!(
            without_phantoms("Untertitel der Amara.org-Community", &[]),
            ""
        );
        assert_eq!(
            without_phantoms("Sous-titres réalisés par la communauté d'Amara.org", &[]),
            ""
        );
        assert_eq!(without_phantoms("字幕由Amara.org社区提供", &[]), "");
        assert_eq!(
            without_phantoms("明天见。字幕由Amara.org社区提供", &[]),
            "明天见。"
        );
    }

    #[test]
    fn ordinary_dictations_stay_as_they_are() {
        for text in [
            "Vielen Dank.",
            "Thank you for watching the demo, see you tomorrow!",
            "Die Untertitel im ZDF waren gestern zu klein.",
            "Version 2.5 ist fertig. Bitte testen!",
            "你好。我们明天见。",
            "Wait... what?",
        ] {
            assert_eq!(without_phantoms(text, &[]), text);
        }
    }

    #[test]
    fn sentences_put_the_text_back_together() {
        let text = "  Eins. Zwei! Drei? 3.14 bleibt. Ende";
        assert_eq!(sentences(text).collect::<String>(), text);
        assert_eq!(
            sentences(text).collect::<Vec<_>>(),
            ["  Eins.", " Zwei!", " Drei?", " 3.14 bleibt.", " Ende"]
        );
    }

    #[test]
    fn the_vocabulary_repeated_back_counts_as_nothing_said() {
        let vocabulary = terms(&["Postgres", "Traefik", "Kubernetes", "Lowspeak"]);
        assert_eq!(
            without_phantoms("Postgres, Traefik, Kubernetes, Lowspeak.", &vocabulary),
            ""
        );
        assert_eq!(
            without_phantoms("Postgres, Traefik, Kubernetes", &vocabulary),
            ""
        );
        let two = terms(&["Postgres", "Traefik"]);
        assert_eq!(without_phantoms("Postgres, Traefik.", &two), "");
    }

    #[test]
    fn dictated_terms_are_kept() {
        let vocabulary = terms(&["Postgres", "Traefik", "Kubernetes"]);
        for text in [
            "Postgres",
            "Postgres und Traefik",
            "Postgres, Traefik",
            "Traefik, Postgres, Kubernetes",
            "Postgres läuft hinter Traefik auf Kubernetes",
        ] {
            assert_eq!(without_phantoms(text, &vocabulary), text);
        }
        assert_eq!(
            without_phantoms("Postgres", &terms(&["Postgres"])),
            "Postgres"
        );
    }
}
