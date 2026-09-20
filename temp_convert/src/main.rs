use std::io;

fn main() {
    const F_TO_C_FACTOR: f32 = 5.0 / 9.0;
    const C_TO_F_FACTOR: f32 = 9.0 / 5.0;
    const CONSTANT_32: f32 = 32.0;

    println!("Welcome to temperature converter!");

    loop {
        println!("Enter your temperature with its symbol, eg. 32F or 0C");

        let mut input_temp = String::new();
        io::stdin()
            .read_line(&mut input_temp)
            .expect("Failed to read the temperature");

        let trimmed = input_temp.trim();
        if trimmed.is_empty() {
            continue;
        }

        let is_celcius = trimmed.ends_with("C") || trimmed.ends_with("c");
        let is_farenheit = trimmed.ends_with("F") || trimmed.ends_with("f");

        if !is_celcius && !is_farenheit {
            println!("Kindly enter the correct symbol at the end.");
            continue;
        }

        let num_part = &trimmed[..trimmed.len() - 1];
        let input_int: f32 = match num_part.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number format!");
                continue;
            }
        };

        let converted_temp: f32;

        if is_celcius {
            converted_temp = (input_int * C_TO_F_FACTOR) + CONSTANT_32;
            println!("{trimmed} = {converted_temp}°F")
        } else {
            converted_temp = (input_int - CONSTANT_32) * F_TO_C_FACTOR;
            println!("{trimmed} = {converted_temp}°C")
        }
    }
}
