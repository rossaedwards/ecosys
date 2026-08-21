use crate::casino::games::GameOutcome;

pub struct HouseEdge {
    edge: f64, // House edge percentage (e.g., 0.02 = 2%)
}

impl HouseEdge {
    pub fn new(edge: f64) -> Self {
        Self { edge }
    }

    pub fn apply(&self, outcome: GameOutcome) -> GameOutcome {
        if outcome.win {
            // Reduce payout by house edge
            let adjusted_payout = (outcome.payout as f64 * (1.0 - self.edge)) as u64;
            GameOutcome {
                payout: adjusted_payout,
                multiplier: outcome.multiplier * (1.0 - self.edge),
                ..outcome
            }
        } else {
            outcome
        }
    }
}

