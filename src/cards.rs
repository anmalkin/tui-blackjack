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

#[derive(Debug)]
pub struct Hand {
    pub cards: Vec<Card>,
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

    pub fn draw(&mut self) {
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn draw_cards() {
        let mut hand = Hand::new();
        for _ in 0..10 {
            hand.draw();
        }
        assert_eq!(hand.count(), 10);
        hand.clear();
        assert_eq!(hand.count(), 0);
    }

    #[test]
    fn get_first_card() {
        let hand: Vec<Card> = vec![
            Card {
                suit: Suit::Hearts,
                rank: Rank::Ace,
            },
            Card {
                suit: Suit::Spades,
                rank: Rank::Pip(2),
            },
        ];
        matches!(hand.first().unwrap().suit, Suit::Hearts);
        matches!(hand.first().unwrap().rank, Rank::Ace);
    }
}
