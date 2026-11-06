//! Finished dictations on their way into the frontmost app, one paste at a time.

use std::collections::VecDeque;

use super::policy::{Dictations, Notice, Notices};
use crate::models::TranscriptionMode;
use crate::transcription::error::TranscriptionError;
use crate::transcription::service::{Outcome, Transcript};

/// Kept in memory only, for copying a dictation again when pasting went to the wrong place.
const RECENT_DICTATIONS: usize = 10;

pub type Result = std::result::Result<Outcome, TranscriptionError>;

#[derive(Default)]
pub struct Delivery {
    queue: Dictations<Result>,
    /// The clipboard is not restored yet from the last paste.
    pasting: bool,
    /// Newest first.
    recent: VecDeque<String>,
    notices: Notices,
}

impl Delivery {
    pub fn expect(&mut self, recording: u64) {
        self.queue.push(recording);
    }

    pub fn finish(&mut self, recording: u64, result: Result) {
        self.queue.finish(recording, result);
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    /// The oldest dictation's result once it is ready, unless the last paste still has the
    /// clipboard: pasting now would save that text as the content to restore.
    pub fn next(&mut self) -> Option<Result> {
        if self.pasting {
            None
        } else {
            self.queue.pop_ready()
        }
    }

    pub fn paste_started(&mut self) {
        self.pasting = true;
    }

    pub fn paste_done(&mut self) {
        self.pasting = false;
    }

    pub fn remember(&mut self, text: String) {
        self.recent.push_front(text);
        self.recent.truncate(RECENT_DICTATIONS);
    }

    pub fn recent(&self) -> impl Iterator<Item = &String> {
        self.recent.iter()
    }

    pub fn notices_after<'a>(
        &mut self,
        transcript: &'a Transcript,
        mode: TranscriptionMode,
    ) -> Vec<Notice<'a>> {
        self.notices.after(transcript, mode)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_paste_holds_back_the_next_dictation_until_the_clipboard_is_restored() {
        let mut delivery = Delivery::default();
        delivery.expect(1);
        delivery.expect(2);
        delivery.finish(1, Ok(Outcome::NoSpeech));
        delivery.finish(2, Ok(Outcome::TooShort));
        delivery.paste_started();
        assert!(delivery.next().is_none());
        delivery.paste_done();
        assert!(matches!(delivery.next(), Some(Ok(Outcome::NoSpeech))));
        assert!(matches!(delivery.next(), Some(Ok(Outcome::TooShort))));
        assert!(delivery.is_empty());
    }

    #[test]
    fn only_the_last_ten_dictations_are_kept() {
        let mut delivery = Delivery::default();
        for index in 0..12 {
            delivery.remember(format!("dictation {index}"));
        }
        let recent: Vec<&String> = delivery.recent().collect();
        assert_eq!(recent.len(), RECENT_DICTATIONS);
        assert_eq!(recent[0], "dictation 11");
        assert_eq!(recent[9], "dictation 2");
    }
}
