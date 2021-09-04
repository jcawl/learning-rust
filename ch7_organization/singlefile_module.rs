//the module is in the file that it is being used for
//so it never loses scope and we dont have to make it public
mod house {
    pub const HOUSE_NUMBER : i32 = 11;

    //both the module and function need to be public to be used in main
    pub mod bedroom1 {
        pub fn is_light_on() -> bool {
            false
        }

        //"use super::" to go up a level and check another mods status
        pub fn is_bedroom2_light_on() -> bool {
            use super::bedroom2;
            bedroom2::is_light_on()
        }
    }

    pub mod bedroom2 {
        pub fn is_light_on() -> bool {
            true
        }
    }
}

//specify what mod we are using to shoter implementation
//use as to use alias
use house::{bedroom1 as masterbedroom, bedroom2};

fn main() {
    //reference mod with "::"
    //no longer need to specify house because we specified it
    println!("bedroom1 light is on: {}", masterbedroom::is_light_on());
    println!("bedroom2 light is on: {}", bedroom2::is_light_on());
    println!("house number is: {}", house::HOUSE_NUMBER);
    println!("other bedroom light is on: {}", masterbedroom::is_bedroom2_light_on());
}
