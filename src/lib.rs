#![allow(dead_code, unused_imports)]

pub mod utils;

use core::{num, panic};
use std::{
    fmt::Display,
    io::{self, Write},
};

use crate::utils::*;

// Magic numbers
const BLACKJACK: u8 = 21;
const FACECARD: u8 = 10;
const ACE_HIGH: u8 = 11;
const ACE_LOW: u8 = 1;
const MIN_BET: u32 = 5;

#[derive(Debug)]
enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

#[derive(Debug)]
enum Rank {
    Ace,
    Pip(u8),
    Jack,
    Queen,
    King,
}

#[derive(Debug)]
enum GameResult {
    Blackjack,
    Score(u8),
    Bust,
}

#[derive(Debug)]
enum Command {
    Hit,
    Stay,
    Split,
    Invalid,
}

type Hand = Vec<Card>;

/// Run blackjack game loop
pub fn run_game_loop() {
    let mut bank = 100;
    let mut user_command: String;
    let mut active_bet;

    'game: loop {
        println!("Current bank balance is ${}", bank);
        print!("Place bet (min: {}): ", MIN_BET);
        io::stdout()
            .flush()
            .expect("Failed to print to screen. Exiting game...");
        match get_user_number() {
            Ok(num) => {
                active_bet = num;
            }
            Err(_) => panic!("Error reading user input. Exiting game..."),
        }

        if active_bet < MIN_BET || active_bet > bank {
            println!(
                "Bet amount must be valid number of at least ${MIN_BET} and less than the total bank balance."
            );
            continue 'game;
        }

        'round: loop {
            // TODO: Display initial deal

            print_input_command();
            match get_user_string() {
                Ok(command) => user_command = command,
                Err(_) => panic!("Error reading user input. Exiting game..."),
            }

            match get_command(&user_command) {
                Command::Hit => println!("Hit!"),
                Command::Stay => println!("Staying put..."),
                // TODO: Implement split functionality
                Command::Split => println!("Split functionality not yet implemented"),
                Command::Invalid => {
                    print_invalid_command();
                    continue 'round;
                }
            }

            break;
        }

        break;
    }

    // print!("Enter move. (h)it or (s)tay: ");
}

fn calc_score(hand: &Hand) -> u8 {
    let mut aces = 0;
    let mut score = 0;
    for card in hand.iter() {
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

fn get_command(s: &str) -> Command {
    match s {
        "h" => Command::Hit,
        "hit" => Command::Hit,
        "s" => Command::Stay,
        "stay" => Command::Stay,
        _ => Command::Invalid,
    }
}

#[derive(Debug)]
struct Card {
    suit: Suit,
    rank: Rank,
}

impl Card {
    fn new() -> Self {
        let value = fastrand::u8(1..=13);
        let rank = match value {
            1 => Rank::Ace,
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            _ => Rank::Pip(value),
        };

        let suit = match fastrand::u8(0..4) {
            0 => Suit::Hearts,
            1 => Suit::Diamonds,
            2 => Suit::Spades,
            3 => Suit::Clubs,
            _ => panic!("Not a valid suit"),
        };

        Card { suit, rank }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rank = match self.rank {
            Rank::Ace => String::from("Ace"),
            Rank::Pip(num) => num.to_string(),
            Rank::Jack => String::from("Jack"),
            Rank::Queen => String::from("Queen"),
            Rank::King => String::from("King"),
        };

        let suit = match self.suit {
            Suit::Hearts => String::from("Hearts"),
            Suit::Diamonds => String::from("Diamonds"),
            Suit::Spades => String::from("Spades"),
            Suit::Clubs => String::from("Clubs"),
        };

        write!(f, "[**{} of {}**]", rank, suit)
    }
}

struct Round {
    player: Hand,
    dealer: Hand,
    result: GameResult,
}

impl Round {
    fn new() -> Self {
        // Initialize new player
        let player = vec![Card::new(), Card::new()];
        let result = match calc_score(&player) {
            21 => GameResult::Blackjack,
            num => GameResult::Score(num),
        };

        // Initialize dealer
        let dealer = vec![Card::new(), Card::new()];

        Self {
            player,
            dealer,
            result,
        }
    }

    fn hit(&mut self) {
        self.player.push(Card::new());

        // State cannot be Blackjack after initial draw
        self.result = match calc_score(&self.player) {
            (22..) => GameResult::Bust,
            num => GameResult::Score(num),
        };
    }

    fn run_dealer(&mut self) {
        let mut dealer_score = calc_score(&self.dealer);
        while dealer_score < 17 {
            self.dealer.push(Card::new());
            dealer_score = calc_score(&self.dealer);
        }
    }
}

struct Game {
    bank: u32,
    active_bet: u32,
}

impl Game {
    fn new() -> Self {
        Self {
            bank: 100,
            active_bet: 0,
        }
    }

    fn place_bet(&mut self, amt: u32) {
        if amt > self.bank {
            self.active_bet = self.bank;
            println!("Bet is larger than remaining chips.")
        } else {
            self.active_bet += amt;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn calc_score_test() {
        let hand = vec![
            Card {
                suit: Suit::Clubs,
                rank: Rank::Jack,
            },
            Card {
                suit: Suit::Diamonds,
                rank: Rank::Queen,
            },
        ];
        assert_eq!(calc_score(&hand), 20);

        let hand = vec![
            Card {
                suit: Suit::Clubs,
                rank: Rank::Ace,
            },
            Card {
                suit: Suit::Diamonds,
                rank: Rank::Queen,
            },
        ];
        assert_eq!(calc_score(&hand), 21);

        let hand = vec![
            Card {
                suit: Suit::Clubs,
                rank: Rank::Ace,
            },
            Card {
                suit: Suit::Diamonds,
                rank: Rank::Ace,
            },
        ];
        assert_eq!(calc_score(&hand), 12);

        let hand = vec![
            Card {
                suit: Suit::Clubs,
                rank: Rank::Pip(8),
            },
            Card {
                suit: Suit::Diamonds,
                rank: Rank::Pip(10),
            },
        ];
        assert_eq!(calc_score(&hand), 18);
    }

    #[test]
    fn calc_score_test_hard() {
        let hand = vec![
            Card {
                suit: Suit::Clubs,
                rank: Rank::Pip(8),
            },
            Card {
                suit: Suit::Diamonds,
                rank: Rank::Pip(10),
            },
            Card {
                suit: Suit::Spades,
                rank: Rank::Ace,
            },
            Card {
                suit: Suit::Spades,
                rank: Rank::Ace,
            },
            Card {
                suit: Suit::Spades,
                rank: Rank::Ace,
            },
        ];
        assert_eq!(calc_score(&hand), 21);

        let hand = vec![
            Card {
                suit: Suit::Clubs,
                rank: Rank::Pip(8),
            },
            Card {
                suit: Suit::Diamonds,
                rank: Rank::Pip(10),
            },
            Card {
                suit: Suit::Spades,
                rank: Rank::Ace,
            },
            Card {
                suit: Suit::Diamonds,
                rank: Rank::Ace,
            },
            Card {
                suit: Suit::Clubs,
                rank: Rank::Ace,
            },
            Card {
                suit: Suit::Clubs,
                rank: Rank::Ace,
            },
        ];
        assert_eq!(calc_score(&hand), 22);

        let hand = vec![
            Card {
                suit: Suit::Clubs,
                rank: Rank::Pip(3),
            },
            Card {
                suit: Suit::Clubs,
                rank: Rank::Ace,
            },
            Card {
                suit: Suit::Clubs,
                rank: Rank::Jack,
            },
            Card {
                suit: Suit::Spades,
                rank: Rank::Jack,
            },
        ];
        assert_eq!(calc_score(&hand), 24);
    }

    #[test]
    fn display_cards() {
        let num_card = Card {
            suit: Suit::Clubs,
            rank: Rank::Pip(3),
        };
        let face_card = Card {
            suit: Suit::Diamonds,
            rank: Rank::King,
        };
        let ace = Card {
            suit: Suit::Hearts,
            rank: Rank::Ace,
        };

        assert_eq!(num_card.to_string(), "[**3 of Clubs**]");
        assert_eq!(face_card.to_string(), "[**King of Diamonds**]");
        assert_eq!(ace.to_string(), "[**Ace of Hearts**]");
    }
}
