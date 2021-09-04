enum Option<T> {
    Some(T),
    None
}

fn main() {
    let some: Option<u32> = Option::Some(5);
    let none: Option<u32> = Option::None;

    check_val(&some);
    check_val(&none);
}

fn check_val(x: &Option<u32>) {
    match x {
        //check forv values
        //the none case has to be matched or there will be an error
        Option::Some(val) => {
            println!("something was hit, value: {}", val);
        }
        Option::None => {
            println!("nothing was hit");
        }
    }
}