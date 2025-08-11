fn main() {
    let mut s = String::from("Hello");
    s.push_str(", world");
    let slice: &str = &s[0..5];
    println!("string='{s}', slice='{slice}'");
}
