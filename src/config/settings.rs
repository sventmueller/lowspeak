//! Persisted user choices in the `com.lowspeak.app` defaults domain.

use objc2_foundation::{NSString, NSUserDefaults};

use crate::models::{Language, LocalModel, OnlineModel, PostProcessingMode, TranscriptionMode};

const MODE: &str = "mode";
const LOCAL_MODEL: &str = "localModel";
const LANGUAGE: &str = "language";
const ONLINE_MODEL: &str = "onlineModel";
const PREPROCESSING: &str = "preprocessingEnabled";
const POST_PROCESSING: &str = "postProcessingMode";
const START_SOUND: &str = "startSound";
const MICROPHONE_ID: &str = "microphoneID";
const MICROPHONE_NAME: &str = "microphoneName";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Settings {
    pub mode: TranscriptionMode,
    pub local_model: LocalModel,
    pub online_model: OnlineModel,
    pub language: Language,
    pub post_processing: PostProcessingMode,
    /// Only `defaults write` changes it, so `save` leaves it out and a new default still reaches
    /// everyone who never set it.
    pub preprocessing: bool,
    /// A short sound once the microphone delivers audio.
    pub start_sound: bool,
}

/// A microphone chosen in the menu instead of the system default.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChosenMicrophone {
    pub id: String,
    /// Shown while the device is disconnected.
    pub name: String,
}

fn string(defaults: &NSUserDefaults, key: &str) -> Option<String> {
    defaults
        .stringForKey(&NSString::from_str(key))
        .map(|value| value.to_string())
}

fn bool_or(defaults: &NSUserDefaults, key: &str, default: bool) -> bool {
    let key = NSString::from_str(key);
    if defaults.objectForKey(&key).is_none() {
        default
    } else {
        defaults.boolForKey(&key)
    }
}

fn set_string(defaults: &NSUserDefaults, key: &str, value: &str) {
    let value = NSString::from_str(value);
    // SAFETY: NSString is a property-list type.
    unsafe { defaults.setObject_forKey(Some(&value), &NSString::from_str(key)) };
}

impl Settings {
    pub fn load() -> Self {
        Self::load_from(&NSUserDefaults::standardUserDefaults())
    }

    pub fn save(&self) {
        self.save_to(&NSUserDefaults::standardUserDefaults());
    }

    pub fn load_from(defaults: &NSUserDefaults) -> Self {
        let read = |key| string(defaults, key).unwrap_or_default();
        Self {
            mode: TranscriptionMode::from_raw(&read(MODE)).unwrap_or(TranscriptionMode::Online),
            local_model: LocalModel::from_raw(&read(LOCAL_MODEL))
                .unwrap_or(LocalModel::LargeV3Turbo),
            online_model: OnlineModel::from_raw(&read(ONLINE_MODEL))
                .unwrap_or(OnlineModel::GptTranscribe),
            language: Language::from_raw(&read(LANGUAGE)).unwrap_or(Language::Auto),
            post_processing: PostProcessingMode::from_raw(&read(POST_PROCESSING))
                .unwrap_or(PostProcessingMode::Off),
            preprocessing: bool_or(defaults, PREPROCESSING, true),
            start_sound: bool_or(defaults, START_SOUND, false),
        }
    }

    pub fn save_to(&self, defaults: &NSUserDefaults) {
        set_string(defaults, MODE, self.mode.raw_value());
        set_string(defaults, LOCAL_MODEL, self.local_model.raw_value());
        set_string(defaults, ONLINE_MODEL, self.online_model.raw_value());
        set_string(defaults, LANGUAGE, self.language.raw_value());
        set_string(defaults, POST_PROCESSING, self.post_processing.raw_value());
        defaults.setBool_forKey(self.start_sound, &NSString::from_str(START_SOUND));
    }
}

pub fn chosen_microphone() -> Option<ChosenMicrophone> {
    chosen_microphone_in(&NSUserDefaults::standardUserDefaults())
}

pub fn set_chosen_microphone(microphone: Option<&ChosenMicrophone>) {
    set_chosen_microphone_in(&NSUserDefaults::standardUserDefaults(), microphone);
}

pub fn chosen_microphone_in(defaults: &NSUserDefaults) -> Option<ChosenMicrophone> {
    Some(ChosenMicrophone {
        id: string(defaults, MICROPHONE_ID).filter(|id| !id.is_empty())?,
        name: string(defaults, MICROPHONE_NAME).unwrap_or_default(),
    })
}

pub fn set_chosen_microphone_in(defaults: &NSUserDefaults, microphone: Option<&ChosenMicrophone>) {
    match microphone {
        Some(microphone) => {
            set_string(defaults, MICROPHONE_ID, &microphone.id);
            set_string(defaults, MICROPHONE_NAME, &microphone.name);
        }
        None => {
            for key in [MICROPHONE_ID, MICROPHONE_NAME] {
                defaults.removeObjectForKey(&NSString::from_str(key));
            }
        }
    }
}

/// A defaults domain of its own, removed again on drop, so tests leave the app's settings alone.
#[cfg(test)]
pub(crate) struct TestDefaults {
    pub defaults: objc2::rc::Retained<NSUserDefaults>,
    name: objc2::rc::Retained<NSString>,
}

#[cfg(test)]
impl TestDefaults {
    pub fn new(test: &str) -> Self {
        use objc2::AnyThread as _;
        let name = NSString::from_str(&format!("com.lowspeak.tests.{test}.{}", std::process::id()));
        let defaults = NSUserDefaults::initWithSuiteName(NSUserDefaults::alloc(), Some(&name))
            .expect("a defaults suite");
        defaults.removePersistentDomainForName(&name);
        Self { defaults, name }
    }
}

#[cfg(test)]
impl Drop for TestDefaults {
    fn drop(&mut self) {
        self.defaults.removePersistentDomainForName(&self.name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nothing_saved_loads_the_defaults() {
        let test = TestDefaults::new("settings-empty");
        let settings = Settings::load_from(&test.defaults);
        assert_eq!(
            settings,
            Settings {
                mode: TranscriptionMode::Online,
                local_model: LocalModel::LargeV3Turbo,
                online_model: OnlineModel::GptTranscribe,
                language: Language::Auto,
                post_processing: PostProcessingMode::Off,
                preprocessing: true,
                start_sound: false,
            }
        );
        assert_eq!(chosen_microphone_in(&test.defaults), None);
    }

    #[test]
    fn saved_choices_load_again() {
        let test = TestDefaults::new("settings-roundtrip");
        let chosen = Settings {
            mode: TranscriptionMode::Local,
            local_model: LocalModel::ParakeetUltra,
            online_model: OnlineModel::GroqWhisperLargeV3,
            language: Language::PtPt,
            post_processing: PostProcessingMode::Format,
            preprocessing: true,
            start_sound: true,
        };
        chosen.save_to(&test.defaults);
        assert_eq!(Settings::load_from(&test.defaults), chosen);
    }

    #[test]
    fn preprocessing_is_read_but_never_written() {
        let test = TestDefaults::new("settings-preprocessing");
        let mut settings = Settings::load_from(&test.defaults);
        settings.preprocessing = false;
        settings.save_to(&test.defaults);
        assert!(Settings::load_from(&test.defaults).preprocessing);

        test.defaults
            .setBool_forKey(false, &NSString::from_str(PREPROCESSING));
        assert!(!Settings::load_from(&test.defaults).preprocessing);
    }

    #[test]
    fn unknown_values_fall_back_and_earlier_spellings_still_load() {
        let test = TestDefaults::new("settings-unknown");
        set_string(&test.defaults, MODE, "telepathy");
        set_string(&test.defaults, LOCAL_MODEL, "parakeet-v3");
        assert_eq!(
            Settings::load_from(&test.defaults),
            Settings {
                local_model: LocalModel::ParakeetUltra,
                ..Settings::load_from(&TestDefaults::new("settings-unknown-empty").defaults)
            }
        );
    }

    #[test]
    fn a_chosen_microphone_is_kept_until_the_default_is_chosen() {
        let test = TestDefaults::new("settings-microphone");
        let airpods = ChosenMicrophone {
            id: "coreaudio:AirPods".into(),
            name: "AirPods".into(),
        };
        set_chosen_microphone_in(&test.defaults, Some(&airpods));
        assert_eq!(chosen_microphone_in(&test.defaults), Some(airpods));
        set_chosen_microphone_in(&test.defaults, None);
        assert_eq!(chosen_microphone_in(&test.defaults), None);
    }
}
