fn main() {
	let ohio_quarter = Coin::Quarter(UsState::Ohio);
	value_in_cents(ohio_quarter);
}

enum Coin {
    _Penny, 
    _Nickel,
    _Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    _Alabama, 
    _Alaska,
    _California, 
    _Maine, 
    _Arizona,
    _Florida, 
    Ohio,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::_Penny => {
            println!("Lucky Penny!");
            return 1;
        }, 
        Coin::_Nickel => 5,
        Coin::_Dime => 10,
        Coin::Quarter(state) => {
			println!("State quarter from {:?}!", state);
			return 25;
		},
    }
}



