fn main() {
	let m = Message::Write(String::from("Hellooo"));
	m.call(); 
}

enum Message {
	_Quit, 
	_Move { x:i32, y: i32},
	Write(String),
	_ChangeColor( i32, i32, i32),
}

impl Message {
	fn call (&self) {
		// empty function for demonstration purposes
	}
}
