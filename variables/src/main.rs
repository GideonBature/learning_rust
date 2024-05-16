const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    println!("The value of 3 hours in seconds is: {THREE_HOURS_IN_SECONDS}");

    let a = 5;
    let a = a + 1;
    {
        let a = a * 2;
        println!("The value of a in the inner scope is: {a}");
    }
    println!("The value of a is: {a}");

    let spaces = "    ";
    let spaces = spaces.len();

    println!("The value of spaces is: {spaces}");

    let guess: u32 = "42".parse().expect("Not a number!");

    println!("The value of guess is: {guess}");
}
