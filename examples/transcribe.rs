//! Transcribes a 16 kHz WAV file with an installed local model:
//! `cargo run --release --example transcribe -- <file.wav> [model] [language]`, e.g. `parakeet-ultra` or `large-v3-turbo de`.

use std::time::Instant;

use anyhow::{Context as _, Result, ensure};
use lowspeak::audio::wav;
use lowspeak::models::{Language, LocalModel};
use lowspeak::transcription::local::{LocalEngine as _, LocalTranscriber};

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let path = args
        .next()
        .context("usage: transcribe <file.wav> [model] [language]")?;
    let model = args.next().unwrap_or_else(|| "parakeet-ultra".into());
    let model = LocalModel::from_raw(&model).with_context(|| format!("unknown model {model}"))?;
    let language = args
        .next()
        .map(|code| Language::from_raw(&code).context("unknown language"))
        .transpose()?;
    let audio = wav::decode(&std::fs::read(&path)?)?;
    ensure!(
        audio.sample_rate == 16_000,
        "expected 16 kHz audio, got {} Hz",
        audio.sample_rate
    );

    let local = LocalTranscriber::spawn();
    let seconds = audio.samples.len() as f64 / 16_000.0;
    for run in ["first (includes loading)", "second"] {
        let started = Instant::now();
        let text = local.transcribe(
            audio.samples.clone(),
            model,
            language.and_then(Language::code),
            None,
        )?;
        println!(
            "{run}: {:.0} ms for {seconds:.1} s audio\n{text}",
            started.elapsed().as_secs_f64() * 1000.0
        );
    }
    local.unload();
    Ok(())
}
