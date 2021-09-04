//define the struct
struct User {
    name: String,
    age: u8,
    active: bool,
    count: u32
}

fn main() {
    //create instance of the struct
    let user1 = User {
        name: String::from("jake"),
        age: 23,
        active: true,
        count: 1
    };
    println!("name of first user: {}", user1.name);

    //mutable instance of the struct
    let mut user2 = User {
        name: String::from("jake"),
        age: 24,
        active: true,
        count: 1
    };
    user2.name = String::from("cawley");
    println!("name of second user: {}", user2.name);

    //creating a user by calling function that returns struct
    let user3 = create_user();
    println!("name of the third user: {}", user3.name);

}

fn create_user() -> User {
    User {
        name: String::from("joe"),
        age: 50,
        active: true,
        count: 1
    }
}