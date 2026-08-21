use crate::prelude::*;
use crate::casino::games::{Game, GameOutcome};
use async_trait::async_trait;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rank {
    Ace,
    Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten,
    Jack, Queen, King,
}

#[derive(Debug, Clone)]
pub struct Card {
    suit: Suit,
    rank: Rank,
}

impl Card {
    pub fn value(&self) -> u8 {
        match self.rank {
            Rank::Ace => 11, // Can be 1 or 11
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten | Rank::Jack | Rank::Queen | Rank::King => 10,
        }
    }
}

pub struct BlackjackTable {
    deck: Vec<Card>,
    dealer_hand: Vec<Card>,
    player_hand: Vec<Card>,
}

impl BlackjackTable {
    pub fn new() -> Self {
        Self {
            deck: Self::create_deck(),
            dealer_hand: Vec::new(),
            player_hand: Vec::new(),
        }
    }

    fn create_deck() -> Vec<Card> {
        let mut deck = Vec::new();
        let suits = [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades];
        let ranks = [
            Rank::Ace, Rank::Two, Rank::Three, Rank::Four, Rank::Five,
            Rank::Six, Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten,
            Rank::Jack, Rank::Queen, Rank::King,
        ];

        for suit in suits {
            for rank in ranks {
                deck.push(Card { suit, rank });
            }
        }
        deck
    }

    fn hand_value(hand: &[Card]) -> u8 {
        let mut value = 0;
        let mut aces = 0;

        for card in hand {
            if card.rank == Rank::Ace {
                aces += 1;
            }
            value += card.value();
        }

        // Adjust for aces
        while value > 21 && aces > 0 {
            value -= 10;
            aces -= 1;
        }

        value
    }
}

#[async_trait::async_trait]
impl Game for BlackjackTable {
    type Outcome = GameOutcome;

    fn min_bet(&self) -> u64 {
        1000
    }

    fn max_bet(&self) -> u64 {
        10_000_000_000 // 10,000 tokens
    }

    fn name(&self) -> &str {
        "Blackjack"
    }

    async fn play(&mut self, bet: u64, randomness: Vec<u8>) -> Result<Self::Outcome> {
        // Shuffle deck using randomness
        // Simplified: use randomness to select cards
        
        // Deal initial cards
        self.player_hand.clear();
        self.dealer_hand.clear();
        
        // Deal player two cards
        for i in 0..2 {
            let idx = (randomness[i] as usize) % self.deck.len();
            self.player_hand.push(self.deck[idx].clone());
        }
        
        // Deal dealer one card (face up)
        let idx = (randomness[2] as usize) % self.deck.len();
        self.dealer_hand.push(self.deck[idx].clone());
        
        // Dealer draws until 17+
        let mut dealer_idx = 3;
        while Self::hand_value(&self.dealer_hand) < 17 && dealer_idx < randomness.len() {
            let idx = (randomness[dealer_idx] as usize) % self.deck.len();
            self.dealer_hand.push(self.deck[idx].clone());
            dealer_idx += 1;
        }

        let player_value = Self::hand_value(&self.player_hand);
        let dealer_value = Self::hand_value(&self.dealer_hand);

        let (win, multiplier) = if player_value > 21 {
            (false, 0.0) // Player bust
        } else if dealer_value > 21 {
            (true, 2.0) // Dealer bust
        } else if player_value > dealer_value {
            (true, 2.0) // Player wins
        } else if player_value == dealer_value {
            (false, 1.0) // Push
        } else {
            (false, 0.0) // Dealer wins
        };

        let payout = if win {
            (bet as f64 * multiplier) as u64
        } else if multiplier == 1.0 {
            bet // Push - return bet
        } else {
            0
        };

        Ok(GameOutcome {
            win,
            payout,
            multiplier,
            game_data: serde_json::json!({
                "player_hand": self.player_hand.len(),
                "dealer_hand": self.dealer_hand.len(),
                "player_value": player_value,
                "dealer_value": dealer_value,
            }),
        })
    }
}

