struct User {
    id: u8,
    username: String,
    email: String,
    active: bool,
}

fn main() {
    let p1 = User {
        id: 1,
        username: "John".to_string(),
        email: "john@mail.com".to_string(),
        active: true,
    };
    println!("name = {}", p1.id);
}
