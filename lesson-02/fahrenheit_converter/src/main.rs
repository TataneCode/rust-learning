use std::io;

fn main() {
    println!("*** Welcome to the fahrenheit converter ***");
    println!("==> Enter temperature in ° celsius : ");
    let mut input_temperature = String::new();
    io::stdin()
        .read_line(&mut input_temperature)
        .expect("Error getting temperature");
    let temperature_celsius: i16 = input_temperature
        .trim()
        .parse()
        .expect("No a valid temperature");
    let temperature_fahrenheit = temperature_celsius * 9 / 5 + 32;
    println!(
        "Chosen Temperature {}°C, equals {}°F /|\\ ^._.^ /|\\",
        temperature_celsius, temperature_fahrenheit
    );
}
