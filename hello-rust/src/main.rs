fn data_types() {
    // variables:
    // mutable:
    let mut mutable_num: i64 = 100;
    // immutable:
    let my_immutable_num: i64 = 50;
    // string
    let name = "MacBobby";
    // constants
    const THREE_HOURS_IN_SECONDS: u64 = 60 * 60 * 3;
    // Shadowing
    let a = 5;
    let a = a + 1;
    {
        let a = a * 2;
        println!("The value of a in the inner scope is: {a}");
    }
    println!{"The value of the outer a is {a}"};
    // Data Types: scalar and compound data types
    // scalar represent single value: integers, floating point numbers, booleans,characters.
    // float
    let fpn = 2.0; // f64
    // character    uss single quotes, and string literals use double quotes
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';


    // compound types are multiple values in a group: tuples and arrays
    //tuple
    let tuppy: (i32, f64, u8) = (500, 12.2, 1);
    // get individual values
    let (m, n, o) = tuppy;
    printtln("The value of ");


    println!("Hello!, my name is {}. My mutable is {}, and my immutable is {}", name, mutable_num, my_immutable_num);
    // another way to use formatting
    println!("Goodbye, now. It's {name}. And the time taken is {}", THREE_HOURS_IN_SECONDS);
}

fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("Hello!, my name is {}. My mutable is {}, and my immutable is {}", name, mutable_num, my_immutable_num);
    // another way to use formatting
    println!("Goodbye, now. It's {name}. And the time taken is {}", THREE_HOURS_IN_SECONDS);
}
