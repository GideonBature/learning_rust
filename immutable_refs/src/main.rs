fn main() {
    let mut name = String::from("Gideon Bature");

    change(&mut name);

    println!("{}", name);
}

fn change(name: &mut String) {
    name.push_str(" is my name");
}
