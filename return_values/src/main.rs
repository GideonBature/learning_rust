fn main() {
    let x = return_5();

    println!("fn 1: {x}");

    let y = return_num(15);

    println!("fn 2: {y}");
}

fn return_5() -> i32 {
    5
}

fn return_num(n: i32) -> i32 {
    let num = n *10;
    num
}
