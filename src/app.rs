#![allow(dead_code, unused_imports)]

use core::{num, panic};
use std::{
    fmt::{Debug, Display},
    io::{self, Write},
};

use crate::errors::CliError;
use crate::utils::*;
use crate::cards::*;

const PAYOUT: f32 = 1.0;
const BLACKJACK_PAYOUT: f32 = 1.5;

#[derive(Debug)]
pub struct App {
    bank: u32,
    player_hand: Hand,
    dealer_hand: Hand,
    current_bet: u32,
}

impl App {
    pub fn new(bank: u32) -> Self {
        App {
            bank,
            player_hand: Hand::new(),
            dealer_hand: Hand::new(),
            current_bet: 0,
        }
    }

    /// Initialize bank with argument. Bank defaults to $100 if not explicitly set.
    pub fn set_bank(&mut self, amt: u32) {
        self.bank = amt;
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

    pub fn place_bet(&mut self, bet: u32) {
        self.current_bet = bet;
    }

    pub fn player_draw(&mut self) {
        self.player_hand.add_card();
    }

    pub fn dealer_draw(&mut self) {
        self.dealer_hand.add_card();
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
        }
    }
}

#[derive(Debug)]
enum GameState {
    Blackjack,
    Score(u8),
    Bust,
}

#[derive(Debug)]
enum Command {
    Hit,
    Stay,
    Split,
}

/// Run blackjack game loop
pub fn run_game_loop() {
    let mut bank = 100.0;
    let mut user_command: String;
    let mut active_bet;

    'game: loop {
        println!("Current bank balance is ${}", bank);
        if bank <= 0.0 {
            println!("You are out of money. Better luck next time!");
            break;
        }

        print!("Place bet: ");
        io::stdout()
            .flush()
            .expect("Failed to print to screen. Exiting game...");

        match get_float_from_stdin() {
            Ok(num) => {
                active_bet = num;
            }
            Err(CliError::ParseError(_)) => {
                println!("Not a valid number");
                continue 'game;
            }
            Err(_) => panic!("Error reading user input. Exiting game..."),
        }

        if active_bet == 0.0 || active_bet > bank {
            println!("Bet amount must be greater than 0 and less than the bank balance.");
            continue 'game;
        }

        println!("Starting new round...");
        sleep(time::Duration::from_secs(2));
        let mut round = Round::new();
        print_player_hand(&round.player);

        // Check for Blackjack
        if let GameState::Blackjack = round.result {
            let payout = active_bet * BLACKJACK_PAYOUT;
            println!("Blackjack! +${}", payout);
            bank += payout;
            continue 'game;
        }

        println!("Dealer showing...");
        sleep(time::Duration::from_secs(2));
        println!("{}", round.dealer.first().unwrap());
        sleep(time::Duration::from_secs(2));

        'round: loop {
            print_input_command();
            match get_string_from_stdin() {
                Ok(command) => user_command = command,
                Err(_) => panic!("Uh oh. Looks like we ran into technical difficulties :("),
            }

            match get_command(&user_command) {
                Ok(Command::Hit) => {
                    round.hit();
                    println!("Your hand:");
                    print_player_hand(&round.player);
                    sleep(time::Duration::from_secs(1));
                    println!();
                    println!("Dealer showing...");
                    sleep(time::Duration::from_secs(1));
                    println!("{}", round.dealer.first().unwrap());
                    match round.result {
                        GameState::Bust => {
                            println!("Bust! -${}", active_bet);
                            bank -= active_bet;
                            continue 'game;
                        }
                        GameState::Score(_) => continue 'round,
                        GameState::Blackjack => println!("21!"),
                    }
                }
                Ok(Command::Stay) => {
                    round.run_dealer();
                    let player_score = round.player.calc_score();
                    let dealer_score = round.dealer.calc_score();
                    println!();
                    println!("Dealer drawing...");
                    print_dealer_hand(round.dealer);
                    if dealer_score > 21 {
                        println!("Dealer busts! You win! +{}", active_bet);
                        bank += active_bet;
                        continue 'game;
                    }
                    match player_score.cmp(&dealer_score) {
                        std::cmp::Ordering::Less => {
                            println!("You lose! -${}", active_bet);
                            bank -= active_bet;
                        }
                        std::cmp::Ordering::Equal => {
                            println!("Draw! No payout");
                        }
                        std::cmp::Ordering::Greater => {
                            println!("You win! +${}", active_bet);
                            bank += active_bet;
                        }
                    }
                    continue 'game;
                }
                Ok(Command::Split) => {
                    println!("Split functionality not yet implemented!");
                    continue 'round;
                }
                Err(_) => {
                    println!("Not a valid command");
                    print_invalid_command();
                    continue 'round;
                }
            }
        }
    }
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
