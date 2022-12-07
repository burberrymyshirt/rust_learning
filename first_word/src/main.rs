fn main() {
	let s = String::from("de her er dem her");

	let s1 = first_word(&s);

	println!("{}", s1);


}

	//Old version without slices, which returns only the index where the first space is
// fn first_word(s: &str) -> usize {
// 	let mut res = s.len();

// 	let bytes = s.as_bytes();

// 	for (i, &item) in bytes.iter().enumerate() {
// 		if item == b' ' {
// 			res = i;
// 			break;
// 		}

// 	}

// 	return res;
// }

	//New version which uses string slices and returns a slice, instead of just an index
fn first_word(s: &str) -> &str {
	let mut res = &s[..];

	let bytes = s.as_bytes();

	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			res = &s[..i];
			break;
		}
	}

	return res;
}