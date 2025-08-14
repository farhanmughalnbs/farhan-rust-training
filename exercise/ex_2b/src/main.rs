// Write a function that takes in a string of any format and converts it to datetime object.

use chrono::{NaiveDateTime, ParseError};

fn parse_any_datetime(datetime_str: &str) -> Result<NaiveDateTime, ParseError> {
    let formats = [
        "%Y-%m-%d %H:%M:%S",
        "%d-%m-%Y %H:%M:%S",
        "%Y/%m/%d %H:%M:%S",
        "%d/%m/%Y %H:%M:%S",
        "%Y-%m-%d",
        "%d-%m-%Y",
    ];

    let mut last_err: Option<ParseError> = None;

    for fmt in &formats {
        match NaiveDateTime::parse_from_str(datetime_str, fmt) {
            Ok(dt) => return Ok(dt),
            Err(e) => last_err = Some(e),
        }
    }

    Err(last_err.unwrap()) // return last error if all formats fail
}

fn main() {
    let date_strings = [
        "2014-07-01 14:43:00",
        "01-07-2014 14:43:00",
        "2014/07/01 14:43:00",
        "invalid-date",
    ];

    for date_str in &date_strings {
        match parse_any_datetime(date_str) {
            Ok(dt) => println!("Parsed: {} -> {}", date_str, dt),
            Err(e) => println!("Failed to parse '{}': {}", date_str, e),
        }
    }
}
