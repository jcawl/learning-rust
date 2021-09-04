struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    //associated functions do not use an instance of the struct
    //often used as contructors to return new instance of struct
    fn square (len: u32) -> Rectangle {
        Rectangle {
            width: len,
            height: len
        }
    }
}

//allowed to have multiple impl blocks
//valid syntax but no reason for it
impl Rectangle {
    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

fn main() {
    //init instance of struct
    let rect = Rectangle {
        width: 10,
        height: 15
    };

    //create square by calling associated method
    let square = Rectangle::square(5);

    //use second impl block to show it is valid
    println!("is rect a square: {}", rect.is_square());
    println!("is square a square: {}", square.is_square());

}