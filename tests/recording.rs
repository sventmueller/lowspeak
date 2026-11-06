//! Records from the default microphone: `cargo test --release --test recording -- --ignored`.
//! The terminal that runs it needs microphone access.

use std::time::Duration;

use lowspeak::audio::SAMPLE_RATE;
use lowspeak::audio::recorder::Recorder;

fn record(capacity: Duration, length: Duration) -> Vec<f32> {
    let mut recorder = Recorder::new();
    recorder
        .start(None, capacity, Box::new(|| {}))
        .unwrap_or_else(|error| panic!("{error}"));
    std::thread::sleep(length);
    assert!(recorder.level().is_some(), "no audio arrived");
    recorder.stop().into_16k().unwrap()
}

#[test]
#[ignore = "records from the default microphone"]
fn records_from_the_default_microphone_within_its_capacity() {
    let rate = usize::try_from(SAMPLE_RATE).unwrap();
    let samples = record(Duration::from_secs(10), Duration::from_millis(1_500));
    // The microphone needs up to 0.5 s to start, AirPods longer.
    assert!(
        (rate / 2..rate * 2).contains(&samples.len()),
        "{} samples for 1.5 s",
        samples.len()
    );

    let capped = record(Duration::from_secs(1), Duration::from_millis(2_500));
    assert!(
        capped.len() <= rate + rate / 100,
        "{} samples kept with room for 1 s",
        capped.len()
    );
}
