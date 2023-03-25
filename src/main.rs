use std::io;

fn main() {
    println!("Convert from Fahrenheit and Celcius (0) (default), or Celcius to Fahrenheit (1)? ");

    let mut from_celcius = String::new();

    io::stdin()
        .read_line(&mut from_celcius)
        .expect("Error reading line.");

    let from_celcius: u32 = from_celcius
        .trim()
        .parse()
        .expect("Please enter a valid number.");

    let from_celcius: bool = if from_celcius == 1 { true } else { false };

    println!("Enter value to convert: ");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Error reading line.");

    let input: f64 = input
        .trim()
        .parse::<f64>()
        .expect("Please enter a decimal value!!");

    let result: f64 = if from_celcius {
        (input) * (9.0 / 5.0) + 32.0
    } else {
        (5.0 / 9.0) * (input - 32.0)
    };

    println!("Result: {}", result);
}
