use crate::prelude::*;
use crate::casino::games::{Game, GameOutcome};
use async_trait::async_trait;

pub struct SlotMachine {
    reels: usize,
    symbols_per_reel: usize,
    paylines: Vec<Vec<usize>>,
}

impl SlotMachine {
    pub fn new(reels: usize, symbols_per_reel: usize) -> Self {
        Self {
            reels,
            symbols_per_reel,
            paylines: vec![vec![0, 1, 2]], // Simple payline
        }
    }
}

#[async_trait::async_trait]
impl Game for SlotMachine {
    type Outcome = GameOutcome;

    fn min_bet(&self) -> u64 {
        1000 // 0.001 tokens
    }

    fn max_bet(&self) -> u64 {
        1_000_000_000 // 1000 tokens
    }

    fn name(&self) -> &str {
        "Video Slots"
    }

    async fn play(&mut self, bet: u64, randomness: Vec<u8>) -> Result<Self::Outcome> {
        // Generate reel positions from randomness
        let mut positions = Vec::new();
        for i in 0..self.reels {
            let idx = (randomness[i % randomness.len()] as usize) % self.symbols_per_reel;
            positions.push(idx);
        }

        // Check paylines for wins
        let mut max_multiplier = 0.0;
        for payline in &self.paylines {
            let symbols: Vec<usize> = payline.iter().map(|&pos| positions[pos]).collect();
            if symbols.iter().all(|&s| s == symbols[0]) {
                // All symbols match - calculate multiplier based on symbol
                max_multiplier = max_multiplier.max(2.0 * (symbols[0] as f64 + 1.0));
            }
        }

        let win = max_multiplier > 0.0;
        let payout = if win {
            (bet as f64 * max_multiplier) as u64
        } else {
            0
        };

        Ok(GameOutcome {
            win,
            payout,
            multiplier: max_multiplier,
            game_data: serde_json::json!({
                "reels": positions,
                "paylines": self.paylines,
            }),
        })
    }
}

