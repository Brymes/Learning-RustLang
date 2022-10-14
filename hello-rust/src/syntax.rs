fn syntax() {
    // variable declaration
    let mut a: i16 = 10;
    let b = "Good morning";
    // data structures elementary
    let my_array = [10, 20, 30];
    let my_tuple = (500, 6.4, 1, "fifteen");
    // pattern matching
    let first_array_element = my_array[0];
    let second_array_element = my_array[1];
    let first_tuple_element = my_tuple.0;

    println!("Hello, {}, you should be the {}th person here. Here is my array: {}\
    and here is my tuple: {}", b, a, my_array, my_tuple);
}

fn main() {
    syntax();
}