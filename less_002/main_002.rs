//
// Read and understand the Rust compiler's error/warning messages
//

// Playing with uninitialized variables
fn main() {
    let a: i32;
    a = 123;
    println!("a: {a}");

    let b: i32;
    println!("b: {b}");
    b = 123;
}

// ->
// warning: value assigned to `b` is never read
//   --> main_002.rs:49:5
//    |
// 49 |     b = 123;
//    |     ^
//    |
//    = help: maybe it is overwritten before being read?
//    = note: `#[warn(unused_assignments)]` on by default

// error[E0381]: used binding `b` is possibly-uninitialized
//   --> main_002.rs:48:19
//    |
// 47 |     let b: i32;
//    |         - binding declared here but left uninitialized
// 48 |     println!("b: {b}");
//    |                   ^ `b` used here but it is possibly-uninitialized
//    |
//    = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

// error: aborting due to previous error; 1 warning emitted

// For more information about this error, try `rustc --explain E0381`.