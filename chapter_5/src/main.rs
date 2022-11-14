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
    user1.email = String::from("macbobbychibuzor@gmail.com");
    
}
