fn main() {
    let mut str = String::from("Hello");
    str.push_str(", world!");
    println!("{}", str);
    //ownership fail
    let s1 = String::from("Welcome");
    //Works on strings, via a deep copy (heap-only data)
    let s2 = s1.clone();
    println!("{}, world!", s1);
    println!("s1 = {}, s2 = {}", s1, s2);
    //Shallow copy/stack copy data
    let x = 10;
    let y = x;
    println!("x = {}, y = {}", x, y);
    takes_ownership(str);
    makes_copy(x);
    let string1 = gives_ownership();
    let string2 = String::from("Give Back");
    let string3 = take_n_give_back(string1);
    let string4 = take_n_give_back(string2);
    println!("{}", find_str_length(&string3));
    println!("{}", find_str_length(&string4));
   

}

//Updates the word with a mutuable reference
fn update(word: &mut String){
    word.push_str(" updated!");
}

fn find_str_length(s: &String)->usize {
    return s.len();
}   


fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
}

fn gives_ownership() -> String { // gives_ownership will move its
                                // return value into the function
                                // that calls it
    let string = String::from("Now you own it"); // string comes into scope
    string //returns the string
}

fn take_n_give_back(st: String) -> String{
    st //st is returned and moves out to the calling function
}