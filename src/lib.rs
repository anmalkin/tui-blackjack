#![allow(dead_code, unused_imports)]

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

impl Rank {
    fn get_score(&self) -> u8 {
        match *self {
            Rank::Ace => 1,
            Rank::Pip(num) => num,
            _ => 10,
        }
    }
}

type Hand = Vec<Card>;

#[derive(Debug)]
struct Card {
    suit: Suit,
    rank: Rank,
}

struct Game {
    player_hand: Hand,
    player_score: u8,
    dealer_hand: Hand,
    dealer_score: u8,
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn initial_deal() {
        let cards: Hand = vec![Card::new(), Card::new()];
        for card in cards {
            let score = card.rank.get_score();
            assert!(score > 0 && score <= 10);
        }
    }
}
