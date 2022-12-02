use std::io;

fn main() {
    println!("This program will convert between Fahrenheit and Celsius");

    loop {
        println!("Would you like to convert from Celsius(1) or from Fahrenheit(2)");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        if input == 1 {
            celcius();
        } else if input == 2 {
            fahrenheit();
        }
    }
}


fn celcius() {
    loop {
        print!("Input your degrees in celcius: ");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input :f32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        
    }

}

fn fahrenheit() {


}
