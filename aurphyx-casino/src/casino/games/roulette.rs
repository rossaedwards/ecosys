use crate::prelude::*;
use crate::casino::games::{Game, GameOutcome};
use async_trait::async_trait;

pub struct RouletteTable {
    variant: RouletteVariant,
}

#[derive(Debug, Clone, Copy)]
pub enum RouletteVariant {
    American,  // 38 numbers (0, 00, 1-36)
    European,  // 37 numbers (0, 1-36)
}

impl RouletteTable {
    pub fn new(variant: RouletteVariant) -> Self {
        Self { variant }
    }
}

#[async_trait]
impl Game for RouletteTable {
    type Outcome = GameOutcome;

    fn min_bet(&self) -> u64 {
        1000
    }

    fn max_bet(&self) -> u64 {
        10_000_000_000
    }

    fn name(&self) -> &str {
        match self.variant {
            RouletteVariant::American => "American Roulette",
            RouletteVariant::European => "European Roulette",
        }
    }

    async fn play(&mut self, bet: u64, randomness: Vec<u8>) -> Result<Self::Outcome> {
        let max_number = match self.variant {
            RouletteVariant::American => 38,
            RouletteVariant::European => 37,
        };

        let winning_number = (randomness[0] as usize) % max_number;
        
        // Simplified: bet on number 0 wins (1/37 or 1/38 chance)
        let win = winning_number == 0;
        let multiplier = if win {
            match self.variant {
                RouletteVariant::American => 36.0,
                RouletteVariant::European => 37.0,
            }
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
                "winning_number": winning_number,
                "variant": format!("{:?}", self.variant),
            }),
        })
    }
}

