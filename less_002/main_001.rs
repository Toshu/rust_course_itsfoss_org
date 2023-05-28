//
// Read and understand the Rust compiler's error/warning messages
//



// Testing variable immutability - "a" is mutable, "b" is immutable
fn main() {
    let mut a = 172;
    let b = 273;
    println!("a: {a}, b: {b}");

    a = 380;
    b = 420;
    println!("a: {}, b: {}", a, b);
}

// ->
// error[E0384]: cannot assign twice to immutable variable `b`
//   --> main.rs:12:5
//    |
// 8  |     let b = 273;
//    |         -
//    |         |
//    |         first assignment to `b`
//    |         help: consider making this binding mutable: `mut b`
//...
// 12 |     b = 420;
//    |     ^^^^^^^ cannot assign twice to immutable variable
//
// error: aborting due to previous error

// with line "let mut b = 273;" (-> mutable b) the programm gives:
//
// a: 172. b: 273
// a: 380, b: 420
