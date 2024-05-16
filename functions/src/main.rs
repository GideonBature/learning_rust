fn main() {
    println!("This is the main function");

    another_fn();

    value(26);

    quantity(10, 's');
}

fn another_fn() {
    println!("This is another function");
}

fn value(x: i32) {
    println!("The value of the argument is: {}", x);
}

fn quantity(x: u64, qty: char) {
    println!("The quantity of measurement is: {x}{qty}");
}
