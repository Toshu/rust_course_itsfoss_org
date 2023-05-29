// CONDITIONS

fn main() {

    // If  -  else

    // let a = 36;
    // let b = 25;

    // if a > b {
    //     println!("a is greater than b");
    // } else {
    //     println!("b is greater than a");
    // }


    // if  -  else if -  else

    // let a = 40;
    // let b = 40;

    // if a == b {
    //     println!("a and b are equal");
    // } else if a > b {
    //     println!("a is greater than b");
    // } else {
    //     println!("b is greater than a");
    // }


    // more real logic
    let a = 73;
    let b = 56;
    let c = 15;

    if (a != b) && (a != c) && (b != c) {
        if (a > b) && (a > c) {
            println!("a is the greatest");
        } else if (b > a) && (b > c) {
            println!("b is the greatest");
        } else {
            println!("c is the greatest");
        }
    }
}