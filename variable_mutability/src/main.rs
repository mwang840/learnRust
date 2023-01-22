fn main() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");


    let sum = sum(5, 10);
    let difference = difference(95.5, 4.3);
    let product = product(9, 10);
    let quotient = divide(30.5, 15.25);
    let remain = modulus(40, 7);
    println!("Sum: {sum}, Difference: {difference}, Product: {product}, Quotient: {quotient},  Remainder: {remain}");

    let tru = true;
    let fal: bool = false;
    println!("{tru} != {fal}");

    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😻';
    println!("{c}, {z}, {heart_eyed_cat}");

    let tup :(i32, f64, u8) = (540, 2.7, 1);
    let (x, y, z) = tup;
    println!("The value of x is {x}, followed by the value of y is {y} and finally the value of z is {z}");
    //Another way of doing it without reassigning the tuple to another value
    let one: i32 = tup.0;
    let two: f64 = tup.1; 
    let three: u8 = tup.2;
    println!("Another way of doing it {one}, {two}, {three}");

}

fn sum(x: i32, y: i32) -> i32 {
   return x + y;
}

fn difference(x: f32, y: f32) -> f32 {
    return x - y;
}

fn product(x: i32, y: i32) -> i32 {
    return x * y;
}

fn divide(x: f32, y:f32) -> f32 {
    return x / y;
}

fn modulus(x: i32, y:i32) -> i32 {
    return x % y;
}