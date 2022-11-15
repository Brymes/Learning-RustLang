// empty here

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // to use a struct, create an instance of it
    let user1 = User {
        active: true,
        username: String::from("MacBobby Chibuzor"),
        email: String::from("theghostmac@gmaiil.com"),
        sign_in_count: 1,
    };
    // access a single field in the struct:
    user1.email = String::from("macbobbychibuzor@gmail.com");

    // struct update
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("peekube01@gmail.com"),
        sign_in_count: user1.sign_in_count,
    };
    let user3 = User {
        email: String::from("alphamax.rdj@gmail.com"),
        ..user1
    };

}

// Tuple Structs
    // Used to create nameless fields of different types
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn particle() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

// Unit-Like Structs with no fields
// useful in implementing a trait on some type but without a data in the type
struct AlwaysEqual;

fn status() {
    let subject = AlwaysEqual;
}