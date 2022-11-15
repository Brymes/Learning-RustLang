#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Rectangle is a method implementation of the Rectangle struct
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let first_rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    let solution = first_rectangle.area();

    println!("The area of the rectangle is {} square pixels.", solution);

    let second_rectangle = Rectangle {
        width: 10,
        height: 40,
    };
    let third_rectangle = Rectangle {
        width: 50,
        height: 45,
    };

    println!("can first rectangle hold second? ", first_rectangle.can_hold(&second_rectangle));
    println!("can first rectangle hold third?", first_rectangle.can_hold(&third_rectangle));
}