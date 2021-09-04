//code for bedroom1 module
pub fn is_light_on() -> bool {
    false
}

pub fn is_bedroom2_light_on() -> bool {
    use super::bedroom2;
    bedroom2::is_light_on()
}
