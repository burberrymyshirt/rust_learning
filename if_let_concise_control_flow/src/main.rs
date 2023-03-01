fn main() {

	// These two blocks of code do the exact same thing, where the only difference is, that the if let statement is more concise and short.
	// It lets you omit the "_" in the match statement, which is less boilerplate
	let config_max = Some(3u8);

	//match config_max {
		//Some(max) => println!("The maximum is configured to be {}", max),
		//_ => (), 
	//}

	if let Some(max) = config_max {
		println!("The maximum is configured to be {}", max);
	}
	// you can include an else statement after the if let, corresponding to
	// "_" in the match statement.

}