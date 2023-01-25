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

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {} high and {} wide", rect1.height, rect1.width);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }
}