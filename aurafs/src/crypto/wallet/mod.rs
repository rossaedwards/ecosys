//! Wallet and Key Management
//! f0rg3d in l0v3 by Ross Edwards & Aurphyx 💎

pub mod backup_manager;
pub mod hd_wallet;
pub mod multi_sig;
pub mod node_shards;
pub mod recovery_sharding;
pub mod shard_vault;
pub mod signing_engine;
pub mod vault_storage;

pub use shard_vault::ShardVault;
pub use signing_engine::SigningEngine;
