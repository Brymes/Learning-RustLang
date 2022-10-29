fn main() {
    let sum: i32 = printer_fn(11, 22);
    println!("the sum is {sum}");

    // Control Flow
    /* Rust has three different types of loops: loop, while, and for
    */
    loop {
        println!("again");
        break;
    }
    
    let mut counter: i32 = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter;
        }
    };
    println!("the result is {result}");

    let mut a_number: i32 = 3;

    while a_number != 0 {
        println!("{}!", a_number);

        a_number -= 1;
    }
    println!("happy birthday!!!");
}

// learning functions
fn printer_fn(x: i32, y: i32)  -> i32 {
    println!("the value of x is {x}");
    println!("the value of y is {y}");
    x + y
}