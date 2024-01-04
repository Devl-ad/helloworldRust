struct User {
    id: u8,
    username: String,
    email: String,
    active: bool,
}

impl User {
    fn new(id: u8, username: String, email: String) -> User {
        User {
            id,
            username,
            email,
            active: true,
        }
    }

    fn deactivat(&mut self) {
        self.active = false;
    }
}

fn main() {
    let mut p1 = User::new(1, "sammy".to_string(), "sam@mail.com".to_string());
    println!("Account {} status is {}", p1.username, p1.active);
    p1.deactivat();
    println!("Account {} status is {}", p1.username, p1.active);
}
