fn main() {
    let mut count = 10;

    'outer_loop: loop {
        println!("count down: {count}");
        count -= 1;

        let mut new_count = 0;

        loop {
            new_count += 1;
            println!("count up: {new_count}");
            if new_count == 3 {
                break;
            }
            if count == 5 {
                break 'outer_loop;
            }
        }
    }
}
