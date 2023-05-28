//
// Read and understand the Rust compiler's error/warning messages
//


// Swap numbers
fn main() {
    let mut a = 7186932;
    let mut b = 1276561;

    println!("a: {a}, b: {b}");

    // swap the values
    let temp = a;
    a = b;
    b = temp;

    println!("a: {}, b: {}", a, b);
}