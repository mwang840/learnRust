fn main() {
    let mut str = String::from("Hello");
    str.push_str(", world!");
    println!("{}", str);
    //ownership fail
    let s1 = STring::from("Max");
    let s2 = s1;
}
