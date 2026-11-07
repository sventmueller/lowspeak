//! End-to-end transcription with locally installed models:
//! `cargo test --release --test transcription -- --ignored` (filter with `parakeet` or `whisper`).
//! The expected Parakeet texts are the output of FluidAudio 0.17.7 for the same audio.

use std::path::Path;

use lowspeak::audio::wav;
use lowspeak::config::settings::Settings;
use lowspeak::models::{Language, LocalModel, OnlineModel, PostProcessingMode, TranscriptionMode};
use lowspeak::transcription::Api;
use lowspeak::transcription::local::LocalTranscriber;
use lowspeak::transcription::model_store::{self, Disk};
use lowspeak::transcription::parakeet::{Parakeet, Version};
use lowspeak::transcription::postprocess::ActiveApp;
use lowspeak::transcription::service::{self, Job, Outcome, Services};
use lowspeak::util::numeric::floor_usize;

fn fixture() -> Vec<f32> {
    let bytes =
        std::fs::read(Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/builtin.wav"))
            .unwrap();
    let audio = wav::decode(&bytes).unwrap();
    assert_eq!(audio.sample_rate, 16_000);
    audio.samples
}

fn require(model: LocalModel) {
    assert!(
        model_store::is_installed(model),
        "{} is not installed; download it in the app first",
        model.raw_value()
    );
}

fn parakeet_ultra() -> Parakeet {
    require(LocalModel::ParakeetUltra);
    Parakeet::load(
        &model_store::location(LocalModel::ParakeetUltra),
        Version::Ultra,
    )
    .unwrap()
}

#[test]
#[ignore = "needs Parakeet Ultra installed"]
fn parakeet_ultra_matches_fluidaudio_on_short_audio() {
    assert_eq!(
        parakeet_ultra().transcribe(&fixture()).unwrap(),
        "Test 12345. Test 1234."
    );
}

#[test]
#[ignore = "needs Parakeet Ultra installed"]
fn parakeet_ultra_matches_fluidaudio_on_audio_longer_than_one_window() {
    let samples = fixture().repeat(4);
    assert_eq!(
        parakeet_ultra().transcribe(&samples).unwrap(),
        "Test 12345. Test 1, 2, 3. Test 1, 2, 3, 4, 5. Test 1, 2, 3. Test 1, 2, 3, 4, 5. \
         Test 1, 2, 3. Test 1, 2, 3, 4, 5. Test 1, 2, 3, 5."
    );
}

fn local_settings(model: LocalModel) -> Settings {
    Settings {
        mode: TranscriptionMode::Local,
        local_model: model,
        online_model: OnlineModel::GptTranscribe,
        language: Language::Auto,
        post_processing: PostProcessingMode::Off,
        preprocessing: true,
        start_sound: false,
    }
}

fn run_pipeline(
    model: LocalModel,
    samples: Vec<f32>,
    raw_parakeet_first: bool,
    vocabulary: Vec<String>,
) -> Outcome {
    let local = LocalTranscriber::spawn();
    let job = Job {
        samples,
        settings: local_settings(model),
        active_app: ActiveApp::default(),
        raw_parakeet_first,
        vocabulary,
    };
    let services = Services {
        local: &local,
        models: &Disk,
        api: Api::SYSTEM,
    };
    service::run(job, &services).unwrap()
}

#[test]
#[ignore = "needs Parakeet Ultra installed"]
fn pipeline_transcribes_with_parakeet() {
    require(LocalModel::ParakeetUltra);
    for raw_first in [true, false] {
        match run_pipeline(LocalModel::ParakeetUltra, fixture(), raw_first, Vec::new()) {
            Outcome::Text(transcript) => {
                assert!(
                    transcript.text.starts_with("Test 12345."),
                    "unexpected text: {}",
                    transcript.text
                );
                assert!(transcript.local_fallback.is_none());
            }
            _ => panic!("expected text"),
        }
    }
}

#[test]
#[ignore = "needs Parakeet Ultra installed"]
fn pipeline_with_parakeet_reports_silence_and_short_audio() {
    require(LocalModel::ParakeetUltra);
    assert!(matches!(
        run_pipeline(
            LocalModel::ParakeetUltra,
            vec![0.0; 32_000],
            true,
            Vec::new()
        ),
        Outcome::TooShort
    ));
    assert!(matches!(
        run_pipeline(
            LocalModel::ParakeetUltra,
            fixture()[..2_000].to_vec(),
            true,
            Vec::new()
        ),
        Outcome::TooShort
    ));
}

#[test]
#[ignore = "needs a Whisper model installed (Tiny, Base, Small or Large v3 Turbo)"]
fn pipeline_transcribes_with_whisper() {
    let model = [
        LocalModel::Tiny,
        LocalModel::Base,
        LocalModel::Small,
        LocalModel::LargeV3Turbo,
    ]
    .into_iter()
    .find(|&model| model_store::is_installed(model))
    .expect("no Whisper model is installed; download one in the app first");
    for vocabulary in [
        Vec::new(),
        vec!["Testlauf".to_owned(), "Lowspeak".to_owned()],
    ] {
        match run_pipeline(model, fixture(), false, vocabulary) {
            Outcome::Text(transcript) => {
                let lower = transcript.text.to_lowercase();
                assert!(
                    lower.contains("test")
                        && (lower.contains('3')
                            || lower.contains("drei")
                            || lower.contains("three")),
                    "unexpected text: {}",
                    transcript.text
                );
            }
            _ => panic!("expected text"),
        }
    }
}

/// Long dictations whose pauses and repeated words land on different seams between Parakeet's 15 s windows. Parakeet
/// itself sometimes drops the second "das" of "Das das" even in a single window, so the check allows a few words.
const SEAM_SCRIPTS: [(&str, [&str; 5]); 2] = [
    (
        "Samantha",
        [
            "This is a long dictation for the window seams. The first window has to end somewhere in the middle of \
             all of this, and nothing may be lost or written twice.",
            "Now comes a word that I say twice on purpose: very very important. The meeting moves from Tuesday to \
             Thursday morning.",
            "After a long pause the speaker goes on, and the next sentence runs straight across the border between \
             two windows without stopping for breath at all, because people do that.",
            "Please order twelve chairs, three tables and one whiteboard for the new office in Hamburg.",
            "That that was the last part of the test. Thank you.",
        ],
    ),
    (
        "Anna",
        [
            "Das ist ein langes Diktat, mit dem ich die Übergänge zwischen den Fenstern prüfe. Irgendwo hier endet \
             das erste Fenster, und trotzdem darf kein Wort fehlen oder doppelt erscheinen.",
            "Jetzt sage ich absichtlich ein Wort zweimal: sehr sehr wichtig. Das Treffen wird von Dienstag auf \
             Donnerstag verschoben.",
            "Nach einer langen Pause spricht die Person weiter, und dieser Satz läuft ohne Atempause quer über die \
             Grenze zwischen zwei Fenstern hinweg, weil Menschen eben so reden.",
            "Bitte bestelle zwölf Stühle, drei Tische und ein Whiteboard für das neue Büro in Hamburg.",
            "Das das war der letzte Teil des Tests. Vielen Dank.",
        ],
    ),
];
const SEAM_PAUSES: [f64; 4] = [3.5, 4.5, 2.5, 5.0];

/// Words a long transcription may differ in from its pieces transcribed one by one: the model picks "a" or "the",
/// "12" or "zwölf" by context. A phrase lost or written twice at a seam is more.
const SEAM_TOLERANCE: usize = 3;

fn silence(seconds: f64) -> impl Iterator<Item = f32> {
    std::iter::repeat_n(0.0, floor_usize(seconds * 16_000.0))
}

/// What macOS's speech synthesis says, as 16 kHz samples.
fn spoken(voice: &str, text: &str) -> Vec<f32> {
    let path = std::env::temp_dir().join(format!(
        "lowspeak-say-{}-{}.wav",
        std::process::id(),
        text.len()
    ));
    let status = std::process::Command::new("say")
        .args([
            "-v",
            voice,
            "--file-format=WAVE",
            "--data-format=LEI16@16000",
            "-o",
        ])
        .arg(&path)
        .arg(text)
        .status()
        .unwrap();
    assert!(status.success(), "say failed");
    let audio = wav::decode(&std::fs::read(&path).unwrap()).unwrap();
    std::fs::remove_file(&path).unwrap();
    assert_eq!(audio.sample_rate, 16_000);
    audio.samples
}

fn words(text: &str) -> Vec<String> {
    text.to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|word| !word.is_empty())
        .map(str::to_owned)
        .collect()
}

/// The words of `found` that `expected` lacks and the other way round, by their longest common subsequence.
fn word_diff(expected: &[String], found: &[String]) -> (Vec<String>, Vec<String>) {
    let (n, m) = (expected.len(), found.len());
    let mut lcs = vec![vec![0usize; m + 1]; n + 1];
    for i in (0..n).rev() {
        for j in (0..m).rev() {
            lcs[i][j] = if expected[i] == found[j] {
                lcs[i + 1][j + 1] + 1
            } else {
                lcs[i + 1][j].max(lcs[i][j + 1])
            };
        }
    }
    let (mut i, mut j) = (0, 0);
    let (mut missing, mut extra) = (Vec::new(), Vec::new());
    while i < n && j < m {
        if expected[i] == found[j] {
            i += 1;
            j += 1;
        } else if lcs[i + 1][j] >= lcs[i][j + 1] {
            missing.push(expected[i].clone());
            i += 1;
        } else {
            extra.push(found[j].clone());
            j += 1;
        }
    }
    missing.extend(expected[i..].iter().cloned());
    extra.extend(found[j..].iter().cloned());
    (missing, extra)
}

/// White noise at about −38 dBFS, louder than the speech gate's ceiling, so the gate counts it as speech.
fn add_noise(samples: &mut [f32]) {
    let mut state = 0x2545_f491_u32;
    for sample in samples {
        state ^= state << 13;
        state ^= state >> 17;
        state ^= state << 5;
        *sample += (state as f32 / u32::MAX as f32 - 0.5) * 0.0416;
    }
}

#[test]
#[ignore = "needs Parakeet Ultra installed; speaks through macOS's speech synthesis"]
fn parakeet_keeps_every_word_across_window_seams() {
    let parakeet = parakeet_ultra();
    let mut failures = Vec::new();
    for (voice, script) in SEAM_SCRIPTS {
        let pieces: Vec<Vec<f32>> = script.iter().map(|text| spoken(voice, text)).collect();
        for (condition, scale, noisy) in [
            ("clean", 1.0f32, false),
            ("quiet", 0.03, false),
            ("noisy", 1.0, true),
        ] {
            let prepare = |samples: &mut Vec<f32>| {
                samples.iter_mut().for_each(|sample| *sample *= scale);
                if noisy {
                    add_noise(samples);
                }
            };
            let expected: Vec<String> = pieces
                .iter()
                .flat_map(|piece| {
                    let mut piece = piece.clone();
                    prepare(&mut piece);
                    words(&parakeet.transcribe(&piece).unwrap())
                })
                .collect();
            for lead in [0.0, 3.0, 6.0, 9.0] {
                let mut long: Vec<f32> = silence(lead).collect();
                for (index, piece) in pieces.iter().enumerate() {
                    long.extend_from_slice(piece);
                    if let Some(&pause) = SEAM_PAUSES.get(index) {
                        long.extend(silence(pause));
                    }
                }
                prepare(&mut long);
                let found = words(&parakeet.transcribe(&long).unwrap());
                let (missing, extra) = word_diff(&expected, &found);
                let run = format!(
                    "{voice} {condition}, {lead} s lead: missing {missing:?}, extra {extra:?}"
                );
                println!("{run}");
                if missing.len() > SEAM_TOLERANCE || extra.len() > SEAM_TOLERANCE {
                    failures.push(run);
                }
            }
        }
    }
    assert!(failures.is_empty(), "{failures:#?}");
}
