//! Conversions between counts, frame indices and floats. Sample, frame and token counts stay far
//! below the limits of these types, so the conversions saturate instead of failing.

/// Rounds toward zero; negative values and NaN give 0.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "a float-to-integer `as` saturates, which is the intent"
)]
pub fn floor_usize(value: f64) -> usize {
    value as usize
}

/// Rounds toward zero, saturating at the ends of `i64`.
#[expect(
    clippy::cast_possible_truncation,
    reason = "a float-to-integer `as` saturates, which is the intent"
)]
pub fn floor_i64(value: f64) -> i64 {
    value as i64
}

pub fn signed(value: usize) -> i64 {
    i64::try_from(value).unwrap_or(i64::MAX)
}

/// Negative values give 0.
pub fn unsigned(value: i64) -> usize {
    usize::try_from(value).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversions_saturate() {
        assert_eq!(floor_usize(2.9), 2);
        assert_eq!(floor_usize(-1.0), 0);
        assert_eq!(floor_usize(f64::NAN), 0);
        assert_eq!(floor_i64(-2.5), -2);
        assert_eq!(signed(usize::MAX), i64::MAX);
        assert_eq!(unsigned(-5), 0);
        assert_eq!(unsigned(7), 7);
    }
}
