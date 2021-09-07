//Convert strings to pig latin. The first consonant of each word is moved
//to the end of the word and “ay” is added, so “first” becomes “irst-fay.”
//Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”).
//Keep in mind the details about UTF-8 encoding!

fn main() {

    let str = "We're getting into more complex programs in which operations can fail, so, it’s a perfect time to discuss error handling. We’ll do that next!".to_string();
    let mut new_str = String::new();

    for word in str.split_whitespace() {
        let first = &word[0..1];
        if !is_vowel(first){
            let ending = format!("-{}ay", first);
            let new_first = format!("{}{} ",&word[1..], &ending);
            new_str.push_str(&new_first);
        } else {
            let ending = "-hay".to_string();
            let new_first = format!("{}{} ", &word, ending);
            new_str.push_str(&new_first);
        }
    }

    println!("\nOriginal string: {}\n", str);
    println!("New string: {}", new_str);
}

fn is_vowel(c: &str) -> bool {
    match c {
        "a" => {
            true
        },
        "e" => {
            true
        },
        "i" => {
            true
        },
        "o" => {
            true
        },
        "u" => {
            true
        },
        _ => {
            false
        }
    }
}