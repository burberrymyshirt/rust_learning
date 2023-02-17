fn main() {
	let ohio_quarter = Coin::Quarter(UsState::Ohio);
	value_in_cents(ohio_quarter);
}

enum Coin {
    Penny, 
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama, 
    Alaska,
    California, 
    Maine, 
    Arizona,
    Florida, 
    Ohio,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            return 1;
        }, 
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
			println!("State quarter from {:?}!", state);
			return 25;
		},
    }
}



