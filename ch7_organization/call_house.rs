//specify the module
mod house;

//bring in bedrooms
use house::{bedroom1, bedroom2};

fn main() {
    println!("bedroom1 light is on: {}", bedroom1::is_light_on());
    println!("bedroom2 light is on: {}", bedroom2::is_light_on());
    println!("house number is: {}", house::HOUSE_NUMBER);
    println!("other bedroom light is on: {}", bedroom1::is_bedroom2_light_on());
}