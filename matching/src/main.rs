enum Coins {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn main() {
    let five = Some(5);
    let six = plus_one(five).unwrap();
    let none = plus_one(None);

    println!("If I get a 5, and plus one: {six}");
    //println!("If I get a None, and plus one: {none}");

    let cents = value_in_cents(Coins::Quarter(UsState::Alaska));
    println!("Cents of a Quarter is: {cents}");
}

fn value_in_cents(coin: Coins) -> u8 {
    match coin {
        Coins::Penny => {
            println!("Lucky Penny!");
            1
        }
        Coins::Nickel => 5,
        Coins::Dime => 10,
        Coins::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
        other => {
            println!("An unknown coin was catched!");
            1
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
