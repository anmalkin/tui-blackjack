pub use std::thread::sleep;
pub use std::{
    io::{self, Write},
    time,
};

pub fn display_welcome_message() {
    println!("Welcome to command line blackjack!");
    sleep(time::Duration::from_secs(3));
    println!("Type 'help' (or 'h') at any time for instructions. Enjoy!");
    sleep(time::Duration::from_secs(3));
    println!("Beginning new game...");
    sleep(time::Duration::from_secs(2));
}

pub fn display_new_round_msg() {
    println!("Starting new round...");
    sleep(time::Duration::from_secs(2));
}

pub fn get_user_number() -> Result<f32, io::Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let res = input.trim().parse().unwrap_or(0.0); // TODO: Better error handling
    Ok(res)
}

pub fn get_user_string() -> Result<String, io::Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

pub fn print_input_command() {
    print!("Enter move. (h)it or (s)tay: ");
    io::stdout()
        .flush()
        .expect("Failed to print to screen. Exiting game...");
}

pub fn print_invalid_command() {
    println!("Invalid command. Please enter (s)tay or (h)it.");
}
