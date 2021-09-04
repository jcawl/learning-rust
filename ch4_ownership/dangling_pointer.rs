// !!code causes error!!

fn main() {
    let ref_to_nothing = return_something();
}

fn return_nothing() -> &String {
    //bring string into scope
    let s = String::from("hello");

    //return reference to string
    &s
    //`drop` is called on the memory storing s
    //as it goes out of scope
}

fn return_something() -> String {
    //bring string into scope
    let s = String::from("hello");

    //return string so ownership will be passed
    //and `drop` will not be called on the memory
    s
}