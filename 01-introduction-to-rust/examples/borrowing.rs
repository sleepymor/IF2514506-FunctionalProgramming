// Example: borrowing in Rust
// Run with: cargo run --example borrowing
//
// Borrowing = letting a function use a value WITHOUT taking ownership.
// We do it with references (&).

// Takes a reference, so the caller keeps ownership
fn show_length(s: &str) {
    println!("'{}' has {} letters.", s, s.len());
}

// Takes a mutable reference, so it can change the value
fn add_hello(s: &mut String) {
    s.push_str(" hello!");
}

fn main() {
    // 1. Immutable borrow: lend the value for reading
    let name = String::from("John");
    show_length(&name); // borrow it
    println!("I can still use name: {}", name); // still ours!

    // 2. Mutable borrow: lend the value for changing
    let mut text = String::from("hi");
    add_hello(&mut text);
    println!("{}", text);

    // 3. Rule: many readers OR one writer, never both at once
    let data = String::from("abc");
    let r1 = &data;
    let r2 = &data; // two readers is fine
    println!("{} {}", r1, r2);
}
