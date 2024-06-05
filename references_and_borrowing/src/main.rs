fn main() {
    let s1 = String::from("Gideon");

    let len = cal_len(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn cal_len(s: &String) -> usize {
    s.len()
}
