// Read from stdin
use std::io;

fn main() {
    // Welcome message
    println!("Farenheit--2--Celsius");

    // It rather looks good in a loop
    loop {
        // User given input
        let mut farenheit = String::new();

        // Read user input
        println!("Enter temperature. ");
    
        io::stdin().read_line(&mut farenheit).expect("Error occured reading.");

        // String to f32
        let farenheit: f32 = match farenheit.trim().parse() {
            Ok(temperature) => temperature,
            Err(_) => continue
        };

        // Convert F to C
        let celsius = farenheit_2_celsius(farenheit);

        // Result
        println!("{} F is {} C", farenheit, celsius);
    }
}

fn farenheit_2_celsius(farenheit: f32) -> f32 {
    (farenheit - 32.0) * (5.0 / 9.0)
}
