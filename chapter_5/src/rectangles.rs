// rectangles take the width and height of a rectangle in pixels and calculates the area of the rectangle
// area of rectangle = width x height

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn main() {
    let width1 = 30;
    let height1 = 50;
    let solution = area(width1, height1);
    println!("The area of the rectangle is {} square pixels.", solution);

    let ref_rect = (40, 60);
    let ref_solution = ref_area(ref_rect);
    println!("The refactored area of the rectangle is {} square pixels", ref_solution);

    let new_rect = Rectangle {
        width: 20,
        height: 40,
    };

    let new_solution = new_area(new_rect);
    println!("The new area of the rectangle is {} square pixels", new_solution);
}

// Refactoring with Tuples
fn ref_area(dimensions: (u32, u32)) ->  u32 {
    dimensions.0 * dimensions.1
}

// Refactoring with Structs

struct Rectangle {
    width: u32,
    height: u32,
}

fn new_area(rectangle: Rectangle) -> u32 {
    rectangle.width * rectangle.height
}