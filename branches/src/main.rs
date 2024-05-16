fn main() {
    let num = 3;

    if num < 10 {
        println!("{num} is less than 10");
    } else {
        println!("{num} is greater than 10");
    }

    let condition = true;

    let y = if !condition {26} else {24};

    println!("{y}");
}
