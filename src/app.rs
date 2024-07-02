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
const ACE_HIGH: u8 = 11;
const ACE_LOW: u8 = 1;
const BLACKJACK: u8 = 21;
const FACECARD: u8 = 10;
const DEALER_STAND: u8 = 17;


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
        calc_score(&self.player_hand)
    }

    pub fn dealer_score(&self) -> u8 {
        calc_score(&self.dealer_hand)
    }

    pub fn run(&mut self, command: Command) {
        match command {
            Command::Hit => {
                self.player_hand.add_card();
                if self.player_score() > BLACKJACK {
                    self.state = GameState::Lose;
                }
            }
            Command::Stay => {
                let mut dealer_score = self.dealer_score();
                let player_score = self.player_score();
                while dealer_score < DEALER_STAND {
                    self.dealer_hand.add_card();
                    dealer_score = self.dealer_score();
                }

                if dealer_score > BLACKJACK || dealer_score < player_score {
                    // Ensure dealer does not run after player has already lost
                    assert!(self.player_score() <= BLACKJACK);
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

/// Calculate current score of blackjack hand. Aces are scored as 11 unless the total score is
/// above 21, in which case they are scored as 1.
fn calc_score(hand: &Hand) -> u8 {
    let mut aces = 0;
    let mut score = 0;
    for card in hand.cards.iter() {
        match card.rank {
            Rank::Ace => {
                aces += 1;
                score += ACE_HIGH;
            }
            Rank::Pip(num) => {
                score += num;
            }
            Rank::Jack => {
                score += FACECARD;
            }
            Rank::Queen => {
                score += FACECARD;
            }
            Rank::King => {
                score += FACECARD;
            }
        }
    }

    // Adjust Aces value downward if necessary
    while score > BLACKJACK && aces > 0 {
        score -= ACE_HIGH - ACE_LOW; // note operator precedence
        aces -= 1;
        assert!(score >= 2);
    }
    score
}

// Helper functions for displaying various inputs

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn deal() {
        let mut app = App::default();
        app.start();
        let player_count = app.player_hand.count();
        let dealer_count = app.dealer_hand.count();
        assert_eq!(2, player_count);
        assert_eq!(2, dealer_count);
        assert!(app.player_score() > 1);
        assert!(app.dealer_score() > 1);
    }

    #[test]
    fn hit() {
        let mut app = App::default();
        app.start();
        let old_player_score = app.player_score();

        app.run(Command::Hit);
        let new_player_score = app.player_score();
        assert!(new_player_score > old_player_score);

        let player_count = app.player_hand.count();
        let dealer_count = app.dealer_hand.count();
        assert_eq!(3, player_count);
        assert_eq!(2, dealer_count);
    }
}
