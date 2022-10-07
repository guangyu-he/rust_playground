struct User {
    name: String,
    id: i32,
    address: String,
}

impl User {
    fn new(name: &str) -> User {
        return User {
            name: String::from(name),
            id: 0_i32,
            address: String::from(""),
        };
    }

    fn set_id(mut self, id: i32) -> User {
        // initializing value in the constructor, use mutable self
        self.id = id;
        return self;
    }

    fn update_address(&mut self, address: String) {
        // updating value in the constructor, use mutable borrow?
        self.address = address;
    }
}

fn main() -> () {
    let mut user_1: User = User::new("Guangyu").set_id(611);
    user_1.update_address(String::from("Ilsenburger Str."));
    println!("{},{},{}", user_1.name, user_1.id, user_1.address);
}
