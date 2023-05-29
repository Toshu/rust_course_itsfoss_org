// compound data types

fn main() {


    // ARRAYS ("[]")


    // without type annotation
    let greeting = ['H', 'e', 'l', 'l', 'o', ' ', 'w', 'o', 'r', 'l', 'd', '!'];
    for character in greeting {
        print!("{}", character);
    }


    // with type annotation
    let pi: [i32; 10] = [1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    println!("\nPi: 3.{}{}{}{}", pi[0], pi[1], pi[2], pi[3]);


    // an array with 5 tens
    let a = [10; 5]; // some as [10, 10, 10, 10, 10]
    for i in a {
       print!("{i} ");
    }
    println!("");



    // TUPLES ("()")

    let a = (38, 923.329, true);
    let b: (char, i32, f64, bool) = ('r', 43, 3.14, false);

    println!("a.0: {}, a.1: {}, a.2: {}", a.0, a.1, a.2);
    println!("b.0: {}, b.1: {}, b.2: {}, b.3: {}", b.0, b.1, b.2, b.3);


    // destructuring a tuple
    let pixel = (50, 0, 200);
    println!("red: {}, green: {}, blue: {}", pixel.0, pixel.1, pixel.2);
    let (red, green, blue) = pixel;
    println!("red: {}, green: {}, blue: {}", red, green, blue);



    // SLICES
    let my_array = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let my_slice = &my_array[0..4]; // -> 0,1,2,3 / [0..=4] -> 0,1,2,3,4

    for element in my_slice {
        println!("{element}");
    }
}