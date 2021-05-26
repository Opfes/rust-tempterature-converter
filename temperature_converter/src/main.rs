use std::io;

fn main() {
    println!("Welcome to the temperature convertor!");

    loop {
        println!("Input \"F\" to convert from Fahrenheit or \"C\" to convert from Celsius.");

        let mut temperature_unit = String::new();

        io::stdin()
            .read_line(&mut temperature_unit)
            .expect("Failed to read line");

        let temperature_unit: String = temperature_unit.trim().to_lowercase();

        if temperature_unit == "f" {
            println!("Fahrenheit");
            break;
        } else if temperature_unit == "c" {
            println!("Celsius");
            break;
        } else {
            println!("Incorrect selection, must be \"C\" or \"F\"");
        }
    }
}
