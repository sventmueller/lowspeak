//! The recorder on a thread of its own: opening a microphone takes up to a second or two
//! (Bluetooth headsets), which would otherwise stall the menu bar and the next fn events.

use std::panic::AssertUnwindSafe;
use std::sync::mpsc::{self, Receiver, Sender};
use std::time::Duration;

use crate::audio::recorder::{Captured, OnLive, Recorder, StartError};
use crate::util::filelog;

type Started = Box<dyn FnOnce(Result<(), StartError>) + Send>;
type Stopped = Box<dyn FnOnce(Captured) + Send>;
type Measured = Box<dyn FnOnce(Option<f32>) + Send>;

enum Command {
    Start {
        microphone: Option<String>,
        on_live: OnLive,
        started: Started,
    },
    Stop(Stopped),
    Level(Measured),
}

impl Command {
    /// Answers without a recorder, for when its thread is gone.
    fn fail(self) {
        match self {
            Self::Start { started, .. } => started(Err(StartError::Device(
                "the recorder thread has stopped".into(),
            ))),
            Self::Stop(stopped) => stopped(Recorder::new().stop()),
            Self::Level(measured) => measured(None),
        }
    }
}

/// Commands run in the order they were given, so a stop always follows its start.
pub struct Capture {
    commands: Sender<Command>,
}

impl Capture {
    /// `capacity` is the longest recording the buffer holds.
    pub fn spawn(capacity: Duration) -> Self {
        let (commands, receiver) = mpsc::channel();
        std::thread::Builder::new()
            .name("lowspeak.recorder".into())
            .spawn(move || run(&receiver, capacity))
            .expect("spawning the recorder thread");
        Self { commands }
    }

    fn send(&self, command: Command) {
        if let Err(mpsc::SendError(command)) = self.commands.send(command) {
            command.fail();
        }
    }

    pub fn start(
        &self,
        microphone: Option<String>,
        on_live: OnLive,
        started: impl FnOnce(Result<(), StartError>) + Send + 'static,
    ) {
        self.send(Command::Start {
            microphone,
            on_live,
            started: Box::new(started),
        });
    }

    pub fn stop(&self, stopped: impl FnOnce(Captured) + Send + 'static) {
        self.send(Command::Stop(Box::new(stopped)));
    }

    pub fn level(&self, measured: impl FnOnce(Option<f32>) + Send + 'static) {
        self.send(Command::Level(Box::new(measured)));
    }
}

fn run(commands: &Receiver<Command>, capacity: Duration) {
    let mut recorder = Recorder::new();
    for command in commands {
        match command {
            Command::Start {
                microphone,
                on_live,
                started,
            } => started(
                guarded(&mut recorder, |recorder| {
                    recorder.start(microphone.as_deref(), capacity, on_live)
                })
                .unwrap_or_else(|| Err(StartError::Device("the recorder panicked".into()))),
            ),
            Command::Stop(stopped) => stopped(
                guarded(&mut recorder, Recorder::stop).unwrap_or_else(|| Recorder::new().stop()),
            ),
            Command::Level(measured) => {
                measured(guarded(&mut recorder, |recorder| recorder.level()).flatten());
            }
        }
    }
}

/// Every command gets its answer even if the recorder panics, since a dictation without one
/// would hold up every later dictation.
fn guarded<T>(recorder: &mut Recorder, work: impl FnOnce(&mut Recorder) -> T) -> Option<T> {
    match std::panic::catch_unwind(AssertUnwindSafe(|| work(recorder))) {
        Ok(value) => Some(value),
        Err(_) => {
            flog!(
                filelog::AUDIO,
                "The recorder panicked; starting over with a new one"
            );
            *recorder = Recorder::new();
            None
        }
    }
}
