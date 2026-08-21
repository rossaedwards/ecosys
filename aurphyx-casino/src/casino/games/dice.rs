use crate::prelude::*;
use crate::casino::games::{Game, GameOutcome};
use async_trait::async_trait;

pub struct ProvablyFairDice {
    sides: u8,
}

impl ProvablyFairDice {
    pub fn new(sides: u8) -> Self {
        Self { sides }
    }
}

#[async_trait]
impl Game for ProvablyFairDice {
    type Outcome = GameOutcome;

    fn min_bet(&self) -> u64 {
        1000
    }

    fn max_bet(&self) -> u64 {
        1_000_000_000
    }

    fn name(&self) -> &str {
        "Provably Fair Dice"
    }

    async fn play(&mut self, bet: u64, randomness: Vec<u8>) -> Result<Self::Outcome> {
        let roll = (randomness[0] as u8 % self.sides) + 1;
        
        // Win if roll is >= 50 (for 100-sided die)
        let win = if self.sides == 100 {
            roll >= 50
        } else {
            roll > self.sides / 2
        };

        let multiplier = if win {
            2.0
        } else {
            0.0
        };

        let payout = if win {
            (bet as f64 * multiplier) as u64
        } else {
            0
        };

        Ok(GameOutcome {
            win,
            payout,
            multiplier,
            game_data: serde_json::json!({
                "roll": roll,
                "sides": self.sides,
            }),
        })
    }
}

