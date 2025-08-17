use chrono::{NaiveDateTime, DateTime, TimeZone}; // <-- import TimeZone trait
use chrono_tz::Tz;

// Convert a naive datetime from a current timezone to a target timezone safely
fn convert_timezone_safe(
    dt: NaiveDateTime,
    current_tz: Tz,
    target_tz: Tz,
) -> Result<DateTime<Tz>, &'static str> {
    let dt_current = current_tz
        .from_local_datetime(&dt)
        .single()
        .ok_or("Ambiguous or invalid datetime in current timezone")?;

    Ok(dt_current.with_timezone(&target_tz))
}

fn main() {
    let dt_naive = NaiveDateTime::parse_from_str("2025-12-02 01:30:00", "%Y-%m-%d %H:%M:%S")
        .unwrap();

    // Current timezone: New York
    let ny_tz: Tz = "America/New_York".parse().unwrap();

    // Target timezone: Tokyo
    let tokyo_tz: Tz = "Asia/Tokyo".parse().unwrap();

    match convert_timezone_safe(dt_naive, ny_tz, tokyo_tz) {
        Ok(dt_tokyo) => println!("Converted datetime (Tokyo): {}", dt_tokyo),
        Err(e) => println!("Error converting datetime: {}", e),
    }
}
