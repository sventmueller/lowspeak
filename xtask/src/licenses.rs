//! The license texts of everything the app contains, which their licenses ask to go along with it: Lowspeak's own,
//! those of the crates in its program, of whisper.cpp inside whisper-rs-sys and of the FluidAudio code translated in
//! `src/transcription/parakeet`.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Result, bail};

use crate::*;

/// The file in Contents/Resources.
pub(crate) const ACKNOWLEDGEMENTS: &str = "Acknowledgements.txt";

const TARGET: &str = "aarch64-apple-darwin";

/// The version of FluidAudio that `src/transcription/parakeet` follows.
const FLUIDAUDIO_VERSION: &str = "0.17.7";

/// Of the licenses a crate offers as alternatives, Lowspeak takes the first one of these.
const PREFERRED: [&str; 9] = [
    "MIT",
    "Zlib",
    "BSD-2-Clause",
    "BSD-3-Clause",
    "BSL-1.0",
    "0BSD",
    "ISC",
    "Unlicense",
    "Apache-2.0",
];

/// A phrase only the full text of a license holds. BSD stands for both of its kinds; ISC comes before 0BSD, whose text
/// starts with the same words.
const MARKS: [(&str, &str); 11] = [
    (
        "TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION",
        "Apache-2.0",
    ),
    ("Boost Software License", "BSL-1.0"),
    ("UNICODE LICENSE V3", "Unicode-3.0"),
    ("UNICODE, INC. LICENSE AGREEMENT", "Unicode-DFS-2016"),
    (
        "Community Data License Agreement - Permissive",
        "CDLA-Permissive-2.0",
    ),
    ("Permission is hereby granted, free of charge", "MIT"),
    ("Redistribution and use in source and binary forms", "BSD"),
    ("provided 'as-is'", "Zlib"),
    (
        "free and unencumbered software released into the public domain",
        "Unlicense",
    ),
    (
        "Permission to use, copy, modify, and/or distribute this software for any purpose with or without fee is hereby granted, provided that the above copyright notice and this permission notice appear in all copies",
        "ISC",
    ),
    (
        "Permission to use, copy, modify, and/or distribute this software for any purpose with or without fee",
        "0BSD",
    ),
];

/// A crate in the program, from a line of `cargo tree` in the format `{p}|{l}|{r}`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Crate {
    name: String,
    version: String,
    path: Option<PathBuf>,
    license: String,
    repository: String,
}

fn parse_crate(line: &str) -> Option<Crate> {
    let mut fields = line.split('|');
    let package = fields.next()?;
    let license = fields.next()?.trim().to_owned();
    let repository = fields.next().unwrap_or("").trim().to_owned();
    let (name, rest) = package.split_once(" v")?;
    let mut rest = rest.splitn(2, ' ');
    let version = rest
        .next()
        .filter(|v| v.starts_with(|c: char| c.is_ascii_digit()))?
        .to_owned();
    let path = rest
        .next()
        .and_then(|p| p.strip_prefix('(')?.strip_suffix(')'))
        .filter(|p| p.starts_with('/'))
        .map(PathBuf::from);
    Some(Crate {
        name: name.to_owned(),
        version,
        path,
        license,
        repository,
    })
}

/// The crates linked into the program, without the workspace's own.
fn crates(root: &Path) -> Result<Vec<Crate>> {
    let tree = output(Command::new(env!("CARGO")).current_dir(root).args([
        "tree",
        "--locked",
        "--package",
        "lowspeak",
        "--edges",
        "normal,no-proc-macro",
        "--target",
        TARGET,
        "--prefix",
        "none",
        "--no-dedupe",
        "--format",
        "{p}|{l}|{r}",
    ]))?;
    let mut found = BTreeSet::new();
    for line in tree.lines().filter(|l| !l.trim().is_empty()) {
        found.insert(parse_crate(line).with_context(|| format!("cargo tree: {line:?}"))?);
    }
    let own = |c: &Crate| c.path.as_ref().is_some_and(|p| p.starts_with(root));
    Ok(found.into_iter().filter(|c| !own(c)).collect())
}

/// Where cargo unpacked a crate from the registry, or the folder of a path dependency.
fn source(c: &Crate) -> Result<PathBuf> {
    if let Some(path) = &c.path {
        return Ok(path.clone());
    }
    let home = std::env::var_os("CARGO_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::home_dir().map(|home| home.join(".cargo")))
        .context("neither CARGO_HOME nor a home directory is set")?;
    let registries = home.join("registry/src");
    std::fs::read_dir(&registries)
        .with_context(|| format!("reading {}", registries.display()))?
        .flatten()
        .map(|registry| registry.path().join(format!("{}-{}", c.name, c.version)))
        .find(|dir| dir.is_dir())
        .with_context(|| {
            format!(
                "{} {}: no source in {}",
                c.name,
                c.version,
                registries.display()
            )
        })
}

/// The licenses a crate is used under: the preferred one of each set of alternatives in its SPDX expression, and every
/// license it requires together with another.
fn chosen(expression: &str) -> Result<Vec<String>> {
    let required: Vec<&str> = expression.split(" AND ").map(str::trim).collect();
    required
        .iter()
        .map(|part| {
            let alternatives = match part.strip_prefix('(').and_then(|p| p.strip_suffix(')')) {
                Some(inner) => inner,
                // SPDX binds AND before OR, so `A OR B AND C` is not a choice between all three.
                None if required.len() > 1 && part.contains(" OR ") => {
                    bail!("license {expression:?} is not understood")
                }
                None => part,
            };
            let options: Vec<&str> = alternatives
                .split(" OR ")
                .flat_map(|o| o.split('/'))
                .map(str::trim)
                .collect();
            let valid = |o: &&str| {
                !o.is_empty()
                    && o.chars()
                        .all(|c| c.is_ascii_alphanumeric() || "-.+".contains(c))
            };
            if !options.iter().all(valid) {
                bail!("license {expression:?} is not understood");
            }
            let taken = PREFERRED
                .iter()
                .find(|p| options.contains(p))
                .map_or(options[0], |p| *p);
            Ok(taken.to_owned())
        })
        .collect()
}

/// The words of a text, without the comment markers some license files put in front of each line.
fn words(text: &str) -> String {
    text.split_whitespace()
        .filter(|word| !matches!(*word, "//" | "#" | "*" | "/*" | "*/"))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Which license a file holds the full text of. The phrase that comes first decides, since a file may go on with the
/// licenses of code its crate took in, and the Boost license repeats MIT's phrase after its name.
fn kind(text: &str) -> Option<&'static str> {
    let text = words(text);
    let (at, license) = MARKS
        .iter()
        .filter_map(|(mark, license)| text.find(mark).map(|at| (at, *license)))
        .min_by_key(|(at, _)| *at)?;
    if license != "BSD" {
        return Some(license);
    }
    let conditions = &text[at..];
    let conditions = conditions
        .find("THIS SOFTWARE IS PROVIDED")
        .map_or(conditions, |end| &conditions[..end]);
    Some(if conditions.contains("Neither the name") {
        "BSD-3-Clause"
    } else {
        "BSD-2-Clause"
    })
}

/// The names in the `authors` of a crate's manifest, without their addresses.
fn authors(manifest: &str) -> Vec<String> {
    let Some((_, rest)) = manifest.split_once("\nauthors = [") else {
        return Vec::new();
    };
    let list = rest.split(']').next().unwrap_or("");
    list.split('"')
        .skip(1)
        .step_by(2)
        .map(|author| {
            author
                .split(" <")
                .next()
                .unwrap_or(author)
                .trim()
                .to_owned()
        })
        .filter(|author| !author.is_empty())
        .collect()
}

/// The texts a crate is used under, from its own license files. A crate that names MIT and ships no text of it gets
/// Lowspeak's MIT text with the holders its manifest names; the Unlicense asks for no text to go along.
fn texts(c: &Crate, dir: &Path, mit_terms: &str) -> Result<Vec<String>> {
    let mut paths: Vec<PathBuf> = std::fs::read_dir(dir)
        .with_context(|| format!("reading {}", dir.display()))?
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_file())
        .collect();
    paths.sort();
    let mut licenses = Vec::new();
    let mut notices = Vec::new();
    for path in paths {
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_ascii_uppercase())
            .unwrap_or_default();
        if ["LICENSE", "LICENCE", "COPYING", "UNLICENSE"]
            .iter()
            .any(|p| name.starts_with(p))
        {
            let text = read(&path)?;
            if let Some(license) = kind(&text) {
                licenses.push((license, text));
            }
        } else if name.starts_with("NOTICE") {
            notices.push(read(&path)?);
        }
    }
    // Unicode-3.0 replaced Unicode-DFS-2016, and crates name the one while shipping the other.
    let same =
        |a: &str, b: &str| a == b || (a.starts_with("Unicode-") && b.starts_with("Unicode-"));
    let mut out: Vec<String> = Vec::new();
    for license in chosen(&c.license)? {
        let before = out.len();
        for (_, text) in licenses.iter().filter(|(l, _)| same(l, &license)) {
            if !out.iter().any(|t| words(t) == words(text)) {
                out.push(text.clone());
            }
        }
        if out.len() == before {
            match license.as_str() {
                "MIT" => {
                    let named = authors(&read(&dir.join("Cargo.toml"))?);
                    let holders = if !named.is_empty() {
                        named.join(", ")
                    } else if !c.repository.is_empty() {
                        format!("the authors of {}", c.repository)
                    } else {
                        format!("the authors of {}", c.name)
                    };
                    out.push(format!("Copyright (c) {holders}\n\n{mit_terms}"));
                }
                "Unlicense" => out.push(
                    "Released into the public domain under the Unlicense (https://unlicense.org)."
                        .to_owned(),
                ),
                _ => bail!(
                    "{} {}: no {license} text in {}",
                    c.name,
                    c.version,
                    dir.display()
                ),
            }
        }
        // Apache 2.0 asks for a NOTICE file to go along; the other licenses do not.
        if license == "Apache-2.0" {
            out.extend(notices.iter().cloned());
        }
    }
    Ok(out)
}

fn read(path: &Path) -> Result<String> {
    std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))
}

/// A license text as written, without the blank lines and trailing spaces around and in it.
fn tidy(text: &str) -> String {
    let lines: Vec<&str> = text.lines().map(str::trim_end).collect();
    lines.join("\n").trim_matches('\n').to_owned()
}

/// The text of `ACKNOWLEDGEMENTS`: Lowspeak's licenses first, then each license text once, under the components it
/// covers.
pub(crate) fn acknowledgements() -> Result<String> {
    let root = root();
    let mit = read(&root.join("LICENSE-MIT"))?;
    let mit_terms = mit
        .find("Permission is hereby granted")
        .map(|start| &mit[start..])
        .context("LICENSE-MIT holds no MIT terms")?;
    let mut covered: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut whisper_cpp = None;
    for c in crates(&root)? {
        let dir = source(&c)?;
        if c.name == "whisper-rs-sys" {
            whisper_cpp = Some((
                format!("whisper.cpp (in whisper-rs-sys {})", c.version),
                read(&dir.join("whisper.cpp").join("LICENSE"))?,
            ));
        }
        let text = texts(&c, &dir, mit_terms)?
            .iter()
            .map(|t| tidy(t))
            .collect::<Vec<_>>()
            .join("\n\n");
        covered
            .entry(text)
            .or_default()
            .push(format!("{} {}", c.name, c.version));
    }
    let whisper_cpp = whisper_cpp.context("whisper-rs-sys is not in the program")?;
    let fluidaudio = (
        format!("FluidAudio {FLUIDAUDIO_VERSION} (parts translated to Rust)"),
        read(
            &root
                .join("src")
                .join("transcription")
                .join("parakeet")
                .join("LICENSE-FluidAudio"),
        )?,
    );
    for (label, text) in [whisper_cpp, fluidaudio] {
        covered.entry(tidy(&text)).or_default().push(label);
    }
    let mut sections: Vec<(Vec<String>, String)> = covered
        .into_iter()
        .map(|(text, mut names)| {
            names.sort_by_key(|n| n.to_lowercase());
            (names, text)
        })
        .collect();
    sections.sort_by_key(|(names, _)| names[0].to_lowercase());
    let own = [tidy(&mit), tidy(&read(&root.join("LICENSE-APACHE"))?)].join("\n\n");
    let mut out = format!("## {APP_NAME}\n\n{own}\n");
    for (names, text) in sections {
        out.push_str(&format!("\n## {}\n\n{text}\n", names.join(", ")));
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_component_of_the_program_comes_with_its_license_text() {
        let text = acknowledgements().unwrap();
        for part in [
            "## Lowspeak",
            "Copyright (c) 2026 Sven Müller",
            "whisper.cpp (in whisper-rs-sys",
            "Copyright (c) 2023-2024 The ggml authors",
            "FluidAudio 0.17.7",
            "objc2 0.6",
            "ring 0.17",
            "webpki-roots",
            "Community Data License Agreement",
        ] {
            assert!(text.contains(part), "{part}");
        }
    }

    #[test]
    fn licenses_are_told_apart_by_their_text() {
        let isc = "Permission to use, copy, modify, and/or distribute this software for any\n// purpose with or \
                   without fee is hereby granted, provided that the above copyright notice and this permission \
                   notice appear in all copies.";
        assert_eq!(kind(isc), Some("ISC"));
        let zero = "Permission to use, copy, modify, and/or distribute this software for any purpose with or \
                    without fee is hereby granted.";
        assert_eq!(kind(zero), Some("0BSD"));
        assert_eq!(chosen("Apache-2.0 AND ISC").unwrap(), ["Apache-2.0", "ISC"]);
        assert_eq!(chosen("Apache-2.0 OR ISC OR MIT").unwrap(), ["MIT"]);
        assert_eq!(chosen("MIT/Apache-2.0").unwrap(), ["MIT"]);
        assert!(chosen("MIT OR Apache-2.0 AND ISC").is_err());
    }

    #[test]
    fn a_line_of_cargo_tree_names_the_crate() {
        let registry = parse_crate("objc2 v0.6.5|MIT|https://github.com/madsmtm/objc2").unwrap();
        assert_eq!(
            (
                registry.name.as_str(),
                registry.version.as_str(),
                registry.path
            ),
            ("objc2", "0.6.5", None)
        );
        let own = parse_crate("lowspeak v0.0.0 (/Users/x/lowspeak)|MIT OR Apache-2.0|").unwrap();
        assert_eq!(own.path, Some(PathBuf::from("/Users/x/lowspeak")));
    }
}
