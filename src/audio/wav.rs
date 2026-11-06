use anyhow::{Context as _, Result, bail, ensure};

const BYTES_PER_SAMPLE: u16 = 2;
const HEADER_BYTES: usize = 44;
/// The RIFF size counts the file after its own 8-byte header: the rest of the header plus the data.
const RIFF_OVERHEAD: u32 = 36;

fn header(sample_count: usize, sample_rate: u32) -> Result<Vec<u8>> {
    const TOO_LONG: &str = "recording too long for a WAV file";
    let data_size = sample_count
        .checked_mul(usize::from(BYTES_PER_SAMPLE))
        .and_then(|size| u32::try_from(size).ok())
        .context(TOO_LONG)?;
    let riff_size = data_size.checked_add(RIFF_OVERHEAD).context(TOO_LONG)?;
    let byte_rate = sample_rate
        .checked_mul(u32::from(BYTES_PER_SAMPLE))
        .context("sample rate too high for a WAV file")?;
    let mut out = Vec::with_capacity(HEADER_BYTES);
    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&riff_size.to_le_bytes());
    out.extend_from_slice(b"WAVEfmt ");
    out.extend_from_slice(&16u32.to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&sample_rate.to_le_bytes());
    out.extend_from_slice(&byte_rate.to_le_bytes());
    out.extend_from_slice(&BYTES_PER_SAMPLE.to_le_bytes());
    out.extend_from_slice(&16u16.to_le_bytes());
    out.extend_from_slice(b"data");
    out.extend_from_slice(&data_size.to_le_bytes());
    Ok(out)
}

/// 16-bit PCM WAV of mono float samples; fails for recordings beyond the format's 4 GiB.
pub fn try_encode(samples: &[f32], sample_rate: u32) -> Result<Vec<u8>> {
    let mut out = header(samples.len(), sample_rate)?;
    out.reserve_exact(samples.len() * usize::from(BYTES_PER_SAMPLE));
    for &sample in samples {
        out.extend_from_slice(&to_i16(sample).to_le_bytes());
    }
    Ok(out)
}

/// [`try_encode`] for recordings known to fit a WAV file.
///
/// # Panics
/// If the recording exceeds the format's 4 GiB (37 hours at 16 kHz) or the sample rate 2 GHz.
pub fn encode(samples: &[f32], sample_rate: u32) -> Vec<u8> {
    try_encode(samples, sample_rate).expect("recordings are capped far below the WAV size limit")
}

#[expect(
    clippy::cast_possible_truncation,
    reason = "the sample is clamped to the i16 range first"
)]
pub fn to_i16(sample: f32) -> i16 {
    (sample.clamp(-1.0, 1.0) * 32767.0) as i16
}

pub struct Decoded {
    pub samples: Vec<f32>,
    pub sample_rate: u32,
}

/// Reads integer or float PCM, mixing all channels down to mono.
pub fn decode(bytes: &[u8]) -> Result<Decoded> {
    ensure!(
        bytes.len() >= 12 && &bytes[0..4] == b"RIFF" && &bytes[8..12] == b"WAVE",
        "not a WAV file"
    );
    let mut format: Option<(u16, u16, u32, u16)> = None;
    let mut offset = 12;
    while offset + 8 <= bytes.len() {
        let id = &bytes[offset..offset + 4];
        let size = u32::from_le_bytes(bytes[offset + 4..offset + 8].try_into()?) as usize;
        let body = bytes
            .get(offset + 8..offset + 8 + size)
            .context("truncated WAV chunk")?;
        match id {
            b"fmt " => {
                ensure!(body.len() >= 16, "short fmt chunk");
                let mut tag = u16::from_le_bytes(body[0..2].try_into()?);
                let channels = u16::from_le_bytes(body[2..4].try_into()?);
                let rate = u32::from_le_bytes(body[4..8].try_into()?);
                let bits = u16::from_le_bytes(body[14..16].try_into()?);
                if tag == 0xFFFE && body.len() >= 26 {
                    tag = u16::from_le_bytes(body[24..26].try_into()?);
                }
                format = Some((tag, channels, rate, bits));
            }
            b"data" => {
                let (tag, channels, sample_rate, bits) = format.context("data before fmt chunk")?;
                ensure!(channels > 0, "WAV without channels");
                let read: fn(&[u8]) -> f32 = match (tag, bits) {
                    (1, 16) => {
                        |bytes| f32::from(i16::from_le_bytes([bytes[0], bytes[1]])) / 32768.0
                    }
                    (1, 24) => |bytes| {
                        (i32::from_le_bytes([0, bytes[0], bytes[1], bytes[2]]) >> 8) as f32
                            / 8_388_608.0
                    },
                    (1, 32) => |bytes| {
                        i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as f32
                            / 2_147_483_648.0
                    },
                    (3, 32) => |bytes| f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
                    _ => bail!("unsupported WAV encoding (format {tag}, {bits} bit)"),
                };
                let width = usize::from(bits / 8);
                let frame = width * usize::from(channels);
                let mut samples = Vec::with_capacity(body.len() / frame);
                for chunk in body.chunks_exact(frame) {
                    let sum = chunk
                        .chunks_exact(width)
                        .map(read)
                        .fold(0.0f32, |sum, sample| sum + sample);
                    samples.push(sum / f32::from(channels));
                }
                return Ok(Decoded {
                    samples,
                    sample_rate,
                });
            }
            _ => {}
        }
        offset += 8 + size + size % 2;
    }
    bail!("WAV file has no data chunk")
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    proptest! {
        #[test]
        fn encoded_audio_decodes_within_quantisation(
            samples in prop::collection::vec(-1.0f32..=1.0, 0..2_000),
            rate in 8_000u32..96_000,
        ) {
            let decoded = decode(&encode(&samples, rate)).unwrap();
            prop_assert_eq!(decoded.sample_rate, rate);
            prop_assert_eq!(decoded.samples.len(), samples.len());
            for (a, b) in samples.iter().zip(&decoded.samples) {
                prop_assert!((a - b).abs() < 1e-4);
            }
        }

        #[test]
        fn arbitrary_bytes_do_not_panic(bytes in prop::collection::vec(any::<u8>(), 0..512)) {
            let _ = decode(&bytes);
        }

        #[test]
        fn any_declared_format_is_decoded_or_rejected(
            tag in prop_oneof![Just(1u16), Just(3u16), any::<u16>()],
            channels in 0u16..4,
            bits in 0u16..40,
            data in prop::collection::vec(any::<u8>(), 0..64),
        ) {
            let mut fmt = Vec::new();
            fmt.extend(tag.to_le_bytes());
            fmt.extend(channels.to_le_bytes());
            fmt.extend(16_000u32.to_le_bytes());
            fmt.extend(0u32.to_le_bytes());
            fmt.extend(0u16.to_le_bytes());
            fmt.extend(bits.to_le_bytes());
            let mut file = b"RIFF\0\0\0\0WAVEfmt ".to_vec();
            file.extend(16u32.to_le_bytes());
            file.extend(fmt);
            file.extend(b"data");
            file.extend(u32::try_from(data.len()).unwrap().to_le_bytes());
            file.extend(data);
            let _ = decode(&file);
        }
    }

    #[test]
    fn sizes_beyond_the_format_are_rejected() {
        let largest = usize::try_from((u32::MAX - RIFF_OVERHEAD) / 2).unwrap();
        let bytes = header(largest, 16_000).unwrap();
        assert_eq!(bytes.len(), HEADER_BYTES);
        assert_eq!(&bytes[4..8], &(u32::MAX - 1).to_le_bytes());
        assert!(header(largest + 1, 16_000).is_err());
        assert!(header(usize::MAX, 16_000).is_err());
        assert!(header(1, u32::MAX / 2).is_ok());
        assert!(header(1, u32::MAX / 2 + 1).is_err());
        assert!(try_encode(&[0.0; 4], u32::MAX).is_err());
    }

    #[test]
    fn round_trip_keeps_samples_within_quantisation() {
        let samples: Vec<f32> = (0..1600).map(|i| (i as f32 * 0.01).sin() * 0.5).collect();
        assert_eq!(
            encode(&samples, 16_000),
            try_encode(&samples, 16_000).unwrap()
        );
        let decoded = decode(&encode(&samples, 16_000)).unwrap();
        assert_eq!(decoded.sample_rate, 16_000);
        assert_eq!(decoded.samples.len(), samples.len());
        for (a, b) in samples.iter().zip(&decoded.samples) {
            assert!((a - b).abs() < 1e-4);
        }
    }
}
