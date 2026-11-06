//! How each local model runs, and the repository commits its files are downloaded from.

use super::hf::{Checksum, RemoteFile, Repo};
use super::parakeet::Version;
use crate::models::LocalModel;

pub const WHISPER_REPO: Repo = Repo {
    name: "ggerganov/whisper.cpp",
    revision: "5359861c739e955e79d9a303bcbc70fb988958b1",
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Engine {
    Whisper(WhisperFile),
    Parakeet(Version),
}

/// A ggml file of [`WHISPER_REPO`]: its size and SHA-256 come from the repository's file listing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WhisperFile {
    pub name: &'static str,
    pub size: u64,
    pub sha256: [u8; 32],
}

impl WhisperFile {
    pub fn remote(self) -> RemoteFile {
        RemoteFile {
            path: self.name.to_owned(),
            size: self.size,
            checksum: Checksum::Sha256(self.sha256),
        }
    }
}

const fn whisper(name: &'static str, size: u64, sha256: &str) -> Engine {
    let Some(sha256) = Checksum::parse_hex(sha256) else {
        panic!("invalid SHA-256");
    };
    Engine::Whisper(WhisperFile { name, size, sha256 })
}

const TINY: Engine = whisper(
    "ggml-tiny.bin",
    77_691_713,
    "be07e048e1e599ad46341c8d2a135645097a538221678b7acdd1b1919c6e1b21",
);
const BASE: Engine = whisper(
    "ggml-base.bin",
    147_951_465,
    "60ed5bc3dd14eea856493d334349b405782ddcaf0028d4b5df4088345fba2efe",
);
const SMALL: Engine = whisper(
    "ggml-small.bin",
    487_601_967,
    "1be3a9b2063867b937e64e2ec7483364a79917e157fa98c5d94b5c1fffea987b",
);
const MEDIUM: Engine = whisper(
    "ggml-medium.bin",
    1_533_763_059,
    "6c14d5adee5f86394037b4e4e8b59f1673b6cee10e3cf0b11bbdbee79c156208",
);
const LARGE_V3: Engine = whisper(
    "ggml-large-v3.bin",
    3_095_033_483,
    "64d182b440b98d5203c4f9bd541544d84c605196c4f7b845dfa11fb23594d1e2",
);
const LARGE_V3_TURBO: Engine = whisper(
    "ggml-large-v3-turbo.bin",
    1_624_555_275,
    "1fc70f774d38eb169993ac391eea357ef47c88757ef72ee5943879b7e8e2bc69",
);

pub fn engine(model: LocalModel) -> Engine {
    match model {
        LocalModel::Tiny => TINY,
        LocalModel::Base => BASE,
        LocalModel::Small => SMALL,
        LocalModel::Medium => MEDIUM,
        LocalModel::LargeV3 => LARGE_V3,
        LocalModel::LargeV3Turbo => LARGE_V3_TURBO,
        LocalModel::ParakeetV2 => Engine::Parakeet(Version::V2),
        LocalModel::ParakeetUltra => Engine::Parakeet(Version::Ultra),
    }
}

/// The checksums of every file come from the listing at this commit.
pub fn parakeet_repo(version: Version) -> Repo {
    Repo {
        name: version.repo(),
        revision: match version {
            Version::V2 => "ee09c569f73759e6d44c9bd16766f477b2b36d39",
            Version::Ultra => "95eaa59a39d4394f047a4dc5cce480388a60d1b6",
        },
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    use crate::models::ModelFamily;

    #[test]
    fn every_local_model_has_its_own_engine_matching_its_family() {
        let engines: HashSet<String> = LocalModel::ALL
            .iter()
            .map(|&model| format!("{:?}", engine(model)))
            .collect();
        assert_eq!(engines.len(), LocalModel::ALL.len());
        for model in LocalModel::ALL {
            let family = match engine(model) {
                Engine::Whisper(_) => ModelFamily::Whisper,
                Engine::Parakeet(_) => ModelFamily::Parakeet,
            };
            assert_eq!(model.family(), family, "{}", model.raw_value());
        }
    }

    #[test]
    fn pinned_revisions_are_full_commit_hashes() {
        for repo in [
            WHISPER_REPO,
            parakeet_repo(Version::V2),
            parakeet_repo(Version::Ultra),
        ] {
            assert_eq!(repo.revision.len(), 40, "{}", repo.name);
            assert!(repo.revision.bytes().all(|byte| byte.is_ascii_hexdigit()));
        }
    }
}
