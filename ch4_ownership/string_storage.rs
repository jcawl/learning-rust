fn main() {

    //storage allocation must be dynamic in order to
    //have a mutable string
    let mut s = String::from("hello");

    //string is stored in the heap and can allocate more memory
    s.push_str(", world!");
    println!("{}", s);

    //this string is stored on the stack and
    //the stack allocates memory at compile time and isnt mutable
    let s2 = "hello";
    let s3 = "world";
    println!("{}, {}!", s2, s3);

}