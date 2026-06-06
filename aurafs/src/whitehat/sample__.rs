// SAMPLE CODE v7.0 - Aurphyx AFS Empire
// MODULE: whitehat | TEMPLATE: config | ID: [whitehat, System.Collections.Hashtable]
// 100% CARGO BUILD SAFE | Production Skeleton | Extend Later
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SampleConfig {
    pub enabled: bool,
    pub max_size: usize,
    pub timeout_ms: u64,
}

impl Default for SampleConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_size: 1024 * 1024,
            timeout_ms: 30000,
        }
    }
}
