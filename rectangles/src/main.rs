#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// 関連関数
// ex.) Rectangle::square(3)
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size}
    }
}

fn main() {
    // タプル
    // let rect1 = (30, 50);

    // 構造体
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("rect1 is {:#?}", rect1);

    println!(
        "the area of rectangle is {} square pixels.",
        rect1.area()
    );
}

// fn area(dimensions: (u32, u32)) -> u32 {
    // dimensions.0 * dimensions.1
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }
