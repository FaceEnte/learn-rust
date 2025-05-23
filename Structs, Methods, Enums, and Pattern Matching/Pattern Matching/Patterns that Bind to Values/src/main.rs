#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    let alb_quarter = Coin::Quarter(UsState::Alabama);
    println!("{:?}", value_in_cents(&alb_quarter));

    println!("{:?}", alb_quarter);
    println!("{:?}", alb_quarter);

    let value = value_in_cents(&Coin::Quarter(UsState::Alabama));
}
