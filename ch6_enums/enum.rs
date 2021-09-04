//define an enum
enum IpAddrKind {
    V4,
    V6
}

fn main() {
    //create an instance of both variants
    //the variants of the enum are namespaced under its identifier
    //we know this because of the double semi colon
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    //pass enum instance to function and see what type of connectio
    route(four);
    route(six);
}

//both instances are of the same type so functions
//can accept both types
fn route(ip_kind: IpAddrKind) {
    match ip_kind {
        IpAddrKind::V6 => {
            println!("This connection is V6");
        }
        IpAddrKind::V4 => {
            println!("This connection is V4");
        }
    }
}