use crate::prelude::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RngAuditEntry {
    pub timestamp: DateTime<Utc>,
    pub source: String,
    pub randomness: Vec<u8>,
    pub game_id: Option<String>,
    pub hash: String,
}

pub struct AuditTrail {
    entries: Vec<RngAuditEntry>,
}

impl AuditTrail {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn log(&mut self, entry: RngAuditEntry) {
        self.entries.push(entry);
    }

    pub fn get_entries(&self) -> &[RngAuditEntry] {
        &self.entries
    }
}

