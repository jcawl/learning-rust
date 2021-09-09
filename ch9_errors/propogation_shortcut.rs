#![allow(unused)]
use std::fs::File;
use std::io;
use std::io::Read;


fn main() {
    //call function that may return an error
    let res = read_username_from_file();

    //no we have the result we can perform logic
    //on how we handle an ok result, and an error result
    match res {
        Ok(x) => {
            println!("Sucess: {:#?}",x);
        },
        Err(e) => {
            println!("Fail: {:#?}",e);
        }
    }
}

fn read_username_from_file() -> Result<String, io::Error> {

    let mut s = String::new();

    //the '?' works similarr to match if ok will return vale inside
    //if theres an error '?' is a bit more versatile by using the 'from' function
    //this function is used to convert error types to other types
    //'?' converts any error type into the one being returned by the function it is in
    //useful for when a function can fail in multiple different ways
    File::open("hellto.txt")?.read_to_string(&mut s)?;

    //if error this will not be hit
    //if ok return value stored in s
    Ok(s)
}
