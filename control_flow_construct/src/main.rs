enum Coin {
    Penny,
    Nickel, 
    Dime,
    Quarter
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime  => 10,
        Coin::Quarter  => 25,
    }
}


fn main() {
    let coins = vec![Coin::Penny, Coin::Nickel, Coin::Dime, Coin::Quarter];
    for coin in coins {
        let value = value_in_cents(coin);
        println!("The value of the coin is: {} cents", value)
    }
}
