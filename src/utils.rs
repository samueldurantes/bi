use std::time::UNIX_EPOCH;

use chrono::{DateTime, Utc};
use serde::Serializer;

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

/// Convert a f64 value to String
/// You can use this function directly on enum definition, like this:
///
/// #[derive(Deserialize, Serialize)]
/// enum Foo {
///     #[serde(serialize_with = "f64_to_string")]
///     Bar: f64,
/// }
pub fn f64_to_string<S>(val: &Option<f64>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match val {
        Some(v) => serializer.serialize_str(&v.to_string()),
        None => serializer.serialize_none(),
    }
}
