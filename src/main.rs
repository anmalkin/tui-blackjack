use cli_blackjack::{self, run_game_loop, utils::*};

fn main() {
    display_welcome_message();
    run_game_loop();
}

fn display_welcome_message() {
    println!("Welcome to command line blackjack!");
    sleep(time::Duration::from_secs(3));
    println!("Type 'help' (or 'h') at any time for instructions. Enjoy!");
    sleep(time::Duration::from_secs(3));
    println!("Beginning new game...");
    sleep(time::Duration::from_secs(2));
}

