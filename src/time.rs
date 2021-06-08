use std::time::{SystemTime, UNIX_EPOCH};

pub fn time() -> f64 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap();
    let in_secs =
        since_the_epoch.as_secs_f64() + since_the_epoch.subsec_nanos() as f64 / 1_000_000_000.0_f64;
    return in_secs;
}
