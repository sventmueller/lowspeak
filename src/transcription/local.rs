//! Owns the loaded local model on a dedicated thread, so loading and inference never block the UI
//! and requests are handled one at a time.

use std::any::Any;
use std::panic::{self, AssertUnwindSafe};
use std::sync::Arc;
use std::sync::mpsc::{self, Receiver, Sender};
use std::time::{Duration, Instant};

use anyhow::anyhow;

use super::catalog::{self, Engine};
use super::error::TranscriptionError;
use super::model_store;
use super::parakeet::Parakeet;
use super::whisper::Whisper;
use crate::models::LocalModel;
use crate::util::filelog;

/// Transcribes with a model on this Mac.
pub trait LocalEngine {
    /// Blocks until the transcription is done; call it off the main thread. Only Whisper uses
    /// `prompt`. The caller keeps its share of `samples` for another attempt without a copy.
    fn transcribe_shared(
        &self,
        samples: Arc<Vec<f32>>,
        model: LocalModel,
        language: Option<&'static str>,
        prompt: Option<String>,
    ) -> Result<String, TranscriptionError>;

    fn transcribe(
        &self,
        samples: Vec<f32>,
        model: LocalModel,
        language: Option<&'static str>,
        prompt: Option<String>,
    ) -> Result<String, TranscriptionError> {
        self.transcribe_shared(Arc::new(samples), model, language, prompt)
    }
}

/// A model in memory.
pub trait LoadedModel {
    fn transcribe(
        &mut self,
        samples: &[f32],
        language: Option<&str>,
        prompt: Option<&str>,
    ) -> anyhow::Result<String>;

    /// Runs once after a preload, so the first dictation does not pay for one-time setup.
    fn warm_up(&mut self) {}
}

/// Loads models for the model thread.
pub trait ModelLoader: Send + 'static {
    fn load(&self, model: LocalModel) -> Result<Box<dyn LoadedModel>, TranscriptionError>;
}

/// The models installed on this Mac.
pub struct FromDisk;

impl ModelLoader for FromDisk {
    fn load(&self, model: LocalModel) -> Result<Box<dyn LoadedModel>, TranscriptionError> {
        if !model_store::is_installed(model) {
            return Err(TranscriptionError::ModelNotAvailable);
        }
        let location = model_store::location(model);
        let loaded: Box<dyn LoadedModel> = match catalog::engine(model) {
            Engine::Whisper(_) => {
                Box::new(Whisper::load(&location).map_err(TranscriptionError::Engine)?)
            }
            Engine::Parakeet(version) => {
                Box::new(Parakeet::load(&location, version).map_err(TranscriptionError::Engine)?)
            }
        };
        Ok(loaded)
    }
}

impl LoadedModel for Whisper {
    fn transcribe(
        &mut self,
        samples: &[f32],
        language: Option<&str>,
        prompt: Option<&str>,
    ) -> anyhow::Result<String> {
        Whisper::transcribe(self, samples, language, prompt)
    }

    fn warm_up(&mut self) {
        Whisper::warm_up(self);
    }
}

impl LoadedModel for Parakeet {
    fn transcribe(
        &mut self,
        samples: &[f32],
        _language: Option<&str>,
        _prompt: Option<&str>,
    ) -> anyhow::Result<String> {
        Parakeet::transcribe(self, samples)
    }
}

enum Command {
    Preload(LocalModel),
    Transcribe {
        samples: Arc<Vec<f32>>,
        model: LocalModel,
        language: Option<&'static str>,
        prompt: Option<String>,
        reply: Sender<Result<String, TranscriptionError>>,
    },
    Unload {
        reply: Sender<()>,
    },
}

#[derive(Clone)]
pub struct LocalTranscriber {
    commands: Sender<Command>,
}

fn thread_gone() -> TranscriptionError {
    TranscriptionError::Engine(anyhow!("the local model thread has stopped"))
}

impl LocalEngine for LocalTranscriber {
    fn transcribe_shared(
        &self,
        samples: Arc<Vec<f32>>,
        model: LocalModel,
        language: Option<&'static str>,
        prompt: Option<String>,
    ) -> Result<String, TranscriptionError> {
        let (reply, result) = mpsc::channel();
        self.commands
            .send(Command::Transcribe {
                samples,
                model,
                language,
                prompt,
                reply,
            })
            .map_err(|_| thread_gone())?;
        result.recv().map_err(|_| thread_gone())?
    }
}

impl LocalTranscriber {
    pub fn spawn() -> Self {
        Self::spawn_with(FromDisk)
    }

    pub fn spawn_with(loader: impl ModelLoader) -> Self {
        let (commands, receiver) = mpsc::channel();
        std::thread::Builder::new()
            .name("lowspeak.local-model".into())
            .spawn(move || {
                Actor {
                    loader,
                    loaded: None,
                }
                .run(receiver);
            })
            .expect("spawning the local model thread");
        Self { commands }
    }

    /// Loads a model ahead of the first dictation and warms it up.
    pub fn preload(&self, model: LocalModel) {
        let _ = self.commands.send(Command::Preload(model));
    }

    /// Loads, warms up and releases a model, so it is ready for an unplanned use: CoreML compiles
    /// a Parakeet model for the Neural Engine on its first load in an app (~10 s) and caches the
    /// result, and whisper.cpp compiles its Metal pipelines on the first run.
    pub fn prime(&self, model: LocalModel) {
        let (reply, _) = mpsc::channel();
        let _ = self.commands.send(Command::Preload(model));
        let _ = self.commands.send(Command::Unload { reply });
    }

    /// Releases the loaded model once every earlier request, including a running preload, is done.
    pub fn unload(&self) {
        let (reply, done) = mpsc::channel();
        if self.commands.send(Command::Unload { reply }).is_ok() {
            let _ = done.recv();
        }
    }

    /// Like `unload`, but gives up after `timeout`; returns whether the model was released.
    pub fn unload_within(&self, timeout: Duration) -> bool {
        let (reply, done) = mpsc::channel();
        self.commands.send(Command::Unload { reply }).is_ok() && done.recv_timeout(timeout).is_ok()
    }
}

struct Slot {
    model: LocalModel,
    engine: Box<dyn LoadedModel>,
    warm: bool,
}

struct Actor<L> {
    loader: L,
    loaded: Option<Slot>,
}

impl<L: ModelLoader> Actor<L> {
    fn run(mut self, commands: Receiver<Command>) {
        for command in commands {
            match command {
                Command::Preload(model) => {
                    if let Err(error) = self.guarded(|actor| actor.preload(model)) {
                        flog!(
                            filelog::APP,
                            "Failed to preload {}: {error}",
                            model.raw_value()
                        );
                    }
                }
                Command::Transcribe {
                    samples,
                    model,
                    language,
                    prompt,
                    reply,
                } => {
                    let result = self.guarded(|actor| {
                        let slot = actor.ensure_loaded(model)?;
                        slot.warm = true;
                        slot.engine
                            .transcribe(&samples, language, prompt.as_deref())
                            .map_err(TranscriptionError::Engine)
                    });
                    let _ = reply.send(result);
                }
                Command::Unload { reply } => {
                    let _ = self.guarded(|actor| {
                        actor.loaded = None;
                        Ok(())
                    });
                    flog!(filelog::APP, "Local model unloaded");
                    let _ = reply.send(());
                }
            }
        }
    }

    fn preload(&mut self, model: LocalModel) -> Result<(), TranscriptionError> {
        let slot = self.ensure_loaded(model)?;
        if !slot.warm {
            slot.engine.warm_up();
            slot.warm = true;
        }
        Ok(())
    }

    fn ensure_loaded(&mut self, model: LocalModel) -> Result<&mut Slot, TranscriptionError> {
        let slot = match self.loaded.take() {
            Some(slot) if slot.model == model => slot,
            previous => {
                // Released before the next one loads, so two models never share the memory.
                drop(previous);
                let started = Instant::now();
                flog!(filelog::APP, "Loading {}", model.raw_value());
                let engine = self.loader.load(model)?;
                flog!(
                    filelog::APP,
                    "Model {} loaded in {:.1}s",
                    model.raw_value(),
                    started.elapsed().as_secs_f64()
                );
                Slot {
                    model,
                    engine,
                    warm: false,
                }
            }
        };
        Ok(self.loaded.insert(slot))
    }

    /// Turns a panic into an error and drops the model, which the panic may have left unusable,
    /// so the thread keeps serving the following requests.
    fn guarded<T>(
        &mut self,
        work: impl FnOnce(&mut Self) -> Result<T, TranscriptionError>,
    ) -> Result<T, TranscriptionError> {
        let payload = match panic::catch_unwind(AssertUnwindSafe(|| work(self))) {
            Ok(result) => return result,
            Err(payload) => payload,
        };
        let message = panic_message(&*payload);
        flog!(filelog::APP, "The local model panicked: {message}");
        if panic::catch_unwind(AssertUnwindSafe(|| self.loaded = None)).is_err() {
            flog!(filelog::APP, "Releasing the model panicked too");
        }
        Err(TranscriptionError::Engine(anyhow!(
            "the local model panicked: {message}"
        )))
    }
}

fn panic_message(payload: &(dyn Any + Send)) -> &str {
    payload
        .downcast_ref::<&str>()
        .copied()
        .or_else(|| payload.downcast_ref::<String>().map(String::as_str))
        .unwrap_or("no message")
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::*;

    type Log = Arc<Mutex<Vec<String>>>;

    /// Small panics while transcribing, Medium while loading, Large v3 is not installed; the
    /// others work.
    struct FakeLoader {
        log: Log,
        load_time: Duration,
    }

    struct FakeModel {
        name: &'static str,
        log: Log,
    }

    impl FakeModel {
        fn note(&self, event: &str) {
            self.log
                .lock()
                .unwrap()
                .push(format!("{event} {}", self.name));
        }
    }

    impl LoadedModel for FakeModel {
        fn transcribe(
            &mut self,
            samples: &[f32],
            _language: Option<&str>,
            _prompt: Option<&str>,
        ) -> anyhow::Result<String> {
            self.note("transcribe");
            assert!(self.name != "small", "Metal command buffer failed");
            Ok(format!("{} samples", samples.len()))
        }

        fn warm_up(&mut self) {
            self.note("warm");
        }
    }

    impl Drop for FakeModel {
        fn drop(&mut self) {
            self.note("drop");
        }
    }

    impl ModelLoader for FakeLoader {
        fn load(&self, model: LocalModel) -> Result<Box<dyn LoadedModel>, TranscriptionError> {
            std::thread::sleep(self.load_time);
            let name = model.raw_value();
            self.log.lock().unwrap().push(format!("load {name}"));
            match model {
                LocalModel::Medium => panic!("corrupt model file"),
                LocalModel::LargeV3 => Err(TranscriptionError::ModelNotAvailable),
                _ => Ok(Box::new(FakeModel {
                    name,
                    log: Arc::clone(&self.log),
                })),
            }
        }
    }

    fn spawn(load_time: Duration) -> (LocalTranscriber, Log) {
        let log = Log::default();
        let transcriber = LocalTranscriber::spawn_with(FakeLoader {
            log: Arc::clone(&log),
            load_time,
        });
        (transcriber, log)
    }

    fn events(log: &Log) -> Vec<String> {
        std::mem::take(&mut *log.lock().unwrap())
    }

    fn transcribe(
        local: &LocalTranscriber,
        model: LocalModel,
    ) -> Result<String, TranscriptionError> {
        local.transcribe(vec![0.0; 8_000], model, None, None)
    }

    #[test]
    fn a_preloaded_model_is_warmed_up_once_and_reused() {
        let (local, log) = spawn(Duration::ZERO);
        local.preload(LocalModel::Tiny);
        local.preload(LocalModel::Tiny);
        assert_eq!(
            transcribe(&local, LocalModel::Tiny).unwrap(),
            "8000 samples"
        );
        assert_eq!(
            transcribe(&local, LocalModel::Tiny).unwrap(),
            "8000 samples"
        );
        assert_eq!(
            events(&log),
            [
                "load tiny",
                "warm tiny",
                "transcribe tiny",
                "transcribe tiny"
            ]
        );
    }

    #[test]
    fn another_model_loads_after_the_previous_one_is_released() {
        let (local, log) = spawn(Duration::ZERO);
        transcribe(&local, LocalModel::Tiny).unwrap();
        transcribe(&local, LocalModel::Base).unwrap();
        local.unload();
        assert_eq!(
            events(&log),
            [
                "load tiny",
                "transcribe tiny",
                "drop tiny",
                "load base",
                "transcribe base",
                "drop base"
            ]
        );
    }

    #[test]
    fn unload_waits_for_a_running_preload_and_prime_releases_the_model() {
        let (local, log) = spawn(Duration::from_millis(100));
        local.prime(LocalModel::Tiny);
        local.unload();
        assert_eq!(events(&log), ["load tiny", "warm tiny", "drop tiny"]);
    }

    #[test]
    fn a_panic_becomes_an_error_and_the_thread_keeps_serving() {
        let (local, log) = spawn(Duration::ZERO);
        let error = transcribe(&local, LocalModel::Small).unwrap_err();
        assert!(
            error.to_string().contains("Metal command buffer failed"),
            "{error}"
        );
        assert_eq!(
            events(&log),
            ["load small", "transcribe small", "drop small"]
        );

        let error = transcribe(&local, LocalModel::Medium).unwrap_err();
        assert!(matches!(error, TranscriptionError::Engine(_)), "{error}");
        local.preload(LocalModel::Medium);
        assert!(transcribe(&local, LocalModel::Tiny).is_ok());
        assert_eq!(
            events(&log),
            ["load medium", "load medium", "load tiny", "transcribe tiny"]
        );
    }

    #[test]
    fn a_model_that_cannot_load_is_reported() {
        let (local, _) = spawn(Duration::ZERO);
        assert!(matches!(
            transcribe(&local, LocalModel::LargeV3),
            Err(TranscriptionError::ModelNotAvailable)
        ));
    }
}
