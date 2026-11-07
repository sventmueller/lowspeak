//! The app on the main thread: carries out what `State` decides with AppKit, the recorder thread,
//! the clipboard and background threads, and hands their results back to it.

pub mod about;
mod capture;
mod delivery;
mod dialogs;
mod feedback;
pub mod gesture;
pub mod menu;
mod model_library;
pub mod notify;
pub mod policy;
mod recording;
pub mod reference;
pub mod state;
pub mod updates;

use std::cell::RefCell;
use std::collections::HashSet;
use std::panic::AssertUnwindSafe;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use objc2::rc::Retained;
use objc2_app_kit::{NSApplication, NSMenu, NSSound, NSWorkspace};
use objc2_foundation::{MainThreadMarker, ns_string};

use self::capture::Capture;
use self::menu::{PULSE_INTERVAL, StatusUi};
use self::policy::MAX_HANDS_FREE_RECORDING;
use self::state::{DownloadEnd, Effect, KeyChange, Launch, State, Transcription};
pub use self::state::{Snapshot, Status};
use crate::audio::aac;
use crate::audio::recorder::{self, Captured, Microphone, OnLive};
use crate::config::settings::{self, Settings};
use crate::config::{api_keys, login_item, vocabulary};
use crate::hotkey::Hotkey;
use crate::models::{Language, LocalModel, OnlineModel, PostProcessingMode};
use crate::paste;
use crate::transcription::Api;
use crate::transcription::error::TranscriptionError;
use crate::transcription::hf::Cancelled;
use crate::transcription::local::LocalTranscriber;
use crate::transcription::model_store::{self, Disk, ModelStore};
use crate::transcription::online;
use crate::transcription::postprocess::ActiveApp;
use crate::transcription::service::{self, Job, Outcome, Services};
use crate::util::{filelog, main_thread};

/// The recording buffer outlasts the longest time limit, whose timer may fire a little late.
const RECORDING_CAPACITY: Duration =
    MAX_HANDS_FREE_RECORDING.saturating_add(Duration::from_secs(10));

#[derive(Clone, Debug)]
pub enum Action {
    SelectLocal(LocalModel),
    SelectOnline(OnlineModel),
    SelectLanguage(Language),
    SelectPostProcessing(PostProcessingMode),
    /// `None` is the system default.
    SelectMicrophone(Option<Microphone>),
    ToggleStartSound,
    CopyRecent(String),
    Vocabulary,
    Download(LocalModel),
    Delete(LocalModel),
    ApiKey,
    ToggleLogin,
    CheckForUpdates,
    ToggleUpdateChecks,
    ToggleUpdateInstalls,
    QuickReference,
    ReportProblem,
    SuggestFeature,
    OpenRepository,
    About,
    Quit,
}

pub struct App {
    state: State,
    ui: StatusUi,
    capture: Capture,
    local: LocalTranscriber,
    hotkey: Hotkey,
    start_sound: Option<Retained<NSSound>>,
    download_cancel: Option<Arc<AtomicBool>>,
    /// A refresh of the microphones and the login item runs.
    refreshing: bool,
}

thread_local! {
    static APP: RefCell<Option<App>> = const { RefCell::new(None) };
}

/// Runs `work` on the app state right away; `None` before launch or while the state is borrowed,
/// so only callers that cannot wait use it.
fn with_app<R>(work: impl FnOnce(&mut App) -> R) -> Option<R> {
    APP.with(|cell| match cell.try_borrow_mut() {
        Ok(mut app) => app.as_mut().map(work),
        Err(_) => {
            flog!(
                filelog::APP,
                "App state is busy, skipping a re-entrant call"
            );
            None
        }
    })
}

/// Queues `work` for the main thread, behind whatever currently holds the app state.
pub fn dispatch(work: impl FnOnce(&mut App) + Send + 'static) {
    main_thread::run(move || run_or_requeue(work));
}

fn dispatch_after(delay: Duration, work: impl FnOnce(&mut App) + Send + 'static) {
    main_thread::after(delay, move || run_or_requeue(work));
}

/// Runs `work` now if the app state is free, else queues it again; dropped only before launch.
fn run_or_requeue<F: FnOnce(&mut App) + Send + 'static>(work: F) {
    APP.with(|cell| match cell.try_borrow_mut() {
        Ok(mut app) => {
            if let Some(app) = app.as_mut() {
                work(app);
            }
        }
        Err(_) => main_thread::run(move || run_or_requeue(work)),
    });
}

fn update(event: impl FnOnce(&mut State) + Send + 'static) {
    dispatch(move |app| app.update(event));
}

fn update_now_or_later(event: impl FnOnce(&mut State) + Send + 'static) {
    run_or_requeue(move |app| app.update(event));
}

/// An accessory app is never active by itself, so its windows would open behind other apps.
fn bring_to_front(mtm: MainThreadMarker) {
    #[allow(deprecated)]
    NSApplication::sharedApplication(mtm).activateIgnoringOtherApps(true);
}

/// Runs `work` on a thread of its own and hands its result to `done`, or `panicked()` if it panics,
/// so whatever waits for the result is never left waiting.
fn spawn_guarded<T: Send + 'static>(
    name: &str,
    work: impl FnOnce() -> T + Send + 'static,
    panicked: impl FnOnce() -> T + Send + 'static,
    done: impl FnOnce(T) + Send + 'static,
) {
    let thread = name.to_owned();
    std::thread::Builder::new()
        .name(name.into())
        .spawn(move || {
            let result = std::panic::catch_unwind(AssertUnwindSafe(work)).unwrap_or_else(|_| {
                flog!(filelog::APP, "The thread {thread} panicked");
                panicked()
            });
            done(result);
        })
        .expect("spawning a worker thread");
}

pub fn launch(mtm: MainThreadMarker) {
    let hotkey = Hotkey::start(|key| {
        let at = Instant::now();
        update(move |state| state.key(key, at));
    });
    api_keys::migrate();
    let state = State::new(Launch {
        settings: Settings::load(),
        providers: api_keys::configured_providers(),
        installed: model_store::installed_models(),
        vocabulary: vocabulary::load(),
        microphone: settings::chosen_microphone(),
    });
    let mut app = App {
        state,
        ui: StatusUi::new(mtm),
        capture: Capture::spawn(RECORDING_CAPACITY),
        local: LocalTranscriber::spawn(),
        hotkey,
        start_sound: NSSound::soundNamed(ns_string!("Tink")),
        download_cancel: None,
        refreshing: false,
    };
    recorder::request_mic_permission();
    notify::request_permission();
    aac::warm_up();
    app.update(State::launch);
    app.refresh_system();
    flog!(
        filelog::APP,
        "Lowspeak initialized (mode: {}, providers: {:?}, models: {:?})",
        app.state.settings().mode.raw_value(),
        app.state.providers(),
        app.state
            .installed()
            .iter()
            .map(|model| model.raw_value())
            .collect::<Vec<_>>()
    );
    APP.with(|cell| *cell.borrow_mut() = Some(app));
    updates::start(mtm);
}

pub fn shutdown() {
    crate::transcription::shutdown();
    let released = APP.with(|cell| match cell.try_borrow_mut() {
        Ok(mut app) => app.as_mut().is_none_or(|app| {
            if let Some(cancel) = &app.download_cancel {
                cancel.store(true, Ordering::Relaxed);
            }
            app.hotkey.stop();
            app.local.unload_within(Duration::from_secs(3))
        }),
        Err(_) => false,
    });
    filelog::flush_all();
    if !released {
        // whisper.cpp aborts in its exit-time destructors while a model is still loaded.
        // SAFETY: logs are flushed; skipping the remaining exit handlers is intended.
        unsafe { libc::_exit(0) };
    }
}

/// Every menu item ends here, so the compiler checks that each one does something.
fn perform(action: Action) {
    let Some(mtm) =
        MainThreadMarker::new().filter(|_| APP.with(|cell| cell.try_borrow_mut().is_ok()))
    else {
        main_thread::run(move || perform(action));
        return;
    };
    match action {
        Action::SelectLocal(model) => update_now_or_later(move |state| state.select_local(model)),
        Action::SelectOnline(model) => {
            update_now_or_later(move |state| state.select_online(model));
        }
        Action::SelectLanguage(language) => {
            update_now_or_later(move |state| state.select_language(language));
        }
        Action::SelectPostProcessing(mode) => {
            update_now_or_later(move |state| state.select_post_processing(mode));
        }
        Action::SelectMicrophone(microphone) => {
            update_now_or_later(move |state| state.select_microphone(microphone));
        }
        Action::ToggleStartSound => update_now_or_later(State::toggle_start_sound),
        Action::CopyRecent(text) => update_now_or_later(move |state| state.copy_recent(text)),
        Action::Download(model) => update_now_or_later(move |state| state.download(model)),
        Action::Delete(model) => update_now_or_later(move |state| state.delete(model)),
        Action::ToggleLogin => update_now_or_later(State::toggle_login_item),
        Action::CheckForUpdates => updates::check(),
        Action::ToggleUpdateChecks => updates::toggle_automatic_checks(),
        Action::ToggleUpdateInstalls => updates::toggle_automatic_installs(),
        // Modal dialogs spin the run loop, which runs queued work on the app state, so they run
        // outside its borrow.
        Action::ApiKey => {
            let Some((has_key, texts)) =
                with_app(|app| (app.state.has_api_key(), app.state.texts()))
            else {
                return;
            };
            if let Some(key) = dialogs::prompt_api_key(mtm, has_key, texts) {
                update_now_or_later(move |state| state.enter_api_key(&key));
            }
        }
        Action::Vocabulary => {
            let Some((current, texts)) =
                with_app(|app| (app.state.vocabulary_text(), app.state.texts()))
            else {
                return;
            };
            if let Some(input) = dialogs::prompt_vocabulary(mtm, &current, texts) {
                update_now_or_later(move |state| state.set_vocabulary(&input));
            }
        }
        Action::QuickReference => {
            if let Some(texts) = with_app(|app| app.state.texts()) {
                reference::show(mtm, texts);
            }
        }
        Action::ReportProblem => feedback::report_problem(),
        Action::SuggestFeature => feedback::suggest_feature(),
        Action::OpenRepository => feedback::show_repository(),
        Action::About => {
            if let Some(texts) = with_app(|app| app.state.texts()) {
                about::show(mtm, texts);
            }
        }
        // Termination runs `shutdown`, which needs the app state.
        Action::Quit => NSApplication::sharedApplication(mtm).terminate(None),
    }
}

fn menu_needs_update(menu: &NSMenu) {
    with_app(|app| {
        app.ui.rebuild(menu, &app.snapshot());
        app.refresh_system();
    });
}

fn active_app() -> ActiveApp {
    let Some(app) = NSWorkspace::sharedWorkspace().frontmostApplication() else {
        return ActiveApp::default();
    };
    ActiveApp {
        bundle_id: app.bundleIdentifier().map(|id| id.to_string()),
        name: app.localizedName().map(|name| name.to_string()),
    }
}

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

fn transcribe(
    captured: Captured,
    job: Transcription,
    active_app: ActiveApp,
    local: &LocalTranscriber,
) -> Result<Outcome, TranscriptionError> {
    let samples = captured.into_16k().map_err(TranscriptionError::Audio)?;
    let models = Usable {
        unavailable: job.unavailable,
    };
    let services = Services {
        local,
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

impl App {
    fn snapshot(&self) -> Snapshot {
        Snapshot {
            updates: updates::menu(),
            ..self.state.snapshot()
        }
    }

    fn update(&mut self, event: impl FnOnce(&mut State)) {
        event(&mut self.state);
        while let Some(effect) = self.state.next_effect() {
            self.carry_out(effect);
        }
        self.refresh_status_item();
    }

    fn refresh_status_item(&mut self) {
        let started = self.ui.show(
            self.state.status(),
            self.state.is_hands_free(),
            self.state.download_progress(),
        );
        if let Some(generation) = started {
            schedule_pulse(generation);
        }
    }

    fn carry_out(&mut self, effect: Effect) {
        match effect {
            Effect::StartRecorder {
                recording,
                microphone,
            } => {
                let on_live: OnLive =
                    Box::new(move || update(move |state| state.microphone_live(recording)));
                self.capture.start(microphone, on_live, move |started| {
                    if let Err(error) = started {
                        update(move |state| state.recorder_failed(recording, &error));
                    }
                });
            }
            Effect::Transcribe(job) => self.transcribe(job),
            Effect::DiscardRecording => self.capture.stop(drop),
            Effect::MeasureLevel(recording) => self.capture.level(move |level| {
                update(move |state| state.level_measured(recording, level));
            }),
            Effect::Schedule(timer, delay) => {
                dispatch_after(delay, move |app| app.update(|state| state.timer(timer)));
            }
            Effect::ResetHotkey => self.hotkey.reset_fn_state(),
            Effect::WarmUp(provider) => online::warm_up(provider),
            Effect::PlayStartSound => {
                if let Some(sound) = &self.start_sound {
                    sound.stop();
                    sound.play();
                }
            }
            Effect::Paste(text) => {
                let pasted = paste::paste(&text, || update(State::clipboard_restored));
                self.state.pasted(pasted);
            }
            Effect::Copy(text) => paste::copy(&text),
            Effect::Notify { title, body } => notify::send(title, &body),
            Effect::SaveSettings(settings) => settings.save(),
            Effect::SaveMicrophone(microphone) => {
                settings::set_chosen_microphone(microphone.as_ref());
            }
            Effect::SaveVocabulary(terms) => vocabulary::save(&terms),
            Effect::StoreApiKey(key) => {
                let stored = match api_keys::save(&key) {
                    Ok(provider) => {
                        flog!(
                            filelog::APP,
                            "API key saved for {}",
                            provider.display_name()
                        );
                        true
                    }
                    Err(error) => {
                        flog!(filelog::APP, "Failed to save API key: {error:#}");
                        false
                    }
                };
                self.state.api_keys_stored(
                    KeyChange::Saved,
                    stored,
                    api_keys::configured_providers(),
                );
            }
            Effect::RemoveApiKeys => {
                let removed = match api_keys::delete_all() {
                    Ok(()) => {
                        flog!(filelog::APP, "All API keys removed");
                        true
                    }
                    Err(error) => {
                        flog!(filelog::APP, "Failed to remove the API keys: {error:#}");
                        false
                    }
                };
                self.state.api_keys_stored(
                    KeyChange::Removed,
                    removed,
                    api_keys::configured_providers(),
                );
            }
            Effect::SetLoginItem(enabled) => {
                let result = login_item::set_enabled(enabled);
                if let Err(error) = &result {
                    flog!(filelog::APP, "Login item update failed: {error:#}");
                    self.refresh_system();
                }
                self.state.login_item_set(enabled, result.is_ok());
            }
            Effect::Preload(model) => self.local.preload(model),
            Effect::Prime(model) => self.local.prime(model),
            Effect::Download(model) => self.download(model),
            Effect::Delete(model) => self.delete(model),
        }
    }

    fn transcribe(&self, job: Transcription) {
        let active_app = active_app();
        let local = self.local.clone();
        self.capture.stop(move |captured| {
            let recording = job.recording;
            spawn_guarded(
                "lowspeak.transcription",
                move || transcribe(captured, job, active_app, &local),
                || {
                    Err(TranscriptionError::Engine(anyhow::anyhow!(
                        "transcription panicked"
                    )))
                },
                move |result| update(move |state| state.transcribed(recording, result)),
            );
        });
    }

    fn download(&mut self, model: LocalModel) {
        let cancel = Arc::new(AtomicBool::new(false));
        self.download_cancel = Some(Arc::clone(&cancel));
        spawn_guarded(
            "lowspeak.download",
            move || {
                let mut reported = 0.0;
                model_store::download(model, &cancel, &mut |progress| {
                    if progress - reported >= 0.01 || progress >= 1.0 {
                        reported = progress;
                        update(move |state| state.download_progressed(progress));
                    }
                })
            },
            || Err(anyhow::anyhow!("the download panicked")),
            move |result| dispatch(move |app| app.download_finished(model, result)),
        );
    }

    fn download_finished(&mut self, model: LocalModel, result: anyhow::Result<()>) {
        self.download_cancel = None;
        let end = match result {
            Ok(()) => DownloadEnd::Done,
            Err(error) if error.downcast_ref::<Cancelled>().is_some() => DownloadEnd::Cancelled,
            Err(error) => {
                flog!(filelog::APP, "Download failed: {error:#}");
                DownloadEnd::Failed
            }
        };
        let installed = model_store::installed_models();
        self.update(|state| state.download_finished(model, end, installed));
    }

    fn delete(&self, model: LocalModel) {
        let local = self.local.clone();
        spawn_guarded(
            "lowspeak.delete",
            move || {
                // Waits for a model that is still loading, so its files are not deleted underneath it.
                local.unload();
                model_store::delete(model)
            },
            || Err(anyhow::anyhow!("deleting panicked")),
            move |result| {
                dispatch(move |app| {
                    if let Err(error) = &result {
                        flog!(
                            filelog::APP,
                            "Deleting {} failed: {error:#}",
                            model.raw_value()
                        );
                    }
                    let installed = model_store::installed_models();
                    app.update(|state| state.deleted(model, result.is_ok(), installed));
                });
            },
        );
    }

    /// Lists the microphones and asks for the login item off the main thread; the menu shows the
    /// previous answers until these arrive.
    fn refresh_system(&mut self) {
        if self.refreshing {
            return;
        }
        self.refreshing = true;
        spawn_guarded(
            "lowspeak.system",
            || Some((recorder::microphones(), login_item::is_enabled())),
            || None,
            |found| dispatch(move |app| app.system_refreshed(found)),
        );
    }

    fn system_refreshed(&mut self, found: Option<(Vec<Microphone>, bool)>) {
        self.refreshing = false;
        if let Some((microphones, login_item)) = found
            && self.state.system_refreshed(microphones, login_item)
            && let Some(menu) = self.ui.menu()
        {
            self.ui.rebuild(&menu, &self.snapshot());
        }
    }
}

fn schedule_pulse(generation: u64) {
    dispatch_after(PULSE_INTERVAL, move |app| {
        if app.ui.pulse(generation) {
            schedule_pulse(generation);
        }
    });
}
