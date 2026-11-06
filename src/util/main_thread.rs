use std::time::Duration;

use dispatch2::{DispatchQueue, DispatchTime};

pub fn run<F: FnOnce() + Send + 'static>(work: F) {
    DispatchQueue::main().exec_async(work);
}

pub fn after<F: FnOnce() + Send + 'static>(delay: Duration, work: F) {
    let when = DispatchTime::try_from(delay).unwrap_or(DispatchTime::NOW);
    let _ = DispatchQueue::main().after(when, work);
}
