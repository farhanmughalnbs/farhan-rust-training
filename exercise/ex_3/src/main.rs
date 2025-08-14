use chrono::NaiveDateTime;

fn datetime_to_string(dt: &NaiveDateTime) -> String {
    dt.format("%Y-%m-%d %H:%M:%S").to_string()
}

fn main() {
    let dt = NaiveDateTime::parse_from_str("2014-07-01 14:43:00", "%Y-%m-%d %H:%M:%S").unwrap();
    
    let dt_string = datetime_to_string(&dt);
    println!("Formatted datetime: {}", dt_string);
}
