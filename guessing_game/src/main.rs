use std::io; // standard libary :: input/output extension
use rand::Rng; // external ranc crate :: Rng extension
use std::cmp::Ordering; // standard libary :: comparison extension :: comparison between two values extension

fn main() { // main function used to run the program
	println!("Guess the number!"); // print to console

	let secret_number = rand::thread_rng().gen_range(1..=100); // generates a random number within the range 1-100, with the "rand" crate, which is defined in the dependencies of "cargo.toml", and assigns it to an unmutateable unsigned 32-bit integer with the name "secret_number"

//    println!("The secret number is: {secret_number}"); // for testing the code

	loop { // an infinite loop is defined
		println!("Please input your guess"); // print to console

		let mut guess  = String::new(); // creates a mutateable variable with the name "guess", and assigns a new empty String to it.

		io::stdin() // uses the std::io libary to take inputs from the console
			.read_line(&mut guess)  // read_line is the function that actually reads the input, and assigns it to the "guess" variable created earlier
			.expect("Failed to read line?"); // in case a catastrophical error occours, this will crash the program. The only reason this is handled with a .expect call, is because the only reason this will fail, is if the OS fails, and in that case it makes sense to crash the program

		let guess: u32 = match guess.trim().parse() { // here another unmutatable variable is created, with the name "guess", just like the earlier String variant. This shadows the old guess with a new one of the type unsigned 32-bit integer, which is the correct type for the rng.
													  // match is another error handeling method, where instead of crashing the program, it runs the bit of code we till it.
			Ok(num) => num, // the method parse returns an Result, which is either an "Ok" or "Err" enum. If the input is parsable, the output is returned as a numer in the Ok enum.
			Err(_) => continue, // in case the input is not parsable, fx. a letter instead of a number, it will return a value in the Err enum, where the "_" means that it runs the in Err, regardless of what is actually returned through the Err enum.
		};

		println!("You guessed: {guess}"); // print to console, where {guess} is the variable "guess" in form of a printable string?

		match guess.cmp(&secret_number) {  // another match, to handle the outcome of the comparison. The function "cmp" is executed on the guess variable, which is what is used to actually compare the two values, in this case "secret_number", and it returns an "Ordering", which will be handled below
			Ordering::Less => println!("Too small!"),  // in case the input, which comes from "guess" is lower than the secret number, it will be caught in the Ordering::Less, where it will print to console, that the input value was smaller than the secret randomly generated value
			Ordering::Greater => println!("Too big!"), // this is the same, but in case the input is larger than the random number
			Ordering::Equal => { // this is in case the input number is the same as the random number, and will print out a win message, where it will break the loop afterwards
				println!("You win!");
				break; // ez break gg B-)
			}
		}
	}
}
