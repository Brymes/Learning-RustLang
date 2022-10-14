fn main() {
    // variables:
    // mutable:
    let mut mutable_num: i64 = 100;
    // immutable:
    let my_immutable_num: i64 = 50;
    // string
    let name:&str = "MacBobby";
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

    // DATA Types: scalar and compound data types
    // scalar represent single value: integers, floating point numbers, booleans,characters.
    // float
    let fpn = 2.0; // f64
    // character    uss single quotes, and string literals use double quotes
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';


    // COMPOUND types are multiple values in a group: tuples and arrays
    // tuple
    let tuppy: (i32, f64, u8) = (500, 12.2, 1);
    // use pattern matching get individual values
    let (m, n, o) = tuppy;
    println!("The value of n is: {n}");
    // also use the dot notation
    let five_hundred = tuppy.0;
    let six_point_four = tuppy.1;
    let one = tuppy.2;
    // array: stores data on the stack not heap
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "December"];
    // writing the type annotation
    let my_array: [i64; 5] = [6, 7, 8, 9, 10];
    // initializing
    let another_array = [3; 5]; // let array = [3,3,3,3,3]
    // accessing array elements
    let first_element = my_array[0];
    let second_element = my_array[1];

    println!("Hello!, my name is {}. My mutable is {}, and my immutable is {}", name, mutable_num, my_immutable_num);
    // another way to use formatting
    println!("Goodbye, now. It's {name}. And the time taken is {}", THREE_HOURS_IN_SECONDS);


    println!("Hello!, my name is {}. My mutable is {}, and my immutable is {}", name, mutable_num, my_immutable_num);
    // another way to use formatting
    println!("Goodbye, now. It's {name}. And the time taken is {}", THREE_HOURS_IN_SECONDS);

    // Statements and Expressions
    //statements are instructions that perform some action but don't return a value
    // expressions evaluate to a resulting value
    let  first_var = {
        let second_var = 3;
        second_var + 1 //expressions doesn't include ending semicolons
    };
    println!("The value of first_var is {first_var}");

}
