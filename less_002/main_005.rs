// program using constants: Calculate area of circle

fn main() {
    const PI: f64 = 3.14;
    let radius: f64 = 50.0;

    let circle_area = PI * (radius * radius);
    let circle_perimeter = 2.0 * PI * radius;

    println!("There is a circle with the radius of {radius} centimetres.");
    println!("Its area is {} centimetre square.", circle_area);
    println!("And it has circumference of {} centimetres.", circle_perimeter);
}