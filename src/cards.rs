use std::fmt::{Debug, Display};

const SPADE_UNICODE: &str = "\u{2660}";
const HEART_UNICODE: &str = "\u{2665}";
const CLUB_UNICODE: &str = "\u{2663}";
const DIAMOND_UNICODE: &str = "\u{2666}";

#[derive(Debug)]
pub enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

#[derive(Debug, PartialEq)]
pub enum Rank {
    Ace,
    Pip(u8),
    Jack,
    Queen,
    King,
}

#[derive(Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Default for Card {
    fn default() -> Self {
        Self::new()
    }
}

impl Card {
    pub fn new() -> Self {
        let value = fastrand::u8(1..=13);
        assert!(value > 0 && value < 14);
        let rank = match value {
            1 => Rank::Ace,
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            _ => Rank::Pip(value),
        };

        let suit = match fastrand::u8(0..=3) {
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
        // if self.down {
        //     return write!(f, "| HOLE CARD |");
        // }

        let rank = match self.rank {
            Rank::Ace => String::from(" A "),
            Rank::Pip(10) => String::from(" T "),
            Rank::Pip(num) => {
                format!(" {} ", num)
            }
            Rank::Jack => String::from(" J "),
            Rank::Queen => String::from(" Q "),
            Rank::King => String::from(" K "),
        };

        let suit = match self.suit {
            Suit::Hearts => HEART_UNICODE,
            Suit::Diamonds => DIAMOND_UNICODE,
            Suit::Spades => SPADE_UNICODE,
            Suit::Clubs => CLUB_UNICODE,
        };

        write!(f, "| {}  {}  {} |", suit, rank, suit)
    }
}
