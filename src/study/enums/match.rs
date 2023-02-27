use rand::Rng;

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
    Custom(u8)
}

enum IpAddrSpecificEnum { // variant can store different types and amounts of data
    V4(u8, u8, u8, u8),
    V6(String)
}

fn main() {
    // let penny = Coin::Penny;

    // println!("Penny value is {} cents", value_in_cents(penny));

    // let alabama_quarter = Coin::Quarter(UsState::Alabama);

    // println!("Quarter value is {} cents", value_in_cents(alabama_quarter));

    // let custom_coin = Coin::Custom(56);

    // println!("Custom coin value is {} cents", value_in_cents(custom_coin));
    
    // let home3 = IpAddrSpecificEnum::V4(127, 0, 0, 1);

    // let readable_address = get_route(home3);

    // println!("{readable_address}");

    // let five = Some(5);
    // let six = option_plus_one(five);
    // let none = option_plus_one(None);

    roll_the_dice();
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Can write a lot of code here!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Quarter from {:?}!", state);
            25
        },
        Coin::Custom(value) => value
    }
}

fn get_route(address: IpAddrSpecificEnum) -> String {
    match address {
        IpAddrSpecificEnum::V4(n1, n2, n3, n4) => {
            format!("{}.{}.{}.{}", n1.to_string(), n2.to_string(), n3.to_string(), n4.to_string())
        },
        IpAddrSpecificEnum::V6(addr) => addr, 
    }
}

fn option_plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn roll_the_dice() {
    let dice_roll: i32 = rand::thread_rng().gen_range(1..=6);

    //catch-all pattern: specific arms for 1 and 6, common arm for every other possible values
    match dice_roll {
        1 => {
            println!("Critical fail");
        },
        6 => {
            println!("Criticall success");
        },
        roll_result => {
            println!("{roll_result}")
        },
    }

    // if we don't need value in catch-all we can use _
    match dice_roll { 
        6 => {
            println!("Great");
        },
        5 => {
            println!("Cool!");
        },
        _ => (), // do nothing
    }
}