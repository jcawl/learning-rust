//the file is the module so no need to specify it
pub const HOUSE_NUMBER : i32 = 11;

pub mod bedroom1 {
    pub fn is_light_on() -> bool {
        false
    }

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