fn main() {
    let number = 8;

    if number % 4 == 0 {
        println!("this number is divisable by 4");
    } else if number % 3 == 0 {
        println!("this number is divisable by 3");
    } else if number % 2 == 0 {
        println!("this number is divisable by 2");
    } else {
        println!("this number is neither divisable by 4, 3 or 2")
    }
}
//