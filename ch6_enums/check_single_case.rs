enum IpAddr {
    V4(u8,u8,u8,u8),
    V6(String),
    V8{x:u32, y:u32}
}

fn main() {
    let address1 = IpAddr::V4(127, 0, 0, 1);
    let address2 = IpAddr::V6(String::from("::1"));
    let address3 = IpAddr::V8{x: 16, y: 19};

    //this will cover one single case
    if let IpAddr::V4(127, 0, 0, 1) = address1 {
        println!("localhost");
    }
}