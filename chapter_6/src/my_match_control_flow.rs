#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

pub fn my_match_control_flow_function() {
    let coin = Coin::Quarter(UsState::Alaska);
    println!("Value is {}",value_in_cent(coin));

}

fn value_in_cent(coin:Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime =>10,
        Coin::Quarter(UsState::Alaska) => {
            println!("Hello from Alaska");
            25
        }

    }
}