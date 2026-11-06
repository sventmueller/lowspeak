//! OpenAI Whisper through whisper.cpp, on the GPU with Metal on Apple Silicon.

use std::path::Path;
use std::time::Instant;

use anyhow::{Context as _, Result, anyhow};
use whisper_rs::{
    FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters, WhisperState,
};

use crate::audio::SAMPLE_RATE;
use crate::util::filelog;

/// Whisper conditions on at most half of its 448-token text context.
const MAX_PROMPT_TOKENS: usize = 224;
const WARM_UP_SECONDS: u32 = 3;

pub struct Whisper {
    context: WhisperContext,
    /// Kept across dictations: a new state allocates the KV caches and compute buffers again, and
    /// with `no_context` a run starts from the same blank state as on a fresh one.
    state: WhisperState,
}

impl Whisper {
    pub fn load(path: &Path) -> Result<Self> {
        // Without a logging backend this only silences whisper.cpp's output on stderr.
        whisper_rs::install_logging_hooks();
        let mut parameters = WhisperContextParameters::default();
        parameters.flash_attn(true);
        // ggml's fast Metal kernels need an Apple GPU (simdgroup matrices), so Intel Macs run Whisper on the CPU.
        parameters.use_gpu(cfg!(target_arch = "aarch64"));
        let context = WhisperContext::new_with_params(path, parameters)
            .map_err(|error| anyhow!("loading {}: {error}", path.display()))?;
        let state = context.create_state().context("creating whisper state")?;
        Ok(Self { context, state })
    }

    /// The first run compiles the Metal pipelines; running it on a tone keeps that off a dictation.
    pub fn warm_up(&mut self) {
        let started = Instant::now();
        let rate = SAMPLE_RATE as f32;
        let tone: Vec<f32> = (0..WARM_UP_SECONDS * SAMPLE_RATE)
            .map(|i| 0.3 * (2.0 * std::f32::consts::PI * 440.0 * i as f32 / rate).sin())
            .collect();
        match self.transcribe(&tone, None, None) {
            Ok(text) => flog!(
                filelog::APP,
                "  Whisper warm-up done in {:.1}s: {text:?}",
                started.elapsed().as_secs_f64()
            ),
            Err(error) => flog!(filelog::APP, "  Whisper warm-up failed: {error:#}"),
        }
    }

    /// `prompt` lists terms whose spelling the model should follow.
    pub fn transcribe(
        &mut self,
        samples: &[f32],
        language: Option<&str>,
        prompt: Option<&str>,
    ) -> Result<String> {
        // Tokens instead of `set_initial_prompt`, which leaks its string in whisper-rs 0.16.
        let prompt_tokens =
            match prompt.map(|prompt| self.context.tokenize(prompt, MAX_PROMPT_TOKENS)) {
                Some(Ok(tokens)) => tokens,
                Some(Err(error)) => {
                    flog!(filelog::APP, "Whisper prompt skipped: {error}");
                    Vec::new()
                }
                None => Vec::new(),
            };
        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
        if !prompt_tokens.is_empty() {
            params.set_tokens(&prompt_tokens);
        }
        params.set_language(Some(language.unwrap_or("auto")));
        let threads = std::thread::available_parallelism().map_or(4, |n| n.get().min(8));
        params.set_n_threads(i32::try_from(threads).unwrap_or(4));
        params.set_no_context(true);
        params.set_print_progress(false);
        params.set_print_realtime(false);
        params.set_print_special(false);
        params.set_print_timestamps(false);
        self.state
            .full(params, samples)
            .map_err(|error| anyhow!("whisper transcription failed: {error}"))?;
        let mut text = String::new();
        for segment in self.state.as_iter() {
            text.push_str(
                &segment
                    .to_str_lossy()
                    .map_err(|error| anyhow!("reading whisper segment: {error}"))?,
            );
        }
        Ok(text.trim().to_owned())
    }
}
