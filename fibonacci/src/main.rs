use std::io;

fn main() {
	loop {
		println!("which fibo number do you wanna calculate? O.O");

		let mut input = String::new();

		io::stdin()
			.read_line(&mut input)
			.expect("de her");

		let input: u128 = match input.trim().parse() {
			Ok(num) => num, 
			Err(_) => continue,
		};

		let fib_num = fib(&input);

		println!("{fib_num}");
	}
}

fn fib(n: &u128) -> u128 {

	let mut n1 = 1;
	let mut n2 = 1; 
	let mut n3 = 0;
	
	if *n < 3 {
		n3 = 1;
	} else {
		for _ in 3..*n + 1 {
			n3 = n1 + n2;
			n1 = n2;
			n2 = n3;
		}
	}

	return n3;
}


