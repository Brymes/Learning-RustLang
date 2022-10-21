#![allow(unused_variables)]

fn main() {
    let duck = "Duck";
    let airlines = "Airlines";

    let airline_name = [duck, " ", airlines].concat();
    println!("{}", airline_name);

    let airline_name_2 = format!("{} {}", duck, airlines);
    println!("{}", airline_name_2);
}
