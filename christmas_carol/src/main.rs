fn main() {
    christmas_carol_song();
}

fn christmas_carol_song() {
    let days = ["twelfth", "eleventh", "tenth", "ninth", "eighth", "seventh", "sixth", "fifth", "fourth", "third", "second", "first"];
    for count in days {
        println!("On the {} day of Christmas my true love sent to me\n", count);
    }
}
