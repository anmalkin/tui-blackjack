use std::{io::{self, Write}, time};
use std::thread::sleep;

pub fn display_welcome_message() {
    println!("Welcome to command line blackjack!");
    sleep(time::Duration::from_secs(1));
    println!("Beginning new round...");
    sleep(time::Duration::from_secs(2));
    println!("Your starting balance is: $100");
    sleep(time::Duration::from_secs(1));
    println!("Type 'help' (or 'h') at any time for instructions. Enjoy!");
}

pub fn get_user_number() -> Result<u32, io::Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let res = input.trim().parse().unwrap_or(0); // TODO: Better error handling
    Ok(res)
}
