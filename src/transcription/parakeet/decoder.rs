//! Greedy TDT decoding of one encoder window, after FluidAudio's `TdtDecoderV3`.

use anyhow::{Result, anyhow, bail, ensure};

use super::coreml::Strided;
use super::{ENCODER_HIDDEN, Token};
use crate::util::filelog;
use crate::util::numeric::{signed, unsigned};

/// Tokens one frame may emit before decoding is pushed to the next frame, and the number of extra
/// steps at the end of the last window.
const MAX_SYMBOLS_PER_STEP: usize = 10;
const DURATION_BINS: [i64; 5] = [0, 1, 2, 3, 4];
/// FluidAudio's `maxTokensPerChunk`, sized for 11.2 s windows: a 15 s window stops after 10 tokens
/// per second.
const MAX_TOKENS_PER_WINDOW: usize = 150;
const CONSECUTIVE_BLANK_LIMIT: usize = 5;

/// Encoder output copied into `[frame][hidden]` order.
pub struct EncoderFrames {
    data: Vec<f32>,
    pub count: usize,
}

impl EncoderFrames {
    pub fn new(raw: &Strided<f32>, valid_length: usize) -> Result<Self> {
        let shape = &raw.shape;
        ensure!(
            shape.len() == 3 && shape[0] == 1,
            "unexpected encoder output shape {shape:?}"
        );
        let (hidden_axis, time_axis) = if shape[1] == ENCODER_HIDDEN {
            (1, 2)
        } else if shape[2] == ENCODER_HIDDEN {
            (2, 1)
        } else {
            bail!("encoder hidden size mismatch: {shape:?}");
        };
        let count = valid_length.min(shape[time_axis]);
        ensure!(count > 0, "encoder output has no frames");
        let mut data = Vec::with_capacity(count * ENCODER_HIDDEN);
        let mut index = [0usize; 3];
        for frame in 0..count {
            index[time_axis] = frame;
            for hidden in 0..ENCODER_HIDDEN {
                index[hidden_axis] = hidden;
                data.push(raw.get(&index));
            }
        }
        Ok(Self { data, count })
    }

    pub fn frame(&self, index: usize) -> &[f32] {
        &self.data[index * ENCODER_HIDDEN..(index + 1) * ENCODER_HIDDEN]
    }
}

/// The joint network's choice for one frame.
pub struct Decision {
    pub token: usize,
    pub probability: f32,
    pub duration_bin: i64,
}

/// The prediction and joint networks of a TDT model, which the search drives.
pub trait Networks {
    type State;

    /// The prediction network's state before any token.
    fn initial_state(&self) -> Result<Self::State>;

    /// Feeds one token to the prediction network: its projection and the state after it.
    fn predict(&self, token: usize, state: &Self::State) -> Result<(Vec<f32>, Self::State)>;

    /// The joint network's decision for one encoder frame after the given projection.
    fn joint(&self, frames: &EncoderFrames, frame: usize, projection: &[f32]) -> Result<Decision>;
}

/// Where a window sits in the recording, and which of its tokens count.
#[derive(Clone, Copy, Debug)]
pub struct Placement {
    pub actual_audio_frames: usize,
    pub context_frame_adjustment: usize,
    pub is_last_chunk: bool,
    pub global_frame_offset: i64,
    pub emit_tokens_after_global_frame: Option<i64>,
    pub initial_time_index_override: Option<usize>,
}

pub struct Window<'a> {
    pub frames: &'a EncoderFrames,
    pub encoder_sequence_length: usize,
    pub placement: Placement,
}

fn clamp_probability(value: f32) -> f32 {
    if value.is_finite() {
        value.clamp(0.0, 1.0)
    } else {
        0.0
    }
}

fn map_duration_bin(bin: i64) -> Result<i64> {
    usize::try_from(bin)
        .ok()
        .and_then(|index| DURATION_BINS.get(index).copied())
        .ok_or_else(|| anyhow!("duration bin index out of range: {bin}"))
}

fn emit(tokens: &mut Vec<Token>, decision: &Decision, frame: i64, duration: i64, at: Placement) {
    let timestamp = frame + at.global_frame_offset;
    if at
        .emit_tokens_after_global_frame
        .is_none_or(|after| timestamp >= after)
    {
        tokens.push(Token {
            id: decision.token,
            timestamp,
            confidence: clamp_probability(decision.probability),
            duration,
            window_start: at.global_frame_offset,
        });
    }
}

/// Greedy TDT decoding of one encoder window, starting from a fresh prediction-network state.
pub fn decode<N: Networks>(networks: &N, blank_id: usize, window: &Window) -> Result<Vec<Token>> {
    let mut tokens = Vec::new();
    if window.encoder_sequence_length <= 1 {
        return Ok(tokens);
    }
    let frames = window.frames;
    let placement = window.placement;
    // FluidAudio 0.17.7 leaves the mel-context frame out of the window's length, so its v2 windows
    // never decode their last frame.
    let effective = i64::try_from(
        window
            .encoder_sequence_length
            .min(placement.actual_audio_frames + placement.context_frame_adjustment),
    )?;
    let mut time = i64::try_from(
        placement
            .initial_time_index_override
            .unwrap_or(placement.context_frame_adjustment),
    )?;
    if time >= effective {
        return Ok(tokens);
    }
    let last_frame = effective - 1;

    let (mut projection, mut state) = networks.predict(blank_id, &networks.initial_state()?)?;
    let mut last_emission_frame = -1i64;
    let mut emissions_at_frame = 0usize;
    let mut emitted = 0usize;
    while time < effective {
        let mut decision = networks.joint(frames, unsigned(time.min(last_frame)), &projection)?;
        let mut blank = decision.token == blank_id;
        let mut duration = map_duration_bin(decision.duration_bin)?;
        // A blank, and a second token on a frame that already emitted, must move on.
        if duration == 0 && (blank || (time == last_emission_frame && emissions_at_frame >= 1)) {
            duration = 1;
        }
        let mut label_frame = time;
        time += duration;

        // Blanks leave the prediction network unchanged, so its projection is reused.
        while blank && time < effective {
            label_frame = time;
            decision = networks.joint(frames, unsigned(time.min(last_frame)), &projection)?;
            blank = decision.token == blank_id;
            duration = map_duration_bin(decision.duration_bin)?;
            if blank && duration == 0 {
                duration = 1;
            }
            time += duration;
        }
        // Here the decision is a token unless the window ran out; one that jumps past the end is
        // dropped, as in FluidAudio.
        if time >= effective {
            break;
        }

        emitted += 1;
        if emitted > MAX_TOKENS_PER_WINDOW {
            flog!(
                filelog::APP,
                "Parakeet stopped a window after {MAX_TOKENS_PER_WINDOW} tokens at frame {}",
                label_frame + placement.global_frame_offset
            );
            break;
        }
        emit(&mut tokens, &decision, label_frame, duration, placement);
        (projection, state) = networks.predict(decision.token, &state)?;

        if label_frame == last_emission_frame {
            emissions_at_frame += 1;
        } else {
            last_emission_frame = label_frame;
            emissions_at_frame = 1;
        }
        if emissions_at_frame >= MAX_SYMBOLS_PER_STEP {
            time = (time + 1).min(last_frame);
            emissions_at_frame = 0;
            last_emission_frame = -1;
        }
    }

    if placement.is_last_chunk {
        let last_encoder_frame = signed(frames.count) - 1;
        let mut final_time = time;
        let mut consecutive_blanks = 0;
        for step in 0..MAX_SYMBOLS_PER_STEP {
            if consecutive_blanks >= CONSECUTIVE_BLANK_LIMIT {
                break;
            }
            // FluidAudio probes the frames at the end in turn, for tokens due at the boundary.
            let probes = [final_time, effective - 1, (effective - 2).max(0)];
            let frame = probes[step % probes.len()].min(last_encoder_frame);
            let decision = networks.joint(frames, unsigned(frame), &projection)?;
            let duration = map_duration_bin(decision.duration_bin)?;
            if decision.token == blank_id {
                consecutive_blanks += 1;
            } else {
                consecutive_blanks = 0;
                emit(
                    &mut tokens,
                    &decision,
                    final_time.min(effective - 1),
                    duration,
                    placement,
                );
                (projection, state) = networks.predict(decision.token, &state)?;
            }
            final_time = (final_time + duration.max(1)).min(effective);
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use super::*;
    use crate::util::numeric::floor_usize;

    const BLANK: usize = 1_024;

    /// Networks whose joint answers `decide(frame, last token fed)` with (token, duration bin).
    struct Scripted<F> {
        decide: F,
        joints: Cell<usize>,
    }

    impl<F: Fn(usize, usize) -> (usize, i64)> Scripted<F> {
        fn new(decide: F) -> Self {
            Self {
                decide,
                joints: Cell::new(0),
            }
        }
    }

    impl<F: Fn(usize, usize) -> (usize, i64)> Networks for Scripted<F> {
        type State = ();

        fn initial_state(&self) -> Result<()> {
            Ok(())
        }

        fn predict(&self, token: usize, _: &()) -> Result<(Vec<f32>, ())> {
            Ok((vec![token as f32], ()))
        }

        fn joint(
            &self,
            frames: &EncoderFrames,
            frame: usize,
            projection: &[f32],
        ) -> Result<Decision> {
            ensure!(frame < frames.count, "frame {frame} out of range");
            self.joints.set(self.joints.get() + 1);
            let last = floor_usize(f64::from(projection[0]));
            let (token, duration_bin) = (self.decide)(frame, last);
            Ok(Decision {
                token,
                probability: 0.9,
                duration_bin,
            })
        }
    }

    fn frames(count: usize) -> EncoderFrames {
        EncoderFrames {
            data: vec![0.0; count * ENCODER_HIDDEN],
            count,
        }
    }

    fn placement(frames: usize) -> Placement {
        Placement {
            actual_audio_frames: frames,
            context_frame_adjustment: 0,
            is_last_chunk: false,
            global_frame_offset: 0,
            emit_tokens_after_global_frame: None,
            initial_time_index_override: None,
        }
    }

    fn run<F: Fn(usize, usize) -> (usize, i64)>(
        networks: &Scripted<F>,
        count: usize,
        placement: Placement,
    ) -> Vec<(usize, i64)> {
        let frames = frames(count);
        let window = Window {
            frames: &frames,
            encoder_sequence_length: count,
            placement,
        };
        decode(networks, BLANK, &window)
            .unwrap()
            .iter()
            .map(|token| (token.id, token.timestamp))
            .collect()
    }

    /// Token `id` once at `frame`, blank everywhere else.
    fn word_at(words: &[(usize, usize)]) -> impl Fn(usize, usize) -> (usize, i64) {
        let words = words.to_vec();
        move |frame, last| match words.iter().find(|&&(_, at)| at == frame) {
            Some(&(id, _)) if last != id => (id, 1),
            _ => (BLANK, 1),
        }
    }

    #[test]
    fn tokens_are_stamped_with_their_frame_in_the_recording() {
        let networks = Scripted::new(word_at(&[(5, 3), (6, 7)]));
        let mut at = placement(20);
        at.global_frame_offset = 100;
        assert_eq!(run(&networks, 20, at), [(5, 103), (6, 107)]);
    }

    #[test]
    fn a_frame_may_emit_twice_before_it_must_move_on() {
        let networks = Scripted::new(|frame, last| match (frame, last) {
            (3, BLANK) => (5, 0),
            (3, 5) => (6, 0),
            (3, 6) => (7, 0),
            _ => (BLANK, 1),
        });
        assert_eq!(run(&networks, 10, placement(10)), [(5, 3), (6, 3)]);
    }

    #[test]
    fn decoding_starts_after_the_context_and_stops_at_the_real_audio() {
        // Eight frames of real audio follow the context frame; a token whose duration runs past the
        // last of them is the next window's.
        let networks = Scripted::new(word_at(&[(5, 0), (6, 2), (7, 7), (8, 8)]));
        let mut at = placement(8);
        at.context_frame_adjustment = 1;
        assert_eq!(run(&networks, 12, at), [(6, 2), (7, 7)]);
    }

    #[test]
    fn warmup_tokens_are_suppressed() {
        let networks = Scripted::new(word_at(&[(5, 2), (6, 6)]));
        let mut at = placement(10);
        at.global_frame_offset = 50;
        at.emit_tokens_after_global_frame = Some(55);
        at.initial_time_index_override = Some(0);
        assert_eq!(run(&networks, 10, at), [(6, 56)]);
    }

    #[test]
    fn only_the_last_window_recovers_a_token_that_runs_past_its_end() {
        let decide = |frame, last| match (frame, last) {
            (9, BLANK) => (7, 4),
            _ => (BLANK, 1),
        };
        assert!(run(&Scripted::new(decide), 10, placement(10)).is_empty());
        let mut at = placement(10);
        at.is_last_chunk = true;
        assert_eq!(run(&Scripted::new(decide), 10, at), [(7, 9)]);
    }

    #[test]
    fn a_window_stops_after_its_token_limit() {
        let networks = Scripted::new(|_, last| (if last == 5 { 6 } else { 5 }, 0));
        assert_eq!(
            run(&networks, 400, placement(400)).len(),
            MAX_TOKENS_PER_WINDOW
        );
    }

    #[test]
    fn windows_too_short_to_decode_are_empty() {
        let networks = Scripted::new(|_, _| (5, 1));
        assert!(run(&networks, 1, placement(1)).is_empty());
        let mut at = placement(5);
        at.context_frame_adjustment = 5;
        assert!(run(&networks, 5, at).is_empty());
        assert_eq!(networks.joints.get(), 0);
    }

    #[test]
    fn an_unknown_duration_bin_is_an_error() {
        let networks = Scripted::new(|_, _| (BLANK, 5));
        let frames = frames(4);
        let window = Window {
            frames: &frames,
            encoder_sequence_length: 4,
            placement: placement(4),
        };
        assert!(decode(&networks, BLANK, &window).is_err());
    }
}
