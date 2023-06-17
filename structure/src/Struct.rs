pub struct Student {
    firstname: String,
    lastname: String,
    age: u64,
    classification: String,
    major: String,
    city: String,
    state: String,
    email: String,
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
    println!("{}", chris.firstname);
    
}
