// Variable shadowing

fn main() {
    let a = 108;
    println!("addr of a: {:p}, value of a: {a}", &a);
    let a = 56;
    println!("addr of a: {:p}, value of a: {a} // post shadowing", &a);

    let mut b = 82;
    println!("\naddr of b: {:p}, value of b: {b}", &b);
    let mut b = 120;
    println!("addr of b: {:p}, value of b: {b} // post shadowing", &b);

    let mut c = 18;
    println!("\naddr of c: {:p}, value of c: {c}", &c);
    c = 29;
    println!("addr of c: {:p}, value of c: {c} // post shadowing", &c);
}