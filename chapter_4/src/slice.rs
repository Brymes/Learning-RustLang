// slice is a kind of reference without ownership.
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() { // i for the index in the tuple, and &item for the single byte in the tuple
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn main() {
    let mut s = String::from("Hello World.");

    let slice = &s[..2];
    
    let len = s.len();
    let slice = &s[3..len];

    let word = first_word(&s);

    s.clear(); // empty the string, to ""
}