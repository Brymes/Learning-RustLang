#![allow(unused_variables)]
fn main() {
    // VARIABLES
    // mutable:
    let mut mutable_num: i64 = 100;
    // immutable:
    let my_immutable_num: i64 = 50;


    // string (u8)
    // has 2 types: String and &str (string slice)
    let name:&str = "MacBobby";
    // String is mutable, &str is not mutable. String is stored on heap, &str is stored on heap, stack,or embedded
    let mut slogan = String::new();
    slogan.push_str("Just do it");

    // converting:
    let new_name_1 = name.to_string();
    let new_name_2 = "MacBobby".to_string();
    let new_name_3 = String::from("MacBobby");
    // Concatenating strings
    // method 1: the concatenation macro
    let duck = "Duck";
    let airlines = "Airlines";
    let airline_name = [duck, " ", airlines].concat();
    println!("{}", airline_name);
    // method 2: the format macro
    let airline_name_2 = format!("{} {}", duck, airlines);
    println!("{}", airline_name_2);

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

    // DATA TYPES: scalar and compound data types
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

    // Functions
    // called functions and used macros are expressions
    // functions return with -> and the type of the value parameter

    // Control flow
    // if Expressions and loops
    // if expressions
    let threshold = 3;
    if threshold < 5 {
        println!("the threshold is below five");
    } else { println!("the threshold is over five"); }
    if threshold != 4 {
        println!("threshold isn't 4");
    }

    let dividend = 6;
    if dividend % 4 == 0 {
        println!("the number is divisible by 4");
    } else if dividend % 3 == 0 {
        println!("the number is divisible by 3");
    } else if dividend %2 == 0 {
        println!("the number is divisible by 2");
    } else {
        println!("the number is not divisible by 2, 3, or 4.");
    }
    // Loops: loop, while, for
    // `loop` repeats infinitely unless when stopped `break`
    loop {
        println!("again!");
    }
    // continue skips over the rest iteration in a loop to the next iteration
    let mut counter = 0; // always create and initialize counter to zero
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2
        }
    };
    println!("the result is {result}");
    // nested loops
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {

            }
        }
    }
}
