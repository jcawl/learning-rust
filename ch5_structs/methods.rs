#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32
}

//define a method using "impl" (implementation) block
impl Rectangle {
    //rarely will a struct pass ownership to a method
    //we pass a reference to the struct because we only read values
    //we can yse &mut self if we want to mutate the struct
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect2: &Rectangle) -> bool {
        if &self.area() > &rect2.area() {
            true
        } else {
            false
        }
    }
}

fn main() {

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    //methods are prefered over functions simply for organization
    println!("get area from method: {}", rect1.area());

    //pass referenct of another instance of rect to a method
    println!("can rect1 hold rect2: {}", rect1.can_hold(&rect2));
    println!("can rect1 hold rect3: {}", rect1.can_hold(&rect3));
}