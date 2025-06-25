use std::time::UNIX_EPOCH;

use chrono::{DateTime, Utc};

/// Converts sats (u64) to bitcoin (f64).
/// 1 BTC = 10^8 sats.
pub fn sats_to_btc(sats: u64) -> f64 {
    sats as f64 / 100_000_000.0
}

/// Converts a Unix time to timestamptz.
pub fn unix_to_timestamptz(unix_time: u64) -> DateTime<Utc> {
    let d = UNIX_EPOCH + std::time::Duration::from_secs(unix_time);
    DateTime::<Utc>::from(d)
}
