fn main() {
    let sum: i32 = printer_fn(11, 22);
    println!("the sum is {sum}");
}

fn printer_fn(x: i32, y: i32)  -> i32 {
    println!("the value of x is {x}");
    println!("the value of y is {y}");
    x + y
}