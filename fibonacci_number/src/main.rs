fn main() {
    let nth: u32 = 20;
    let result: u32 = fib(20);
    println!("The {nth}th fibonacci number is: {result}");
}

fn fib(n: u32) -> u32 {
    if n == 0 || n == 1 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}
