use std::collections::{HashMap, HashSet};
use std::path::Path;

use anyhow::{Context as _, Result, bail, ensure};
use unicode_general_category::{GeneralCategory, get_general_category};

pub const WORD_BOUNDARY: char = '▁';
/// Parakeet's vocabularies have 1,024 or 8,192 pieces; an id far beyond that is a damaged file,
/// which would otherwise size the piece table.
const MAX_PIECES: usize = 1 << 16;

pub fn is_punctuation(c: char) -> bool {
    use GeneralCategory::*;
    matches!(
        get_general_category(c),
        ConnectorPunctuation
            | DashPunctuation
            | OpenPunctuation
            | ClosePunctuation
            | InitialPunctuation
            | FinalPunctuation
            | OtherPunctuation
    )
}

pub fn is_symbol(c: char) -> bool {
    use GeneralCategory::*;
    matches!(
        get_general_category(c),
        MathSymbol | CurrencySymbol | ModifierSymbol | OtherSymbol
    )
}

pub fn is_whitespace(c: char) -> bool {
    c == '\t' || get_general_category(c) == GeneralCategory::SpaceSeparator
}

pub fn starts_word(piece: &str) -> bool {
    piece.starts_with(WORD_BOUNDARY) || piece.starts_with(' ')
}

pub fn strip_word_boundary(piece: &str) -> &str {
    piece
        .strip_prefix(WORD_BOUNDARY)
        .or_else(|| piece.strip_prefix(' '))
        .unwrap_or(piece)
}

pub fn is_punctuation_or_symbol_piece(piece: &str) -> bool {
    !piece.is_empty() && piece.chars().all(|c| is_punctuation(c) || is_symbol(c))
}

pub fn is_splice_safe_piece(piece: &str) -> bool {
    !piece.is_empty() && (starts_word(piece) || is_punctuation_or_symbol_piece(piece))
}

pub struct Vocabulary {
    pieces: Vec<String>,
}

impl Vocabulary {
    pub fn load(path: &Path) -> Result<Self> {
        let text = std::fs::read_to_string(path)
            .with_context(|| format!("reading vocabulary {}", path.display()))?;
        Self::parse(&text).with_context(|| format!("parsing vocabulary {}", path.display()))
    }

    pub fn parse(text: &str) -> Result<Self> {
        let json: serde_json::Value = serde_json::from_str(text)?;
        let mut pieces = Vec::new();
        match json {
            serde_json::Value::Object(map) => {
                for (key, value) in map {
                    let (Ok(id), Some(piece)) = (key.parse::<usize>(), value.as_str()) else {
                        continue;
                    };
                    ensure!(id < MAX_PIECES, "vocabulary id {id} is out of range");
                    if pieces.len() <= id {
                        pieces.resize(id + 1, String::new());
                    }
                    pieces[id] = piece.to_owned();
                }
            }
            serde_json::Value::Array(items) => {
                ensure!(
                    items.len() <= MAX_PIECES,
                    "vocabulary has {} pieces",
                    items.len()
                );
                pieces = items
                    .iter()
                    .map(|item| item.as_str().unwrap_or_default().to_owned())
                    .collect();
            }
            _ => bail!("unexpected vocabulary format"),
        }
        Ok(Self { pieces })
    }

    #[cfg(test)]
    pub fn from_entries(entries: &[(usize, &str)]) -> Self {
        let mut pieces =
            vec![String::new(); entries.iter().map(|(id, _)| id + 1).max().unwrap_or(0)];
        for (id, piece) in entries {
            pieces[*id] = (*piece).to_owned();
        }
        Self { pieces }
    }

    pub fn piece(&self, id: usize) -> &str {
        self.pieces.get(id).map(String::as_str).unwrap_or_default()
    }

    pub fn decode(&self, tokens: &[usize]) -> String {
        let joined: String = tokens.iter().map(|&id| self.piece(id)).collect();
        joined
            .replace(WORD_BOUNDARY, " ")
            .trim_matches(is_whitespace)
            .to_owned()
    }

    pub fn splice_safe_ids(&self) -> HashSet<usize> {
        self.pieces
            .iter()
            .enumerate()
            .filter(|(_, piece)| is_splice_safe_piece(piece))
            .map(|(id, _)| id)
            .collect()
    }

    /// Maps ids of pieces that differ only in case to one shared id, preferring the lower-case piece.
    pub fn case_variant_ids(&self) -> HashMap<usize, usize> {
        let mut groups: HashMap<String, Vec<usize>> = HashMap::new();
        for (id, piece) in self.pieces.iter().enumerate() {
            groups.entry(piece.to_lowercase()).or_default().push(id);
        }
        let mut canonical = HashMap::new();
        for (folded, ids) in groups {
            if ids.len() < 2 {
                continue;
            }
            let target = ids
                .iter()
                .copied()
                .find(|&id| self.pieces[id] == folded)
                .unwrap_or_else(|| *ids.iter().min().expect("group is not empty"));
            for id in ids {
                canonical.insert(id, target);
            }
        }
        canonical
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn object_vocabularies_are_read_by_id_and_skip_foreign_entries() {
        let vocabulary =
            Vocabulary::parse(r#"{"0": "▁a", "2": "b", "x": "ignored", "3": 5}"#).unwrap();
        assert_eq!(
            (0..5).map(|id| vocabulary.piece(id)).collect::<Vec<_>>(),
            ["▁a", "", "b", "", ""]
        );
    }

    #[test]
    fn array_vocabularies_are_read_in_order() {
        let vocabulary = Vocabulary::parse(r#"["▁a", "b", 7]"#).unwrap();
        assert_eq!(
            (0..3).map(|id| vocabulary.piece(id)).collect::<Vec<_>>(),
            ["▁a", "b", ""]
        );
    }

    #[test]
    fn damaged_vocabularies_are_rejected() {
        assert!(Vocabulary::parse(&format!(r#"{{"{}": "x"}}"#, MAX_PIECES - 1)).is_ok());
        for text in [
            format!(r#"{{"{MAX_PIECES}": "x"}}"#),
            r#"{"18446744073709551615": "x"}"#.to_owned(),
            r#""▁a""#.to_owned(),
            "not json".to_owned(),
        ] {
            assert!(Vocabulary::parse(&text).is_err(), "{text}");
        }
    }

    #[test]
    fn decoding_turns_word_boundaries_into_spaces() {
        let vocabulary =
            Vocabulary::from_entries(&[(1, "▁Hello"), (2, "▁wor"), (3, "ld"), (4, ","), (5, "▁")]);
        assert_eq!(vocabulary.decode(&[1, 2, 3, 4]), "Hello world,");
        assert_eq!(vocabulary.decode(&[5, 1, 99, 5]), "Hello");
        assert_eq!(vocabulary.decode(&[]), "");
    }

    #[test]
    fn case_variants_share_the_lower_case_id() {
        let vocabulary = Vocabulary::from_entries(&[
            (1, "▁The"),
            (2, "▁the"),
            (3, "▁THE"),
            (4, "▁cat"),
            (5, "▁Ab"),
            (6, "▁AB"),
        ]);
        let variants = vocabulary.case_variant_ids();
        for id in [1, 2, 3] {
            assert_eq!(variants.get(&id), Some(&2), "{id}");
        }
        assert_eq!(variants.get(&4), None);
        assert_eq!((variants.get(&5), variants.get(&6)), (Some(&5), Some(&5)));
    }
}
