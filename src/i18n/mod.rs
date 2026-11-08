//! What the user reads, in the language chosen for dictation; logs stay English.

mod ar;
mod ca;
mod cs;
mod da;
mod de;
mod el;
mod en;
mod es;
mod fi;
mod fr;
mod he;
mod hi;
mod hr;
mod hu;
mod id;
mod it;
mod ja;
mod ko;
mod ms;
mod nl;
mod no;
mod pl;
mod pt;
mod pt_pt;
mod ro;
mod ru;
mod sk;
mod sl;
mod sv;
mod th;
mod tr;
mod uk;
mod vi;
mod zh;
mod zh_hant;

use objc2_foundation::{NSLocale, NSProcessInfo};

use crate::models::{Language, LanguageNote, LocalModel, OnlineModel, PostProcessingMode};

pub struct Texts {
    pub recent_dictations: &'static str,
    pub none_yet: &'static str,
    /// Offered while the recording of a failed dictation is kept.
    pub retry_failed_dictation: &'static str,
    pub local: &'static str,
    pub not_installed: fn(model: &str) -> String,
    pub no_key: fn(provider: &str) -> String,
    pub language: &'static str,
    pub auto_detect: &'static str,
    pub post_processing: &'static str,
    pub post_processing_off: &'static str,
    pub post_processing_cleanup: &'static str,
    pub post_processing_format: &'static str,
    /// Zero terms shows the plain entry.
    pub vocabulary_menu: fn(terms: usize) -> String,
    pub microphone: &'static str,
    pub system_default: &'static str,
    pub not_connected: fn(microphone: &str) -> String,
    pub play_sound_when_ready: &'static str,
    pub manage_models: &'static str,
    pub delete_model: fn(model: &str) -> String,
    pub download_model: fn(model: &str) -> String,
    pub set_api_key: &'static str,
    pub update_api_key: &'static str,
    pub open_at_login: &'static str,
    pub check_for_updates: &'static str,
    pub check_updates_automatically: &'static str,
    /// Sparkle downloads and installs on its own only with automatic checks.
    pub install_updates_automatically: &'static str,
    pub help: &'static str,
    pub quick_reference: &'static str,
    pub report_problem: &'static str,
    pub suggest_feature: &'static str,
    pub on_github: &'static str,
    pub about: &'static str,
    /// Names "MIT" and "Apache 2.0" as written, since the About panel links them.
    pub license: &'static str,
    pub quit: &'static str,

    pub reference_hold: &'static str,
    pub reference_double_tap: &'static str,
    pub reference_shift: &'static str,
    pub reference_stop: &'static str,
    pub reference_emoji: &'static str,
    pub keyboard_settings: &'static str,

    pub megabytes: &'static str,
    pub gigabytes: &'static str,
    pub decimal_separator: &'static str,
    pub english: &'static str,
    pub languages: fn(count: usize) -> String,
    pub legacy: &'static str,
    pub fast: &'static str,

    pub set_api_key_title: &'static str,
    pub update_api_key_title: &'static str,
    pub api_key_info: &'static str,
    pub api_key_placeholder: &'static str,
    pub save: &'static str,
    pub cancel: &'static str,
    pub ok: &'static str,
    pub remove_all_keys: &'static str,
    pub unknown_key_format: &'static str,
    pub unknown_key_format_info: &'static str,
    pub vocabulary_title: &'static str,
    pub vocabulary_info: &'static str,

    pub recording_failed: &'static str,
    pub microphone_silent: &'static str,
    pub microphone_silent_info: &'static str,
    pub accessibility_required: &'static str,
    pub accessibility_required_info: fn(setting: &str) -> String,
    /// Privacy & Security › Accessibility up to macOS 26, as Apple names it in the language.
    pub accessibility_list: &'static str,
    /// The same list from macOS 27 on, as Apple names it in the language.
    pub device_control_list: &'static str,
    pub transcription_failed: &'static str,
    pub retry_failed_dictation_info: fn(menu_item: &str) -> String,
    pub transcribed_locally: &'static str,
    pub transcribed_locally_info: fn(reason: &str) -> String,
    pub post_processing_failed: &'static str,
    pub post_processing_failed_info: fn(reason: &str) -> String,
    pub no_api_key: &'static str,
    pub no_api_key_info: fn(provider: &str, menu_item: &str) -> String,
    pub login_item_error: &'static str,
    pub login_item_error_info: fn(menu_item: &str) -> String,
    pub api_key_error: &'static str,
    pub api_key_not_saved: &'static str,
    pub api_keys_not_removed: &'static str,
    pub download_failed: &'static str,
    pub download_failed_info: fn(model: &str) -> String,
    pub cannot_delete: &'static str,
    pub cannot_delete_info: &'static str,
    pub delete_failed: &'static str,
    pub delete_failed_info: fn(model: &str) -> String,

    pub no_api_key_error: fn(provider: &str) -> String,
    pub invalid_response: &'static str,
    pub authentication_failed: &'static str,
    pub rate_limited: &'static str,
    pub server_error: fn(status: u16) -> String,
    pub api_error: fn(status: u16) -> String,
    pub model_not_available: fn(menu_item: &str) -> String,
    pub network_error: fn(message: &str) -> String,
    pub key_rejected: fn(provider: &str) -> String,
    pub quota_exhausted: fn(provider: &str) -> String,
    pub model_retired: fn(provider: &str) -> String,
    /// Post-processing works with a key of either provider.
    pub no_post_processing_key: fn(first: &str, second: &str) -> String,
    /// The provider answers 404 for the transcription model.
    pub online_model_retired: fn(provider: &str, model: &str) -> String,
    pub request_rejected: fn(provider: &str, status: u16) -> String,

    pub microphone_permission_denied: &'static str,
    pub no_microphone: &'static str,
    pub microphone_error: fn(detail: &str) -> String,
    pub paste_failed: &'static str,
    pub paste_failed_info: &'static str,
    /// Another app came to the front between the end of the recording and the paste.
    pub not_pasted: &'static str,
    pub not_pasted_info: &'static str,
    pub request_timed_out: &'static str,
    pub audio_failed: &'static str,
    pub local_model_failed: &'static str,
}

/// The texts for `language`; auto-detection follows the macOS language order.
pub fn texts(language: Language) -> &'static Texts {
    match ui_language(language) {
        Language::Auto | Language::En => &en::EN,
        Language::De => &de::DE,
        Language::Es => &es::ES,
        Language::Fr => &fr::FR,
        Language::It => &it::IT,
        Language::Ja => &ja::JA,
        Language::Ko => &ko::KO,
        Language::Nl => &nl::NL,
        Language::Pl => &pl::PL,
        Language::Pt => &pt::PT,
        Language::Ru => &ru::RU,
        Language::Uk => &uk::UK,
        Language::Zh => &zh::ZH,
        Language::Tr => &tr::TR,
        Language::Ar => &ar::AR,
        Language::Sv => &sv::SV,
        Language::Da => &da::DA,
        Language::No => &no::NO,
        Language::Fi => &fi::FI,
        Language::Cs => &cs::CS,
        Language::Ro => &ro::RO,
        Language::Hu => &hu::HU,
        Language::El => &el::EL,
        Language::Hi => &hi::HI,
        Language::Ca => &ca::CA,
        Language::Hr => &hr::HR,
        Language::He => &he::HE,
        Language::Id => &id::ID,
        Language::Ms => &ms::MS,
        Language::Sk => &sk::SK,
        Language::Sl => &sl::SL,
        Language::Th => &th::TH,
        Language::Vi => &vi::VI,
        Language::ZhHant => &zh_hant::ZH_HANT,
        Language::PtPt => &pt_pt::PT_PT,
    }
}

fn ui_language(language: Language) -> Language {
    if language != Language::Auto {
        return language;
    }
    NSLocale::preferredLanguages()
        .iter()
        .find_map(|tag| from_locale(&tag.to_string()))
        .unwrap_or(Language::En)
}

/// Maps a language tag such as `de-CH`, `zh-Hant-TW` or `nb-NO` to a dictation language. Chinese without a script
/// is Traditional in Taiwan, Hong Kong and Macao, and Portuguese without a region is Brazilian, as in macOS.
fn from_locale(tag: &str) -> Option<Language> {
    let mut parts = tag.split(['-', '_']);
    let primary = parts.next()?;
    let rest: Vec<&str> = parts.collect();
    let has = |part: &str| rest.iter().any(|p| p.eq_ignore_ascii_case(part));
    let language = match primary {
        "zh" if has("Hans") => Language::Zh,
        "zh" if has("Hant") || has("TW") || has("HK") || has("MO") => Language::ZhHant,
        "pt" if !rest.is_empty() && !has("BR") => Language::PtPt,
        "nb" | "nn" => Language::No,
        // Codes older systems still report for Hebrew and Indonesian.
        "iw" => Language::He,
        "in" => Language::Id,
        "zh" | "pt" => Language::from_raw(primary)?,
        other => {
            Language::from_raw(other).filter(|l| !matches!(l, Language::ZhHant | Language::PtPt))?
        }
    };
    (language != Language::Auto).then_some(language)
}

impl Texts {
    /// The list in Privacy & Security that grants Lowspeak the Accessibility permission, named as this Mac shows it.
    pub fn accessibility_setting(&self) -> &'static str {
        if NSProcessInfo::processInfo()
            .operatingSystemVersion()
            .majorVersion
            >= 27
        {
            self.device_control_list
        } else {
            self.accessibility_list
        }
    }

    pub fn language_name(&self, language: Language) -> &'static str {
        match language {
            Language::Auto => self.auto_detect,
            other => other.display_name(),
        }
    }

    pub fn post_processing_mode(&self, mode: PostProcessingMode) -> &'static str {
        match mode {
            PostProcessingMode::Off => self.post_processing_off,
            PostProcessingMode::Cleanup => self.post_processing_cleanup,
            PostProcessingMode::Format => self.post_processing_format,
        }
    }

    pub fn local_model(&self, model: LocalModel) -> String {
        let spec = model.spec();
        let note = match spec.languages {
            LanguageNote::Omitted => String::new(),
            LanguageNote::English => format!(", {}", self.english),
            LanguageNote::Count(count) => format!(", {}", (self.languages)(count)),
        };
        format!("{} (~{}{note})", spec.name, self.size(spec.megabytes))
    }

    pub fn online_model(&self, model: OnlineModel) -> String {
        match model {
            OnlineModel::Whisper1 => format!("{} ({})", model.name(), self.legacy),
            OnlineModel::GroqWhisperLargeV3Turbo => format!("{} ({})", model.name(), self.fast),
            _ => model.name().to_owned(),
        }
    }

    fn size(&self, megabytes: u32) -> String {
        if megabytes < 1000 {
            return format!("{megabytes} {}", self.megabytes);
        }
        let tenths = megabytes % 1000 / 100;
        if tenths == 0 {
            format!("{} {}", megabytes / 1000, self.gigabytes)
        } else {
            format!(
                "{}{}{tenths} {}",
                megabytes / 1000,
                self.decimal_separator,
                self.gigabytes
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn locale_tags_map_to_languages() {
        assert_eq!(from_locale("de-CH"), Some(Language::De));
        assert_eq!(from_locale("zh-Hans-CN"), Some(Language::Zh));
        assert_eq!(from_locale("zh-Hant-TW"), Some(Language::ZhHant));
        assert_eq!(from_locale("zh-HK"), Some(Language::ZhHant));
        assert_eq!(from_locale("zh-Hans-HK"), Some(Language::Zh));
        assert_eq!(from_locale("pt_BR"), Some(Language::Pt));
        assert_eq!(from_locale("pt"), Some(Language::Pt));
        assert_eq!(from_locale("pt-PT"), Some(Language::PtPt));
        assert_eq!(from_locale("he-IL"), Some(Language::He));
        assert_eq!(from_locale("iw"), Some(Language::He));
        assert_eq!(from_locale("ms-MY"), Some(Language::Ms));
        assert_eq!(from_locale("zh-hant"), Some(Language::ZhHant));
        assert_eq!(from_locale("nb-NO"), Some(Language::No));
        assert_eq!(from_locale("en"), Some(Language::En));
        assert_eq!(from_locale("ga-IE"), None);
        assert_eq!(from_locale("auto"), None);
    }

    #[test]
    fn chosen_language_selects_its_texts() {
        assert_eq!(texts(Language::De).quit, de::DE.quit);
        assert_eq!(texts(Language::En).quit, en::EN.quit);
    }

    #[test]
    fn model_labels_use_the_language_number_format() {
        let english = texts(Language::En);
        assert_eq!(english.local_model(LocalModel::Medium), "Medium (~1.5 GB)");
        assert_eq!(english.local_model(LocalModel::LargeV3), "Large v3 (~3 GB)");
        assert_eq!(
            english.local_model(LocalModel::ParakeetUltra),
            "Parakeet Ultra (~630 MB, 25 Languages)"
        );
        assert_eq!(
            english.online_model(OnlineModel::GroqWhisperLargeV3Turbo),
            "Whisper Large V3 Turbo (Fast)"
        );
        assert_eq!(
            texts(Language::De).local_model(LocalModel::Medium),
            "Medium (~1,5 GB)"
        );
    }

    /// Catches a translation that drops a parameter, e.g. a status code or the menu item a message refers to.
    #[test]
    fn every_language_keeps_the_parameters() {
        for language in Language::ALL.into_iter().filter(|l| *l != Language::Auto) {
            let t = texts(language);
            let name = language.raw_value();
            let keeps = |text: String, part: &str| {
                assert!(text.contains(part), "{name}: {text:?} lacks {part:?}");
            };
            for terms in [1, 2, 5, 21, 100] {
                keeps((t.vocabulary_menu)(terms), &terms.to_string());
            }
            keeps((t.languages)(25), "25");
            keeps((t.not_installed)("‹m›"), "‹m›");
            keeps((t.no_key)("‹p›"), "‹p›");
            keeps((t.not_connected)("‹m›"), "‹m›");
            keeps((t.delete_model)("‹m›"), "‹m›");
            keeps((t.download_model)("‹m›"), "‹m›");
            keeps((t.transcribed_locally_info)("‹r›"), "‹r›");
            keeps((t.post_processing_failed_info)("‹r›"), "‹r›");
            let info = (t.no_api_key_info)("‹p›", "‹i›");
            keeps(info.clone(), "‹p›");
            keeps(info, "‹i›");
            keeps((t.login_item_error_info)("‹i›"), "‹i›");
            keeps((t.download_failed_info)("‹m›"), "‹m›");
            keeps((t.delete_failed_info)("‹m›"), "‹m›");
            keeps((t.no_api_key_error)("‹p›"), "‹p›");
            keeps((t.server_error)(503), "503");
            keeps((t.api_error)(418), "418");
            keeps((t.model_not_available)("‹i›"), "‹i›");
            keeps((t.network_error)("‹e›"), "‹e›");
            keeps((t.key_rejected)("‹p›"), "‹p›");
            keeps((t.quota_exhausted)("‹p›"), "‹p›");
            keeps((t.retry_failed_dictation_info)("‹i›"), "‹i›");
            keeps((t.model_retired)("‹p›"), "‹p›");
            let either = (t.no_post_processing_key)("‹a›", "‹b›");
            keeps(either.clone(), "‹a›");
            keeps(either, "‹b›");
            let retired = (t.online_model_retired)("‹p›", "‹m›");
            keeps(retired.clone(), "‹p›");
            keeps(retired, "‹m›");
            let rejected = (t.request_rejected)("‹p›", 422);
            keeps(rejected.clone(), "‹p›");
            keeps(rejected, "422");
            keeps((t.microphone_error)("‹d›"), "‹d›");
            keeps((t.accessibility_required_info)("‹s›"), "‹s›");
            keeps(t.license.to_owned(), "MIT");
            keeps(t.license.to_owned(), "Apache 2.0");
        }
    }

    /// Items that ask for more input end in the ellipsis character, as the Human Interface Guidelines ask.
    #[test]
    fn items_that_ask_for_input_end_in_an_ellipsis() {
        for language in Language::ALL.into_iter().filter(|l| *l != Language::Auto) {
            let t = texts(language);
            for item in [
                t.set_api_key.to_owned(),
                t.update_api_key.to_owned(),
                t.report_problem.to_owned(),
                t.suggest_feature.to_owned(),
                t.check_for_updates.to_owned(),
                (t.vocabulary_menu)(0),
                (t.vocabulary_menu)(3),
            ] {
                assert!(
                    item.ends_with('…') && !item.contains("..."),
                    "{}: {item:?}",
                    language.raw_value()
                );
            }
        }
    }

    #[test]
    fn every_language_is_translated() {
        for language in Language::ALL
            .into_iter()
            .filter(|l| !matches!(l, Language::Auto | Language::En))
        {
            let t = texts(language);
            for (text, english) in [
                (t.recent_dictations, en::EN.recent_dictations),
                (t.accessibility_list, en::EN.accessibility_list),
                (t.device_control_list, en::EN.device_control_list),
                (t.vocabulary_info, en::EN.vocabulary_info),
                (
                    t.microphone_permission_denied,
                    en::EN.microphone_permission_denied,
                ),
                (t.local_model_failed, en::EN.local_model_failed),
                (t.api_key_not_saved, en::EN.api_key_not_saved),
                (t.api_keys_not_removed, en::EN.api_keys_not_removed),
                (t.quick_reference, en::EN.quick_reference),
                (t.reference_emoji, en::EN.reference_emoji),
                (t.retry_failed_dictation, en::EN.retry_failed_dictation),
                (t.not_pasted, en::EN.not_pasted),
                (t.not_pasted_info, en::EN.not_pasted_info),
            ] {
                assert_ne!(text, english, "{} is not translated", language.raw_value());
            }
            for (text, english) in [
                (
                    (t.download_failed_info)("m"),
                    (en::EN.download_failed_info)("m"),
                ),
                (
                    (t.delete_failed_info)("m"),
                    (en::EN.delete_failed_info)("m"),
                ),
                (
                    (t.online_model_retired)("p", "m"),
                    (en::EN.online_model_retired)("p", "m"),
                ),
                (
                    (t.no_post_processing_key)("a", "b"),
                    (en::EN.no_post_processing_key)("a", "b"),
                ),
                ((t.quota_exhausted)("p"), (en::EN.quota_exhausted)("p")),
                (
                    (t.retry_failed_dictation_info)("i"),
                    (en::EN.retry_failed_dictation_info)("i"),
                ),
            ] {
                assert_ne!(text, english, "{} is not translated", language.raw_value());
            }
        }
    }
}
