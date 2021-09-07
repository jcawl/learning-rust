use std::io::{self, Write};

fn main() {
    //create an empty string
    let _ = String::new();

    //create string with initial data
    let str = "String created with .to_string()".to_string();
    println!("{}", str);

    //this is the same as above
    let str = String::from("String created with String::from()");
    println!("{}", str);

    //strings can grow and shrink similar to vectors
    let mut str = "foo".to_string();
    str.push_str("bar");
    println!("String initialized with `foo` and is now {}", str);

    //adding strings together
    let mut str = "foo".to_string();
    let str2 = "bar";
    str.push_str(str2);
    println!("String str was initialized with `foo` and we appended str2 ({}) to get {}", str2, str);

    //another way to add strings
    let str = "Hello, ".to_string();
    let str2 = "World!".to_string();
    //cannot add two reference strings
    //String concatenation appends the string on the right to the string on the left and may require reallocation
    //This requires ownership of the string on the left
    let str3 = str + &str2;
    println!("Adding two string using `+` to get: {}", str3);

    //add a single letter to the string
    let mut str = "lol".to_string();
    println!("String str = {}", str);
    str.push("!");
    println!("We used str.push(`!`) to add some excitement: {}", str);

    //making complicated concatenations reasable with format macro
    let str = "tic".to_string();
    let str2 = "tac".to_string();
    let str3 = "toe".to_string();
    let str4 = " - ".to_string();
    let str5 = format!("{}{}{}{}{}", str, str4, str2, str4, str3);
    println!("Used `format!` to create: {}", str5);

    //slicing strings
    let str = "123456789".to_string();
    let first_five = &str[0..5];
    println!("Sliced 123456789 to {}", first_five);

    //iterate over the string
    println!("Looping over the string");
    io::stdout().flush().expect("Unable to flush stdout");
    for c in str.chars() {
        print!("{} /", c);
    }
    println!("");



}