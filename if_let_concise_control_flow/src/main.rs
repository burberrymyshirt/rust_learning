fn main() {
    //println!("Hello, world!");
	
	let config_max = Some(3u8);

	//match config_max {
		//Some(max) => println!("The maximum is configured to be {}", max),
		//_ => (), 
	//}

	if let Some(max) = config_max {
		println!("The maximum is configured to be {}", max);
	}
	// you can include an else statement after the if let, corrosponding to
	// "_" in the match statement.

}