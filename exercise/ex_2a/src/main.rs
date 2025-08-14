// Write a function that takes in a string of following format and converts it to datetime object
// "2014-07-01 14:43:00"

use chrono::{NaiveDateTime, ParseError};

fn parse_fixed_datetime(datetime_str: &str) -> Result<NaiveDateTime, ParseError> {
    NaiveDateTime::parse_from_str(datetime_str, "%Y-%m-%d %H:%M:%S")
}

fn main() {
    let datetime_str = "2014-07-01 14:43:00";
    match parse_fixed_datetime(datetime_str) {
        Ok(dt) => println!("Parsed datetime: {}", dt),
        Err(e) => println!("Failed to parse datetime: {}",e),
    }
}