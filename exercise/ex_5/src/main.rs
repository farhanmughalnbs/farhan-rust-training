use chrono::{DateTime, TimeZone, Utc};

fn unix_to_datetime(timestamp: i64) -> DateTime<Utc> {
    Utc.timestamp_opt(timestamp, 0).unwrap()
}

fn main() {
    let ts = 1609459200; // Example Unix timestamp
    let dt = unix_to_datetime(ts);
    // The original ts in main is still valid after the function call as it is copy type which copy in same context instead of
    // moving ownership like Vec, String which are Non-Copy types.
    println!("Unix timestamp {} -> datetime: {}", ts, dt);
}