//! What the app does about keys, timers, finished work and menu choices, decided without AppKit or
//! threads: events go in, `Effect`s come out, and `App` carries them out.

use std::collections::{HashSet, VecDeque};
use std::time::{Duration, Instant};

use super::delivery::{self, Delivery};
use super::gesture::{Command, SECOND_TAP_WINDOW};
use super::model_library::ModelLibrary;
use super::policy::{self, MAX_HANDS_FREE_RECORDING, MAX_RECORDING, Notice};
use super::recording::Recording;
use super::updates::UpdateMenu;
use crate::audio::recorder::{Microphone, StartError};
use crate::config::settings::{ChosenMicrophone, Settings};
use crate::config::vocabulary;
use crate::hotkey::Key;
use crate::i18n::{self, Texts};
use crate::models::{
    ApiProvider, Language, LocalModel, ModelFamily, OnlineModel, PostProcessingMode,
    TranscriptionMode,
};
use crate::paste::Pasted;
use crate::transcription::error::TranscriptionError;
use crate::transcription::service::{self, Outcome};
use crate::util::filelog;

/// AirPods whose microphone is claimed by another device deliver silence instead of an error.
const SILENT_MIC_CHECK: Duration = Duration::from_secs(5);
const SILENT_MIC_RMS: f32 = 0.001;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    Idle,
    /// fn is held but the microphone does not deliver audio yet.
    Starting,
    Recording,
    /// No recording runs, and at least one dictation is not pasted yet.
    Transcribing,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer {
    SecondTap(u64),
    SilentMic(u64),
    /// A recording and one of its two time limits.
    Limit(u64, Duration),
}

#[derive(Debug, PartialEq)]
pub struct Transcription {
    pub recording: u64,
    pub settings: Settings,
    pub raw_parakeet_first: bool,
    pub vocabulary: Vec<String>,
    /// Being deleted, so it must not step in for a failed online request.
    pub unavailable: Option<LocalModel>,
}

#[derive(Debug, PartialEq)]
pub enum Effect {
    StartRecorder {
        recording: u64,
        microphone: Option<String>,
    },
    Transcribe(Transcription),
    /// Stops the recorder and drops what it captured.
    DiscardRecording,
    MeasureLevel(u64),
    Schedule(Timer, Duration),
    ResetHotkey,
    WarmUp(ApiProvider),
    PlayStartSound,
    /// Answer with `State::pasted`.
    Paste(String),
    Copy(String),
    Notify {
        title: &'static str,
        body: String,
    },
    SaveSettings(Settings),
    SaveMicrophone(Option<ChosenMicrophone>),
    SaveVocabulary(Vec<String>),
    /// Answer with `State::api_keys_stored`.
    StoreApiKey(String),
    /// Answer with `State::api_keys_stored`.
    RemoveApiKeys,
    /// Answer with `State::login_item_set`.
    SetLoginItem(bool),
    Preload(LocalModel),
    Prime(LocalModel),
    /// Answer with `State::download_progressed` and `State::download_finished`.
    Download(LocalModel),
    /// Unload the local model first; answer with `State::deleted`.
    Delete(LocalModel),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DownloadEnd {
    Done,
    Cancelled,
    Failed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyChange {
    Saved,
    Removed,
}

/// What the menu shows, copied out of the app state.
#[derive(Clone, Debug)]
pub struct Snapshot {
    pub mode: TranscriptionMode,
    pub local_model: LocalModel,
    pub online_model: OnlineModel,
    /// Also the language of the menu.
    pub language: Language,
    pub post_processing: PostProcessingMode,
    pub downloaded: Vec<LocalModel>,
    pub downloading: Option<(LocalModel, f64)>,
    pub deleting: Option<LocalModel>,
    pub providers: Vec<ApiProvider>,
    pub microphones: Vec<Microphone>,
    pub microphone: Option<ChosenMicrophone>,
    pub start_sound: bool,
    /// Newest first.
    pub recent: Vec<String>,
    pub vocabulary_terms: usize,
    pub login_item: bool,
    /// `None` without Sparkle; `State` leaves it to `App`, which asks Sparkle.
    pub updates: Option<UpdateMenu>,
}

pub struct Launch {
    pub settings: Settings,
    pub providers: Vec<ApiProvider>,
    pub installed: HashSet<LocalModel>,
    pub vocabulary: Vec<String>,
    pub microphone: Option<ChosenMicrophone>,
}

/// Facts about the Mac that take too long to ask for each time the menu opens.
#[derive(Default)]
struct System {
    microphones: Vec<Microphone>,
    login_item: bool,
}

pub struct State {
    settings: Settings,
    providers: Vec<ApiProvider>,
    vocabulary: Vec<String>,
    microphone: Option<ChosenMicrophone>,
    recording: Recording,
    delivery: Delivery,
    library: ModelLibrary,
    system: System,
    effects: VecDeque<Effect>,
}

impl State {
    pub fn new(launch: Launch) -> Self {
        Self {
            settings: launch.settings,
            providers: launch.providers,
            vocabulary: launch.vocabulary,
            microphone: launch.microphone,
            recording: Recording::default(),
            delivery: Delivery::default(),
            library: ModelLibrary::new(launch.installed),
            system: System::default(),
            effects: VecDeque::new(),
        }
    }

    pub fn next_effect(&mut self) -> Option<Effect> {
        self.effects.pop_front()
    }

    fn push(&mut self, effect: Effect) {
        self.effects.push_back(effect);
    }

    fn notify(&mut self, title: &'static str, body: impl Into<String>) {
        self.push(Effect::Notify {
            title,
            body: body.into(),
        });
    }

    fn schedule(&mut self, timer: Timer, delay: Duration) {
        self.push(Effect::Schedule(timer, delay));
    }

    pub fn texts(&self) -> &'static Texts {
        i18n::texts(self.settings.language)
    }

    pub fn settings(&self) -> Settings {
        self.settings
    }

    pub fn providers(&self) -> &[ApiProvider] {
        &self.providers
    }

    pub fn installed(&self) -> &HashSet<LocalModel> {
        self.library.installed()
    }

    pub fn has_api_key(&self) -> bool {
        !self.providers.is_empty()
    }

    pub fn vocabulary_text(&self) -> String {
        self.vocabulary.join(", ")
    }

    pub fn status(&self) -> Status {
        if self.recording.is_live() {
            Status::Recording
        } else if self.recording.is_active() {
            Status::Starting
        } else if self.delivery.is_empty() {
            Status::Idle
        } else {
            Status::Transcribing
        }
    }

    pub fn is_hands_free(&self) -> bool {
        self.recording.is_hands_free()
    }

    pub fn download_progress(&self) -> Option<f64> {
        self.library.download().map(|(_, progress)| progress)
    }

    pub fn snapshot(&self) -> Snapshot {
        Snapshot {
            mode: self.settings.mode,
            local_model: self.settings.local_model,
            online_model: self.settings.online_model,
            language: self.settings.language,
            post_processing: self.settings.post_processing,
            downloaded: LocalModel::ALL
                .into_iter()
                .filter(|&model| self.library.is_installed(model))
                .collect(),
            downloading: self.library.download(),
            deleting: self.library.deleting(),
            providers: self.providers.clone(),
            microphones: self.system.microphones.clone(),
            microphone: self.microphone.clone(),
            start_sound: self.settings.start_sound,
            recent: self.delivery.recent().cloned().collect(),
            vocabulary_terms: self.vocabulary.len(),
            login_item: self.system.login_item,
            updates: None,
        }
    }

    pub fn launch(&mut self) {
        self.reconcile_selections();
        if self.settings.mode == TranscriptionMode::Local {
            if self.library.is_installed(self.settings.local_model) {
                self.push(Effect::Preload(self.settings.local_model));
            } else {
                flog!(
                    filelog::APP,
                    "WARNING: Local mode but {} is not installed",
                    self.settings.local_model.raw_value()
                );
            }
        } else {
            self.prime_fallback();
        }
    }

    /// Falls back to an available provider or installed model when the saved choice is no longer usable.
    fn reconcile_selections(&mut self) {
        let before = self.settings;
        self.settings = policy::reconciled(before, &self.providers, self.library.installed());
        if self.settings != before {
            self.push(Effect::SaveSettings(self.settings));
        }
    }

    fn save_settings(&mut self) {
        self.push(Effect::SaveSettings(self.settings));
    }

    /// Gets the model that steps in for failed online requests ready, so the first fallback is fast.
    fn prime_fallback(&mut self) {
        if let Some(model) =
            service::fallback_model(self.settings.local_model, self.library.installed())
            && model.family() == ModelFamily::Parakeet
        {
            self.push(Effect::Prime(model));
        }
    }

    pub fn key(&mut self, key: Key, at: Instant) {
        let command = self.recording.key(key, at);
        self.command(command);
    }

    fn command(&mut self, command: Command) {
        match command {
            Command::Start => self.start_recording(),
            Command::StartHandsFree => {
                flog!(filelog::APP, "[App] Hands-free recording");
                self.start_recording();
            }
            Command::Lock => flog!(filelog::APP, "[App] Hands-free recording"),
            Command::Stop => self.stop_recording(),
            Command::AwaitSecondTap(tap) => {
                self.schedule(Timer::SecondTap(tap), SECOND_TAP_WINDOW);
            }
            Command::None => {}
        }
    }

    pub fn timer(&mut self, timer: Timer) {
        match timer {
            Timer::SecondTap(tap) => {
                let command = self.recording.second_tap_missed(tap);
                self.command(command);
            }
            Timer::SilentMic(id) => {
                if self.recording.is_current(id) {
                    self.push(Effect::MeasureLevel(id));
                }
            }
            Timer::Limit(id, limit) => {
                if self.recording.is_current(id)
                    && policy::timeout_applies(self.recording.is_hands_free(), limit)
                {
                    flog!(
                        filelog::APP,
                        "[App] Recording timeout ({}s) reached, auto-stopping",
                        limit.as_secs()
                    );
                    // A recording that ran into its limit most likely ends in minutes of silence
                    // (a lost fn release, a forgotten hands-free recording), which the trimmed
                    // pipeline removes before transcribing.
                    self.finish_recording(false);
                }
            }
        }
    }

    /// Opens the API connections the dictation will need while the user is still speaking.
    fn warm_up_connections(&mut self) {
        if self.settings.mode == TranscriptionMode::Online {
            self.push(Effect::WarmUp(self.settings.online_model.provider()));
        }
        if self.settings.post_processing != PostProcessingMode::Off
            && let Some(provider) = ApiProvider::POST_PROCESSING_PREFERENCE
                .into_iter()
                .find(|provider| self.providers.contains(provider))
        {
            self.push(Effect::WarmUp(provider));
        }
    }

    fn start_recording(&mut self) {
        flog!(
            filelog::APP,
            "[App] start recording, status={:?}",
            self.status()
        );
        if self.recording.is_active() {
            flog!(
                filelog::APP,
                "[App] Already recording, ignoring — resetting fn state"
            );
            self.recording.forget_gesture();
            self.push(Effect::ResetHotkey);
            return;
        }
        let id = self.recording.begin();
        self.push(Effect::StartRecorder {
            recording: id,
            microphone: self
                .microphone
                .as_ref()
                .map(|microphone| microphone.id.clone()),
        });
        self.warm_up_connections();
        self.schedule(Timer::SilentMic(id), SILENT_MIC_CHECK);
        for limit in [MAX_RECORDING, MAX_HANDS_FREE_RECORDING] {
            self.schedule(Timer::Limit(id, limit), limit);
        }
    }

    pub fn recorder_failed(&mut self, id: u64, error: &StartError) {
        flog!(filelog::APP, "[App] Recording FAILED: {error}");
        let t = self.texts();
        self.notify(t.recording_failed, error.user_facing_description(t));
        if self.recording.is_current(id) {
            self.recording.end();
            self.deliver_ready();
        }
    }

    pub fn microphone_live(&mut self, id: u64) {
        if self.recording.go_live(id) {
            flog!(filelog::AUDIO, "Microphone live");
            if self.settings.start_sound {
                self.push(Effect::PlayStartSound);
            }
        }
    }

    pub fn level_measured(&mut self, id: u64, level: Option<f32>) {
        if self.recording.is_current(id) && level.is_none_or(|level| level < SILENT_MIC_RMS) {
            flog!(
                filelog::AUDIO,
                "Silent mic detected: RMS={level:?} — mic may be unavailable"
            );
            let t = self.texts();
            self.notify(t.microphone_silent, t.microphone_silent_info);
        }
    }

    fn stop_recording(&mut self) {
        if !self.recording.is_active() {
            flog!(
                filelog::APP,
                "[App] stop requested but status={:?}, ignoring",
                self.status()
            );
            return;
        }
        self.finish_recording(policy::raw_parakeet_first(&self.settings));
    }

    fn finish_recording(&mut self, raw_parakeet_first: bool) {
        let Some(id) = self.recording.end() else {
            return;
        };
        self.delivery.expect(id);
        let deleting = self.library.deleting();
        if self.settings.mode == TranscriptionMode::Local
            && deleting == Some(self.settings.local_model)
        {
            flog!(
                filelog::APP,
                "[App] {} is being deleted, the recording is discarded",
                self.settings.local_model.raw_value()
            );
            self.push(Effect::DiscardRecording);
            self.delivery
                .finish(id, Err(TranscriptionError::ModelNotAvailable));
        } else {
            self.push(Effect::Transcribe(Transcription {
                recording: id,
                settings: self.settings,
                raw_parakeet_first,
                vocabulary: self.vocabulary.clone(),
                unavailable: deleting,
            }));
        }
        self.deliver_ready();
    }

    pub fn transcribed(&mut self, id: u64, result: delivery::Result) {
        self.delivery.finish(id, result);
        self.deliver_ready();
    }

    /// Hands finished dictations over in the order they were recorded.
    fn deliver_ready(&mut self) {
        // Pasting waits for the recording to end, so ⌘V is never sent while fn or Shift is held.
        if self.recording.is_active() {
            return;
        }
        while let Some(result) = self.delivery.next() {
            self.deliver(result);
        }
    }

    fn deliver(&mut self, result: delivery::Result) {
        let t = self.texts();
        match result {
            Ok(Outcome::Text(transcript)) => {
                self.delivery.paste_started();
                self.push(Effect::Paste(transcript.text.clone()));
                let notices: Vec<(&'static str, String)> = self
                    .delivery
                    .notices_after(&transcript, self.settings.mode)
                    .into_iter()
                    .map(|notice| match notice {
                        Notice::TranscribedLocally(error) => (
                            t.transcribed_locally,
                            (t.transcribed_locally_info)(&error.user_facing_description(t)),
                        ),
                        Notice::PostProcessingRejected(rejection) => (
                            t.post_processing_failed,
                            (t.post_processing_failed_info)(&rejection.description(t)),
                        ),
                    })
                    .collect();
                for (title, body) in notices {
                    self.notify(title, body);
                }
                self.delivery.remember(transcript.text);
            }
            Ok(Outcome::NoSpeech) => flog!(filelog::APP, "[App] No speech detected"),
            Ok(Outcome::TooShort) => flog!(filelog::APP, "[App] Recording too short, skipped"),
            Err(error) => {
                flog!(filelog::APP, "[App] Transcription FAILED: {error}");
                self.notify(t.transcription_failed, error.user_facing_description(t));
            }
        }
    }

    pub fn pasted(&mut self, result: Pasted) {
        let t = self.texts();
        match result {
            Pasted::Yes => return,
            Pasted::NeedsAccessibility => self.notify(
                t.accessibility_required,
                (t.accessibility_required_info)(t.accessibility_setting()),
            ),
            Pasted::Failed => self.notify(t.paste_failed, t.paste_failed_info),
        }
        self.delivery.paste_done();
        self.deliver_ready();
    }

    pub fn clipboard_restored(&mut self) {
        self.delivery.paste_done();
        self.deliver_ready();
    }

    pub fn select_local(&mut self, model: LocalModel) {
        if !self.library.is_installed(model) {
            flog!(
                filelog::APP,
                "{} is not installed, keeping the selection",
                model.raw_value()
            );
            return;
        }
        self.settings.mode = TranscriptionMode::Local;
        self.settings.local_model = model;
        self.save_settings();
        flog!(
            filelog::APP,
            "Switched to local mode ({})",
            model.raw_value()
        );
        self.push(Effect::Preload(model));
    }

    pub fn select_online(&mut self, model: OnlineModel) {
        if !self.providers.contains(&model.provider()) {
            let t = self.texts();
            self.notify(
                t.no_api_key,
                (t.no_api_key_info)(model.provider().display_name(), t.set_api_key),
            );
            return;
        }
        self.settings.mode = TranscriptionMode::Online;
        self.settings.online_model = model;
        self.save_settings();
        flog!(
            filelog::APP,
            "Switched to online mode ({})",
            model.raw_value()
        );
    }

    pub fn select_language(&mut self, language: Language) {
        self.settings.language = language;
        self.save_settings();
    }

    pub fn select_post_processing(&mut self, mode: PostProcessingMode) {
        self.settings.post_processing = mode;
        self.save_settings();
    }

    pub fn toggle_start_sound(&mut self) {
        self.settings.start_sound = !self.settings.start_sound;
        self.save_settings();
    }

    /// `None` is the system default.
    pub fn select_microphone(&mut self, microphone: Option<Microphone>) {
        self.microphone = microphone.map(|microphone| ChosenMicrophone {
            id: microphone.id,
            name: microphone.name,
        });
        flog!(
            filelog::AUDIO,
            "Microphone set to {}",
            self.microphone
                .as_ref()
                .map_or("system default", |microphone| &microphone.name)
        );
        self.push(Effect::SaveMicrophone(self.microphone.clone()));
    }

    pub fn copy_recent(&mut self, text: String) {
        self.push(Effect::Copy(text));
    }

    pub fn set_vocabulary(&mut self, input: &str) {
        self.vocabulary = vocabulary::parse(input);
        flog!(
            filelog::APP,
            "Vocabulary set to {} terms",
            self.vocabulary.len()
        );
        self.push(Effect::SaveVocabulary(self.vocabulary.clone()));
    }

    /// An empty key removes all keys.
    pub fn enter_api_key(&mut self, key: &str) {
        if key.is_empty() {
            self.push(Effect::RemoveApiKeys);
        } else {
            self.push(Effect::StoreApiKey(key.to_owned()));
        }
    }

    pub fn api_keys_stored(
        &mut self,
        change: KeyChange,
        succeeded: bool,
        providers: Vec<ApiProvider>,
    ) {
        if !succeeded {
            let t = self.texts();
            let body = match change {
                KeyChange::Saved => t.api_key_not_saved,
                KeyChange::Removed => t.api_keys_not_removed,
            };
            self.notify(t.api_key_error, body);
        }
        self.providers = providers;
        self.reconcile_selections();
    }

    pub fn toggle_login_item(&mut self) {
        self.push(Effect::SetLoginItem(!self.system.login_item));
    }

    pub fn login_item_set(&mut self, enabled: bool, succeeded: bool) {
        if succeeded {
            self.system.login_item = enabled;
        } else {
            let t = self.texts();
            self.notify(
                t.login_item_error,
                (t.login_item_error_info)(t.open_at_login),
            );
        }
    }

    /// Returns whether anything the menu shows changed.
    pub fn system_refreshed(&mut self, microphones: Vec<Microphone>, login_item: bool) -> bool {
        let changed =
            self.system.microphones != microphones || self.system.login_item != login_item;
        self.system = System {
            microphones,
            login_item,
        };
        changed
    }

    pub fn download(&mut self, model: LocalModel) {
        if self.library.start_download(model) {
            self.push(Effect::Download(model));
        }
    }

    pub fn download_progressed(&mut self, progress: f64) {
        self.library.download_progressed(progress);
    }

    pub fn download_finished(
        &mut self,
        model: LocalModel,
        end: DownloadEnd,
        installed: HashSet<LocalModel>,
    ) {
        match end {
            DownloadEnd::Done => {}
            DownloadEnd::Cancelled => {
                flog!(filelog::APP, "Download cancelled for {}", model.raw_value());
            }
            DownloadEnd::Failed => {
                let t = self.texts();
                self.notify(
                    t.download_failed,
                    (t.download_failed_info)(&t.local_model(model)),
                );
            }
        }
        self.library.download_finished(installed);
        if self.settings.mode == TranscriptionMode::Online {
            self.prime_fallback();
        }
    }

    pub fn delete(&mut self, model: LocalModel) {
        if self.status() != Status::Idle {
            let t = self.texts();
            self.notify(t.cannot_delete, t.cannot_delete_info);
            return;
        }
        if self.library.start_deletion(model) {
            self.push(Effect::Delete(model));
            self.reconcile_selections();
        }
    }

    pub fn deleted(&mut self, model: LocalModel, succeeded: bool, installed: HashSet<LocalModel>) {
        self.library.deletion_finished(installed);
        if !succeeded {
            let t = self.texts();
            self.notify(
                t.delete_failed,
                (t.delete_failed_info)(&t.local_model(model)),
            );
        }
        self.reconcile_selections();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transcription::postprocess;
    use crate::transcription::service::Transcript;

    const EN: Language = Language::En;

    fn settings(mode: TranscriptionMode) -> Settings {
        Settings {
            mode,
            local_model: LocalModel::ParakeetUltra,
            online_model: OnlineModel::GroqWhisperLargeV3,
            language: EN,
            post_processing: PostProcessingMode::Off,
            preprocessing: true,
            start_sound: false,
        }
    }

    fn state_with(settings: Settings, installed: &[LocalModel]) -> State {
        State::new(Launch {
            settings,
            providers: vec![ApiProvider::Groq],
            installed: installed.iter().copied().collect(),
            vocabulary: vec!["Traefik".into()],
            microphone: None,
        })
    }

    fn local() -> State {
        state_with(
            settings(TranscriptionMode::Local),
            &[LocalModel::ParakeetUltra, LocalModel::Base],
        )
    }

    fn effects(state: &mut State) -> Vec<Effect> {
        std::iter::from_fn(|| state.next_effect()).collect()
    }

    fn texts() -> &'static Texts {
        i18n::texts(EN)
    }

    fn notifications(effects: &[Effect]) -> Vec<(&'static str, &str)> {
        effects
            .iter()
            .filter_map(|effect| match effect {
                Effect::Notify { title, body } => Some((*title, body.as_str())),
                _ => None,
            })
            .collect()
    }

    fn pastes(effects: &[Effect]) -> Vec<&str> {
        effects
            .iter()
            .filter_map(|effect| match effect {
                Effect::Paste(text) => Some(text.as_str()),
                _ => None,
            })
            .collect()
    }

    fn transcription(effects: &[Effect]) -> &Transcription {
        effects
            .iter()
            .find_map(|effect| match effect {
                Effect::Transcribe(job) => Some(job),
                _ => None,
            })
            .expect("a transcription")
    }

    fn text(text: &str) -> delivery::Result {
        Ok(Outcome::Text(Transcript {
            text: text.into(),
            local_fallback: None,
            post_processing: postprocess::Status::Skipped,
        }))
    }

    /// Holds fn for two seconds; returns the recording's id.
    fn dictate(state: &mut State, at: Instant) -> u64 {
        state.key(Key::FnDown { shift: false }, at);
        let started = effects(state);
        let id = started
            .iter()
            .find_map(|effect| match effect {
                Effect::StartRecorder { recording, .. } => Some(*recording),
                _ => None,
            })
            .unwrap_or_else(|| panic!("no recording started: {started:?}"));
        state.microphone_live(id);
        state.key(Key::FnUp, at + Duration::from_secs(2));
        id
    }

    #[test]
    fn a_held_fn_records_transcribes_and_pastes() {
        let mut state = local();
        let start = Instant::now();
        state.key(Key::FnDown { shift: false }, start);
        assert_eq!(state.status(), Status::Starting);
        let started = effects(&mut state);
        assert_eq!(
            started,
            [
                Effect::StartRecorder {
                    recording: 1,
                    microphone: None
                },
                Effect::Schedule(Timer::SilentMic(1), SILENT_MIC_CHECK),
                Effect::Schedule(Timer::Limit(1, MAX_RECORDING), MAX_RECORDING),
                Effect::Schedule(
                    Timer::Limit(1, MAX_HANDS_FREE_RECORDING),
                    MAX_HANDS_FREE_RECORDING
                ),
            ]
        );
        state.microphone_live(1);
        assert_eq!(state.status(), Status::Recording);
        assert!(effects(&mut state).is_empty());

        state.key(Key::FnUp, start + Duration::from_secs(3));
        assert_eq!(state.status(), Status::Transcribing);
        let stopped = effects(&mut state);
        assert_eq!(
            transcription(&stopped),
            &Transcription {
                recording: 1,
                settings: settings(TranscriptionMode::Local),
                raw_parakeet_first: true,
                vocabulary: vec!["Traefik".into()],
                unavailable: None,
            }
        );

        state.transcribed(1, text("Hello"));
        assert_eq!(pastes(&effects(&mut state)), ["Hello"]);
        state.pasted(Pasted::Yes);
        assert_eq!(state.status(), Status::Idle);
        assert_eq!(state.snapshot().recent, ["Hello"]);
    }

    #[test]
    fn the_start_sound_plays_once_the_microphone_is_live() {
        let mut state = local();
        state.toggle_start_sound();
        effects(&mut state);
        state.key(Key::FnDown { shift: false }, Instant::now());
        effects(&mut state);
        state.microphone_live(1);
        state.microphone_live(1);
        assert_eq!(effects(&mut state), [Effect::PlayStartSound]);
    }

    #[test]
    fn online_dictation_opens_the_connections_it_will_need() {
        let mut online = settings(TranscriptionMode::Online);
        online.post_processing = PostProcessingMode::Cleanup;
        let mut state = state_with(online, &[]);
        state.key(Key::FnDown { shift: false }, Instant::now());
        let started = effects(&mut state);
        assert_eq!(
            started
                .iter()
                .filter(|effect| matches!(effect, Effect::WarmUp(ApiProvider::Groq)))
                .count(),
            2
        );
    }

    #[test]
    fn dictations_are_pasted_in_recording_order_one_at_a_time() {
        let mut state = local();
        let start = Instant::now();
        let first = dictate(&mut state, start);
        let second = dictate(&mut state, start + Duration::from_secs(5));
        effects(&mut state);

        state.transcribed(second, text("second"));
        assert!(pastes(&effects(&mut state)).is_empty());
        state.transcribed(first, text("first"));
        assert_eq!(pastes(&effects(&mut state)), ["first"]);
        state.pasted(Pasted::Yes);
        assert!(effects(&mut state).is_empty(), "waits for the clipboard");
        assert_eq!(state.status(), Status::Transcribing);
        state.clipboard_restored();
        assert_eq!(pastes(&effects(&mut state)), ["second"]);
        state.pasted(Pasted::Yes);
        state.clipboard_restored();
        assert_eq!(state.status(), Status::Idle);
        assert_eq!(state.snapshot().recent, ["second", "first"]);
    }

    #[test]
    fn nothing_is_pasted_while_a_recording_runs() {
        let mut state = local();
        let start = Instant::now();
        let first = dictate(&mut state, start);
        state.key(Key::FnDown { shift: false }, start + Duration::from_secs(4));
        effects(&mut state);
        state.transcribed(first, text("first"));
        assert!(pastes(&effects(&mut state)).is_empty());
        state.key(Key::FnUp, start + Duration::from_secs(6));
        assert_eq!(pastes(&effects(&mut state)), ["first"]);
    }

    #[test]
    fn a_paste_without_accessibility_goes_on_to_the_next_dictation() {
        let mut state = local();
        let start = Instant::now();
        let first = dictate(&mut state, start);
        let second = dictate(&mut state, start + Duration::from_secs(5));
        state.transcribed(first, text("first"));
        state.transcribed(second, text("second"));
        effects(&mut state);
        state.pasted(Pasted::NeedsAccessibility);
        let next = effects(&mut state);
        assert_eq!(notifications(&next)[0].0, texts().accessibility_required);
        assert_eq!(pastes(&next), ["second"]);
        state.pasted(Pasted::Failed);
        assert_eq!(
            notifications(&effects(&mut state)),
            [(texts().paste_failed, texts().paste_failed_info)]
        );
        assert_eq!(state.status(), Status::Idle);
    }

    #[test]
    fn failures_are_reported_in_the_interface_language() {
        let mut state = local();
        let id = dictate(&mut state, Instant::now());
        effects(&mut state);
        state.transcribed(
            id,
            Err(TranscriptionError::Engine(anyhow::anyhow!(
                "/Users/someone/model.bin: bad magic"
            ))),
        );
        let notified = effects(&mut state);
        assert_eq!(
            notifications(&notified),
            [(texts().transcription_failed, texts().local_model_failed)]
        );
    }

    #[test]
    fn a_microphone_that_fails_to_start_ends_the_recording() {
        let mut state = local();
        let start = Instant::now();
        state.key(Key::FnDown { shift: false }, start);
        effects(&mut state);
        state.recorder_failed(1, &StartError::NoMicrophone);
        assert_eq!(
            notifications(&effects(&mut state)),
            [(texts().recording_failed, texts().no_microphone)]
        );
        assert_eq!(state.status(), Status::Idle);
        state.timer(Timer::SilentMic(1));
        state.timer(Timer::Limit(1, MAX_RECORDING));
        state.key(Key::FnUp, start + Duration::from_secs(1));
        assert!(effects(&mut state).is_empty());
        state.key(Key::FnDown { shift: false }, start + Duration::from_secs(2));
        assert!(matches!(
            effects(&mut state).first(),
            Some(Effect::StartRecorder { recording: 2, .. })
        ));
    }

    #[test]
    fn a_start_failure_after_the_release_only_tells_the_user() {
        let mut state = local();
        let id = dictate(&mut state, Instant::now());
        effects(&mut state);
        state.recorder_failed(id, &StartError::PermissionDenied);
        assert_eq!(notifications(&effects(&mut state)).len(), 1);
        assert_eq!(state.status(), Status::Transcribing);
        state.transcribed(id, Ok(Outcome::TooShort));
        assert!(effects(&mut state).is_empty());
        assert_eq!(state.status(), Status::Idle);
    }

    #[test]
    fn a_silent_microphone_is_reported_while_its_recording_runs() {
        let mut state = local();
        let start = Instant::now();
        state.key(Key::FnDown { shift: false }, start);
        effects(&mut state);
        state.timer(Timer::SilentMic(1));
        assert_eq!(effects(&mut state), [Effect::MeasureLevel(1)]);
        state.level_measured(1, Some(0.2));
        assert!(effects(&mut state).is_empty());
        state.level_measured(1, Some(0.0001));
        assert_eq!(
            notifications(&effects(&mut state)),
            [(texts().microphone_silent, texts().microphone_silent_info)]
        );
        state.level_measured(1, None);
        assert_eq!(notifications(&effects(&mut state)).len(), 1);

        state.key(Key::FnUp, start + Duration::from_secs(1));
        effects(&mut state);
        state.timer(Timer::SilentMic(1));
        state.level_measured(1, None);
        assert!(notifications(&effects(&mut state)).is_empty());
    }

    #[test]
    fn a_held_recording_stops_at_its_limit_and_skips_raw_parakeet() {
        let mut state = local();
        state.key(Key::FnDown { shift: false }, Instant::now());
        effects(&mut state);
        state.timer(Timer::Limit(1, MAX_HANDS_FREE_RECORDING));
        assert!(effects(&mut state).is_empty());
        state.timer(Timer::Limit(1, MAX_RECORDING));
        assert!(!transcription(&effects(&mut state)).raw_parakeet_first);
        assert_eq!(state.status(), Status::Transcribing);
    }

    #[test]
    fn a_hands_free_recording_runs_until_its_own_limit() {
        let mut state = local();
        let start = Instant::now();
        state.key(Key::FnDown { shift: true }, start);
        state.key(Key::FnUp, start + Duration::from_millis(100));
        effects(&mut state);
        assert!(state.is_hands_free());
        state.timer(Timer::Limit(1, MAX_RECORDING));
        assert!(effects(&mut state).is_empty());
        state.timer(Timer::Limit(1, MAX_HANDS_FREE_RECORDING));
        assert_eq!(transcription(&effects(&mut state)).recording, 1);
    }

    #[test]
    fn timers_of_an_earlier_recording_leave_the_current_one_alone() {
        let mut state = local();
        let start = Instant::now();
        let first = dictate(&mut state, start);
        state.key(Key::FnDown { shift: false }, start + Duration::from_secs(5));
        effects(&mut state);
        state.timer(Timer::Limit(first, MAX_RECORDING));
        state.timer(Timer::SilentMic(first));
        assert!(effects(&mut state).is_empty());
        assert_eq!(state.status(), Status::Starting);
    }

    #[test]
    fn a_tap_stops_once_the_second_tap_window_has_passed() {
        let mut state = local();
        let start = Instant::now();
        state.key(Key::FnDown { shift: false }, start);
        state.key(Key::FnUp, start + Duration::from_millis(100));
        let tapped = effects(&mut state);
        let Some(Effect::Schedule(timer, SECOND_TAP_WINDOW)) = tapped.last() else {
            panic!("{tapped:?}");
        };
        state.timer(*timer);
        assert_eq!(transcription(&effects(&mut state)).recording, 1);
    }

    #[test]
    fn a_press_while_recording_resets_the_hotkey() {
        let mut state = local();
        state.key(Key::FnDown { shift: false }, Instant::now());
        effects(&mut state);
        state.start_recording();
        assert_eq!(effects(&mut state), [Effect::ResetHotkey]);
        assert!(state.recording.is_current(1));
    }

    #[test]
    fn an_online_model_needs_a_key_and_a_local_one_its_files() {
        let mut state = local();
        state.select_online(OnlineModel::GptTranscribe);
        let refused = effects(&mut state);
        assert_eq!(notifications(&refused)[0].0, texts().no_api_key);
        assert_eq!(state.settings().mode, TranscriptionMode::Local);

        state.select_online(OnlineModel::GroqWhisperLargeV3Turbo);
        assert!(matches!(
            effects(&mut state)[..],
            [Effect::SaveSettings(Settings {
                mode: TranscriptionMode::Online,
                online_model: OnlineModel::GroqWhisperLargeV3Turbo,
                ..
            })]
        ));

        state.select_local(LocalModel::Medium);
        assert!(effects(&mut state).is_empty());
        state.select_local(LocalModel::Base);
        let selected = effects(&mut state);
        assert!(matches!(selected[0], Effect::SaveSettings(_)));
        assert_eq!(selected[1], Effect::Preload(LocalModel::Base));
    }

    #[test]
    fn menu_choices_are_saved() {
        let mut state = local();
        state.select_language(Language::De);
        state.select_post_processing(PostProcessingMode::Format);
        let airpods = Microphone {
            id: "coreaudio:AirPods".into(),
            name: "AirPods".into(),
        };
        state.select_microphone(Some(airpods));
        state.set_vocabulary("Postgres, Traefik");
        state.copy_recent("again".into());
        let saved = effects(&mut state);
        assert!(matches!(
            saved[1],
            Effect::SaveSettings(Settings {
                language: Language::De,
                post_processing: PostProcessingMode::Format,
                ..
            })
        ));
        assert_eq!(
            saved[2..],
            [
                Effect::SaveMicrophone(Some(ChosenMicrophone {
                    id: "coreaudio:AirPods".into(),
                    name: "AirPods".into()
                })),
                Effect::SaveVocabulary(vec!["Postgres".into(), "Traefik".into()]),
                Effect::Copy("again".into()),
            ]
        );
        assert_eq!(state.vocabulary_text(), "Postgres, Traefik");
        state.key(Key::FnDown { shift: false }, Instant::now());
        assert!(matches!(
            effects(&mut state).first(),
            Some(Effect::StartRecorder { microphone: Some(id), .. }) if id == "coreaudio:AirPods"
        ));
    }

    #[test]
    fn a_failed_download_tells_which_model_without_the_error_chain() {
        let mut state = local();
        state.download(LocalModel::Tiny);
        state.download(LocalModel::Small);
        assert_eq!(effects(&mut state), [Effect::Download(LocalModel::Tiny)]);
        state.download_progressed(0.25);
        assert_eq!(state.download_progress(), Some(0.25));
        assert_eq!(state.snapshot().downloading, Some((LocalModel::Tiny, 0.25)));
        state.download_finished(
            LocalModel::Tiny,
            DownloadEnd::Failed,
            state.installed().clone(),
        );
        let t = texts();
        let expected = (t.download_failed_info)(&t.local_model(LocalModel::Tiny));
        assert_eq!(
            notifications(&effects(&mut state)),
            [(t.download_failed, expected.as_str())]
        );
        assert_eq!(state.download_progress(), None);
    }

    #[test]
    fn a_finished_download_in_online_mode_primes_the_fallback() {
        let mut state = state_with(settings(TranscriptionMode::Online), &[]);
        state.download(LocalModel::ParakeetUltra);
        effects(&mut state);
        state.download_finished(
            LocalModel::ParakeetUltra,
            DownloadEnd::Done,
            [LocalModel::ParakeetUltra].into(),
        );
        assert_eq!(
            effects(&mut state),
            [Effect::Prime(LocalModel::ParakeetUltra)]
        );
        assert_eq!(state.snapshot().downloaded, [LocalModel::ParakeetUltra]);
    }

    #[test]
    fn a_cancelled_download_says_nothing() {
        let mut state = local();
        state.download(LocalModel::Tiny);
        effects(&mut state);
        state.download_finished(
            LocalModel::Tiny,
            DownloadEnd::Cancelled,
            state.installed().clone(),
        );
        assert!(effects(&mut state).is_empty());
    }

    #[test]
    fn models_are_deleted_only_while_nothing_else_happens() {
        let mut state = local();
        state.key(Key::FnDown { shift: false }, Instant::now());
        effects(&mut state);
        state.delete(LocalModel::Base);
        assert_eq!(
            notifications(&effects(&mut state)),
            [(texts().cannot_delete, texts().cannot_delete_info)]
        );
    }

    #[test]
    fn a_model_being_deleted_is_neither_selected_nor_used() {
        let mut state = local();
        state.delete(LocalModel::ParakeetUltra);
        let deleting = effects(&mut state);
        assert_eq!(deleting[0], Effect::Delete(LocalModel::ParakeetUltra));
        assert!(matches!(
            deleting[1],
            Effect::SaveSettings(Settings {
                local_model: LocalModel::Base,
                ..
            })
        ));
        assert_eq!(state.snapshot().deleting, Some(LocalModel::ParakeetUltra));
        assert_eq!(state.snapshot().downloaded, [LocalModel::Base]);

        state.select_local(LocalModel::ParakeetUltra);
        state.download(LocalModel::ParakeetUltra);
        assert!(effects(&mut state).is_empty());

        dictate(&mut state, Instant::now());
        let job = effects(&mut state);
        assert_eq!(
            transcription(&job).unavailable,
            Some(LocalModel::ParakeetUltra)
        );
        assert_eq!(transcription(&job).settings.local_model, LocalModel::Base);

        state.deleted(LocalModel::ParakeetUltra, true, [LocalModel::Base].into());
        assert!(effects(&mut state).is_empty());
        assert_eq!(state.snapshot().deleting, None);
    }

    #[test]
    fn a_recording_for_the_only_model_while_it_is_deleted_is_discarded() {
        let mut state = State::new(Launch {
            settings: settings(TranscriptionMode::Local),
            providers: Vec::new(),
            installed: [LocalModel::ParakeetUltra].into(),
            vocabulary: Vec::new(),
            microphone: None,
        });
        state.delete(LocalModel::ParakeetUltra);
        assert_eq!(
            effects(&mut state),
            [Effect::Delete(LocalModel::ParakeetUltra)]
        );
        dictate(&mut state, Instant::now());
        let discarded = effects(&mut state);
        assert!(discarded.contains(&Effect::DiscardRecording));
        assert!(
            !discarded
                .iter()
                .any(|effect| matches!(effect, Effect::Transcribe(_)))
        );
        let t = texts();
        assert_eq!(
            notifications(&discarded),
            [(
                t.transcription_failed,
                (t.model_not_available)(t.manage_models).as_str()
            )]
        );
        assert_eq!(state.status(), Status::Idle);
    }

    #[test]
    fn a_failed_deletion_is_reported_and_the_model_comes_back() {
        let mut state = local();
        state.delete(LocalModel::Base);
        effects(&mut state);
        state.deleted(
            LocalModel::Base,
            false,
            [LocalModel::Base, LocalModel::ParakeetUltra].into(),
        );
        let t = texts();
        let expected = (t.delete_failed_info)(&t.local_model(LocalModel::Base));
        assert_eq!(
            notifications(&effects(&mut state)),
            [(t.delete_failed, expected.as_str())]
        );
        assert!(state.snapshot().downloaded.contains(&LocalModel::Base));
    }

    #[test]
    fn api_keys_are_stored_and_failures_reported_without_details() {
        let mut state = state_with(settings(TranscriptionMode::Online), &[LocalModel::Base]);
        state.enter_api_key("sk-new");
        state.enter_api_key("");
        assert_eq!(
            effects(&mut state),
            [Effect::StoreApiKey("sk-new".into()), Effect::RemoveApiKeys]
        );
        assert!(state.has_api_key());
        state.api_keys_stored(KeyChange::Removed, false, Vec::new());
        let t = texts();
        let reported = effects(&mut state);
        assert_eq!(
            notifications(&reported),
            [(t.api_key_error, t.api_keys_not_removed)]
        );
        assert!(matches!(
            reported.last(),
            Some(Effect::SaveSettings(Settings {
                mode: TranscriptionMode::Local,
                ..
            }))
        ));
        assert!(!state.has_api_key());
        state.api_keys_stored(KeyChange::Saved, false, vec![ApiProvider::OpenAi]);
        assert_eq!(
            notifications(&effects(&mut state)),
            [(t.api_key_error, t.api_key_not_saved)]
        );
    }

    #[test]
    fn the_login_item_follows_what_macos_reports() {
        let mut state = local();
        assert!(state.system_refreshed(Vec::new(), true));
        assert!(!state.system_refreshed(Vec::new(), true));
        state.toggle_login_item();
        assert_eq!(effects(&mut state), [Effect::SetLoginItem(false)]);
        state.login_item_set(false, false);
        assert_eq!(notifications(&effects(&mut state)).len(), 1);
        assert!(state.snapshot().login_item);
        state.login_item_set(false, true);
        assert!(!state.snapshot().login_item);
    }

    #[test]
    fn launch_loads_the_local_model_or_primes_the_fallback() {
        let mut state = local();
        state.launch();
        assert_eq!(
            effects(&mut state),
            [Effect::Preload(LocalModel::ParakeetUltra)]
        );
        let mut online = state_with(
            settings(TranscriptionMode::Online),
            &[LocalModel::ParakeetUltra],
        );
        online.launch();
        assert_eq!(
            effects(&mut online),
            [Effect::Prime(LocalModel::ParakeetUltra)]
        );
    }
}
