// src/error_handling.rs
use std::fs::File;
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example 1: Result propagation with ?
    let contents = read_file_maybe("Cargo.toml").unwrap_or_else(|_| String::from("<missing>"));
    println!("Cargo.toml starts with: {}", &contents.chars().take(20).collect::<String>());

    // Example 2: Custom error via Result
    match divide(10.0, 0.0) {
        Ok(v) => println!("10/2 = {v}"),
        Err(e) => println!("divide error -> {e}"),
    }

    Ok(())
}

fn read_file_maybe(path: &str) -> Result<String, io::Error> {
    let mut s = String::new();
    let mut f = File::open(path)?; // propagate error
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 { Err("division by zero".into()) } else { Ok(a / b) }
}
