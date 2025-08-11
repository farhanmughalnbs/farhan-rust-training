fn main() {
    let mut name = String::from("Rust");

    // Immutable borrow: read-only
    print_len(&name);

    // Mutable borrow: exactly one at a time, in its own scope
    {
        let r = &mut name;
        r.push_str("ace");
    } // r goes out of scope here

    println!("name = {name}");
}

fn print_len(s: &String) { println!("len = {}", s.len()); }