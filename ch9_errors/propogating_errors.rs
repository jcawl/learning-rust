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
    let f = File::open("hellor.txt");

    //if file opens succesfully save contents, else return error and break
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    //will only be hit if file is opened successfully
    //now we match if read_to_string is successful
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
