fn main() {
    //creating empty vector
    let mut ev = Vec::new();
    //creatig a vector with values
    let mut v = vec![1,2,3];

    ev.push(5);
    v.push(4);

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    //iterate over and change the value of the vector
    for i in &mut v {
        *i += 50;
    }

    //iterate over and print each element of the vector
    for i in &v {
        println!("{}", i);
    }


}