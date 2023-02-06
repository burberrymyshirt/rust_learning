// fn main() {
//     let width1 = 30;
//     let height1 = 50;
//
//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width1, height1)
//     );
// }
//
// fn area(width: u32, height: u32) -> u32 {
//     return width * height;
// }

// fn main() {
//     let rect1 = (30, 50);
//
//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     );
// }
//
// fn area(dimentions: (u32, u32)) -> u32 {
//     return dimentions.0 * dimentions.1;
// }
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }

    fn square(size: u32) -> Self {
        return Self {
            width: size, 
            height: size,
        };
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60, 
        height: 45,
    };

    let square = Rectangle::square(32);

    println!("rect1 is {} high and {} wide", rect1.height, rect1.width);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

