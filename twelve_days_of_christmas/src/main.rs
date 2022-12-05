fn main() {
	
	let a = ["A partridge in a pear tree", "Two turtle doves, and", 
		"Three french hens", "Four calling birds", "Five golden rings", 
		"Six geese a-laying", "Seven swans a-swimming", "Eight maids a-milking", 
		"Nine ladies dancing", "Ten lords a-leaping", "Eleven pipers piping", 
		"Twelve drummers drumming"] ;

	let b = ["first", "second", "third", "fourth", "fifth", "sixth", 
		"seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];


	let mut i = 0;

		for number in b {
			println!("On the {number} day of christmas,");
			println!("my true love gave to me");

			for n in (0..i + 1).rev() {
				
				let temp = a[n];

				println!("{temp}");

			}
			i += 1;
			
			println!();
		}

}
