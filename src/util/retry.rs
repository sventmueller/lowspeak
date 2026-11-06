use std::sync::{Condvar, Mutex, PoisonError};
use std::time::{Duration, Instant};

/// Whether a failed attempt may succeed when repeated.
pub trait Retryable {
    fn is_retryable(&self) -> bool;
}

/// How often to try and how long to wait in between; the wait doubles after every attempt.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Backoff {
    pub max_attempts: u32,
    pub first_delay: Duration,
}

impl Backoff {
    /// The wait before attempt `attempt + 1`.
    fn delay(self, attempt: u32) -> Duration {
        let factor = 1u32.checked_shl(attempt - 1).unwrap_or(u32::MAX);
        self.first_delay.saturating_mul(factor)
    }
}

/// Ends retries early; a wait between attempts returns as soon as it is cancelled.
#[derive(Debug, Default)]
pub struct CancelToken {
    cancelled: Mutex<bool>,
    changed: Condvar,
}

impl CancelToken {
    pub const fn new() -> Self {
        Self {
            cancelled: Mutex::new(false),
            changed: Condvar::new(),
        }
    }

    pub fn cancel(&self) {
        *self
            .cancelled
            .lock()
            .unwrap_or_else(PoisonError::into_inner) = true;
        self.changed.notify_all();
    }

    pub fn is_cancelled(&self) -> bool {
        *self
            .cancelled
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
    }

    /// Waits `duration` unless cancelled first; returns whether it was cancelled.
    pub fn wait(&self, duration: Duration) -> bool {
        let deadline = Instant::now() + duration;
        let mut cancelled = self
            .cancelled
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        while !*cancelled {
            let Some(left) = deadline.checked_duration_since(Instant::now()) else {
                break;
            };
            cancelled = self
                .changed
                .wait_timeout(cancelled, left)
                .unwrap_or_else(PoisonError::into_inner)
                .0;
        }
        *cancelled
    }
}

/// Repeats `operation` while it fails with a retryable error; a cancelled `cancel` ends the
/// retries with the last error.
pub fn with_exponential_backoff<T, E: Retryable>(
    backoff: Backoff,
    cancel: &CancelToken,
    mut operation: impl FnMut() -> Result<T, E>,
) -> Result<T, E> {
    let mut attempt = 1;
    loop {
        match operation() {
            Ok(value) => return Ok(value),
            Err(error)
                if !error.is_retryable()
                    || attempt >= backoff.max_attempts
                    || cancel.wait(backoff.delay(attempt)) =>
            {
                return Err(error);
            }
            Err(_) => attempt += 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;

    struct Failure(bool);

    impl Retryable for Failure {
        fn is_retryable(&self) -> bool {
            self.0
        }
    }

    const IMMEDIATE: Backoff = Backoff {
        max_attempts: 3,
        first_delay: Duration::ZERO,
    };

    #[test]
    fn retries_until_success() {
        let mut attempts = 0;
        let value = with_exponential_backoff(IMMEDIATE, &CancelToken::new(), || {
            attempts += 1;
            if attempts < 3 {
                Err(Failure(true))
            } else {
                Ok(42)
            }
        });
        assert!(matches!(value, Ok(42)));
        assert_eq!(attempts, 3);
    }

    #[test]
    fn does_not_retry_permanent_errors() {
        let mut attempts = 0;
        let result: Result<(), _> =
            with_exponential_backoff(IMMEDIATE, &CancelToken::new(), || {
                attempts += 1;
                Err(Failure(false))
            });
        assert!(result.is_err());
        assert_eq!(attempts, 1);
    }

    #[test]
    fn gives_up_after_max_attempts() {
        let mut attempts = 0;
        let result: Result<(), _> =
            with_exponential_backoff(IMMEDIATE, &CancelToken::new(), || {
                attempts += 1;
                Err(Failure(true))
            });
        assert!(result.is_err());
        assert_eq!(attempts, 3);
    }

    #[test]
    fn delays_double_and_saturate() {
        let backoff = Backoff {
            max_attempts: 10,
            first_delay: Duration::from_secs(2),
        };
        let delays: Vec<u64> = (1..=4).map(|n| backoff.delay(n).as_secs()).collect();
        assert_eq!(delays, [2, 4, 8, 16]);
        assert_eq!(backoff.delay(40), Duration::from_secs(2) * u32::MAX);
    }

    #[test]
    fn cancelling_ends_the_wait_and_the_retries() {
        let cancel = Arc::new(CancelToken::new());
        let backoff = Backoff {
            max_attempts: 3,
            first_delay: Duration::from_secs(60),
        };
        let canceller = {
            let cancel = Arc::clone(&cancel);
            std::thread::spawn(move || {
                std::thread::sleep(Duration::from_millis(50));
                cancel.cancel();
            })
        };
        let started = Instant::now();
        let mut attempts = 0;
        let result: Result<(), _> = with_exponential_backoff(backoff, &cancel, || {
            attempts += 1;
            Err(Failure(true))
        });
        canceller.join().unwrap();
        assert!(result.is_err());
        assert_eq!(attempts, 1);
        assert!(started.elapsed() < Duration::from_secs(10));
    }

    #[test]
    fn a_cancelled_token_skips_every_retry() {
        let cancel = CancelToken::new();
        cancel.cancel();
        assert!(cancel.wait(Duration::from_secs(60)));
        let mut attempts = 0;
        let result: Result<(), _> = with_exponential_backoff(IMMEDIATE, &cancel, || {
            attempts += 1;
            Err(Failure(true))
        });
        assert!(result.is_err());
        assert_eq!(attempts, 1);
    }
}
