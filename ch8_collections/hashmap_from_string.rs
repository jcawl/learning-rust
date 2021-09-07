use std::collections::HashMap;

fn main() {

    //create string and hashmap
    let text = "hello world wonderful world".to_string();
    let mut word_count = HashMap::new();

    //sperate string by whitespaces and iterate over them
    for word in text.split_whitespace() {
        //when encountering a new word or_insert will set the value to 0
        //it returns a mutable reference to that value
        let count = word_count.entry(word).or_insert(0);
        //we dereference it so we can assign a new value
        *count += 1;
    }

    //print the entire hashmap
    println!("{:?}", word_count);

}