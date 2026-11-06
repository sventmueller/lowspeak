//! API keys as generic passwords in the login keychain, with the bundle ID as service and the
//! provider as account. `LOWSPEAK_API_KEY_FILE` names a key file to use instead, for tests.

use std::collections::BTreeMap;
use std::io::Write as _;
use std::os::unix::fs::OpenOptionsExt as _;
use std::path::PathBuf;

use anyhow::{Context as _, Result, bail, ensure};
use security_framework::passwords::{
    PasswordOptions, delete_generic_password, generic_password, set_generic_password,
};

use crate::models::ApiProvider;
use crate::util::{filelog, paths};

const OVERRIDE_ENV: &str = "LOWSPEAK_API_KEY_FILE";
const SERVICE: &str = "com.lowspeak.app";
/// `errSecItemNotFound`.
const ITEM_NOT_FOUND: i32 = -25_300;

/// Where the pipeline finds API keys; tests supply their own.
pub trait KeyStore {
    fn load(&self, provider: ApiProvider) -> Option<String>;
}

/// The keys of this Mac: the keychain, or the file `LOWSPEAK_API_KEY_FILE` names.
pub struct KeyFile;

impl KeyStore for KeyFile {
    fn load(&self, provider: ApiProvider) -> Option<String> {
        load(provider)
    }
}

/// A place that holds at most one key per provider.
trait Vault {
    fn read(&self, provider: ApiProvider) -> Result<Option<String>>;
    fn write(&self, provider: ApiProvider, key: &str) -> Result<()>;
    fn erase(&self, provider: ApiProvider) -> Result<()>;
}

struct Keychain {
    service: String,
}

impl Vault for Keychain {
    fn read(&self, provider: ApiProvider) -> Result<Option<String>> {
        match generic_password(PasswordOptions::new_generic_password(
            &self.service,
            provider.raw_value(),
        )) {
            Ok(bytes) => Ok(Some(String::from_utf8(bytes).with_context(|| {
                format!(
                    "the {} key in the keychain is not text",
                    provider.raw_value()
                )
            })?)),
            Err(error) if error.code() == ITEM_NOT_FOUND => Ok(None),
            Err(error) => {
                bail!(
                    "reading the {} key from the keychain: {error}",
                    provider.raw_value()
                )
            }
        }
    }

    fn write(&self, provider: ApiProvider, key: &str) -> Result<()> {
        set_generic_password(&self.service, provider.raw_value(), key.as_bytes())
            .with_context(|| format!("storing the {} key in the keychain", provider.raw_value()))
    }

    fn erase(&self, provider: ApiProvider) -> Result<()> {
        match delete_generic_password(&self.service, provider.raw_value()) {
            Err(error) if error.code() != ITEM_NOT_FOUND => {
                bail!(
                    "removing the {} key from the keychain: {error}",
                    provider.raw_value()
                )
            }
            _ => Ok(()),
        }
    }
}

/// One key per line, mode 0600, replaced atomically.
struct KeyLines {
    path: PathBuf,
}

impl KeyLines {
    fn load_all(&self) -> BTreeMap<ApiProvider, String> {
        let Ok(content) = std::fs::read_to_string(&self.path) else {
            return BTreeMap::new();
        };
        content
            .lines()
            .map(str::trim)
            .filter_map(|key| {
                ApiProvider::from_api_key(key).map(|provider| (provider, key.to_owned()))
            })
            .collect()
    }

    fn remove(&self) -> Result<()> {
        match std::fs::remove_file(&self.path) {
            Err(error) if error.kind() != std::io::ErrorKind::NotFound => {
                Err(error).with_context(|| format!("deleting {}", self.path.display()))
            }
            _ => Ok(()),
        }
    }

    fn write_all(&self, keys: &BTreeMap<ApiProvider, String>) -> Result<()> {
        if keys.is_empty() {
            return self.remove();
        }
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("creating {}", parent.display()))?;
        }
        let content = keys.values().cloned().collect::<Vec<_>>().join("\n");
        let temp = self.path.with_file_name(".api-key.tmp");
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .mode(0o600)
            .open(&temp)
            .with_context(|| format!("writing {}", temp.display()))?;
        file.write_all(content.as_bytes())?;
        file.sync_all()?;
        std::fs::rename(&temp, &self.path)
            .with_context(|| format!("replacing {}", self.path.display()))?;
        Ok(())
    }
}

impl Vault for KeyLines {
    fn read(&self, provider: ApiProvider) -> Result<Option<String>> {
        Ok(self.load_all().remove(&provider))
    }

    fn write(&self, provider: ApiProvider, key: &str) -> Result<()> {
        let mut keys = self.load_all();
        keys.insert(provider, key.to_owned());
        self.write_all(&keys)
    }

    fn erase(&self, provider: ApiProvider) -> Result<()> {
        let mut keys = self.load_all();
        if keys.remove(&provider).is_some() {
            self.write_all(&keys)?;
        }
        Ok(())
    }
}

fn override_file() -> Option<KeyLines> {
    std::env::var_os(OVERRIDE_ENV)
        .filter(|path| !path.is_empty())
        .map(|path| KeyLines {
            path: PathBuf::from(path),
        })
}

/// Where earlier versions kept the keys in plain text.
fn legacy_file() -> KeyLines {
    KeyLines {
        path: paths::app_dir().join(".api-key"),
    }
}

fn keychain() -> Keychain {
    Keychain {
        service: SERVICE.to_owned(),
    }
}

/// Stores a key, replacing an existing key of the same provider.
pub fn save(api_key: &str) -> Result<ApiProvider> {
    let key = api_key.trim();
    let Some(provider) = ApiProvider::from_api_key(key) else {
        bail!("Unknown API key format. Expected sk-... (OpenAI) or gsk_... (Groq).");
    };
    match override_file() {
        Some(file) => file.write(provider, key)?,
        None => keychain().write(provider, key)?,
    }
    Ok(provider)
}

pub fn load(provider: ApiProvider) -> Option<String> {
    let found = match override_file() {
        Some(file) => file.read(provider),
        None => keychain().read(provider).map(|key| {
            // Keys stay readable from the old file until a migration succeeds.
            key.or_else(|| legacy_file().load_all().remove(&provider))
        }),
    };
    found.unwrap_or_else(|error| {
        flog!(filelog::APP, "API key unavailable: {error:#}");
        None
    })
}

pub fn configured_providers() -> Vec<ApiProvider> {
    ApiProvider::ALL
        .into_iter()
        .filter(|&provider| load(provider).is_some())
        .collect()
}

pub fn delete_all() -> Result<()> {
    if let Some(file) = override_file() {
        return file.remove();
    }
    let keychain = keychain();
    for provider in ApiProvider::ALL {
        keychain.erase(provider)?;
    }
    legacy_file().remove()
}

/// Moves keys from the plain-text file of earlier versions into the keychain.
pub fn migrate() {
    let legacy = legacy_file();
    if override_file().is_some() || !legacy.path.exists() {
        return;
    }
    match move_keys(&legacy, &keychain()) {
        Ok(count) => flog!(filelog::APP, "Moved {count} API key(s) into the keychain"),
        Err(error) => flog!(
            filelog::APP,
            "Moving the API keys into the keychain failed, the key file stays: {error:#}"
        ),
    }
}

/// The file is deleted only once the target returns every key; a key the target already holds is
/// newer than the file's and stays.
fn move_keys(from: &KeyLines, to: &dyn Vault) -> Result<usize> {
    let keys = from.load_all();
    let mut moved = 0;
    for (&provider, key) in &keys {
        if to.read(provider)?.is_none() {
            to.write(provider, key)?;
            ensure!(
                to.read(provider)?.as_deref() == Some(key.as_str()),
                "the {} key did not read back",
                provider.raw_value()
            );
            moved += 1;
        }
    }
    from.remove()?;
    Ok(moved)
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::os::unix::fs::PermissionsExt as _;

    use super::*;

    fn temp_file(name: &str) -> KeyLines {
        let dir = std::env::temp_dir().join(format!("lowspeak-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        KeyLines {
            path: dir.join(".api-key"),
        }
    }

    #[test]
    fn keys_of_both_providers_are_stored_privately_and_removed_together() {
        let file = temp_file("keyfile");
        file.write(ApiProvider::OpenAi, "sk-test-openai-key")
            .unwrap();
        file.write(ApiProvider::Groq, "gsk_test_groq_key").unwrap();
        assert_eq!(
            file.read(ApiProvider::OpenAi).unwrap().as_deref(),
            Some("sk-test-openai-key")
        );
        assert_eq!(
            file.read(ApiProvider::Groq).unwrap().as_deref(),
            Some("gsk_test_groq_key")
        );
        assert_eq!(
            std::fs::metadata(&file.path).unwrap().permissions().mode() & 0o777,
            0o600
        );

        file.write(ApiProvider::OpenAi, "sk-replacement").unwrap();
        assert_eq!(
            file.read(ApiProvider::OpenAi).unwrap().as_deref(),
            Some("sk-replacement")
        );
        file.erase(ApiProvider::OpenAi).unwrap();
        assert_eq!(file.read(ApiProvider::OpenAi).unwrap(), None);
        file.erase(ApiProvider::Groq).unwrap();
        assert!(!file.path.exists());
        file.remove().unwrap();
        let _ = std::fs::remove_dir_all(file.path.parent().unwrap());
    }

    #[test]
    fn an_unknown_key_format_is_refused() {
        assert!(save("unknown").is_err());
    }

    /// A vault in memory that can refuse writes or forget what it was given.
    #[derive(Default)]
    struct Memory {
        keys: RefCell<BTreeMap<ApiProvider, String>>,
        refuse: bool,
        forget: bool,
    }

    impl Vault for Memory {
        fn read(&self, provider: ApiProvider) -> Result<Option<String>> {
            Ok(self.keys.borrow().get(&provider).cloned())
        }

        fn write(&self, provider: ApiProvider, key: &str) -> Result<()> {
            ensure!(!self.refuse, "refused");
            if !self.forget {
                self.keys.borrow_mut().insert(provider, key.to_owned());
            }
            Ok(())
        }

        fn erase(&self, provider: ApiProvider) -> Result<()> {
            self.keys.borrow_mut().remove(&provider);
            Ok(())
        }
    }

    fn legacy_with_both_keys(name: &str) -> KeyLines {
        let file = temp_file(name);
        file.write(ApiProvider::OpenAi, "sk-old").unwrap();
        file.write(ApiProvider::Groq, "gsk_old").unwrap();
        file
    }

    #[test]
    fn the_key_file_goes_once_every_key_reads_back() {
        let file = legacy_with_both_keys("migrate");
        let target = Memory::default();
        target.write(ApiProvider::Groq, "gsk_newer").unwrap();
        assert_eq!(move_keys(&file, &target).unwrap(), 1);
        assert!(!file.path.exists());
        assert_eq!(
            target.read(ApiProvider::OpenAi).unwrap().as_deref(),
            Some("sk-old")
        );
        assert_eq!(
            target.read(ApiProvider::Groq).unwrap().as_deref(),
            Some("gsk_newer")
        );
    }

    #[test]
    fn the_key_file_stays_when_the_target_fails() {
        for target in [
            Memory {
                refuse: true,
                ..Memory::default()
            },
            Memory {
                forget: true,
                ..Memory::default()
            },
        ] {
            let file = legacy_with_both_keys("migrate-fails");
            assert!(move_keys(&file, &target).is_err());
            assert_eq!(file.load_all().len(), 2);
            let _ = std::fs::remove_dir_all(file.path.parent().unwrap());
        }
    }

    #[test]
    #[ignore = "writes to and removes from the login keychain"]
    fn the_keychain_keeps_one_key_per_provider() {
        let keychain = Keychain {
            service: format!("com.lowspeak.tests.keychain.{}", std::process::id()),
        };
        assert_eq!(keychain.read(ApiProvider::Groq).unwrap(), None);
        keychain.write(ApiProvider::Groq, "gsk_first").unwrap();
        keychain.write(ApiProvider::Groq, "gsk_second").unwrap();
        assert_eq!(
            keychain.read(ApiProvider::Groq).unwrap().as_deref(),
            Some("gsk_second")
        );
        assert_eq!(keychain.read(ApiProvider::OpenAi).unwrap(), None);
        keychain.erase(ApiProvider::Groq).unwrap();
        keychain.erase(ApiProvider::Groq).unwrap();
        assert_eq!(keychain.read(ApiProvider::Groq).unwrap(), None);
    }
}
