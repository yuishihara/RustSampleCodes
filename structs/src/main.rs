struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user<I: ToString>(email: I, username: I) -> User {
    User {
        email: email.to_string(),
        username: username.to_string(),
        sign_in_count: 1,
        active: true,
    }
}

fn print_user(user: &User) {
    println!("username: {}", user.username)
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        sign_in_count: 1,
        active: true,
    };
    print_user(&user1);

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        sign_in_count: 2,
        active: false,
    };
    user1.username = String::from("ababa");
    print_user(&user1);

    let user1 = build_user("someone@example.com", "abcdef");
    print_user(&user1);
}
