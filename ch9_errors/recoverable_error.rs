use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    //File::open function is a Result<T, E>, so we match the result of that function
    let f = match f {
        Ok(file) => { println!("hello.txt is open"); file },
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    println!("{:#?}", f)
}
