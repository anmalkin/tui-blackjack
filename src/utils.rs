use std::time;
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
