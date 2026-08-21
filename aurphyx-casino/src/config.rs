use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub bind_address: String,
    pub database_url: String,
    pub redis_url: String,
    pub jwt_secret: String,
    pub quantum_rng_url: Option<String>,
    pub chainlink_vrf_key: Option<String>,
    pub blockchain: BlockchainConfig,
    pub casino: CasinoConfig,
    pub sportsbook: SportsbookConfig,
    pub shardenomics: ShardenomicsConfig,
    pub meshtastic: MeshtasticConfig,
    pub sages: SagesConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainConfig {
    pub ethereum_rpc: Option<String>,
    pub solana_rpc: Option<String>,
    pub polygon_rpc: Option<String>,
    pub ineffable_ledger_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CasinoConfig {
    pub house_edge: f64,
    pub min_bet: u64,
    pub max_bet: u64,
    pub jackpot_contribution_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SportsbookConfig {
    pub ml_model_path: Option<PathBuf>,
    pub odds_update_interval_secs: u64,
    pub max_liability: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShardenomicsConfig {
    pub token_address: Option<String>,
    pub bonus_multiplier: f64,
    pub staking_apy: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshtasticConfig {
    pub enabled: bool,
    pub channel: u32,
    pub encryption_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SagesConfig {
    pub api_url: Option<String>,
    pub api_key: Option<String>,
    pub enabled: bool,
}

impl Config {
    pub fn load() -> crate::Result<Self> {
        let config_path = std::env::var("CONFIG_PATH")
            .unwrap_or_else(|_| "config/development.toml".to_string());
        
        let config_str = std::fs::read_to_string(&config_path)
            .map_err(|e| crate::Error::Config(format!("Failed to read config: {}", e)))?;
        
        let mut config: Config = toml::from_str(&config_str)
            .map_err(|e| crate::Error::Config(format!("Failed to parse config: {}", e)))?;
        
        // Override with environment variables
        if let Ok(addr) = std::env::var("BIND_ADDRESS") {
            config.bind_address = addr;
        }
        if let Ok(url) = std::env::var("DATABASE_URL") {
            config.database_url = url;
        }
        if let Ok(url) = std::env::var("REDIS_URL") {
            config.redis_url = url;
        }
        
        Ok(config)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            bind_address: "0.0.0.0:8080".to_string(),
            database_url: "postgresql://aurphyx:changeme@localhost:5432/aurphyx_casino".to_string(),
            redis_url: "redis://localhost:6379".to_string(),
            jwt_secret: "changeme-in-production".to_string(),
            quantum_rng_url: None,
            chainlink_vrf_key: None,
            blockchain: BlockchainConfig {
                ethereum_rpc: None,
                solana_rpc: None,
                polygon_rpc: None,
                ineffable_ledger_url: None,
            },
            casino: CasinoConfig {
                house_edge: 0.02, // 2% house edge
                min_bet: 1000, // 0.001 tokens (assuming 6 decimals)
                max_bet: 1_000_000_000, // 1000 tokens
                jackpot_contribution_rate: 0.01, // 1% to jackpot
            },
            sportsbook: SportsbookConfig {
                ml_model_path: None,
                odds_update_interval_secs: 60,
                max_liability: 10_000_000_000, // 10,000 tokens
            },
            shardenomics: ShardenomicsConfig {
                token_address: None,
                bonus_multiplier: 1.5, // 50% bonus for early users
                staking_apy: 0.12, // 12% APY
            },
            meshtastic: MeshtasticConfig {
                enabled: false,
                channel: 0,
                encryption_key: None,
            },
            sages: SagesConfig {
                api_url: None,
                api_key: None,
                enabled: false,
            },
        }
    }
}

