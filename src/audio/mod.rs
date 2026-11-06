pub mod aac;
mod mp4;
pub mod preprocess;
pub mod recorder;
pub mod vad;
pub mod wav;

/// The rate every transcription backend takes.
pub const SAMPLE_RATE: u32 = 16_000;
/// 0.3 s: the shortest clip worth transcribing, and the shortest input Parakeet accepts.
pub const MIN_SAMPLES: usize = 4_800;
