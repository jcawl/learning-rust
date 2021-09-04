fn main(){

    let s = String::from("hello world");

    //a reference to a specific part of a string is calles a slice
    //specify a range within the bracket to slive the string
    //the slice data structure stores the starting position and the length of the slice
    let hello = &s[0..5];
    let helloworld = &s[..];
    let hel = &s[..3];
    let oworld = &s[4..];

    println!("{}", hello);
    println!("{}", helloworld);
    println!("{}", hel);
    println!("{}", oworld);

    let fw = first_word(&s);
    println!("first word: {}", fw);

    let sw = second_word(&s);
    println!("second word: {}", sw);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i..];
        }
    }

    &s[..]
}