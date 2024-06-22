pub use std::thread::sleep;
pub use std::{
    io::{self, Write},
    time,
};

use crate::errors::CliError;

pub fn get_float_from_stdin() -> Result<f32, CliError> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let res = input.trim().parse()?; 
    Ok(res)
}

pub fn get_string_from_stdin() -> Result<String, CliError> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

pub fn print_invalid_command() {
    println!("Invalid command. Please enter (s)tay or (h)it.");
}
