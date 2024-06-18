#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.width
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };  

    let rect2 = Rectangle {
        width: 20,
        height: 20,
    };

    let rect3 = Rectangle {
        width: 50,
        height: 30,
    };
    
    println!(
        "The area of the rectangle is {} square pxiels",
        rect1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

}