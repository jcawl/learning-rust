struct User {
    name: String,
    age: u8,
    active: bool,
    count: u32
}

fn main(){

    let user1 = User {
        name: String::from("jake"),
        age: 23,
        active: true,
        count: 88
    };
    //create another instance of user with similar information
    let user2 = User {
        name: String::from("jack"),
        ..user1
    };

    //create another instance with one field that depends on another user
    let user3 = User {
        name: String::from("joe"),
        age: 25,
        active: false,
        count: user1.count
    };

    println!("user 1: {}", user1.name);
    println!("user 2: {}", user2.age);
    println!("user 3: {}", user3.count);
}