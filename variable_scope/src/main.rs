fn main() {

    let s = String::from("Buwaya");
    let _scp = s.clone();
    let num = 5;
    let x = num;
    {
        let mut s = String::from("Software engineering");
        s.push_str(" is the best thing that has ever happened to me!");

        println!("{s}");
    }

    println!("{}", s);
    println!("{}", _scp);

    println!("{}", num);
    println!("{}", x);

}
