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
    bank: u32,
    player_hand: Hand,
    dealer_hand: Hand,
    current_bet: u32,
    state: GameState,
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

    pub fn bank(&self) -> u32 {
        self.bank
    }

    pub fn player_score(&self) -> u8 {
        self.player_hand.calc_score()
    }

    pub fn dealer_score(&self) -> u8 {
        self.dealer_hand.calc_score()
    }

    pub fn player_hand(&self) -> &Hand {
        &self.player_hand
    }

    pub fn dealer_hand(&self) -> &Hand {
        &self.dealer_hand
    }

    pub fn place_bet(&mut self, bet: u32) {
        self.current_bet = bet;
    }

    pub fn run_command(&mut self, command: Command) {
        match command {
            Command::Hit => {
                self.player_hand.add_card();
                if self.player_score() > 21 {
                    self.state = GameState::Lose;
                }
            }
            Command::Stay => self.state = GameState::DealerTurn,
            Command::Split => todo!(),
            Command::Dealer => {
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
                } else {
                    self.state = GameState::Lose;
                }
            }
        }
    }

    pub fn win(&mut self, multiplier: u32) {
        self.bank += self.current_bet * multiplier;
        self.current_bet = 0;
    }

    pub fn lose(&mut self) {
        self.bank -= self.current_bet;
        self.current_bet = 0;
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
    Blackjack, // TODO: deprecate
    PlayerTurn,
    DealerTurn,
    Score(u8), // TODO: deprecate
    Bust,      // TODO: deprecate
    Win,
    Lose,
    Quit,
}

#[derive(Debug)]
pub enum Command {
    Hit,
    Stay,
    Split,
    Dealer,
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

struct Round {
    player: Hand,
    dealer: Hand,
    result: GameState,
}

impl Round {
    fn new() -> Self {
        // Initialize new player
        let player = Hand::new();
        let result = match player.calc_score() {
            21 => GameState::Blackjack,
            num => GameState::Score(num),
        };

        // Initialize dealer
        let dealer = Hand::new();

        Self {
            player,
            dealer,
            result,
        }
    }

    fn hit(&mut self) {
        self.player.add_card();

        // State cannot be Blackjack after initial draw
        self.result = match self.player.calc_score() {
            (22..) => GameState::Bust,
            num => GameState::Score(num),
        };
    }

    fn run_dealer(&mut self) {
        let mut dealer_score = self.dealer.calc_score();
        while dealer_score < 17 {
            self.dealer.add_card();
            dealer_score = self.dealer.calc_score();
        }
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
