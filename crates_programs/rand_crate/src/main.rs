// import module we need
use rand::{thread_rng, Rng};
use std::io::{self, Write};
use std::process;

//store state of game
enum Currently {
    MidGame,
    UserBuster,
    DealerBusted,
    UserBlackJack,
    DealerBlackJack,
}

fn main() {
    //intro
    println!("Welcome to the blackjack table");
    println!("------------------------------");

    //store balance
    let mut balance = 100;

    //exit game loop if players balance is 0
    let mut balance_not_zero = true;
    while balance_not_zero {

        //get wager amount and check if player has enough money
        println!("Balance: ${}", balance);
        print!("Enter wager ('0' to quit): ");
        let mut wager = get_wager();
        let mut has_enough = false;
        while !has_enough {
            has_enough = check_balance(wager, balance);
            if !has_enough {
                println!("Not enough skrilla -- Current Balance: ${}", balance);
                print!("Enter wager: ");
                wager = get_wager();
            };
        }

        //deal first two cards
        let (mut player, mut dealer) = play_hand();

        //hit or stand and logic for who won
        let mut keep_looping = true;
        while keep_looping {

            //every card update the state of the game
            let state = current_state(player, dealer);

            match state {
                Currently::UserBuster => {
                    balance = balance - wager;
                    keep_looping = false;
                }
                Currently::DealerBusted => {
                    balance = balance + wager;
                    keep_looping = false;
                }
                Currently::MidGame => {
                    //user can hit or stand, dealer reacts to the result
                    let hit = hit_or_stand();
                    if hit {
                        println!("User Hits");
                        let new_player = hit_me(player);
                        player = new_player;
                        //if user does not bust and is winning dealer hits
                        if player < 21 && player > dealer {
                            println!("Dealer Hits");
                            let new_dealer = hit_me(dealer);
                            dealer = new_dealer;
                        }
                    } else {
                        //if user stands and dealer is losing dealer hits
                        if player < 21 && player > dealer {
                            println!("Dealer Hits");
                            let new_dealer = hit_me(dealer);
                            dealer = new_dealer;
                        } else {
                            //game is over update balance and print result
                            if player > dealer {
                                println!("Player Wins");
                                balance = balance + wager;
                                keep_looping = false;
                            } else if player == dealer {
                                println!("Hand is a push");
                                keep_looping = false;
                            }else {
                                println!("Dealer Wins");
                                balance = balance - wager;
                                keep_looping = false;
                            }
                        }
                    }
                }
                Currently::DealerBlackJack => {
                    println!("Dealer Blackjack");
                    balance = balance - wager;
                    keep_looping = false;
                }
                Currently::UserBlackJack => {
                    println!("User Blackjack");
                    balance = balance + wager;
                    keep_looping = false;
                }
            }
        }
        //break game loop of balance is zero
        if balance == 0 {
            balance_not_zero = false;
            println!("No more skrilla");
        }
    }
}

fn get_card() -> u32 {
    //return random number between 2 and 13
    thread_rng().gen_range(2..14)
}

fn play_hand() -> (u32, u32) {
    //deal first two cards
    let u_card1 = get_card();
    let u_card2 = get_card();
    println!("Player- Card: {}, Card: {}, Total: {}", u_card1, u_card2, u_card1 + u_card2);

    //dealers first two cards
    let d_card1 = get_card();
    let d_card2 = get_card();
    println!("Dealer- Card: {}, Card: {}, Total: {}", d_card1, d_card2, d_card1 + d_card2);

    //return totals
    (u_card1 + u_card2, d_card1 + d_card2)
}

fn current_state(user: u32, dealer: u32) -> Currently {
    //logic for current state of the game
    if user == 21 {
        Currently::UserBlackJack
    } else if dealer == 21 {
        Currently::DealerBlackJack
    } else if user > 21 {
        Currently::UserBuster
    } else if dealer > 21 {
        Currently::DealerBusted
    }  else {
        Currently::MidGame
    }
}

fn hit_me(tot: u32) -> u32 {
    let next_card = get_card();
    println!("Card {}, Total {}", next_card, tot + next_card);
    tot + next_card
}

fn hit_or_stand() -> bool {
    print!("Hit(h) or Stand(s): ");
    let _ = io::stdout().flush();
    let mut input = String::new();

    //get input
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    //get first letter of string
    let ch = input.chars().next().unwrap();

    match ch {
        'h' => true,
        'H' => true,
        's' => false,
        'S' => false,
        _ => true,
    }
}

fn check_balance(wager: u32, balance: u32) -> bool {
    if wager > balance {
        false
    } else {
        true
    }
}

fn get_wager() -> u32 {
    let mut wager = String::new();
    let _ = io::stdout().flush();

    //get input
    io::stdin()
        .read_line(&mut wager)
        .expect("Failed to read line");

    //shadow integer and error check
    let wager: u32 = wager.trim().parse().expect("Please type a number!");
    if wager == 0 {
        process::exit(1)
    } else {
        wager
    }
}
