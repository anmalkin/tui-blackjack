use core::panic;
use std::io::{self, Write};

use cli_blackjack::utils;

fn main() {
    let mut input = String::new();
    utils::display_welcome_message();

    // TODO: Initialize new game and deal first hand to user

    loop {
        print!("Enter move. (h)it or (s)tay: ");
        io::stdout().flush().expect("I/O error. Game ending... :(");
        match io::stdin().read_line(&mut input) {
            Ok(_) => todo!(), // TODO: Implement command dispatch
            Err(_) => panic!("Failed to read user input. Exiting...")
        }
    }
}
