fn main() {
    //init two tuple structs
    //dont have to specify name
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(55, 0, 0);
    let origin = Point(0, 0, 0);

    //use "." followed by the index to access a value
    println!("{}", black.0);
}