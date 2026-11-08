//! The current recording: which one runs, whether its microphone delivers audio yet, and the fn
//! gesture that started it.

use std::time::Instant;

use super::gesture::{Command, FnGesture};
use crate::hotkey::Key;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum Phase {
    #[default]
    Off,
    /// The microphone is opening or delivers only silence so far.
    Starting(u64),
    Live(u64),
}

#[derive(Debug, Default)]
pub struct Recording {
    phase: Phase,
    last_id: u64,
    gesture: FnGesture,
}

impl Recording {
    pub fn key(&mut self, key: Key, at: Instant) -> Command {
        match key {
            Key::FnDown { shift } => self.gesture.down(at, shift),
            Key::FnUp => self.gesture.up(at),
            Key::ShiftDown => self.gesture.shift_down(),
        }
    }

    pub fn second_tap_missed(&mut self, tap: u64) -> Command {
        self.gesture.second_tap_missed(tap)
    }

    pub fn current(&self) -> Option<u64> {
        match self.phase {
            Phase::Off => None,
            Phase::Starting(id) | Phase::Live(id) => Some(id),
        }
    }

    pub fn is_active(&self) -> bool {
        self.current().is_some()
    }

    /// Whether `id` is the recording that runs now, not one that ended or a later one.
    pub fn is_current(&self, id: u64) -> bool {
        self.current() == Some(id)
    }

    pub fn is_live(&self) -> bool {
        matches!(self.phase, Phase::Live(_))
    }

    pub fn is_hands_free(&self) -> bool {
        self.gesture.is_hands_free()
    }

    pub fn begin(&mut self) -> u64 {
        self.phase = Phase::Starting(self.reserve());
        self.last_id
    }

    /// An id for a dictation that needs no recording, such as another attempt at a failed one.
    pub fn reserve(&mut self) -> u64 {
        self.last_id += 1;
        self.last_id
    }

    /// Returns whether this made recording `id` live; it is false for every later call.
    pub fn go_live(&mut self, id: u64) -> bool {
        if self.phase == Phase::Starting(id) {
            self.phase = Phase::Live(id);
            true
        } else {
            false
        }
    }

    /// The gesture starts over, so the next fn press starts a new recording.
    pub fn end(&mut self) -> Option<u64> {
        let ended = self.current();
        self.phase = Phase::Off;
        self.gesture.reset();
        ended
    }

    /// For a press the app rejected: the gesture starts over, the recording goes on.
    pub fn forget_gesture(&mut self) {
        self.gesture.reset();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_recording_gets_a_new_id_and_goes_live_once() {
        let mut recording = Recording::default();
        assert!(!recording.is_active());
        let first = recording.begin();
        assert!(recording.is_current(first) && !recording.is_live());
        assert!(recording.go_live(first));
        assert!(!recording.go_live(first));
        assert!(recording.is_live());
        assert_eq!(recording.end(), Some(first));
        assert_eq!(recording.end(), None);
        let second = recording.begin();
        assert_ne!(first, second);
        assert!(!recording.is_current(first) && !recording.go_live(first));
        let retry = recording.reserve();
        assert!(retry > second && recording.is_current(second));
    }
}
