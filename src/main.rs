use std::io;

fn main() {
    println!("Please input a temperature in Fahrenheit: ");

    let mut fahrenheit = String::new();

    io::stdin().read_line(&mut fahrenheit)
        .expect("Failed to read line.");

    loop {
        let fahrenheit: f32 = match fahrenheit.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue,
        };

        let celsius: f32 = (fahrenheit - 32.0) * (5.0 / 9.0);

        println!("*{:.1} Fahrenheit is *{:.1} Celsius", fahrenheit, celsius);

        break;
    }
}
