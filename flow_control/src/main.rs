fn main() {
    let x = 8;
    let sign = if x >= 0 { "non-negative" } else { "negative" };
    println!("x is {sign}");

    let text = match x {
        0 => "zero",
        1..=5 => "small",
        6 | 7 | 8 => "medium",
        _ => "large",
    };
    println!("match -> {text}");

    let mut n = 0;
    while n < 3 { println!("while {n}"); n += 1; }
    for i in 0..3 { println!("for {i}"); }
    let mut c = 0; loop { if c == 2 { break; } println!("loop {c}"); c += 1; }
}
