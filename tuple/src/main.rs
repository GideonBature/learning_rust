fn main() {
    let tup: (u16, f64, bool, char) = (25, 52.53, true, 'G');

    let (a, b, c, d) = tup;

    println!("{a} {b} {c} {d}");
}
