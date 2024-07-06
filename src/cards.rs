use std::fmt::{Debug, Display};

#[derive(Debug)]
pub enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

#[derive(Debug)]
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

        write!(f, "[** {} of {} **]", rank, suit)
    }
}
