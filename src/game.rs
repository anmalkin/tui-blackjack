use crate::cards::*;

const ACE_HIGH: u8 = 11;
const ACE_LOW: u8 = 1;
const BLACKJACK: u8 = 21;
const FACECARD: u8 = 10;
const DEALER_STAND: u8 = 17;

#[derive(Debug)]
pub struct Game {
    pub bank: u32,
    pub player: Vec<Hand>,
    pub dealer: Dealer,
    pub state: State,
    pub active_hands: usize,
}

impl Game {
    pub fn new(bank: u32) -> Self {
        let player = Vec::new();
        let dealer = Dealer::new();
        let state = State::Bet;
        let active_hands = 1;

        Game {
            bank,
            player,
            dealer,
            state,
            active_hands,
        }
    }

    pub fn place_bet(&mut self, bet: u32) {
        self.player.push(Hand::new(bet));
        self.state = State::Play;
    }

    pub fn reset(&mut self) {
        self.player.clear();
        self.dealer.reset();
        self.active_hands = 1;
        self.state = State::Bet;
    }

    pub fn execute(&mut self, command: Command) {
        match command {
            Command::Hit => {
                let hand = self.player.get_mut(self.active_hands - 1).unwrap();
                hand.hit();
            }

            Command::Stand => {}

            Command::Split => {
                let hand = self.player.get_mut(self.active_hands - 1).unwrap();
                let split_hand = hand.split();
                self.player.push(split_hand);
                self.active_hands += 1;
            }
        }

        if self.active_hands == 0 {
            self.dealer.run();
            self.calc_results();
            self.state = State::Results;
        }
    }

    pub fn active_hand(&self) -> &Hand {
        // FIXME: Panic happens here when UI calls before bet is made
        self.player.get(self.active_hands - 1).unwrap()
    }

    pub fn splittable(&self) -> bool {
        let hand = self.active_hand();
        if hand.cards.len() > 2 {
            return false;
        }
        hand.cards[0].rank == hand.cards[1].rank
    }

    fn calc_results(&mut self) {
        let dealer_score = self.dealer.score();
        for hand in &mut self.player {
            if hand.result.is_none() {
                hand.compare(dealer_score);
            }
            match &hand.result {
                Some(res) => match res {
                    HandResult::Win | HandResult::TwentyOne => {
                        self.bank += hand.bet;
                    }
                    HandResult::Bust | HandResult::Lose => {
                        self.bank -= hand.bet;
                    }
                    HandResult::Draw => {}
                    HandResult::Blackjack => {
                        self.bank += hand.bet * 3 / 2;
                    }
                },
                None => panic!("None result found in hand after running dealer"),
            }
        }
    }
}

/// Default bank amount set to $100
impl Default for Game {
    fn default() -> Self {
        Game::new(100)
    }
}

#[derive(Debug)]
pub enum State {
    Bet,
    Play,
    Results,
}

#[derive(Debug)]
pub enum HandResult {
    Win,
    Bust,
    Lose,
    Draw,
    Blackjack,
    TwentyOne,
}

#[derive(Debug)]
pub enum Command {
    Hit,
    Stand,
    Split,
}

#[derive(Debug)]
pub struct Hand {
    pub cards: Vec<Card>,
    pub bet: u32,
    pub result: Option<HandResult>,
}

impl Hand {
    pub fn splittable(&self) -> bool {
        if self.cards.len() > 2 {
            return false;
        }
        self.cards[0].rank == self.cards[1].rank
    }

    pub fn bet(&self) -> u32 {
        self.bet
    }

    pub fn score(&self) -> u8 {
        calc_hand_score(&self.cards)
    }

    fn new(bet: u32) -> Hand {
        let cards = vec![Card::new(), Card::new()];
        let mut result = None;
        if calc_hand_score(&cards) == BLACKJACK {
            result = Some(HandResult::Blackjack);
        }
        Hand { cards, bet, result }
    }

    fn split(&mut self) -> Hand {
        assert!(self.cards.len() == 2);
        let split_card = self.cards.remove(1);
        self.hit();
        let cards = vec![split_card, Card::new()];
        let bet = self.bet;
        let mut result = None;
        if calc_hand_score(&cards) == BLACKJACK {
            result = Some(HandResult::Blackjack);
        }
        Hand { cards, bet, result }
    }

    fn hit(&mut self) {
        self.cards.push(Card::new());
        if self.score() > BLACKJACK {
            self.result = Some(HandResult::Bust);
            return;
        }
        if self.score() == BLACKJACK {
            self.result = Some(HandResult::TwentyOne);
        }
    }

    fn compare(&mut self, dealer_score: u8) {
        let score = self.score();
        if dealer_score > BLACKJACK || score > dealer_score {
            self.result = Some(HandResult::Win);
        } else if score < dealer_score {
            self.result = Some(HandResult::Lose);
        } else {
            self.result = Some(HandResult::Draw);
        }
    }
}

#[derive(Debug)]
pub struct Dealer {
    pub hand: Vec<Card>,
}

impl Dealer {
    pub fn new() -> Dealer {
        let hand = vec![Card::new(), Card::new()];
        Dealer { hand }
    }

    pub fn run(&mut self) {
        while self.score() < DEALER_STAND {
            self.hand.push(Card::new());
        }
    }

    pub fn showing(&self) -> u8 {
        calc_card_score(&self.hand[1])
    }

    pub fn score(&self) -> u8 {
        calc_hand_score(&self.hand)
    }

    pub fn reset(&mut self) {
        self.hand.clear();
    }
}

impl Default for Dealer {
    fn default() -> Self {
        Self::new()
    }
}

/// Calculate current score of blackjack hand. Aces are scored as 11 unless the total score is
/// above 21, in which case they are scored as 1.
fn calc_hand_score(hand: &[Card]) -> u8 {
    let mut aces = 0;
    let mut score = 0;
    for card in hand {
        if let Rank::Ace = card.rank {
            aces += 1;
        }
        score += calc_card_score(card);
    }

    // Adjust Aces value downward if necessary
    while score > BLACKJACK && aces > 0 {
        score -= ACE_HIGH - ACE_LOW; // note operator precedence
        aces -= 1;
        assert!(score >= 2);
    }
    score
}

fn calc_card_score(card: &Card) -> u8 {
    match card.rank {
        Rank::Ace => ACE_HIGH,
        Rank::Pip(num) => num,
        _ => FACECARD,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn calc_score_test() {
        let jack_of_spades = Card {
            suit: Suit::Spades,
            rank: Rank::Jack,
        };
        let two_of_diamonds = Card {
            suit: Suit::Diamonds,
            rank: Rank::Pip(2),
        };
        let hand = vec![jack_of_spades, two_of_diamonds];
        assert_eq!(calc_hand_score(&hand), 12);

        let ace_of_hearts = Card {
            suit: Suit::Hearts,
            rank: Rank::Ace,
        };
        let king_of_diamonds = Card {
            suit: Suit::Diamonds,
            rank: Rank::King,
        };
        let hand = vec![ace_of_hearts, king_of_diamonds];
        assert_eq!(calc_hand_score(&hand), 21);

        let ace_of_hearts = Card {
            suit: Suit::Hearts,
            rank: Rank::Ace,
        };
        let ace_of_spades = Card {
            suit: Suit::Spades,
            rank: Rank::Ace,
        };
        let hand = vec![ace_of_hearts, ace_of_spades];
        assert_eq!(calc_hand_score(&hand), 12);

        let three_of_hearts = Card {
            suit: Suit::Hearts,
            rank: Rank::Pip(3),
        };
        let four_of_clubs = Card {
            suit: Suit::Hearts,
            rank: Rank::Pip(4),
        };
        let hand = vec![three_of_hearts, four_of_clubs];
        assert_eq!(calc_hand_score(&hand), 7);

        // Ensure scoring logic for aces is working appropriately
        let mut cards: Vec<Card> = Vec::new();
        for _ in 1..13 {
            cards.push(Card {
                suit: Suit::Hearts,
                rank: Rank::Ace,
            })
        }
        assert_eq!(calc_hand_score(&cards), 12);
    }
}
