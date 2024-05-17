const CONSTANT_32: f64 = 32.0;

fn main() {
    // converting 86 degree celsius to fahrenheit
    let celsius_value: f64 = 86.0;
    let fahrenheit: f64 = celsius_to_fahrenheit(celsius_value);
    println!("{celsius_value} Celsius to Fahrenheit is: {fahrenheit}");

    // converting 32 degree fahrenheit to celsius
    let fahrenheit_value: f64 = 32.0;
    let celsius: f64 = fahrenheit_to_celsius(fahrenheit_value);
    println!("{fahrenheit_value} Fahrenheit to Celsius is: {celsius}");
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (9.0/5.0 * celsius) + CONSTANT_32
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (5.0/9.0 * fahrenheit) - CONSTANT_32
}
