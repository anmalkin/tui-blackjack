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

type Hand = Vec<Card>;

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

    fn get_score(&self) -> u8 {
        match self.rank {
            Rank::Ace => 11,
            Rank::Pip(num) => num,
            Rank::Jack => 10,
            Rank::Queen => 10,
            Rank::King => 10,
        }
    }
}

struct Player {
    hand: Hand,
    score: u8,
}

struct Round {
    player: Player,
    dealer: Player,
}

impl Round {
    fn new() -> Self {
        // Initialize new player
        let hand = vec![Card::new(), Card::new()];
        let score = hand.iter().fold(0, |score, card| score + card.get_score());
        let player = Player { hand, score };

        // Initialize dealer
        let hand = vec![Card::new(), Card::new()];
        let score = hand.iter().fold(0, |score, card| score + card.get_score());
        let dealer = Player { hand, score };

        Self { player, dealer }
    }

    fn hit_me(&mut self) -> u8 {
        let card = Card::new();
        self.player.score += card.get_score();
        self.player.hand.push(card);
        self.player.score
    }
}

struct Game {
    bank: u32,
    active_bet: u32,
}

impl Game {
    fn new(bank: u32) -> Self {
        Self { bank, active_bet: 0 }
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
    fn initial_deal() {
        let cards: Hand = vec![Card::new(), Card::new()];
        for card in cards {
            let score = card.get_score();
            assert!(score > 0 && score <= 10);
        }
    }
}
