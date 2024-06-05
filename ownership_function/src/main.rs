fn main() {
    let s = String::from("hello");
    take_ownership(s);
    let x = 5;
    make_copy(x);

    println!("{}", x);

    // println!("{}", s);
}

fn take_ownership(some_str: String) {
    println!("{}", some_str);
}

fn make_copy(some_int: i32) {
    println!("{}", some_int);
}
