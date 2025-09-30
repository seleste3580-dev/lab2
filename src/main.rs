use std::io;

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn fibonacci(n: u32) -> u64 {
    let (mut a, mut b) = (0, 1);
    for _ in 0..n {
        let temp = b;
        b = a + b;
        a = temp;
    }
    a
}

fn main() {
    println!("Choose an option:");
    println!("1. Convert Temperature");
    println!("2. Generate Fibonacci Number");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    match choice.trim() {
        "1" => {
            println!("Enter value:");
            let mut value = String::new();
            io::stdin().read_line(&mut value).unwrap();
            let value: f64 = value.trim().parse().unwrap();

            println!("Convert from:");
            println!("C for Celsius to Fahrenheit");
            println!("F for Fahrenheit to Celsius");

            let mut direction = String::new();
            io::stdin().read_line(&mut direction).unwrap();

            let result = match direction.trim().to_uppercase().as_str() {
                "C" => celsius_to_fahrenheit(value),
                "F" => fahrenheit_to_celsius(value),
                _ => {
                    println!("Invalid direction.");
                    return;
                }
            };

            println!("Converted value: {:.2}", result);
        }

        "2" => {
            println!("Enter n:");
            let mut n = String::new();
            io::stdin().read_line(&mut n).unwrap();
            let n: u32 = n.trim().parse().unwrap();

            let result = fibonacci(n);
            println!("Fibonacci number at position {} is {}", n, result);
        }

        _ => println!("Invalid choice."),
    }
}
