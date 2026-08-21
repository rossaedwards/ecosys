use serde::{Deserialize, Serialize};

pub struct AurphyxCasinoSDK {
    base_url: String,
    client: reqwest::Client,
}

#[derive(Serialize, Deserialize)]
pub struct GameResult {
    pub win: bool,
    pub payout: u64,
    pub multiplier: f64,
}

impl AurphyxCasinoSDK {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: reqwest::Client::new(),
        }
    }

    pub async fn play_game(&self, game_id: &str, bet: u64) -> Result<GameResult, reqwest::Error> {
        let response = self
            .client
            .post(&format!("{}/casino/play", self.base_url))
            .json(&serde_json::json!({
                "gameId": game_id,
                "bet": bet,
            }))
            .send()
            .await?;

        response.json().await
    }

    pub async fn get_balance(&self) -> Result<u64, reqwest::Error> {
        let response = self
            .client
            .get(&format!("{}/wallet/balance", self.base_url))
            .send()
            .await?;

        let json: serde_json::Value = response.json().await?;
        Ok(json["balance"].as_u64().unwrap_or(0))
    }
}

