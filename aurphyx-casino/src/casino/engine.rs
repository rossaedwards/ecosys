use crate::prelude::*;
use crate::casino::games::Game;
use crate::casino::quantum_rng::QuantumRng;
use crate::casino::house_edge::HouseEdge;
use async_trait::async_trait;

pub struct CasinoEngine {
    rng: QuantumRng,
    house_edge: HouseEdge,
}

impl CasinoEngine {
    pub fn new(rng: QuantumRng, house_edge: HouseEdge) -> Self {
        Self { rng, house_edge }
    }

    pub async fn play_game<G: Game>(&self, game: &mut G, bet: u64) -> Result<G::Outcome> {
        // Validate bet amount
        if bet < game.min_bet() || bet > game.max_bet() {
            return Err(Error::Game(format!(
                "Bet amount {} is outside valid range [{}, {}]",
                bet,
                game.min_bet(),
                game.max_bet()
            )));
        }

        // Generate random outcome using quantum RNG
        let randomness = self.rng.generate().await?;
        
        // Play the game
        let outcome = game.play(bet, randomness).await?;
        
        // Apply house edge
        let adjusted_outcome = self.house_edge.apply(outcome);
        
        Ok(adjusted_outcome)
    }
}

