//! fn key gestures: hold to dictate; double-tap or Shift+fn records hands-free until the next fn or Shift press.

use std::time::{Duration, Instant};

/// A press shorter than this is a tap.
const TAP: Duration = Duration::from_millis(300);
/// How long after a tap a second press locks the recording.
pub const SECOND_TAP_WINDOW: Duration = Duration::from_millis(400);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Command {
    None,
    Start,
    StartHandsFree,
    Stop,
    /// Keep recording without fn held.
    Lock,
    /// Call `second_tap_missed` with this tap after `SECOND_TAP_WINDOW`.
    AwaitSecondTap(u64),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum State {
    Idle,
    Holding(Instant),
    TapReleased(u64),
    LockPressed,
    Locked,
}

#[derive(Debug)]
pub struct FnGesture {
    state: State,
    taps: u64,
}

impl Default for FnGesture {
    fn default() -> Self {
        Self {
            state: State::Idle,
            taps: 0,
        }
    }
}

impl FnGesture {
    pub fn down(&mut self, at: Instant, shift: bool) -> Command {
        match self.state {
            State::Idle if shift => {
                self.state = State::LockPressed;
                Command::StartHandsFree
            }
            State::Idle => {
                self.state = State::Holding(at);
                Command::Start
            }
            State::TapReleased(_) => {
                self.state = State::LockPressed;
                Command::Lock
            }
            State::Locked => {
                self.state = State::Idle;
                Command::Stop
            }
            State::Holding(_) | State::LockPressed => Command::None,
        }
    }

    pub fn up(&mut self, at: Instant) -> Command {
        match self.state {
            State::Holding(since) if at.saturating_duration_since(since) < TAP => {
                self.taps += 1;
                self.state = State::TapReleased(self.taps);
                Command::AwaitSecondTap(self.taps)
            }
            State::Holding(_) => {
                self.state = State::Idle;
                Command::Stop
            }
            State::LockPressed => {
                self.state = State::Locked;
                Command::None
            }
            State::Idle | State::TapReleased(_) | State::Locked => Command::None,
        }
    }

    pub fn shift_down(&mut self) -> Command {
        match self.state {
            State::Holding(_) => {
                self.state = State::LockPressed;
                Command::Lock
            }
            State::Locked => {
                self.state = State::Idle;
                Command::Stop
            }
            State::Idle | State::TapReleased(_) | State::LockPressed => Command::None,
        }
    }

    pub fn second_tap_missed(&mut self, tap: u64) -> Command {
        if self.state == State::TapReleased(tap) {
            self.state = State::Idle;
            Command::Stop
        } else {
            Command::None
        }
    }

    /// The recording ended for another reason, e.g. its time limit.
    pub fn reset(&mut self) {
        self.state = State::Idle;
    }

    pub fn is_hands_free(&self) -> bool {
        matches!(self.state, State::LockPressed | State::Locked)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ms(start: Instant, millis: u64) -> Instant {
        start + Duration::from_millis(millis)
    }

    #[test]
    fn holding_dictates_until_release() {
        let start = Instant::now();
        let mut gesture = FnGesture::default();
        assert_eq!(gesture.down(start, false), Command::Start);
        assert_eq!(gesture.up(ms(start, 2_000)), Command::Stop);
        assert!(!gesture.is_hands_free());
    }

    #[test]
    fn double_tap_records_hands_free_until_the_next_press() {
        let start = Instant::now();
        let mut gesture = FnGesture::default();
        assert_eq!(gesture.down(start, false), Command::Start);
        let Command::AwaitSecondTap(tap) = gesture.up(ms(start, 120)) else {
            panic!("a short press is a tap");
        };
        assert_eq!(gesture.down(ms(start, 300), false), Command::Lock);
        assert_eq!(gesture.up(ms(start, 400)), Command::None);
        assert_eq!(gesture.second_tap_missed(tap), Command::None);
        assert!(gesture.is_hands_free());
        assert_eq!(gesture.down(ms(start, 30_000), false), Command::Stop);
        assert_eq!(gesture.up(ms(start, 30_100)), Command::None);
        assert!(!gesture.is_hands_free());
        assert_eq!(gesture.down(ms(start, 40_000), false), Command::Start);
    }

    #[test]
    fn a_single_tap_stops_once_the_second_tap_window_has_passed() {
        let start = Instant::now();
        let mut gesture = FnGesture::default();
        gesture.down(start, false);
        let Command::AwaitSecondTap(tap) = gesture.up(ms(start, 100)) else {
            panic!("a short press is a tap");
        };
        assert_eq!(gesture.second_tap_missed(tap), Command::Stop);
        assert_eq!(gesture.second_tap_missed(tap), Command::None);
        assert_eq!(gesture.down(ms(start, 1_000), false), Command::Start);
    }

    #[test]
    fn a_stale_window_does_not_stop_a_newer_recording() {
        let start = Instant::now();
        let mut gesture = FnGesture::default();
        gesture.down(start, false);
        let Command::AwaitSecondTap(first) = gesture.up(ms(start, 100)) else {
            panic!("a short press is a tap");
        };
        gesture.second_tap_missed(first);
        gesture.down(ms(start, 600), false);
        assert_eq!(gesture.second_tap_missed(first), Command::None);
        assert_eq!(gesture.up(ms(start, 3_000)), Command::Stop);
    }

    #[test]
    fn shift_fn_records_hands_free_until_shift_alone() {
        let start = Instant::now();
        let mut gesture = FnGesture::default();
        assert_eq!(gesture.shift_down(), Command::None);
        assert_eq!(gesture.down(start, true), Command::StartHandsFree);
        assert!(gesture.is_hands_free());
        assert_eq!(gesture.shift_down(), Command::None);
        assert_eq!(gesture.up(ms(start, 100)), Command::None);
        assert!(gesture.is_hands_free());
        assert_eq!(gesture.shift_down(), Command::Stop);
        assert!(!gesture.is_hands_free());
        assert_eq!(gesture.down(ms(start, 1_000), false), Command::Start);
    }

    #[test]
    fn shift_while_holding_fn_locks_the_recording() {
        let start = Instant::now();
        let mut gesture = FnGesture::default();
        assert_eq!(gesture.down(start, false), Command::Start);
        assert_eq!(gesture.shift_down(), Command::Lock);
        assert_eq!(gesture.up(ms(start, 2_000)), Command::None);
        assert!(gesture.is_hands_free());
        assert_eq!(gesture.down(ms(start, 9_000), false), Command::Stop);
    }

    #[test]
    fn shift_after_a_single_tap_changes_nothing() {
        let start = Instant::now();
        let mut gesture = FnGesture::default();
        gesture.down(start, false);
        let Command::AwaitSecondTap(tap) = gesture.up(ms(start, 100)) else {
            panic!("a short press is a tap");
        };
        assert_eq!(gesture.shift_down(), Command::None);
        assert_eq!(gesture.second_tap_missed(tap), Command::Stop);
    }

    #[test]
    fn reset_forgets_a_hands_free_recording() {
        let start = Instant::now();
        let mut gesture = FnGesture::default();
        gesture.down(start, false);
        gesture.up(ms(start, 100));
        gesture.down(ms(start, 200), false);
        gesture.reset();
        assert!(!gesture.is_hands_free());
        assert_eq!(gesture.up(ms(start, 300)), Command::None);
        assert_eq!(gesture.down(ms(start, 500), false), Command::Start);
    }
}
