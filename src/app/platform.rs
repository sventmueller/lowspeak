//! What the app does outside its own state: the recorder, the clipboard, notifications, dialogs, settings, the
//! keychain, the login item, the local models and the network. `Mac` does it for real; the tests of `App` put a
//! substitute in its place that touches none of the user's devices or data.

use std::collections::HashSet;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::time::Duration;

use anyhow::Result;
use objc2::rc::Retained;
use objc2_app_kit::{NSApplication, NSSound, NSWorkspace};
use objc2_foundation::{MainThreadMarker, ns_string};

use super::capture::Capture;
use super::policy::MAX_HANDS_FREE_RECORDING;
use super::state::Transcription;
use super::{dialogs, feedback, notify};
use crate::audio::recorder::{self, Microphone, OnLive, StartError};
use crate::config::settings::{self, ChosenMicrophone, Settings};
use crate::config::{api_keys, login_item, vocabulary};
use crate::hotkey::Hotkey;
use crate::i18n::Texts;
use crate::models::{ApiProvider, LocalModel};
use crate::paste::{self, Pasted};
use crate::transcription::Api;
use crate::transcription::error::TranscriptionError;
use crate::transcription::local::LocalTranscriber;
use crate::transcription::model_store::{self, Disk, ModelStore};
use crate::transcription::online;
use crate::transcription::postprocess::ActiveApp;
use crate::transcription::service::{self, Job, Outcome, Services};

/// The recording buffer outlasts the longest time limit, whose timer may fire a little late.
const RECORDING_CAPACITY: Duration =
    MAX_HANDS_FREE_RECORDING.saturating_add(Duration::from_secs(10));

/// A finished recording, read and resampled to 16 kHz only when called, off the main thread.
pub type Audio = Box<dyn FnOnce() -> Result<Vec<f32>> + Send>;

/// The main thread's side; each call returns at once and answers through its callback, if any.
pub trait Platform {
    fn start_recorder(
        &self,
        microphone: Option<String>,
        on_live: OnLive,
        started: Box<dyn FnOnce(Result<(), StartError>) + Send>,
    );
    fn stop_recorder(&self, stopped: Box<dyn FnOnce(Audio) + Send>);
    fn measure_level(&self, measured: Box<dyn FnOnce(Option<f32>) + Send>);
    fn reset_hotkey(&self);
    fn warm_up(&self, provider: ApiProvider);
    fn play_start_sound(&self);
    /// `restored` runs once the clipboard holds what it held before.
    fn paste(&self, text: &str, restored: Box<dyn FnOnce()>) -> Pasted;
    fn copy(&self, text: &str);
    fn notify(&self, title: &str, body: &str);
    /// Modal, so it runs outside the app state's borrow; `None` when cancelled.
    fn prompt_api_key(&self, has_key: bool, texts: &'static Texts) -> Option<String>;
    /// Modal, so it runs outside the app state's borrow; `None` when cancelled.
    fn prompt_vocabulary(&self, current: &str, texts: &'static Texts) -> Option<String>;
    fn open(&self, address: &str);
    fn save_settings(&self, settings: &Settings);
    fn save_microphone(&self, microphone: Option<&ChosenMicrophone>);
    fn save_vocabulary(&self, terms: &[String]);
    fn store_api_key(&self, key: &str) -> Result<ApiProvider>;
    fn remove_api_keys(&self) -> Result<()>;
    fn configured_providers(&self) -> Vec<ApiProvider>;
    fn set_login_item(&self, enabled: bool) -> Result<()>;
    fn preload(&self, model: LocalModel);
    fn prime(&self, model: LocalModel);
    fn active_app(&self) -> ActiveApp;
    fn quit(&self);
    /// What the app's worker threads call.
    fn background(&self) -> Arc<dyn Background>;
    /// Stops the hotkey and releases the local model; false if the model did not let go in time.
    fn shut_down(&self) -> bool;
}

/// The worker threads' side; each call may take seconds to minutes.
pub trait Background: Send + Sync {
    fn transcribe(
        &self,
        samples: Vec<f32>,
        job: Transcription,
        active_app: ActiveApp,
    ) -> Result<Outcome, TranscriptionError>;
    fn download(
        &self,
        model: LocalModel,
        cancel: &AtomicBool,
        progress: &mut dyn FnMut(f64),
    ) -> Result<()>;
    /// Waits for a model that is still loading, so its files are not deleted underneath it.
    fn delete(&self, model: LocalModel) -> Result<()>;
    fn installed(&self) -> HashSet<LocalModel>;
    fn microphones(&self) -> Vec<Microphone>;
    fn login_item(&self) -> bool;
}

pub struct Mac {
    mtm: MainThreadMarker,
    capture: Capture,
    local: LocalTranscriber,
    hotkey: Hotkey,
    start_sound: Option<Retained<NSSound>>,
}

impl Mac {
    pub fn new(mtm: MainThreadMarker, hotkey: Hotkey) -> Self {
        Self {
            mtm,
            capture: Capture::spawn(RECORDING_CAPACITY),
            local: LocalTranscriber::spawn(),
            hotkey,
            start_sound: NSSound::soundNamed(ns_string!("Tink")),
        }
    }
}

impl Platform for Mac {
    fn start_recorder(
        &self,
        microphone: Option<String>,
        on_live: OnLive,
        started: Box<dyn FnOnce(Result<(), StartError>) + Send>,
    ) {
        self.capture.start(microphone, on_live, started);
    }

    fn stop_recorder(&self, stopped: Box<dyn FnOnce(Audio) + Send>) {
        self.capture
            .stop(move |captured| stopped(Box::new(move || captured.into_16k())));
    }

    fn measure_level(&self, measured: Box<dyn FnOnce(Option<f32>) + Send>) {
        self.capture.level(measured);
    }

    fn reset_hotkey(&self) {
        self.hotkey.reset_fn_state();
    }

    fn warm_up(&self, provider: ApiProvider) {
        online::warm_up(provider);
    }

    fn play_start_sound(&self) {
        if let Some(sound) = &self.start_sound {
            sound.stop();
            sound.play();
        }
    }

    fn paste(&self, text: &str, restored: Box<dyn FnOnce()>) -> Pasted {
        paste::paste(text, restored)
    }

    fn copy(&self, text: &str) {
        paste::copy(text);
    }

    fn notify(&self, title: &str, body: &str) {
        notify::send(title, body);
    }

    fn prompt_api_key(&self, has_key: bool, texts: &'static Texts) -> Option<String> {
        dialogs::prompt_api_key(self.mtm, has_key, texts)
    }

    fn prompt_vocabulary(&self, current: &str, texts: &'static Texts) -> Option<String> {
        dialogs::prompt_vocabulary(self.mtm, current, texts)
    }

    fn open(&self, address: &str) {
        feedback::open(address);
    }

    fn save_settings(&self, settings: &Settings) {
        settings.save();
    }

    fn save_microphone(&self, microphone: Option<&ChosenMicrophone>) {
        settings::set_chosen_microphone(microphone);
    }

    fn save_vocabulary(&self, terms: &[String]) {
        vocabulary::save(terms);
    }

    fn store_api_key(&self, key: &str) -> Result<ApiProvider> {
        api_keys::save(key)
    }

    fn remove_api_keys(&self) -> Result<()> {
        api_keys::delete_all()
    }

    fn configured_providers(&self) -> Vec<ApiProvider> {
        api_keys::configured_providers()
    }

    fn set_login_item(&self, enabled: bool) -> Result<()> {
        login_item::set_enabled(enabled)
    }

    fn preload(&self, model: LocalModel) {
        self.local.preload(model);
    }

    fn prime(&self, model: LocalModel) {
        self.local.prime(model);
    }

    fn active_app(&self) -> ActiveApp {
        let Some(app) = NSWorkspace::sharedWorkspace().frontmostApplication() else {
            return ActiveApp::default();
        };
        ActiveApp {
            bundle_id: app.bundleIdentifier().map(|id| id.to_string()),
            name: app.localizedName().map(|name| name.to_string()),
        }
    }

    fn quit(&self) {
        NSApplication::sharedApplication(self.mtm).terminate(None);
    }

    fn background(&self) -> Arc<dyn Background> {
        Arc::new(MacBackground {
            local: self.local.clone(),
        })
    }

    fn shut_down(&self) -> bool {
        self.hotkey.stop();
        self.local.unload_within(Duration::from_secs(3))
    }
}

struct MacBackground {
    local: LocalTranscriber,
}

/// The installed models, without one being deleted.
struct Usable {
    unavailable: Option<LocalModel>,
}

impl ModelStore for Usable {
    fn installed(&self) -> HashSet<LocalModel> {
        let mut installed = Disk.installed();
        if let Some(model) = self.unavailable {
            installed.remove(&model);
        }
        installed
    }
}

impl Background for MacBackground {
    fn transcribe(
        &self,
        samples: Vec<f32>,
        job: Transcription,
        active_app: ActiveApp,
    ) -> Result<Outcome, TranscriptionError> {
        let models = Usable {
            unavailable: job.unavailable,
        };
        let services = Services {
            local: &self.local,
            models: &models,
            api: Api::SYSTEM,
        };
        service::run(
            Job {
                samples,
                settings: job.settings,
                active_app,
                raw_parakeet_first: job.raw_parakeet_first,
                vocabulary: job.vocabulary,
            },
            &services,
        )
    }

    fn download(
        &self,
        model: LocalModel,
        cancel: &AtomicBool,
        progress: &mut dyn FnMut(f64),
    ) -> Result<()> {
        model_store::download(model, cancel, progress)
    }

    fn delete(&self, model: LocalModel) -> Result<()> {
        self.local.unload();
        model_store::delete(model)
    }

    fn installed(&self) -> HashSet<LocalModel> {
        model_store::installed_models()
    }

    fn microphones(&self) -> Vec<Microphone> {
        recorder::microphones()
    }

    fn login_item(&self) -> bool {
        login_item::is_enabled()
    }
}
