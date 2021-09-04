enum Coins {
    Penny,
    Nickel,
    Dime,
    Quarter
}

fn main() {
    //initializing enum instances
    let penny = Coins::Penny;
    let nickel = Coins::Nickel;
    let dime = Coins::Dime;
    let quarter = Coins::Quarter;

    //match enum instances by calling function
    //return the value of the coin after matching
    let one = match_coin(penny);
    let five = match_coin(nickel);
    let ten = match_coin(dime);
    let twenty_five = match_coin(quarter);

    println!("The value of a penny is {}", one);
    println!("The value of a nickel is {}", five);
    println!("The vale of a dime is {}", ten);
    println!("The value of a quarter is {}", twenty_five);

}

fn match_coin(coin_type: Coins) -> u32 {

    //curly braces are not used if the match arm code is short
    match coin_type {
        Coins::Penny => 1,
        Coins::Nickel => 5,
        Coins::Dime => 10,
        Coins::Quarter => 25
    }
}