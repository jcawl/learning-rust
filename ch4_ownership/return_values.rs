fn main() {

    //ownership transfered
    let new_owner : String = init_string();
    println!("{}", new_owner);

    //init string and send
    let s3 = String::from("created in main");
    //s3 no longer valid after being passed to function
    let s4 = recieve_and_send(s3);
    println!("s4 = {}", s4);

}


fn init_string() -> String {

    let s = String::from("created by init_string");
    //return s to caller
    s
}

fn recieve_and_send(s2 : String) -> String {
    //recieve s2 and return
    s2
}