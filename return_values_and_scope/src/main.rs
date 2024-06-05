fn main() {
    let s1 = give_ownership();

    let s2 = String::from("hello");

    let (s3, len) = calculate_length(s2);
    println!("The length of '{}' is {}.", s3, len);

    let s4 = take_and_give_back(s3);

    println!("{}", s1);
    println!("{}", s4);
}

fn give_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn take_and_give_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
