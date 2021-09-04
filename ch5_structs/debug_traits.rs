//manually opt into the debug functionality
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    let rect = Rectangle {
        width: 10,
        height: 15
    };

    //adding the specifier ":?" inside the curly brackers
    //tells the println! macro we want to use the debug output
    //easy way to print traits of struct when debugging
    println!("debug output for rect: {:?}", rect);

    //for larger structs format the output with ":#?"
    println!("debug pretty output for rect: {:#?}", rect);
}