use std::fmt::{Debug, Display};

pub const ACE_HIGH: u8 = 11;
pub const ACE_LOW: u8 = 1;
pub const BLACKJACK: u8 = 21;
pub const FACECARD: u8 = 10;

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
pub struct Card {
    suit: Suit,
    rank: Rank,
}

impl Default for Card {
    fn default() -> Self {
        Self::new()
    }
}

impl Card {
    pub fn new() -> Self {
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

#[derive(Debug)]
pub struct Hand {
    cards: Vec<Card>,
}

impl Default for Hand {
    fn default() -> Self {
        Self::new()
    }
}

impl Hand {
    pub fn new() -> Self {
        Hand { cards: Vec::new() }
    }

    pub fn add_card(&mut self) {
        self.cards.push(Card::new());
    }

    pub fn first(&self) -> Option<&Card> {
        self.cards.first()
    }

    pub fn as_ref(&self) -> &[Card] {
        self.cards.as_ref()
    }

    pub fn count(&self) -> usize {
        self.cards.len()
    }

    pub fn clear(&mut self) {
        self.cards.clear()
    }

    pub fn calc_score(&self) -> u8 {
        let mut aces = 0;
        let mut score = 0;
        for card in &self.cards {
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
}
