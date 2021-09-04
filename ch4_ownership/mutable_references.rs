fn main() {

    //bring mutable string into scope
    let mut s = String::from("hello");
    println!("{}", s);
    //pass string ref to function to change
    change(&mut s);
    //print string after the reference was changed
    println!("{}",s);
}

fn change(some_string: &mut String) {
    //dont need to return anything
    //as the value passed in was a reference
    //ownership wasnt passed so doesnt need to be given back
    some_string.push_str(", world");
}
