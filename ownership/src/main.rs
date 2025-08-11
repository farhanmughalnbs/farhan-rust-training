fn main() {
    // String moves by default
    let s1 = String::from("hello");
    let s2 = s1; // s1 moved into s2
    println!("s2 = {s2}");
    // println!("{s1}"); // would not compile (moved)

    // Clone when you need two owners
    let a = String::from("keep both");
    let b = a.clone();
    println!("a = {a}, b = {b}");

    // Copy types (numbers, bools, etc.)
    let n = 5; // i32 implements Copy
    print_num(n); // n is copied
    println!("n is still usable: {n}");
}

fn print_num(x: i32) { println!("num = {x}"); }
