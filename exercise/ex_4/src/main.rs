use chrono::{NaiveDateTime, Utc};

fn seconds_since(dt: &NaiveDateTime) -> i64 {
    // Get current UTC time as NaiveDateTime
    let now = Utc::now().naive_utc();
    
    // Calculate the duration between now and the input datetime
    let duration = now - *dt;
    
    // Return the number of seconds
    duration.num_seconds()
}

fn main() {
    let dt = NaiveDateTime::parse_from_str("2025-08-13 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    
    let seconds = seconds_since(&dt);
    println!("Seconds between NOW and {}: {}", dt, seconds);
}
