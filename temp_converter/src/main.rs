use std::io;

fn main() {
	println!("This program will convert between Fahrenheit and Celsius");

	loop {
		println!("Would you like to convert from Celsius(1), from Fahrenheit(2) or exit(3)");

		let mut input = String::new();

		io::stdin()
			.read_line(&mut input)
			.expect("failed to read line");

		let input: i32 = match input.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};
		
		if input == 1 {
			celcius();
		} else if input == 2 {
			fahrenheit();
		} else if input == 3 {
			println!("Program closing");
			break;
		}
	}
}


fn celcius() {
	loop {
		println!("Input your degrees in celcius");

		let mut input = String::new();

		io::stdin()
			.read_line(&mut input)
			.expect("Failed to read line");

		let input :f32 = match input.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};

		let res = (input * 9.0/5.0) + 32.0;

		println!("{input}C is {res}F");

		break;
	}

}

fn fahrenheit() {
	loop {
		println!("Input your degrees in fahrenheit");

		let mut input = String::new();

		io::stdin()
			.read_line(&mut input)
			.expect("Failed to read line");

		let input :f32 = match input.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};

		let res = (input - 32.0) * (5.0 / 9.0);
		
		println!("{input}F is {res}C");

		break;
	}


}
