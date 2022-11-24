fn main() {
//    let mut x = 5; 
//    println!("The value of x is: {x}");
//    x = 6;
//    println!("The value of x is: {x}");


    let _x = 5;

    let x = 5 + 1;

    {
        let x = x * 2;
        println!("The x value of the inner scope is: {x}");
    }

    println!("The x value is: {x}");
}
