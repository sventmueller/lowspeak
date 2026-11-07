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
pub mod platform;
pub mod policy;
mod recording;
pub mod reference;
pub mod state;
pub mod updates;

use std::cell::RefCell;
use std::panic::AssertUnwindSafe;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use objc2_app_kit::{NSApplication, NSMenu};
use objc2_foundation::MainThreadMarker;

use self::menu::{PULSE_INTERVAL, StatusUi};
use self::platform::{Background, Mac, Platform};
use self::state::{DownloadEnd, Effect, KeyChange, Launch, State, Transcription};
pub use self::state::{Snapshot, Status};
use crate::audio::aac;
use crate::audio::recorder::{self, Microphone, OnLive};
use crate::config::settings::{self, Settings};
use crate::config::{api_keys, vocabulary};
use crate::hotkey::{Hotkey, Key};
use crate::models::{Language, LocalModel, OnlineModel, PostProcessingMode};
use crate::transcription::error::TranscriptionError;
use crate::transcription::hf::Cancelled;
use crate::transcription::model_store;
use crate::util::{filelog, main_thread};

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
    platform: Rc<dyn Platform>,
    background: Arc<dyn Background>,
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
    let hotkey = Hotkey::start(key);
    api_keys::migrate();
    let found = Launch {
        settings: Settings::load(),
        providers: api_keys::configured_providers(),
        installed: model_store::installed_models(),
        vocabulary: vocabulary::load(),
        microphone: settings::chosen_microphone(),
    };
    recorder::request_mic_permission();
    notify::request_permission();
    aac::warm_up();
    start(mtm, Box::new(Mac::new(mtm, hotkey)), found);
    updates::start(mtm);
}

/// Builds the app around `platform` from what `launch` found and runs it on the main thread.
pub fn start(mtm: MainThreadMarker, platform: Box<dyn Platform>, launch: Launch) {
    let background = platform.background();
    let mut app = App {
        state: State::new(launch),
        ui: StatusUi::new(mtm),
        platform: Rc::from(platform),
        background,
        download_cancel: None,
        refreshing: false,
    };
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
}

/// A press or release of fn or Shift, as the hotkey reports it.
pub fn key(key: Key) {
    let at = Instant::now();
    update(move |state| state.key(key, at));
}

/// What the menu would show now; `None` before launch.
pub fn snapshot() -> Option<Snapshot> {
    with_app(|app| app.snapshot())
}

pub fn shutdown() {
    crate::transcription::shutdown();
    let released = APP.with(|cell| match cell.try_borrow_mut() {
        Ok(mut app) => app.as_mut().is_none_or(|app| {
            if let Some(cancel) = &app.download_cancel {
                cancel.store(true, Ordering::Relaxed);
            }
            app.platform.shut_down()
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
pub fn perform(action: Action) {
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
            if let Some(key) = prompt(|platform| platform.prompt_api_key(has_key, texts)) {
                update_now_or_later(move |state| state.enter_api_key(&key));
            }
        }
        Action::Vocabulary => {
            let Some((current, texts)) =
                with_app(|app| (app.state.vocabulary_text(), app.state.texts()))
            else {
                return;
            };
            if let Some(input) = prompt(|platform| platform.prompt_vocabulary(&current, texts)) {
                update_now_or_later(move |state| state.set_vocabulary(&input));
            }
        }
        Action::QuickReference => {
            if let Some(texts) = with_app(|app| app.state.texts()) {
                reference::show(mtm, texts);
            }
        }
        Action::ReportProblem => open(&feedback::problem_report()),
        Action::SuggestFeature => open(&feedback::feature_suggestion()),
        Action::OpenRepository => open(feedback::REPOSITORY),
        Action::About => {
            if let Some(texts) = with_app(|app| app.state.texts()) {
                about::show(mtm, texts);
            }
        }
        // Termination runs `shutdown`, which needs the app state.
        Action::Quit => {
            if let Some(platform) = platform() {
                platform.quit();
            }
        }
    }
}

/// The platform outside the app state's borrow, for the calls that spin the run loop: modal dialogs and
/// termination.
fn platform() -> Option<Rc<dyn Platform>> {
    with_app(|app| Rc::clone(&app.platform))
}

fn prompt(ask: impl FnOnce(&dyn Platform) -> Option<String>) -> Option<String> {
    ask(&*platform()?)
}

fn open(address: &str) {
    with_app(|app| app.platform.open(address));
}

fn menu_needs_update(menu: &NSMenu) {
    with_app(|app| {
        app.ui.rebuild(menu, &app.snapshot());
        app.refresh_system();
    });
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
                self.platform.start_recorder(
                    microphone,
                    on_live,
                    Box::new(move |started| {
                        if let Err(error) = started {
                            update(move |state| state.recorder_failed(recording, &error));
                        }
                    }),
                );
            }
            Effect::Transcribe(job) => self.transcribe(job),
            Effect::DiscardRecording => self.platform.stop_recorder(Box::new(drop)),
            Effect::MeasureLevel(recording) => {
                self.platform.measure_level(Box::new(move |level| {
                    update(move |state| state.level_measured(recording, level));
                }));
            }
            Effect::Schedule(timer, delay) => {
                dispatch_after(delay, move |app| app.update(|state| state.timer(timer)));
            }
            Effect::ResetHotkey => self.platform.reset_hotkey(),
            Effect::WarmUp(provider) => self.platform.warm_up(provider),
            Effect::PlayStartSound => self.platform.play_start_sound(),
            Effect::Paste(text) => {
                let pasted = self
                    .platform
                    .paste(&text, Box::new(|| update(State::clipboard_restored)));
                self.state.pasted(pasted);
            }
            Effect::Copy(text) => self.platform.copy(&text),
            Effect::Notify { title, body } => self.platform.notify(title, &body),
            Effect::SaveSettings(settings) => self.platform.save_settings(&settings),
            Effect::SaveMicrophone(microphone) => {
                self.platform.save_microphone(microphone.as_ref());
            }
            Effect::SaveVocabulary(terms) => self.platform.save_vocabulary(&terms),
            Effect::StoreApiKey(key) => {
                let stored = match self.platform.store_api_key(&key) {
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
                    self.platform.configured_providers(),
                );
            }
            Effect::RemoveApiKeys => {
                let removed = match self.platform.remove_api_keys() {
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
                    self.platform.configured_providers(),
                );
            }
            Effect::SetLoginItem(enabled) => {
                let result = self.platform.set_login_item(enabled);
                if let Err(error) = &result {
                    flog!(filelog::APP, "Login item update failed: {error:#}");
                    self.refresh_system();
                }
                self.state.login_item_set(enabled, result.is_ok());
            }
            Effect::Preload(model) => self.platform.preload(model),
            Effect::Prime(model) => self.platform.prime(model),
            Effect::Download(model) => self.download(model),
            Effect::Delete(model) => self.delete(model),
        }
    }

    fn transcribe(&self, job: Transcription) {
        let active_app = self.platform.active_app();
        let background = Arc::clone(&self.background);
        self.platform.stop_recorder(Box::new(move |audio| {
            let recording = job.recording;
            spawn_guarded(
                "lowspeak.transcription",
                move || {
                    let samples = audio().map_err(TranscriptionError::Audio)?;
                    background.transcribe(samples, job, active_app)
                },
                || {
                    Err(TranscriptionError::Engine(anyhow::anyhow!(
                        "transcription panicked"
                    )))
                },
                move |result| update(move |state| state.transcribed(recording, result)),
            );
        }));
    }

    fn download(&mut self, model: LocalModel) {
        let cancel = Arc::new(AtomicBool::new(false));
        self.download_cancel = Some(Arc::clone(&cancel));
        let background = Arc::clone(&self.background);
        spawn_guarded(
            "lowspeak.download",
            move || {
                let mut reported = 0.0;
                background.download(model, &cancel, &mut |progress| {
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
        let installed = self.background.installed();
        self.update(|state| state.download_finished(model, end, installed));
    }

    fn delete(&self, model: LocalModel) {
        let background = Arc::clone(&self.background);
        spawn_guarded(
            "lowspeak.delete",
            move || background.delete(model),
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
                    let installed = app.background.installed();
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
        let background = Arc::clone(&self.background);
        spawn_guarded(
            "lowspeak.system",
            move || Some((background.microphones(), background.login_item())),
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
