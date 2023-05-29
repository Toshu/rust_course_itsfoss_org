// data types

fn main() {

    // INTEGERS

    // use prefix '0b' for Binary representation
    let bin_value = 0b100_0101;

    // use prefix '0o' for Octals
    let oct_value = 0o105;

    // use prefix '0x' for Hexadecimals
    let hex_value = 0x45;

    // same as writing 1 Crore (1,00,00,000 - 10.000.000)
    let dec_value = 1_00_00_000;

    println!("bin_value: {bin_value}");
    println!("oct_value: {oct_value}");
    println!("hex_value: {hex_value}");
    println!("dec_value: {dec_value}");



    // FLOATS

    // f32
    let pi: f32 = 3.1400;

    // f64
    let golden_ratio = 1.610000;

    // decimal point indicates that it must be inferred as a float
    let five = 5.00;

    // even the though type is annotated, a decimal point is still **necessary**
    let six: f64 = 6.;

    println!("pi: {pi}");
    println!("golden_ratio: {golden_ratio}");
    println!("five: {five}");
    println!("six: {six}");



    // Characters

    let a = 'a';
    let p: char = 'p'; // with explicit type annotation
    let crab = '🦀';

    println!("Oh look, {} {}! :{}", a, crab, p);



    // Booleans
    let val_t: bool = true;
    let val_f = false;

    println!("val_t: {val_t}");
    println!("val_f: {val_f}");



    // explicit typecasting
    let a = 3 as f64; // i32 as f64
    let b = 3.14159265359 as i32; // f64 as i32

    println!("a: {a}");
    println!("b: {b}");
}