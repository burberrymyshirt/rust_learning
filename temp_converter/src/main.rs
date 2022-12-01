use std::io;

fn main() {
    println!("This program will convert between Fahrenheit and Celsius");

    println!("Would you like to convert from Celsius(1) or from Fahrenheit(2)");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");


    
    if input == 1 && goon {
        celcius();
    }
}
