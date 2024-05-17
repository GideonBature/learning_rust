fn main() {
    let mut number = 10;

    while number >= 0 {
        println!("count down: {number}");
        number -= 1;
    }
    println!("Starting...");

    while_loop();
}

fn while_loop() {
    let arr = [10, 20, 30, 40, 50];
    let mut i = 0;

    while i < 5 {
        println!("arr[{}] = {}", i, arr[i]);
        i += 1;
    } 
}
