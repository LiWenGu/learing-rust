fn main() {
    let u = build_user(String::from("sad"));
    println!("{}", u.email);
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String) -> User {
    User {
        email,
        username: String::from("asdasd"),
        active: true,
        sign_in_count: 1
    }
}