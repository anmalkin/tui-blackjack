#![allow(dead_code, unused_imports)]

const BLACKJACK: u8 = 21;
const FACECARD: u8 = 10;
const ACE_HIGH: u8 = 11;
const ACE_LOW: u8 = 1;

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

type Hand = Vec<Card>;

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
}

struct Game {
    bank: u32,
    active_bet: u32,
}

impl Game {
    fn new(bank: u32) -> Self {
        Self {
            bank,
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
            Card { suit: Suit::Clubs, rank: Rank::Jack },
            Card { suit: Suit::Diamonds, rank: Rank::Queen },
        ];
        assert_eq!(calc_score(&hand), 20);

        let hand = vec![
            Card { suit: Suit::Clubs, rank: Rank::Ace },
            Card { suit: Suit::Diamonds, rank: Rank::Queen },
        ];
        assert_eq!(calc_score(&hand), 21);

        let hand = vec![
            Card { suit: Suit::Clubs, rank: Rank::Ace },
            Card { suit: Suit::Diamonds, rank: Rank::Ace },
        ];
        assert_eq!(calc_score(&hand), 12);

        let hand = vec![
            Card { suit: Suit::Clubs, rank: Rank::Pip(8) },
            Card { suit: Suit::Diamonds, rank: Rank::Pip(10) },
        ];
        assert_eq!(calc_score(&hand), 18);
    }

    #[test]
    fn calc_score_test_hard() {
        let hand = vec![
            Card { suit: Suit::Clubs, rank: Rank::Pip(8) },
            Card { suit: Suit::Diamonds, rank: Rank::Pip(10) },
            Card { suit: Suit::Spades, rank: Rank::Ace },
            Card { suit: Suit::Spades, rank: Rank::Ace },
            Card { suit: Suit::Spades, rank: Rank::Ace },
        ];
        assert_eq!(calc_score(&hand), 21);

        let hand = vec![
            Card { suit: Suit::Clubs, rank: Rank::Pip(8) },
            Card { suit: Suit::Diamonds, rank: Rank::Pip(10) },
            Card { suit: Suit::Spades, rank: Rank::Ace },
            Card { suit: Suit::Diamonds, rank: Rank::Ace },
            Card { suit: Suit::Clubs, rank: Rank::Ace },
            Card { suit: Suit::Clubs, rank: Rank::Ace },
        ];
        assert_eq!(calc_score(&hand), 22);

        let hand = vec![
            Card { suit: Suit::Clubs, rank: Rank::Pip(3) },
            Card { suit: Suit::Clubs, rank: Rank::Ace },
            Card { suit: Suit::Clubs, rank: Rank::Jack },
            Card { suit: Suit::Spades, rank: Rank::Jack },
        ];
        assert_eq!(calc_score(&hand), 24);
    }
}
