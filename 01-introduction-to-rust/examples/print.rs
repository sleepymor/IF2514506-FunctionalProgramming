// Example: printing in Rust
// Run with: cargo run --example print

fn main() {
    // 1. println! adds a new line, print! does not
    println!("Hello World!");
    println!("I am learning Rust.");

    print!("Hello, ");
    print!("Rust!");
    println!(); // empty println! just to end the line

    // 2. Printing variables with {}
    let name = "John";
    let age = 30;
    println!("My name is {} and I am {} years old.", name, age);

    // 3. Debug print with {:?} or {:#?} (good for arrays and tuples)
    let scores = [90, 85, 70];
    println!("Scores: {:?}", scores);
}
