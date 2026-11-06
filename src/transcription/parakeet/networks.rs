//! Parakeet's prediction (LSTM) and joint networks on CoreML, for the search in `decoder`.

use anyhow::{Context as _, Result, bail, ensure};
use objc2::rc::Retained;
use objc2_core_ml::MLMultiArray;

use super::coreml::{self, Model};
use super::decoder::{Decision, EncoderFrames, Networks};
use super::{DECODER_HIDDEN, ENCODER_HIDDEN};

const DECODER_LAYERS: usize = 2;

pub struct LstmState {
    hidden: Retained<MLMultiArray>,
    cell: Retained<MLMultiArray>,
}

/// The two models with input arrays reused across steps.
pub struct CoreMlNetworks<'a> {
    decoder: &'a Model,
    joint: &'a Model,
    targets: Retained<MLMultiArray>,
    target_length: Retained<MLMultiArray>,
    encoder_step: Retained<MLMultiArray>,
    decoder_step: Retained<MLMultiArray>,
}

impl<'a> CoreMlNetworks<'a> {
    pub fn new(decoder: &'a Model, joint: &'a Model) -> Result<Self> {
        Ok(Self {
            decoder,
            joint,
            targets: coreml::i32_array(&[1, 1], &[0])?,
            target_length: coreml::i32_array(&[1], &[1])?,
            encoder_step: coreml::f32_array(&[1, ENCODER_HIDDEN, 1], &[0.0; ENCODER_HIDDEN])?,
            decoder_step: coreml::f32_array(&[1, DECODER_HIDDEN, 1], &[0.0; DECODER_HIDDEN])?,
        })
    }
}

fn single<T: Copy>(values: &coreml::Strided<T>, label: &str) -> Result<T> {
    ensure!(
        values.data.len() == 1,
        "joint decision returned {} values for {label}",
        values.data.len()
    );
    Ok(values.data[0])
}

impl Networks for CoreMlNetworks<'_> {
    type State = LstmState;

    fn initial_state(&self) -> Result<LstmState> {
        let zeros = vec![0.0; DECODER_LAYERS * DECODER_HIDDEN];
        Ok(LstmState {
            hidden: coreml::f32_array(&[DECODER_LAYERS, 1, DECODER_HIDDEN], &zeros)?,
            cell: coreml::f32_array(&[DECODER_LAYERS, 1, DECODER_HIDDEN], &zeros)?,
        })
    }

    fn predict(&self, token: usize, state: &LstmState) -> Result<(Vec<f32>, LstmState)> {
        coreml::write_i32(&self.targets, &[i32::try_from(token)?])?;
        let output = self.decoder.predict(&[
            ("targets", &self.targets),
            ("target_length", &self.target_length),
            ("h_in", &state.hidden),
            ("c_in", &state.cell),
        ])?;
        let projection = coreml::read_f32(&*output.array("decoder")?, "decoder output")?;
        let shape = &projection.shape;
        ensure!(
            shape.len() == 3 && shape[0] == 1,
            "unexpected decoder output shape {shape:?}"
        );
        let hidden_axis = if shape[2] == DECODER_HIDDEN {
            2
        } else if shape[1] == DECODER_HIDDEN {
            1
        } else {
            bail!("decoder projection hidden size mismatch: {shape:?}");
        };
        let mut index = [0usize; 3];
        let values = (0..DECODER_HIDDEN)
            .map(|hidden| {
                index[hidden_axis] = hidden;
                projection.get(&index)
            })
            .collect();
        let state = LstmState {
            hidden: output.array("h_out")?,
            cell: output.array("c_out")?,
        };
        Ok((values, state))
    }

    fn joint(&self, frames: &EncoderFrames, frame: usize, projection: &[f32]) -> Result<Decision> {
        ensure!(
            frame < frames.count,
            "encoder frame index out of range: {frame}"
        );
        coreml::write_f32(&self.encoder_step, frames.frame(frame))?;
        coreml::write_f32(&self.decoder_step, projection)?;
        let output = self.joint.predict(&[
            ("encoder_step", &self.encoder_step),
            ("decoder_step", &self.decoder_step),
        ])?;
        let token = single(
            &coreml::read_i32(&*output.array("token_id")?, "token_id")?,
            "token_id",
        )?;
        let probability = single(
            &coreml::read_f32(&*output.array("token_prob")?, "token_prob")?,
            "token_prob",
        )?;
        let duration = single(
            &coreml::read_i32(&*output.array("duration")?, "duration")?,
            "duration",
        )?;
        Ok(Decision {
            token: usize::try_from(token)
                .with_context(|| format!("joint decision returned token id {token}"))?,
            probability,
            duration_bin: i64::from(duration),
        })
    }
}
