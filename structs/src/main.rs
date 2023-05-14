fn main() {
    let user1 = User {
        email: String::from("test@email.com"),
        username: String::from("sunrizz"),
        active: true,
        sign_in_count: 1,
    };
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);