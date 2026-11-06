/// A fieldless enum stored as a string, with `ALL` in declaration order and `from_raw`, which also
/// accepts the earlier spellings listed after `|`.
macro_rules! stored_enum {
    (
        $(#[$meta:meta])*
        pub enum $name:ident {
            $($variant:ident = $raw:literal $(| $legacy:literal)*,)+
        }
    ) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub enum $name {
            $($variant,)+
        }

        impl $name {
            pub const ALL: [Self; [$($raw),+].len()] = [$(Self::$variant),+];

            pub fn raw_value(self) -> &'static str {
                match self {
                    $(Self::$variant => $raw,)+
                }
            }

            pub fn from_raw(raw: &str) -> Option<Self> {
                match raw {
                    $($raw $(| $legacy)* => Some(Self::$variant),)+
                    _ => None,
                }
            }
        }
    };
}

stored_enum! {
    pub enum TranscriptionMode {
        Online = "online",
        Local = "local",
    }
}

stored_enum! {
    pub enum PostProcessingMode {
        Off = "off",
        Cleanup = "cleanup",
        Format = "format",
    }
}

stored_enum! {
    pub enum Language {
        Auto = "auto",
        De = "de",
        En = "en",
        Es = "es",
        Fr = "fr",
        It = "it",
        Ja = "ja",
        Ko = "ko",
        Nl = "nl",
        Pl = "pl",
        Pt = "pt",
        PtPt = "pt-pt",
        Ru = "ru",
        Uk = "uk",
        Zh = "zh",
        ZhHant = "zh-hant",
        Tr = "tr",
        Ar = "ar",
        Sv = "sv",
        Da = "da",
        No = "no",
        Fi = "fi",
        Cs = "cs",
        Ro = "ro",
        Hu = "hu",
        El = "el",
        Hi = "hi",
        Ca = "ca",
        Hr = "hr",
        He = "he",
        Id = "id",
        Ms = "ms",
        Sk = "sk",
        Sl = "sl",
        Th = "th",
        Vi = "vi",
    }
}

impl Language {
    /// ISO 639-1 code for the APIs, `None` for auto-detection. Whisper knows no variants of a language.
    pub fn code(self) -> Option<&'static str> {
        match self {
            Self::Auto => None,
            Self::ZhHant => Some("zh"),
            Self::PtPt => Some("pt"),
            other => Some(other.raw_value()),
        }
    }

    /// The code for `model`: gpt-transcribe also takes regional Chinese codes, which select the script.
    pub fn code_for(self, model: OnlineModel) -> Option<&'static str> {
        match (self, model) {
            (Self::ZhHant, OnlineModel::GptTranscribe) => Some("zh-tw"),
            _ => self.code(),
        }
    }

    pub fn display_name(self) -> &'static str {
        match self {
            Self::Auto => "Auto-detect",
            Self::De => "Deutsch",
            Self::En => "English",
            Self::Es => "Español",
            Self::Fr => "Français",
            Self::It => "Italiano",
            Self::Ja => "日本語",
            Self::Ko => "한국어",
            Self::Nl => "Nederlands",
            Self::Pl => "Polski",
            Self::Pt => "Português (Brasil)",
            Self::PtPt => "Português (Portugal)",
            Self::Ru => "Русский",
            Self::Uk => "Українська",
            Self::Zh => "简体中文",
            Self::ZhHant => "繁體中文",
            Self::Tr => "Türkçe",
            Self::Ar => "العربية",
            Self::Sv => "Svenska",
            Self::Da => "Dansk",
            Self::No => "Norsk",
            Self::Fi => "Suomi",
            Self::Cs => "Čeština",
            Self::Ro => "Română",
            Self::Hu => "Magyar",
            Self::El => "Ελληνικά",
            Self::Hi => "हिन्दी",
            Self::Ca => "Català",
            Self::Hr => "Hrvatski",
            Self::He => "עברית",
            Self::Id => "Bahasa Indonesia",
            Self::Ms => "Bahasa Melayu",
            Self::Sk => "Slovenčina",
            Self::Sl => "Slovenščina",
            Self::Th => "ไทย",
            Self::Vi => "Tiếng Việt",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ModelFamily {
    Whisper,
    Parakeet,
}

stored_enum! {
    pub enum LocalModel {
        Tiny = "tiny",
        Base = "base",
        Small = "small",
        Medium = "medium",
        LargeV3 = "large-v3",
        LargeV3Turbo = "large-v3-turbo",
        ParakeetV2 = "parakeet-v2",
        // Saved by versions that offered Parakeet v3.
        ParakeetUltra = "parakeet-ultra" | "parakeet-v3",
    }
}

/// What a model's menu label adds about the languages it understands.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LanguageNote {
    Omitted,
    English,
    Count(usize),
}

/// Everything the app knows about a local model; how it runs is in `transcription::catalog`.
#[derive(Clone, Copy, Debug)]
pub struct ModelSpec {
    pub name: &'static str,
    /// Approximate download size.
    pub megabytes: u32,
    pub languages: LanguageNote,
    pub fallback_rank: u8,
}

impl LocalModel {
    pub fn spec(self) -> ModelSpec {
        let whisper = |name, megabytes, fallback_rank| ModelSpec {
            name,
            megabytes,
            languages: LanguageNote::Omitted,
            fallback_rank,
        };
        match self {
            Self::Tiny => whisper("Tiny", 75, 6),
            Self::Base => whisper("Base", 150, 5),
            Self::Small => whisper("Small", 470, 4),
            Self::Medium => whisper("Medium", 1_500, 3),
            Self::LargeV3 => whisper("Large v3", 3_000, 2),
            Self::LargeV3Turbo => whisper("Large v3 Turbo", 1_600, 1),
            Self::ParakeetV2 => ModelSpec {
                name: "Parakeet v2",
                megabytes: 600,
                languages: LanguageNote::English,
                fallback_rank: 7,
            },
            Self::ParakeetUltra => ModelSpec {
                name: "Parakeet Ultra",
                megabytes: 630,
                languages: LanguageNote::Count(25),
                fallback_rank: 0,
            },
        }
    }

    pub fn family(self) -> ModelFamily {
        match self {
            Self::ParakeetV2 | Self::ParakeetUltra => ModelFamily::Parakeet,
            Self::Tiny
            | Self::Base
            | Self::Small
            | Self::Medium
            | Self::LargeV3
            | Self::LargeV3Turbo => ModelFamily::Whisper,
        }
    }

    pub fn of_family(family: ModelFamily) -> impl Iterator<Item = Self> {
        Self::ALL
            .into_iter()
            .filter(move |model| model.family() == family)
    }
}

stored_enum! {
    #[derive(PartialOrd, Ord)]
    pub enum ApiProvider {
        OpenAi = "openai",
        Groq = "groq",
    }
}

impl ApiProvider {
    /// Groq first: its LLM inference is faster.
    pub const POST_PROCESSING_PREFERENCE: [Self; 2] = [Self::Groq, Self::OpenAi];

    pub fn display_name(self) -> &'static str {
        match self {
            Self::OpenAi => "OpenAI",
            Self::Groq => "Groq",
        }
    }

    pub fn transcription_url(self) -> &'static str {
        match self {
            Self::OpenAi => "https://api.openai.com/v1/audio/transcriptions",
            Self::Groq => "https://api.groq.com/openai/v1/audio/transcriptions",
        }
    }

    pub fn chat_url(self) -> &'static str {
        match self {
            Self::OpenAi => "https://api.openai.com/v1/chat/completions",
            Self::Groq => "https://api.groq.com/openai/v1/chat/completions",
        }
    }

    /// Groq retires models; `GET /openai/v1/models` lists the current ones.
    pub fn post_processing_model(self) -> &'static str {
        match self {
            Self::OpenAi => "gpt-4o-mini",
            Self::Groq => "openai/gpt-oss-120b",
        }
    }

    pub fn from_api_key(key: &str) -> Option<Self> {
        if key.starts_with("sk-") {
            Some(Self::OpenAi)
        } else if key.starts_with("gsk_") {
            Some(Self::Groq)
        } else {
            None
        }
    }
}

stored_enum! {
    pub enum OnlineModel {
        // Saved by versions that offered the GPT-4o transcription models.
        GptTranscribe = "gpt-transcribe" | "gpt-4o-mini-transcribe" | "gpt-4o-transcribe",
        Whisper1 = "whisper-1",
        GroqWhisperLargeV3 = "whisper-large-v3",
        GroqWhisperLargeV3Turbo = "whisper-large-v3-turbo",
    }
}

impl OnlineModel {
    pub fn provider(self) -> ApiProvider {
        match self {
            Self::GptTranscribe | Self::Whisper1 => ApiProvider::OpenAi,
            Self::GroqWhisperLargeV3 | Self::GroqWhisperLargeV3Turbo => ApiProvider::Groq,
        }
    }

    pub fn of_provider(provider: ApiProvider) -> impl Iterator<Item = Self> {
        Self::ALL
            .into_iter()
            .filter(move |model| model.provider() == provider)
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::GptTranscribe => "GPT Transcribe",
            Self::Whisper1 => "Whisper",
            Self::GroqWhisperLargeV3 => "Whisper Large V3",
            Self::GroqWhisperLargeV3Turbo => "Whisper Large V3 Turbo",
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn raw_values_round_trip() {
        for model in LocalModel::ALL {
            assert_eq!(LocalModel::from_raw(model.raw_value()), Some(model));
        }
        for model in OnlineModel::ALL {
            assert_eq!(OnlineModel::from_raw(model.raw_value()), Some(model));
        }
        for language in Language::ALL {
            assert_eq!(Language::from_raw(language.raw_value()), Some(language));
        }
        assert_eq!(Language::ALL.len(), 36);
    }

    #[test]
    fn language_variants_send_the_code_the_model_knows() {
        assert_eq!(Language::ZhHant.code(), Some("zh"));
        assert_eq!(Language::PtPt.code(), Some("pt"));
        assert_eq!(Language::Auto.code(), None);
        assert_eq!(Language::He.code(), Some("he"));
        assert_eq!(
            Language::ZhHant.code_for(OnlineModel::GptTranscribe),
            Some("zh-tw")
        );
        assert_eq!(
            Language::ZhHant.code_for(OnlineModel::GroqWhisperLargeV3),
            Some("zh")
        );
        assert_eq!(
            Language::PtPt.code_for(OnlineModel::GptTranscribe),
            Some("pt")
        );
    }

    #[test]
    fn retired_models_map_to_their_successors() {
        assert_eq!(
            LocalModel::from_raw("parakeet-v3"),
            Some(LocalModel::ParakeetUltra)
        );
        for raw in ["gpt-4o-mini-transcribe", "gpt-4o-transcribe"] {
            assert_eq!(OnlineModel::from_raw(raw), Some(OnlineModel::GptTranscribe));
        }
        assert_eq!(LocalModel::ParakeetUltra.raw_value(), "parakeet-ultra");
    }

    #[test]
    fn every_local_model_has_its_own_fallback_rank() {
        let ranks: HashSet<u8> = LocalModel::ALL
            .iter()
            .map(|model| model.spec().fallback_rank)
            .collect();
        assert_eq!(ranks.len(), LocalModel::ALL.len());
    }

    #[test]
    fn provider_is_detected_from_key_prefix() {
        assert_eq!(
            ApiProvider::from_api_key("sk-abc"),
            Some(ApiProvider::OpenAi)
        );
        assert_eq!(
            ApiProvider::from_api_key("gsk_abc"),
            Some(ApiProvider::Groq)
        );
        assert_eq!(ApiProvider::from_api_key("xyz"), None);
    }
}
