//! Runs the app on the main thread with a substitute platform: dictations, menu choices, keys, downloads and the
//! answers of the platform go through `app` as they do in Lowspeak, while nothing reaches the microphone, the
//! clipboard, the keychain, the settings or the network.

use std::collections::HashSet;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use anyhow::{Result, anyhow};
use objc2_app_kit::NSApplication;
use objc2_foundation::{MainThreadMarker, NSDate, NSRunLoop};

use lowspeak::app::platform::{Audio, Background, Platform};
use lowspeak::app::state::{Launch, Transcription};
use lowspeak::app::{self, Action};
use lowspeak::audio::recorder::{Microphone, OnLive, StartError};
use lowspeak::config::settings::{ChosenMicrophone, Settings};
use lowspeak::hotkey::Key;
use lowspeak::i18n::Texts;
use lowspeak::models::{
    ApiProvider, Language, LocalModel, OnlineModel, PostProcessingMode, TranscriptionMode,
};
use lowspeak::paste::Pasted;
use lowspeak::transcription::error::TranscriptionError;
use lowspeak::transcription::hf::Cancelled;
use lowspeak::transcription::postprocess::{self, ActiveApp};
use lowspeak::transcription::service::{Outcome, Transcript};
use lowspeak::util::url_session::RequestError;

/// What the substitutes were asked to do, in order, and how they answer.
#[derive(Default)]
struct World {
    log: Vec<String>,
    recorder_fails: bool,
    transcript: String,
    api_key_answer: Option<String>,
    login_item_fails: bool,
    removing_keys_fails: bool,
    /// `copy` calls back into the app while the app's state is borrowed, as a modal panel would.
    reenter: bool,
    providers: Vec<ApiProvider>,
    installed: HashSet<LocalModel>,
    /// The next recording stops while another app than `FRONTMOST` is in front.
    other_app_in_front: bool,
}

/// The process ID of the app in front in this world.
const FRONTMOST: i32 = 4_242;

type Shared = Arc<Mutex<World>>;

fn note(world: &Shared, entry: String) {
    world.lock().unwrap().log.push(entry);
}

struct FakePlatform(Shared);

impl Platform for FakePlatform {
    fn start_recorder(
        &self,
        microphone: Option<String>,
        on_live: OnLive,
        started: Box<dyn FnOnce(Result<(), StartError>) + Send>,
    ) {
        note(&self.0, format!("start recorder {microphone:?}"));
        if self.0.lock().unwrap().recorder_fails {
            started(Err(StartError::Device("unplugged".into())));
        } else {
            started(Ok(()));
            on_live();
        }
    }

    fn stop_recorder(&self, stopped: Box<dyn FnOnce(Audio) + Send>) {
        note(&self.0, "stop recorder".into());
        stopped(Box::new(|| Ok(vec![0.1; 32_000])));
    }

    fn measure_level(&self, measured: Box<dyn FnOnce(Option<f32>) + Send>) {
        note(&self.0, "measure level".into());
        measured(Some(0.1));
    }

    fn reset_hotkey(&self) {
        note(&self.0, "reset hotkey".into());
    }

    fn warm_up(&self, provider: ApiProvider) {
        note(&self.0, format!("warm up {}", provider.display_name()));
    }

    fn play_start_sound(&self) {
        note(&self.0, "start sound".into());
    }

    fn paste(&self, text: &str, target: Option<i32>, restored: Box<dyn FnOnce()>) -> Pasted {
        note(&self.0, format!("paste {text}"));
        if target.is_some_and(|target| target != FRONTMOST) {
            note(&self.0, "app changed".into());
            return Pasted::AppChanged;
        }
        restored();
        Pasted::Yes
    }

    fn copy(&self, text: &str) {
        note(&self.0, format!("copy {text}"));
        let reenter = std::mem::take(&mut self.0.lock().unwrap().reenter);
        if reenter {
            assert!(app::snapshot().is_none(), "the state is borrowed here");
            app::perform(Action::CopyRecent("again".into()));
            app::key(Key::ShiftDown);
            spin(0.05);
        }
    }

    fn notify(&self, title: &str, body: &str) {
        note(&self.0, format!("notify {title}: {body}"));
    }

    fn prompt_api_key(&self, has_key: bool, _texts: &'static Texts) -> Option<String> {
        note(&self.0, format!("prompt api key {has_key}"));
        self.0.lock().unwrap().api_key_answer.clone()
    }

    fn prompt_vocabulary(&self, current: &str, _texts: &'static Texts) -> Option<String> {
        note(&self.0, format!("prompt vocabulary {current:?}"));
        Some("Lowspeak, Parakeet".into())
    }

    fn open(&self, address: &str) {
        note(&self.0, format!("open {address}"));
    }

    fn save_settings(&self, settings: &Settings) {
        note(
            &self.0,
            format!(
                "save settings {} {}",
                settings.language.raw_value(),
                settings.start_sound
            ),
        );
    }

    fn save_microphone(&self, microphone: Option<&ChosenMicrophone>) {
        note(
            &self.0,
            format!("save microphone {:?}", microphone.map(|m| m.name.clone())),
        );
    }

    fn save_vocabulary(&self, terms: &[String]) {
        note(&self.0, format!("save vocabulary {terms:?}"));
    }

    fn store_api_key(&self, key: &str) -> Result<ApiProvider> {
        note(&self.0, format!("store api key {key}"));
        if key.starts_with("gsk_") {
            self.0.lock().unwrap().providers = vec![ApiProvider::Groq];
            Ok(ApiProvider::Groq)
        } else {
            Err(anyhow!("the keychain is locked"))
        }
    }

    fn remove_api_keys(&self) -> Result<()> {
        note(&self.0, "remove api keys".into());
        let mut world = self.0.lock().unwrap();
        if world.removing_keys_fails {
            return Err(anyhow!("the keychain is locked"));
        }
        world.providers.clear();
        Ok(())
    }

    fn configured_providers(&self) -> Vec<ApiProvider> {
        self.0.lock().unwrap().providers.clone()
    }

    fn set_login_item(&self, enabled: bool) -> Result<()> {
        note(&self.0, format!("login item {enabled}"));
        if self.0.lock().unwrap().login_item_fails {
            Err(anyhow!("not allowed"))
        } else {
            Ok(())
        }
    }

    fn preload(&self, model: LocalModel) {
        note(&self.0, format!("preload {}", model.raw_value()));
    }

    fn prime(&self, model: LocalModel) {
        note(&self.0, format!("prime {}", model.raw_value()));
    }

    fn active_app(&self) -> ActiveApp {
        let elsewhere = std::mem::take(&mut self.0.lock().unwrap().other_app_in_front);
        ActiveApp {
            process_id: Some(if elsewhere { FRONTMOST + 1 } else { FRONTMOST }),
            ..ActiveApp::default()
        }
    }

    fn quit(&self) {
        note(&self.0, "quit".into());
    }

    fn background(&self) -> Arc<dyn Background> {
        Arc::new(FakeBackground(Arc::clone(&self.0)))
    }

    fn shut_down(&self) -> bool {
        note(&self.0, "shut down".into());
        true
    }
}

struct FakeBackground(Shared);

impl Background for FakeBackground {
    fn transcribe(
        &self,
        samples: Arc<Vec<f32>>,
        _job: Transcription,
        _active_app: ActiveApp,
    ) -> Result<Outcome, TranscriptionError> {
        note(&self.0, format!("transcribe {}", samples.len()));
        // Long enough for the icon to pulse.
        std::thread::sleep(Duration::from_millis(700));
        let text = self.0.lock().unwrap().transcript.clone();
        match text.as_str() {
            "panic" => panic!("the model crashed"),
            "offline" => Err(TranscriptionError::Network(RequestError::Failed(
                "The Internet connection appears to be offline.".into(),
            ))),
            "" => Ok(Outcome::NoSpeech),
            _ => Ok(Outcome::Text(Transcript {
                text,
                local_fallback: None,
                post_processing: postprocess::Status::Skipped,
            })),
        }
    }

    fn download(
        &self,
        model: LocalModel,
        _cancel: &AtomicBool,
        progress: &mut dyn FnMut(f64),
    ) -> Result<()> {
        note(&self.0, format!("download {}", model.raw_value()));
        match model {
            LocalModel::LargeV3 => {
                std::thread::sleep(Duration::from_millis(500));
                Ok(())
            }
            LocalModel::Base => Err(anyhow!("HTTP 500")),
            LocalModel::Small => Err(anyhow::Error::new(Cancelled)),
            LocalModel::Medium => panic!("the disk is gone"),
            _ => {
                progress(0.005);
                progress(0.5);
                progress(1.0);
                self.0.lock().unwrap().installed.insert(model);
                Ok(())
            }
        }
    }

    fn delete(&self, model: LocalModel) -> Result<()> {
        note(&self.0, format!("delete {}", model.raw_value()));
        if model == LocalModel::ParakeetUltra {
            std::thread::sleep(Duration::from_millis(1500));
            return Err(anyhow!("in use"));
        }
        self.0.lock().unwrap().installed.remove(&model);
        Ok(())
    }

    fn installed(&self) -> HashSet<LocalModel> {
        self.0.lock().unwrap().installed.clone()
    }

    fn microphones(&self) -> Vec<Microphone> {
        note(&self.0, "list microphones".into());
        vec![Microphone {
            id: "coreaudio:BuiltIn".into(),
            name: "MacBook Air-Mikrofon".into(),
        }]
    }

    fn login_item(&self) -> bool {
        false
    }
}

fn spin(seconds: f64) {
    NSRunLoop::currentRunLoop().runUntilDate(&NSDate::dateWithTimeIntervalSinceNow(seconds));
}

/// Runs the main run loop until the log holds an entry starting with `entry`, or fails after ten seconds: the silent
/// microphone check waits five.
fn wait_for(world: &Shared, entry: &str) {
    let deadline = Instant::now() + Duration::from_secs(10);
    while Instant::now() < deadline {
        if world
            .lock()
            .unwrap()
            .log
            .iter()
            .any(|line| line.starts_with(entry))
        {
            return;
        }
        spin(0.02);
    }
    panic!("no {entry:?} in {:#?}", world.lock().unwrap().log);
}

fn logged(world: &Shared, entry: &str) -> bool {
    world
        .lock()
        .unwrap()
        .log
        .iter()
        .any(|line| line.starts_with(entry))
}

fn clear(world: &Shared) {
    world.lock().unwrap().log.clear();
}

fn hold_and_speak(world: &Shared) {
    app::key(Key::FnDown { shift: false });
    wait_for(world, "start recorder");
    spin(0.5);
    app::key(Key::FnUp);
}

/// The substitutes crash on purpose; any other panic is reported as usual.
const INTENDED_PANICS: [&str; 2] = ["the model crashed", "the disk is gone"];

fn main() {
    if std::env::var_os("LOWSPEAK_SKIP_WINDOW_TESTS").is_some() {
        println!("app test skipped: LOWSPEAK_SKIP_WINDOW_TESTS is set");
        return;
    }
    let report = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let message = info.payload().downcast_ref::<&str>().copied().unwrap_or("");
        if !INTENDED_PANICS.contains(&message) {
            report(info);
        }
    }));
    let mtm = MainThreadMarker::new().expect("the app test must run on the main thread");
    let _application = NSApplication::sharedApplication(mtm);
    let world: Shared = Arc::new(Mutex::new(World {
        transcript: "hello world".into(),
        installed: HashSet::from([LocalModel::ParakeetUltra]),
        ..World::default()
    }));
    let settings = Settings {
        mode: TranscriptionMode::Local,
        local_model: LocalModel::ParakeetUltra,
        online_model: OnlineModel::GptTranscribe,
        language: Language::En,
        post_processing: PostProcessingMode::Off,
        preprocessing: true,
        start_sound: true,
    };
    app::start(
        mtm,
        Box::new(FakePlatform(Arc::clone(&world))),
        Launch {
            settings,
            providers: Vec::new(),
            installed: HashSet::from([LocalModel::ParakeetUltra]),
            vocabulary: Vec::new(),
            microphone: None,
        },
    );
    assert!(logged(&world, "preload parakeet-ultra"));
    wait_for(&world, "list microphones");
    spin(0.1);
    assert_eq!(
        app::snapshot().unwrap().microphones[0].name,
        "MacBook Air-Mikrofon"
    );

    // Hold fn, speak, release: the recorder starts, the sound plays, the text is pasted and kept.
    hold_and_speak(&world);
    wait_for(&world, "paste hello world");
    assert!(logged(&world, "start sound") && logged(&world, "transcribe 32000"));
    assert_eq!(app::snapshot().unwrap().recent, ["hello world"]);

    // Shift locks the recording; the next press ends it.
    clear(&world);
    world.lock().unwrap().transcript = "locked".into();
    app::key(Key::FnDown { shift: true });
    wait_for(&world, "start recorder");
    spin(0.4);
    app::key(Key::FnUp);
    spin(0.2);
    app::key(Key::FnDown { shift: false });
    app::key(Key::FnUp);
    wait_for(&world, "paste locked");

    // A dictation for an app that is no longer in front stays on the clipboard.
    clear(&world);
    world.lock().unwrap().transcript = "elsewhere".into();
    world.lock().unwrap().other_app_in_front = true;
    hold_and_speak(&world);
    wait_for(&world, "app changed");
    wait_for(&world, "notify");

    // A failed dictation keeps its recording, and the menu transcribes it again.
    clear(&world);
    world.lock().unwrap().transcript = "offline".into();
    hold_and_speak(&world);
    wait_for(&world, "notify");
    assert!(app::snapshot().unwrap().can_retry);
    world.lock().unwrap().transcript = "back online".into();
    clear(&world);
    app::perform(Action::RetryFailed);
    wait_for(&world, "paste back online");
    assert!(
        !logged(&world, "stop recorder"),
        "{:?}",
        world.lock().unwrap().log
    );
    assert!(logged(&world, "transcribe 32000"));
    assert!(!app::snapshot().unwrap().can_retry);

    // Silence, a crashing model and a missing microphone each end in the right place.
    for (transcript, outcome) in [("", "notify"), ("panic", "notify")] {
        clear(&world);
        world.lock().unwrap().transcript = transcript.into();
        hold_and_speak(&world);
        wait_for(&world, "transcribe");
        spin(1.0);
        assert!(
            !logged(&world, "paste"),
            "{transcript:?}: {:?}",
            world.lock().unwrap().log
        );
        if transcript == "panic" {
            wait_for(&world, outcome);
        }
    }
    clear(&world);
    world.lock().unwrap().recorder_fails = true;
    app::key(Key::FnDown { shift: false });
    wait_for(&world, "notify");
    app::key(Key::FnUp);
    world.lock().unwrap().recorder_fails = false;
    spin(0.2);

    // Menu choices reach the settings, the keychain and the clipboard through the platform.
    clear(&world);
    app::perform(Action::SelectLanguage(Language::De));
    wait_for(&world, "save settings de");
    app::perform(Action::ToggleStartSound);
    wait_for(&world, "save settings de false");
    app::perform(Action::SelectMicrophone(Some(Microphone {
        id: "coreaudio:BuiltIn".into(),
        name: "MacBook Air-Mikrofon".into(),
    })));
    wait_for(&world, "save microphone Some(\"MacBook Air-Mikrofon\")");
    app::perform(Action::CopyRecent("hello world".into()));
    wait_for(&world, "copy hello world");
    app::perform(Action::SelectPostProcessing(PostProcessingMode::Cleanup));
    app::perform(Action::Vocabulary);
    wait_for(&world, "save vocabulary [\"Lowspeak\", \"Parakeet\"]");
    assert_eq!(app::snapshot().unwrap().vocabulary_terms, 2);

    world.lock().unwrap().api_key_answer = Some("gsk_test".into());
    app::perform(Action::ApiKey);
    wait_for(&world, "store api key gsk_test");
    spin(0.1);
    assert_eq!(app::snapshot().unwrap().providers, [ApiProvider::Groq]);
    app::perform(Action::SelectOnline(OnlineModel::GroqWhisperLargeV3Turbo));
    spin(0.1);
    assert_eq!(app::snapshot().unwrap().mode, TranscriptionMode::Online);
    world.lock().unwrap().api_key_answer = Some("sk-locked".into());
    app::perform(Action::ApiKey);
    wait_for(&world, "notify");
    // Online, fn opens the connection while the user speaks.
    clear(&world);
    world.lock().unwrap().transcript = "online".into();
    hold_and_speak(&world);
    wait_for(&world, "paste online");
    assert!(
        logged(&world, "warm up Groq"),
        "{:?}",
        world.lock().unwrap().log
    );
    world.lock().unwrap().removing_keys_fails = true;
    world.lock().unwrap().api_key_answer = Some(String::new());
    app::perform(Action::ApiKey);
    wait_for(&world, "remove api keys");
    wait_for(&world, "notify");
    world.lock().unwrap().removing_keys_fails = false;
    clear(&world);
    app::perform(Action::ApiKey);
    wait_for(&world, "remove api keys");
    world.lock().unwrap().api_key_answer = None;
    app::perform(Action::ApiKey);
    spin(0.1);
    app::perform(Action::SelectLocal(LocalModel::ParakeetUltra));

    world.lock().unwrap().login_item_fails = true;
    clear(&world);
    app::perform(Action::ToggleLogin);
    wait_for(&world, "login item true");
    wait_for(&world, "notify");
    world.lock().unwrap().login_item_fails = false;
    app::perform(Action::ToggleLogin);
    wait_for(&world, "login item");

    // Downloads report progress and end done, failed, cancelled or crashed; deleting frees the model.
    clear(&world);
    app::perform(Action::Download(LocalModel::Tiny));
    wait_for(&world, "download tiny");
    spin(0.3);
    assert!(
        app::snapshot()
            .unwrap()
            .downloaded
            .contains(&LocalModel::Tiny)
    );
    for model in [LocalModel::Base, LocalModel::Small, LocalModel::Medium] {
        app::perform(Action::Download(model));
        wait_for(&world, &format!("download {}", model.raw_value()));
        spin(0.3);
        assert!(!app::snapshot().unwrap().downloaded.contains(&model));
    }
    app::perform(Action::Delete(LocalModel::Tiny));
    wait_for(&world, "delete tiny");
    spin(0.3);
    assert!(
        !app::snapshot()
            .unwrap()
            .downloaded
            .contains(&LocalModel::Tiny)
    );
    // A dictation while the chosen model is being deleted is dropped; the failed deletion keeps the model.
    clear(&world);
    app::perform(Action::Delete(LocalModel::ParakeetUltra));
    wait_for(&world, "delete parakeet-ultra");
    hold_and_speak(&world);
    wait_for(&world, "stop recorder");
    spin(1.5);
    assert!(
        !logged(&world, "transcribe"),
        "{:?}",
        world.lock().unwrap().log
    );
    assert!(
        app::snapshot()
            .unwrap()
            .downloaded
            .contains(&LocalModel::ParakeetUltra)
    );

    // Calls from other threads and from inside the app's own work wait their turn.
    clear(&world);
    std::thread::spawn(|| app::perform(Action::CopyRecent("from a thread".into())))
        .join()
        .unwrap();
    wait_for(&world, "copy from a thread");
    world.lock().unwrap().reenter = true;
    app::perform(Action::CopyRecent("first".into()));
    wait_for(&world, "copy again");

    // A recording that runs long gets its level checked, so a silent microphone is noticed.
    clear(&world);
    world.lock().unwrap().transcript = "long".into();
    app::key(Key::FnDown { shift: true });
    wait_for(&world, "measure level");
    app::key(Key::FnUp);
    spin(0.2);
    app::key(Key::FnDown { shift: false });
    app::key(Key::FnUp);
    wait_for(&world, "paste long");

    // Help and the app's own items.
    clear(&world);
    for action in [
        Action::ReportProblem,
        Action::SuggestFeature,
        Action::OpenRepository,
    ] {
        app::perform(action);
    }
    let opened: Vec<String> = world
        .lock()
        .unwrap()
        .log
        .iter()
        .filter(|line| line.starts_with("open https://github.com/sventmueller/lowspeak"))
        .cloned()
        .collect();
    assert_eq!(opened.len(), 3, "{opened:?}");
    for action in [
        Action::CheckForUpdates,
        Action::ToggleUpdateChecks,
        Action::ToggleUpdateInstalls,
        Action::QuickReference,
        Action::About,
    ] {
        app::perform(action);
    }
    app::perform(Action::Quit);
    assert!(logged(&world, "quit"));

    // Quitting during a download cancels it.
    app::perform(Action::Download(LocalModel::LargeV3));
    wait_for(&world, "download large-v3");
    app::shutdown();
    assert!(logged(&world, "shut down"));
    println!("app test passed");
}
