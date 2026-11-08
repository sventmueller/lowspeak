//! Finished dictations on their way into the frontmost app, one paste at a time.

use std::collections::VecDeque;

use super::policy::{Dictations, Notice, Notices};
use crate::models::TranscriptionMode;
use crate::transcription::error::TranscriptionError;
use crate::transcription::service::{Outcome, Transcript};

/// Kept in memory only, for copying a dictation again when pasting went to the wrong place.
const RECENT_DICTATIONS: usize = 10;

pub type Result = std::result::Result<Outcome, TranscriptionError>;

/// The dictation as pasted, with a space so that the next dictation does not run into it; Chinese and Japanese put
/// none between sentences.
pub fn pasted(text: &str) -> String {
    match text.chars().last() {
        Some(last) if !last.is_whitespace() && !written_without_spaces(last) => format!("{text} "),
        _ => text.to_owned(),
    }
}

fn written_without_spaces(c: char) -> bool {
    matches!(
        c,
        '\u{3000}'..='\u{30FF}'
            | '\u{31F0}'..='\u{31FF}'
            | '\u{3400}'..='\u{4DBF}'
            | '\u{4E00}'..='\u{9FFF}'
            | '\u{F900}'..='\u{FAFF}'
            | '\u{FF00}'..='\u{FFEF}'
            | '\u{20000}'..='\u{3FFFF}'
    )
}

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

    /// The oldest dictation and its result once it is ready, unless the last paste still has the
    /// clipboard: pasting now would save that text as the content to restore.
    pub fn next(&mut self) -> Option<(u64, Result)> {
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
        assert!(matches!(delivery.next(), Some((1, Ok(Outcome::NoSpeech)))));
        assert!(matches!(delivery.next(), Some((2, Ok(Outcome::TooShort)))));
        assert!(delivery.is_empty());
    }

    #[test]
    fn a_space_follows_the_dictation_except_in_chinese_and_japanese() {
        assert_eq!(pasted("Hello world."), "Hello world. ");
        assert_eq!(pasted("Привет"), "Привет ");
        assert_eq!(pasted("안녕하세요."), "안녕하세요. ");
        assert_eq!(pasted("สวัสดีครับ"), "สวัสดีครับ ");
        assert_eq!(pasted("already spaced "), "already spaced ");
        assert_eq!(pasted("你好。"), "你好。");
        assert_eq!(pasted("你好"), "你好");
        assert_eq!(pasted("こんにちは"), "こんにちは");
        assert_eq!(pasted("カタカナ"), "カタカナ");
        assert_eq!(pasted("本当？"), "本当？");
        assert_eq!(pasted(""), "");
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
