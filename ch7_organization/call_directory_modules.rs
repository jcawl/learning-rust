//specify the module
mod houses;

//bring in bedrooms
use houses::{bedroom1, bedroom2};

fn main() {
    println!("bedroom1 light is on: {}", bedroom1::is_light_on());
    println!("bedroom2 light is on: {}", bedroom2::is_light_on());
    println!("house number is: {}", houses::HOUSE_NUMBER);
    println!("other bedroom light is on: {}", bedroom1::is_bedroom2_light_on());
}