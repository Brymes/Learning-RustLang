fn main() {
    // variables:
    // mutable:
    let mut mutable_num: i64 = 100;
    // immutable:
    let my_immutable_num: i64 = 50;
    // string
    let name = "MacBobby";
    // constants
    const THREE_HOURS_IN_SECONDS: u64 = 60 * 60 * 3;




    println!("Hello!, my name is {}. My mutable is {}, and my immutable is {}", name, mutable_num, my_immutable_num);
    // another way to use formatting
    println!("Goodbye, now. It's {name}. And the time taken is {}", THREE_HOURS_IN_SECONDS);
}
