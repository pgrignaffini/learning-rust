struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    //In this example, we can no longer use user1 as a whole after creating user2 because the String in the username field of user1 was moved into user2.

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!({ user2 })
}
