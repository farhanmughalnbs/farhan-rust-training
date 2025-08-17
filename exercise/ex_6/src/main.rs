// Write a function that converts a datetime object to unix timestamp.
use chrono::NaiveDateTime;

fn convert_datetime_to_unix_timestamp(dt: NaiveDateTime) -> i64 {
    dt.timestamp()
}

fn main() {
    let dt = NaiveDateTime::parse_from_str("2024-08-13 15:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let ts = convert_datetime_to_unix_timestamp(dt);
    println!("Datetime {} -> Unix timestamp: {}", dt, ts);
}


