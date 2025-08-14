// 1. Write a function that takes in a datetime object, and determines whether the given year is a leap year.

use chrono::{NaiveDate, Datelike};

fn is_leap_year(date: &NaiveDate) -> bool {
    let year = date.year();
    return (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
}

fn main() {
    let date = NaiveDate::from_ymd_opt(2024, 3, 1).unwrap(); // 2024-03-01
    if is_leap_year(&date) {
        println!("{} is a leap year", date.year());
    } else {
        println!("{} is not a leap year", date.year());
    }
}