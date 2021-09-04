fn main() {

    //bring string into scope
    let s = String::from("new string");
    //call get_len but pass s by reference
    let l = get_len(&s);
    //s is still available because we only passed a reference
    println!("the length of {} is {}",s,l)

}

//make sure the argument type is expecting a ref
fn get_len(s2 : &String) -> usize {
    s2.len()
}