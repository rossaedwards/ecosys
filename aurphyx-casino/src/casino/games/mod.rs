pub mod slots;
pub mod blackjack;
pub mod roulette;
pub mod poker;
pub mod baccarat;
pub mod craps;
pub mod dice;
pub mod crash;
pub mod plinko;
pub mod mines;
pub mod wheel;
pub mod keno;

use crate::prelude::*;
use async_trait::async_trait;

/// Trait for all casino games
#[async_trait]
pub trait Game: Send + Sync {
    type Outcome: Send + Sync;
    
    fn min_bet(&self) -> u64;
    fn max_bet(&self) -> u64;
    fn name(&self) -> &str;
    
    async fn play(&mut self, bet: u64, randomness: Vec<u8>) -> Result<Self::Outcome>;
}

#[derive(Debug, Clone)]
pub struct GameOutcome {
    pub win: bool,
    pub payout: u64,
    pub multiplier: f64,
    pub game_data: serde_json::Value,
}

