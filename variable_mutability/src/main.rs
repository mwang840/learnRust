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
   //Looks like we set a variable, outside, have a variable in the inside and increment it by one thus giving it 4.
    let y =  {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");
    
    let num = 10;

    if num % 3 == 0 {
        println!("Divisible by 3");
    }else if num % 4 == 0{
        println!("Remainder of 4");
    }else if num % 5 == 0{
        println!("Divisible by 5");
    }else{
        println!("Neither divisible by 3, 4 or 5");
    }
    //Ifs in a let statement
    let condition = true;
    let outcome = if condition {1} else {0};
    println!("The outcome is: {outcome}");
    println!("{}", looping());
    //Weird looping (nested?)
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
    //While Looping
    let mut number = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("Game Over Son!!");
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

fn looping()->i32{
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 3;
        }
    };
    return result;
}