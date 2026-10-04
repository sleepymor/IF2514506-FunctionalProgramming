// Example: ownership in Rust
// Run with: cargo run --example ownership
//
// The main idea: every value has ONE owner.
// When the owner is done, Rust frees the memory automatically.

fn main() {
    // 1. A value is dropped when its owner goes out of scope
    {
        let message = String::from("hello");
        println!("{}", message);
    } // message is dropped here, we cannot use it anymore
    // println!("dropped:{}", message);

    // 2. Move: giving a String to another variable moves it
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved, so s1 cannot be used anymore
    // println!("{}", s1); // this would NOT compile!
    println!("{}", s2);

    // 3. Clone: make a real copy if we need two copies
    let a = String::from("hi");
    let b = a.clone(); // copies the data, both are usable
    println!("a = {}, b = {}", a, b);

    // 4. Numbers are copied, not moved (they are cheap to copy)
    let x = 5;
    let y = x; // x is still usable because i32 is Copy
    println!("x = {}, y = {}", x, y);
}
