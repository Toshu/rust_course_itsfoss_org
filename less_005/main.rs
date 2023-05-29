
//  FUNCTIONS

fn main() {

    // simplest function - without parameter and return value
    greet();


    // function with parameters
    food(2, 4);


    // function with return value (and parameter)
    println!(
        "If I buy 2 Kilograms of apples from a fruit vendor, I have to pay {} rupees to them.",
        retail_price(2.0)
    );
    println!(
        "But, if I buy 30 Kilograms of apples from a fruit vendor, I have to pay {} rupees to them.",
        wholesale_price(30.0)
    );


    // function with multiple return values
    let (maths, english, science, sanskrit) = tuple_func();
    println!("Marks obtained in Maths: {maths}");
    println!("Marks obtained in English: {english}");
    println!("Marks obtained in Science: {science}");
    println!("Marks obtained in Sanskrit: {sanskrit}");
}


// simplest function - without parameter and return value
fn greet() {
    println!("Hi there!");
}


// function with parameters
fn food(theplas: i32, rotis: i32) {
    println!(
        "I am hungry... I need {} theplas and {} rotis!",
        theplas, rotis
    );
}


// function with return value (and parameter)
fn retail_price(weight: f64) -> f64 {
    // using a statement
    return weight * 500.0;
}
fn wholesale_price(weight: f64) -> f64 {
    // using an expression
    weight * 400.0
}


// function with multiple return values
fn tuple_func() -> (f64, f64, f64, f64) {
    // return marks for a student
    let maths = 84.50;
    let english = 85.00;
    let science = 75.00;
    let sanskrit = 67.25;

    // returns as expression
//    (maths, english, science, sanskrit)
    // returns as statement
    return (maths, english, science, sanskrit);
}