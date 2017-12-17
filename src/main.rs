// Read from stdin
use std::io;

fn main() {
    // Welcome message
    println!("Temperature Converter");

    loop {
        let mut guess = String::new();
        let c_2_f = 1;
        let f_2_c = 2;

        println!("Enter guess. 1) Celsius to Farenheit \t 2) Farenheit to Celsius");

        // Read guess
        io::stdin().read_line(&mut guess)
            .expect("Error occured reading. \n");
    
        // String to i32
        let guess: i32 = match guess.trim().parse() {
            Ok(g) => g,
            Err(_) => continue
        };

        // Process guess
        if guess == c_2_f {
            handler(true);
        }
        else if guess == f_2_c {
            handler(false);
        }
        else {
            continue;
        }
    }
}

fn handler(is_celsius: bool){
    loop {
        let mut temperature = String::new();
        let result: f32;

        // Read user input
        println!("Enter temperature.");

        io::stdin().read_line(&mut temperature).expect("Error occured reading. \n");

        // String to f32
        let temperature: f32 = match temperature.trim().parse() {
            Ok(t) => t,
            Err(_) => continue
        };

        // Handle conversion
        if is_celsius {
            result = celsius_2_farenheit(temperature);

            println!("{} C is {} F \n", temperature, result);
        }
        else{
            result = farenheit_2_celsius(temperature);

            println!("{} F is {} C \n", temperature, result);
        }

        break;
    }
}

fn farenheit_2_celsius(farenheit: f32) -> f32 {
    (farenheit - 32.0) * (5.0 / 9.0)
}

fn celsius_2_farenheit(celsius: f32) -> f32 {
    (celsius * (5.0 / 9.0)) + 32.0
}