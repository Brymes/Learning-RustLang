fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    {
        let ref1 = &mut s;
    }
    let ref2 = &mut s;
}

fn change(some_string: &mut String) {
    some_string.push_str(", world.");
    println!("{}", some_string);
}
// learned mutable references.