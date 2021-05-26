use std::io;

fn main() {
    println!("Welcome to the temperature convertor!");

    let temperature_unit = loop {
        println!("Input \"F\" to convert from Fahrenheit or \"C\" to convert from Celsius.");

        let mut temperature_unit = String::new();

        io::stdin()
            .read_line(&mut temperature_unit)
            .expect("Failed to read line");

        let temperature_unit: String = temperature_unit.trim().to_lowercase();

        if temperature_unit == "f" {
            println!("Fahrenheit");
            break temperature_unit;
        } else if temperature_unit == "c" {
            println!("Celsius");
            break temperature_unit;
        } else {
            println!("Incorrect selection, must be \"C\" or \"F\"");
        }
    };

    let temperature: i32 = loop{
        let mut temperature = String::new();

        println!("Enter a number to convert");

        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line");

        let temperature: i32 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        break temperature;
    };

    if temperature_unit == "f" {
        let converted = (temperature as f64 -32.0) * 5.0 /9.0 ;
        println!("Converted temperature is {:.4} degrees celsius.", converted);
    } else {
        let converted = temperature as f64 * 9.0 / 5.0 + 32.0;
        println!("Converted temperature is {:.4} degrees fahrenheit.", converted);
    };
}

