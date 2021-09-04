enum Payment {
    Cash(f32),
    Credit(String, f32),
    Debit(DebitData),
    Crypto{account_id: String, amount: f32},
    Barter(String, String)
}

//use struct inside of enum
struct DebitData {
    card_number: String,
    amount: f32
}

fn main() {
    //create instances of the enum with all types of data
    let cash = Payment::Cash(100.12);
    let credit = Payment::Credit(String::from("amex"), 200.33);
    let debit = Payment::Debit( DebitData {
        card_number: String::from("1234556789"),
        amount: 500.32
    });
    let crypto = Payment::Crypto{account_id: String::from("coinbase"), amount: 28.99};
    let barter = Payment::Barter(String::from("coffee"), String::from("bread"));

    //pass all enum variants to single function
    process_payment(cash);
    process_payment(credit);
    process_payment(debit);
    process_payment(crypto);
    process_payment(barter);
}

fn process_payment(payment_type: Payment) {
    //match the enums and print values
    //match has two arms the pattern and the operator '=>' which seperates the pattern and the code
    match payment_type {
        Payment::Cash(amt) => {
            println!("Paying with cash...in the amount {}", amt);
        }
        //you will get an error if you do not use a attribute of an enum
        //the underscore tells the compiler that it is expilicitly done
        Payment::Credit(some_string, _) => {
            println!("Paying with credit, type: {}", some_string);
        }
        Payment::Debit(data) => {
            println!("Paying with debit, card_number: {}, amount: {}", data.card_number, data.amount);
        }
        Payment::Crypto{account_id, amount} => {
            println!("Paying with crypto, account_id: {}, amount: {}", account_id, amount)
        }
        //useful for large enum types
        _ => {
            println!("Payment with something else");
        }
    }

}