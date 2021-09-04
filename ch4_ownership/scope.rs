fn main() {

    //bring string into scopr
    let s = String::from("hello world");
    //s moves to function and is no longer valid
    take_ownership(s);

    //certain variable types are passed as copies
    //will be valid after passing them
    let x : i32 = 5;
    make_copy(x);
    //still valid to be used
    println!("{}", x);

}

fn take_ownership(s2 : String) {
    println!("{}", s2);
    //`drop` is called on s2 memory and it is lost
}

fn make_copy(x2 : i32) {
    println!("{}", x2);
}