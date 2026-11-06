//! Decisions of the app that need neither AppKit nor the network, so they are tested on their own.

use std::collections::{HashSet, VecDeque};
use std::time::Duration;

use crate::config::settings::Settings;
use crate::models::{ApiProvider, LocalModel, ModelFamily, OnlineModel, TranscriptionMode};
use crate::transcription::error::TranscriptionError;
use crate::transcription::postprocess::{self, Rejection};
use crate::transcription::service::Transcript;

/// Ends a recording whose fn release got lost.
pub const MAX_RECORDING: Duration = Duration::from_secs(120);
pub const MAX_HANDS_FREE_RECORDING: Duration = Duration::from_secs(300);

/// The saved choices, moved to an available provider or installed model where they are gone.
pub fn reconciled(
    mut settings: Settings,
    providers: &[ApiProvider],
    installed: &HashSet<LocalModel>,
) -> Settings {
    let first_installed = LocalModel::ALL
        .into_iter()
        .find(|model| installed.contains(model));
    let first_online = OnlineModel::ALL
        .into_iter()
        .find(|model| providers.contains(&model.provider()));

    if !providers.contains(&settings.online_model.provider())
        && let Some(fallback) = first_online
    {
        settings.online_model = fallback;
    }
    if !installed.contains(&settings.local_model)
        && let Some(fallback) = first_installed
    {
        settings.local_model = fallback;
    }
    match settings.mode {
        TranscriptionMode::Local => {
            if !installed.contains(&settings.local_model)
                && let Some(fallback) = first_online
            {
                settings.mode = TranscriptionMode::Online;
                settings.online_model = fallback;
            }
        }
        TranscriptionMode::Online => {
            if !providers.contains(&settings.online_model.provider())
                && let Some(fallback) = first_installed
            {
                settings.mode = TranscriptionMode::Local;
                settings.local_model = fallback;
            }
        }
    }
    settings
}

/// Parakeet first gets the untouched recording when it is the model of local mode.
pub fn raw_parakeet_first(settings: &Settings) -> bool {
    settings.mode == TranscriptionMode::Local
        && settings.local_model.family() == ModelFamily::Parakeet
}

pub fn timeout_applies(hands_free: bool, limit: Duration) -> bool {
    if hands_free {
        limit == MAX_HANDS_FREE_RECORDING
    } else {
        limit == MAX_RECORDING
    }
}

/// A notification about a dictation that still produced text.
#[derive(Debug)]
pub enum Notice<'a> {
    TranscribedLocally(&'a TranscriptionError),
    PostProcessingRejected(Rejection),
}

/// Tells the user about a fallback or a refused post-processing once per streak, not on every
/// dictation.
#[derive(Debug, Default)]
pub struct Notices {
    fallback_notified: bool,
    rejection_notified: bool,
}

impl Notices {
    pub fn after<'a>(
        &mut self,
        transcript: &'a Transcript,
        mode: TranscriptionMode,
    ) -> Vec<Notice<'a>> {
        let mut notices = Vec::new();
        match &transcript.local_fallback {
            Some(error) if !self.fallback_notified => {
                self.fallback_notified = true;
                notices.push(Notice::TranscribedLocally(error));
            }
            Some(_) => {}
            None if mode == TranscriptionMode::Online => self.fallback_notified = false,
            None => {}
        }
        match transcript.post_processing {
            postprocess::Status::Rejected(rejection) if !self.rejection_notified => {
                self.rejection_notified = true;
                notices.push(Notice::PostProcessingRejected(rejection));
            }
            postprocess::Status::Applied => self.rejection_notified = false,
            _ => {}
        }
        notices
    }
}

/// Dictations from the end of their recording until their result is taken; results come out in
/// the order the dictations were recorded, whichever transcription finishes first.
#[derive(Debug)]
pub struct Dictations<T> {
    queue: VecDeque<(u64, Option<T>)>,
}

impl<T> Default for Dictations<T> {
    fn default() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }
}

impl<T> Dictations<T> {
    pub fn push(&mut self, id: u64) {
        self.queue.push_back((id, None));
    }

    pub fn finish(&mut self, id: u64, result: T) {
        if let Some((_, slot)) = self.queue.iter_mut().find(|(queued, _)| *queued == id) {
            *slot = Some(result);
        }
    }

    /// The oldest dictation's result, once its transcription has finished.
    pub fn pop_ready(&mut self) -> Option<T> {
        match self.queue.front() {
            Some((_, Some(_))) => self.queue.pop_front()?.1,
            _ => None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Language, PostProcessingMode};

    fn settings(mode: TranscriptionMode) -> Settings {
        Settings {
            mode,
            local_model: LocalModel::LargeV3Turbo,
            online_model: OnlineModel::GptTranscribe,
            language: Language::Auto,
            post_processing: PostProcessingMode::Off,
            preprocessing: true,
            start_sound: false,
        }
    }

    fn installed(models: &[LocalModel]) -> HashSet<LocalModel> {
        models.iter().copied().collect()
    }

    #[test]
    fn online_mode_moves_to_a_provider_that_has_a_key() {
        let result = reconciled(
            settings(TranscriptionMode::Online),
            &[ApiProvider::Groq],
            &installed(&[]),
        );
        assert_eq!(result.mode, TranscriptionMode::Online);
        assert_eq!(result.online_model, OnlineModel::GroqWhisperLargeV3);
    }

    #[test]
    fn online_mode_without_keys_switches_to_an_installed_model() {
        let result = reconciled(
            settings(TranscriptionMode::Online),
            &[],
            &installed(&[LocalModel::ParakeetUltra]),
        );
        assert_eq!(result.mode, TranscriptionMode::Local);
        assert_eq!(result.local_model, LocalModel::ParakeetUltra);
    }

    #[test]
    fn local_mode_with_a_deleted_model_switches_to_online() {
        let result = reconciled(
            settings(TranscriptionMode::Local),
            &[ApiProvider::OpenAi],
            &installed(&[]),
        );
        assert_eq!(result.mode, TranscriptionMode::Online);
        assert_eq!(result.online_model, OnlineModel::GptTranscribe);
    }

    #[test]
    fn nothing_available_keeps_the_saved_choice() {
        let saved = settings(TranscriptionMode::Local);
        assert_eq!(reconciled(saved, &[], &installed(&[])), saved);
    }

    #[test]
    fn raw_audio_goes_first_only_to_parakeet_in_local_mode() {
        let mut local = settings(TranscriptionMode::Local);
        assert!(!raw_parakeet_first(&local));
        local.local_model = LocalModel::ParakeetUltra;
        assert!(raw_parakeet_first(&local));
        local.mode = TranscriptionMode::Online;
        assert!(!raw_parakeet_first(&local));
    }

    #[test]
    fn each_gesture_has_its_own_time_limit() {
        assert!(timeout_applies(false, MAX_RECORDING));
        assert!(!timeout_applies(false, MAX_HANDS_FREE_RECORDING));
        assert!(timeout_applies(true, MAX_HANDS_FREE_RECORDING));
        assert!(!timeout_applies(true, MAX_RECORDING));
    }

    fn transcript(fallback: bool, post_processing: postprocess::Status) -> Transcript {
        Transcript {
            text: "text".into(),
            local_fallback: fallback.then_some(TranscriptionError::Api {
                status: 503,
                message: String::new(),
            }),
            post_processing,
        }
    }

    #[test]
    fn a_fallback_streak_is_announced_once_until_online_works_again() {
        let mut notices = Notices::default();
        let online = TranscriptionMode::Online;
        let fallen_back = transcript(true, postprocess::Status::Skipped);
        assert!(matches!(
            notices.after(&fallen_back, online)[..],
            [Notice::TranscribedLocally(_)]
        ));
        assert!(notices.after(&fallen_back, online).is_empty());
        assert!(
            notices
                .after(
                    &transcript(false, postprocess::Status::Skipped),
                    TranscriptionMode::Local
                )
                .is_empty()
        );
        assert!(notices.after(&fallen_back, online).is_empty());
        assert!(
            notices
                .after(&transcript(false, postprocess::Status::Skipped), online)
                .is_empty()
        );
        assert_eq!(notices.after(&fallen_back, online).len(), 1);
    }

    #[test]
    fn a_refusal_is_announced_once_until_post_processing_works_again() {
        let mut notices = Notices::default();
        let mode = TranscriptionMode::Online;
        let rejection = Rejection::ApiKey(ApiProvider::Groq);
        let refused = transcript(false, postprocess::Status::Rejected(rejection));
        assert!(matches!(
            notices.after(&refused, mode)[..],
            [Notice::PostProcessingRejected(announced)] if announced == rejection
        ));
        assert!(notices.after(&refused, mode).is_empty());
        assert!(
            notices
                .after(&transcript(false, postprocess::Status::Failed), mode)
                .is_empty()
        );
        assert!(notices.after(&refused, mode).is_empty());
        notices.after(&transcript(false, postprocess::Status::Applied), mode);
        assert_eq!(notices.after(&refused, mode).len(), 1);
    }

    #[test]
    fn results_come_out_in_recording_order() {
        let mut dictations = Dictations::default();
        dictations.push(1);
        dictations.push(2);
        dictations.push(3);
        dictations.finish(2, "second");
        assert_eq!(dictations.pop_ready(), None);
        dictations.finish(1, "first");
        assert_eq!(dictations.pop_ready(), Some("first"));
        assert_eq!(dictations.pop_ready(), Some("second"));
        assert_eq!(dictations.pop_ready(), None);
        assert!(!dictations.is_empty());
        dictations.finish(3, "third");
        assert_eq!(dictations.pop_ready(), Some("third"));
        assert!(dictations.is_empty());
    }

    #[test]
    fn a_result_for_an_unknown_dictation_is_dropped() {
        let mut dictations = Dictations::default();
        dictations.push(1);
        dictations.finish(7, "stray");
        assert_eq!(dictations.pop_ready(), None);
        dictations.finish(1, "first");
        assert_eq!(dictations.pop_ready(), Some("first"));
    }
}
