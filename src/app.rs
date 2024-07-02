#![allow(dead_code, unused_imports)]

use core::{num, panic};
use std::{
    fmt::{Debug, Display},
    io::{self, Write},
};

use crate::cards::*;
use crate::errors::CliError;
use crate::utils::*;

const PAYOUT: f32 = 1.0;
const BLACKJACK_PAYOUT: f32 = 1.5;

#[derive(Debug)]
pub struct App {
    pub bank: u32,
    pub player_hand: Hand,
    pub dealer_hand: Hand,
    pub current_bet: u32,
    pub state: GameState,
}

impl App {
    pub fn new(bank: u32) -> Self {
        App {
            bank,
            player_hand: Hand::new(),
            dealer_hand: Hand::new(),
            current_bet: 0,
            state: GameState::Start,
        }
    }

    pub fn start(&mut self) {
        self.current_bet = 0;
        self.player_hand.clear();
        self.player_hand.add_card();
        self.player_hand.add_card();

        self.dealer_hand.clear();
        self.dealer_hand.add_card();
        self.dealer_hand.add_card();
    }

    pub fn player_score(&self) -> u8 {
        self.player_hand.calc_score()
    }

    pub fn dealer_score(&self) -> u8 {
        self.dealer_hand.calc_score()
    }

    pub fn run(&mut self, command: Command) {
        match command {
            Command::Hit => {
                self.player_hand.add_card();
                if self.player_score() > 21 {
                    self.state = GameState::Lose;
                }
            }
            Command::Stay => {
                let mut dealer_score = self.dealer_score();
                let player_score = self.player_score();
                while dealer_score < 17 {
                    self.dealer_hand.add_card();
                    dealer_score = self.dealer_score();
                }
                if dealer_score > 21 || dealer_score < player_score {
                    // Ensure dealer does not run after player has already lost
                    assert!(self.player_score() <= 21);
                    self.state = GameState::Win;
                    self.bank += self.current_bet
                } else {
                    self.state = GameState::Lose;
                    self.bank -= self.current_bet
                }
            }
            Command::Split => todo!(),
        }
    }

}

/// Default bank amount set to $100
impl Default for App {
    fn default() -> Self {
        App {
            bank: 100,
            player_hand: Hand::new(),
            dealer_hand: Hand::new(),
            current_bet: 0,
            state: GameState::Start,
        }
    }
}

#[derive(Debug)]
pub enum GameState {
    Start,
    PlayerTurn,
    Win,
    Lose,
    Quit,
}

#[derive(Debug)]
pub enum Command {
    Hit,
    Stay,
    Split,
}

fn get_command(s: &str) -> Result<Command, CliError> {
    match s {
        "h" => Ok(Command::Hit),
        "hit" => Ok(Command::Hit),
        "s" => Ok(Command::Stay),
        "stay" => Ok(Command::Stay),
        _ => Err(CliError::InvalidCommand),
    }
}

// Helper functions for displaying various inputs

fn print_player_hand(hand: &Hand) {
    for card in hand.as_ref() {
        println!("{card}");
    }
    println!("Score: {}", hand.calc_score());
}

fn print_dealer_hand(hand: Hand) {
    for card in hand.as_ref() {
        println!("{card}");
        sleep(time::Duration::from_secs(2));
    }
    sleep(time::Duration::from_secs(2));
    println!("Dealer score: {}", hand.calc_score());
    sleep(time::Duration::from_secs(2));
}

pub fn print_input_command() {
    print!("Enter move. (h)it or (s)tay: ");
    io::stdout()
        .flush()
        .expect("Failed to print to screen. Exiting game...");
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn calc_score_test() {
        todo!()
    }
}
