use std::collections::HashMap;

fn main() {

    let mut game1 = HashMap::new();

    //all keys and values must have same type
    game1.insert(String::from("Blue"), 10);
    game1.insert(String::from("Red"), 50);

    //creating hashmap out of vectors
    let teams = vec![String::from("Orange"), String::from("Purple")];
    let scores = vec![30,80];
    //use iterator to access elements of vector then zip teams and scores into vector of tuple
    //collect then takes the vector of tuples and turns it into a hashmap
    let mut game2: HashMap<_,_> = teams.into_iter().zip(scores.into_iter()).collect();

    //accessing value via keys
    let team_name = "Blue".to_string();
    let game1_team_name_score = game1.get(&team_name);
    println!("{} score: {:?} -- using game1.get()", team_name, game1_team_name_score);
    println!("Purple score: {:?} -- using game2.get()", game2.get(&"Purple".to_string()));

    //looping over key values
    println!("\nGame 1");
    for (key, value) in &game1 {
        println!("{} Team: {}", key, value);
    }

    //loop over key values
    println!("\nGame 2");
    for (key, value) in &game2 {
        println!("{} Team: {}", key, value);
    }

    //changing the value of the orange team
    println!("\nOrange scored 70 more points, rewrite by replacing value with insert()");
    game2.insert("Orange".to_string(), 100);

    //if there is an entry update it
    println!("Adding a yellow team if there is not already on by using entry(key).or_insert(val)");
    game2.entry(String::from("Yellow")).or_insert(50);
    game2.entry(String::from("Orange")).or_insert(50);

    //loop over key values
    println!("\nGame 2");
    for (key, value) in &game2 {
        println!("{} Team: {}", key, value);
    }
}