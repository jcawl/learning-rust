use std::fs::File;

fn main() {
    //similar to unwrap, will return the value inside if ok
    //if there is an error you can set the error message
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}
