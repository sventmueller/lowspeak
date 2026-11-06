//! The README in every language of the interface: `README.md` in English, `assets/readme/<folder>/README.md` for the
//! others, each linking all of them.

use std::path::{Path, PathBuf};

use crate::*;

/// The README languages in the order of the language bar, by folder and by their own name.
pub(crate) const READMES: [(&str, &str); 35] = [
    ("id", "Bahasa Indonesia"),
    ("ms", "Bahasa Melayu"),
    ("ca", "Català"),
    ("cs", "Čeština"),
    ("da", "Dansk"),
    ("de", "Deutsch"),
    ("en", "English"),
    ("es", "Español"),
    ("fr", "Français"),
    ("hr", "Hrvatski"),
    ("it", "Italiano"),
    ("hu", "Magyar"),
    ("nl", "Nederlands"),
    ("no", "Norsk"),
    ("pl", "Polski"),
    ("pt_BR", "Português (Brasil)"),
    ("pt_PT", "Português (Portugal)"),
    ("ro", "Română"),
    ("sk", "Slovenčina"),
    ("sl", "Slovenščina"),
    ("fi", "Suomi"),
    ("sv", "Svenska"),
    ("vi", "Tiếng Việt"),
    ("tr", "Türkçe"),
    ("el", "Ελληνικά"),
    ("ru", "Русский"),
    ("uk", "Українська"),
    ("he", "עברית"),
    ("ar", "العربية"),
    ("hi", "हिन्दी"),
    ("th", "ไทย"),
    ("ko", "한국어"),
    ("ja", "日本語"),
    ("zh-Hans", "简体中文"),
    ("zh-Hant", "繁體中文"),
];

const DOWNLOAD: &str =
    "https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg";

pub(crate) fn readme_path(folder: &str) -> PathBuf {
    if folder == "en" {
        root().join("README.md")
    } else {
        root().join("assets/readme").join(folder).join("README.md")
    }
}

/// The anchor GitHub gives a heading: lower case, spaces as hyphens, punctuation and symbols dropped.
pub(crate) fn anchor(heading: &str) -> String {
    const DROPPED: &str = "…›‹—–“”„‘’«»、。，：；！？（）・「」『』《》〈〉·•©®";
    heading
        .trim()
        .to_lowercase()
        .chars()
        .filter_map(|c| match c {
            ' ' => Some('-'),
            '-' | '_' => Some(c),
            c if c.is_ascii() => c.is_ascii_alphanumeric().then_some(c),
            c if c.is_whitespace() || DROPPED.contains(c) => None,
            c => Some(c),
        })
        .collect()
}

/// Link targets in Markdown and HTML, without the titles.
fn targets(text: &str) -> Vec<&str> {
    let mut found = Vec::new();
    for (opening, closing) in [("](", ')'), ("src=\"", '"')] {
        let mut rest = text;
        while let Some(at) = rest.find(opening) {
            rest = &rest[at + opening.len()..];
            if let Some(end) = rest.find(closing) {
                found.push(rest[..end].split_whitespace().next().unwrap_or(""));
            }
        }
    }
    found
}

/// What is wrong with the README of `folder`, one line per problem.
pub(crate) fn readme_problems(folder: &str) -> Vec<String> {
    let path = readme_path(folder);
    let Ok(text) = std::fs::read_to_string(&path) else {
        return vec![format!("{folder}: {} is missing", path.display())];
    };
    let dir = path.parent().unwrap_or(Path::new("."));
    let mut problems = Vec::new();
    if !text.contains(DOWNLOAD) {
        problems.push(format!("{folder}: no link to {DOWNLOAD}"));
    }
    for (other, name) in READMES {
        let expected = if other == folder {
            format!("**{name}**")
        } else {
            let target = readme_path(other);
            let relative = relative_to(dir, &target);
            format!("[{name}]({relative})")
        };
        if !text.contains(&expected) {
            problems.push(format!("{folder}: the language bar lacks {expected}"));
        }
    }
    let anchors: Vec<String> = text
        .lines()
        .filter_map(|line| {
            line.strip_prefix("## ")
                .or_else(|| line.strip_prefix("### "))
        })
        .map(anchor)
        .collect();
    for target in targets(&text) {
        if let Some(fragment) = target.strip_prefix('#') {
            if !anchors.iter().any(|a| a == fragment) {
                problems.push(format!("{folder}: no heading for #{fragment}"));
            }
        } else if !target.contains("://") && !target.starts_with("mailto:") {
            let file = target.split('#').next().unwrap_or(target);
            if !dir.join(file).exists() {
                problems.push(format!("{folder}: {target} does not exist"));
            }
        }
    }
    problems
}

/// `target` as a path from `dir`, both inside the repository, over their common folders.
fn relative_to(dir: &Path, target: &Path) -> String {
    let root = root();
    let from: Vec<_> = dir.strip_prefix(&root).unwrap_or(dir).iter().collect();
    let to: Vec<_> = target
        .strip_prefix(&root)
        .unwrap_or(target)
        .iter()
        .collect();
    let common = from.iter().zip(&to).take_while(|(a, b)| a == b).count();
    let mut parts: Vec<String> = vec!["..".to_owned(); from.len() - common];
    parts.extend(
        to[common..]
            .iter()
            .map(|part| part.to_string_lossy().into_owned()),
    );
    parts.join("/")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn languages_link_each_other_over_their_common_folder() {
        let german = readme_path("de");
        let dir = german.parent().unwrap();
        assert_eq!(relative_to(dir, &readme_path("id")), "../id/README.md");
        assert_eq!(relative_to(dir, &readme_path("en")), "../../../README.md");
        assert_eq!(
            relative_to(&root(), &readme_path("de")),
            "assets/readme/de/README.md"
        );
    }

    #[test]
    fn headings_become_github_anchors() {
        assert_eq!(anchor("Build & Deploy"), "build--deploy");
        assert_eq!(
            anchor("On your Mac: Whisper and Parakeet"),
            "on-your-mac-whisper-and-parakeet"
        );
        assert_eq!(
            anchor("Lock with Shift or a double press"),
            "lock-with-shift-or-a-double-press"
        );
        assert_eq!(
            anchor("Datenschutz und Sicherheit"),
            "datenschutz-und-sicherheit"
        );
        assert_eq!(anchor("Größe & Tempo"), "größe--tempo");
        assert_eq!(anchor("隐私与安全"), "隐私与安全");
        assert_eq!(anchor("Was ist „Lowspeak“?"), "was-ist-lowspeak");
    }

    /// Run with `cargo test -p xtask readme -- --nocapture` to see every problem at once.
    #[test]
    fn every_readme_links_all_languages_the_download_and_its_own_headings() {
        let languages = std::fs::read_dir(root().join("src/i18n"))
            .unwrap()
            .flatten()
            .filter(|entry| entry.file_name() != "mod.rs")
            .count();
        assert_eq!(
            READMES.len(),
            languages,
            "one README per interface language"
        );
        let problems: Vec<String> = READMES
            .iter()
            .flat_map(|(folder, _)| readme_problems(folder))
            .collect();
        for problem in &problems {
            println!("{problem}");
        }
        assert!(
            problems.is_empty(),
            "{} problems, listed above",
            problems.len()
        );
    }
}
