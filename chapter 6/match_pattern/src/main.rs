fn main() {
    let coin_value = value_in_cents(Coin::Nickel);
    println!("coin value: {:?}", coin_value);

    let coin_value = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("coin value: {:?}", coin_value);

    //option
    let five = Some(5);
    let six = add_one(five);
    println!("six: {:?}", six);

    let none = add_one(None);
    println!("none: {:?}", none);
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("{:?}", state);
            25
        },
    }
}

fn add_one(number: Option<i32>) -> Option<i32>{
    match number{
        None => None,
        Some(i) => Some(i+1)
    }
}