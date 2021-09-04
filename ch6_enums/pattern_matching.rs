#[derive(Debug)]
enum State {
    Alabama,
    Alaska
}

enum Coins {
    Penny,
    Nickel,
    Dime,
    //the quarter variant will store a value inside of it
    //we can retrieve this value later on
    Quarter(State)
}

fn main() {
    let penny = Coins::Penny;
    let nickel = Coins::Nickel;
    let dime = Coins::Dime;
    let alabama_quarter = Coins::Quarter(State::Alabama);
    let alaska_quarter = Coins::Quarter(State::Alaska);

    //get infromation from each variants
    get_information(&penny);
    get_information(&nickel);
    get_information(&dime);
    get_information(&alabama_quarter);
    get_information(&alaska_quarter);

}

fn get_information(coin: &Coins) {
    match coin {
        //want to match a specific variant
        Coins::Quarter(State::Alabama) => {
            println!("alabama quarters are very rare!");
        }
        Coins::Quarter(state) => {
            //enum does not implement debug so we have to explicitly include it
            println!{"this quarter is from {:?}!", state};
        }
        Coins::Penny => {
            println!("no information stored in penny");
        }
        Coins::Nickel => {
            println!("no information stored in nickel");
        }
        Coins::Dime => {
            println!("no information stored in dime");
        }
    }
}