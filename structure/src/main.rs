#[derive(Debug)]
struct Student {
    firstname: String,
    lastname: String,
    age: u64,
    classification: String,
    major: String,
    city: String,
    state: String,
    email: String,
}

struct Square {
    side: u32
}

fn main() {
    let chris = Student {
        firstname: String::from("Christopher"),
        lastname: String::from("Bennett"),
        age: 21,
        classification: String::from("Senior"),
        major:String::from("Computer Science"),
        city: String::from("Middletown"),
        state: String::from("Delaware"),
        email: String::from("cbcollege@udel.edu")
    };
    let threebythree = Square {
        side: 3
    };
    println!("{}", chris.firstname);
    println!("Area of a 3 X 3 square is {}", area(&threebythree));
    
}

fn area(square: &Square)->u32{
    square.side * square.side
}