//Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company.
//For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a
//department or all people in the company by department, sorted alphabetically.

use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    //create a hashmap of a department and a vector containing employees in that department
    let mut my_map: HashMap<String, Vec<String>> = HashMap::new();

    //intro
    println!("Staff Manager\n-------");

    loop {
        //get input
        let (name, field) = get_input();
        let field_copy = field.clone();
        //adds an empty vector to fields not seen before, does nothing if field has vales
        let names = my_map.entry(field).or_insert(Vec::new());
        //when a field has vales this is hit and pushes the new name to the vector
        names.push(name);

        //print out current staff
        println!("\n{} Staff -", field_copy);
        io::stdout().flush().expect("Unable to flush stdout");
        for i in names {
            print!("{}, ", i);
        }
        println!("\n");


    }


}

fn get_input() -> (String, String) {

    //prompt for name
    let mut name = String::new();
    print!("Enter name: ");
    let _ = io::stdout().flush();

    //get name
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    //prompt field
    let mut field = String::new();
    print!("Enter field: ");
    let _ = io::stdout().flush();

    //get field
    io::stdin()
        .read_line(&mut field)
        .expect("Failed to read line");


    //remove new lines
    name.pop();
    field.pop();

    (name, field)


}