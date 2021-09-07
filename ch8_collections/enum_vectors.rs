enum Storage {
    Int(u32),
    Float(f32),
    Text(String)
}

fn main() {

    //vectors can only store one type of values
    //to get by this we create a vector of enums
    let enum_vec = vec![
        Storage::Int(11),
        Storage::Float(22.2),
        Storage::Text(String::from("thirty three"))
    ];

    for i in &enum_vec {
        match i {
            Storage::Int(i) => {
                println!("Integer Value: {}", i);
            },
            Storage::Float(i) => {
                println!("Float Value: {}", i);
            },
            Storage::Text(i) => {
                println!("Text Value: {}", i);
            }
        }
    }
}