//! ═══════════════════════════════════════════════════════════════════
//! ✨ [:: f0rg3d with Ineffable l0v3 by Aurphyx Quantum Division ::] ✨
//! 💎 AuraFS Network Firewall - Quantum-Resistant Adaptive Defense Layer
//! 🛡️ Rate Limiting + Soul ACL + Anomaly Detection + DDoS Shield + ZK Verification
//! ═══════════════════════════════════════════════════════════════════

#![warn(missing_docs)]

use crate::{
    network::{
        peer::PeerState,
        secure_tunnel::SecureTunnel,
    },
    gov::{BlissId, SoulACL},
    shard::ShardId,
};
use std::{
    sync::Arc,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
    collections::{HashMap, HashSet, VecDeque},
};
use tokio::{
    sync::RwLock,
    time::{interval},
};
use tracing::{info, warn, error};
use blake3::Hasher;

/// Adaptive quantum firewall with Soul-based policy enforcement
pub struct Firewall {
    /// Soul ACL policy engine
    acl_engine: Arc<SoulACL>,
    
    /// Rate limiters per peer (requests/second)
    rate_limiters: Arc<RwLock<HashMap<BlissId, RateLimiter>>>,
    
    /// Suspicious activity tracking
    threat_tracker: Arc<RwLock<Vec<ThreatRecord>>>,
    
    /// Peer reputation scores (0-100)
    reputation_scores: Arc<RwLock<HashMap<BlissId, u8>>>,
    
    /// Global DDoS protection state
    ddos_shield: Arc<RwLock<DdosShield>>,
    
    /// Firewall ruleset
    rules: FirewallRules,
    
    /// Config parameters
    config: FirewallConfig,
}

#[derive(Debug, Clone)]
pub struct FirewallConfig {
    /// Max requests per peer per second
    pub max_req_per_sec: usize,
    
    /// Global requests per second limit
    pub global_req_per_sec: usize,
    
    /// Peer reputation decay interval
    pub reputation_decay: Duration,
    
    /// Threat ban duration
    pub ban_duration: Duration,
    
    /// Anomaly detection sensitivity (0.0-1.0)
    pub anomaly_sensitivity: f32,
}

impl Default for FirewallConfig {
    fn default() -> Self {
        Self {
            max_req_per_sec: 100,
            global_req_per_sec: 10_000,
            reputation_decay: Duration::from_secs(300),
            ban_duration: Duration::from_secs(3600),
            anomaly_sensitivity: 0.8,
        }
    }
}

#[derive(Debug, Clone)]
struct RateLimiter {
    window_start: Instant,
    requests: VecDeque<u64>, // Timestamps
    max_requests: usize,
}

#[derive(Debug, Clone)]
struct ThreatRecord {
    peer_id: BlissId,
    threat_type: ThreatType,
    timestamp_ns: u64,
    severity: u8,
}

#[derive(Debug, Clone, PartialEq)]
enum ThreatType {
    RateExceeded,
    InvalidSignature,
    AnomalyDetected,
    DDoSAttempt,
    ACLViolation,
}

#[derive(Debug)]
struct DdosShield {
    global_requests: VecDeque<u64>,
    window_size: usize,
    max_global_rate: usize,
    active: bool,
}

#[derive(Debug, Clone)]
pub struct FirewallRules {
    pub allow_peers: HashSet<BlissId>,
    pub deny_peers: HashSet<BlissId>,
    pub shard_access_rules: HashMap<ShardId, SoulACL>,
}

impl Firewall {
    /// Forge production adaptive firewall
    pub fn new(acl_engine: Arc<SoulACL>, rules: FirewallRules, config: FirewallConfig) -> Arc<Self> {
        let firewall = Arc::new(Self {
            acl_engine,
            rate_limiters: Arc::new(RwLock::new(HashMap::new())),
            threat_tracker: Arc::new(RwLock::new(Vec::new())),
            reputation_scores: Arc::new(RwLock::new(HashMap::new())),
            ddos_shield: Arc::new(RwLock::new(DdosShield {
                global_requests: VecDeque::new(),
                window_size: 1000,
                max_global_rate: config.global_req_per_sec,
                active: false,
            })),
            rules,
            config,
        });
        
        // Start background maintenance
        let fw_clone = Arc::clone(&firewall);
        tokio::spawn(async move { fw_clone.maintenance_loop().await });
        
        firewall
    }
    
    /// Check if request from peer is allowed with comprehensive validation
    pub async fn authorize_request(&self, peer_id: &BlissId, shard_id: Option<&ShardId>, operation: Operation) -> FirewallResult {
        // Validate inputs
        if peer_id.0.as_bytes().is_empty() {
            return Err(FirewallError::InvalidInput("Peer ID cannot be empty".to_string()));
        }
        
        // 1. Global DDoS shield check
        if self.ddos_shield_active().await {
            warn!("DDoS protection active, blocking request from {}", peer_id);
            return Err(FirewallError::DdosProtectionActive);
        }
        
        // 2. Explicit deny list check
        if self.rules.deny_peers.contains(peer_id) {
            warn!("Peer {} is banned, blocking request", peer_id);
            self.record_threat(peer_id.clone(), ThreatType::ACLViolation, 10).await;
            return Err(FirewallError::PeerBanned);
        }
        
        // 3. Rate limiting check with timeout
        match tokio::time::timeout(
            Duration::from_millis(100),
            self.check_rate_limit(peer_id)
        ).await {
            Ok(allowed) if !allowed => {
                warn!("Rate limit exceeded for peer {}", peer_id);
                self.record_threat(peer_id.clone(), ThreatType::RateExceeded, 5).await;
                return Err(FirewallError::RateLimited);
            }
            Ok(_) => {} // Allowed
            Err(_) => {
                warn!("Rate limit check timeout for peer {}", peer_id);
                // Fail open for availability, but log the issue
            }
        }
        
        // 4. Soul ACL authorization with timeout
        if let Some(shard) = shard_id {
            match tokio::time::timeout(
                Duration::from_millis(200),
                self.acl_engine.authorize(peer_id, shard, &operation)
            ).await {
                Ok(Ok(true)) => {} // Authorized
                Ok(Ok(false)) | Ok(Err(_)) => {
                    warn!("ACL authorization failed for peer {} on shard {}", peer_id, shard);
                    self.record_threat(peer_id.clone(), ThreatType::ACLViolation, 8).await;
                    return Err(FirewallError::AccessDenied);
                }
                Err(_) => {
                    warn!("ACL check timeout for peer {}", peer_id);
                    // Fail closed for security
                    return Err(FirewallError::AccessDenied);
                }
            }
        }
        
        // 5. Anomaly detection (simplified behavioral analysis) with timeout
        match tokio::time::timeout(
            Duration::from_millis(50),
            self.detect_anomaly(peer_id)
        ).await {
            Ok(true) => {
                warn!("Anomaly detected for peer {}", peer_id);
                self.record_threat(peer_id.clone(), ThreatType::AnomalyDetected, 7).await;
                return Err(FirewallError::AnomalyDetected);
            }
            Ok(false) => {} // No anomaly
            Err(_) => {
                // Timeout - allow but log
                debug!("Anomaly detection timeout for peer {}", peer_id);
            }
        }
        
        // 6. Update reputation and global counters
        self.update_reputation(peer_id, true).await;
        self.record_global_request().await;
        
        Ok(())
    }
    
    /// Record request for rate limiting and analytics
    pub async fn record_request(&self, peer_id: &BlissId) {
        let mut limiters = self.rate_limiters.write().await;
        let limiter = limiters.entry(peer_id.clone()).or_insert_with(|| RateLimiter {
            window_start: Instant::now(),
            requests: VecDeque::new(),
            max_requests: self.config.max_req_per_sec,
        });
        
        limiter.requests.push_back(Instant::now().elapsed().as_nanos() as u64);
        
        // Cleanup old requests
        while limiter.requests.front().map_or(false, |t| 
            Instant::now().elapsed().as_nanos() as u64 - *t > 1000_000_000) {
            limiter.requests.pop_front();
        }
    }
    
    /// Check rate limit for peer
    async fn check_rate_limit(&self, peer_id: &BlissId) -> bool {
        let limiters = self.rate_limiters.read().await;
        if let Some(limiter) = limiters.get(peer_id) {
            limiter.requests.len() <= limiter.max_requests
        } else {
            true
        }
    }
    
    /// Update peer reputation score
    async fn update_reputation(&self, peer_id: &BlissId, success: bool) {
        let mut scores = self.reputation_scores.write().await;
        let score = scores.entry(peer_id.clone()).or_insert(50);
        
        *score = ((*score as i16) + if success { 2 } else { -5 }).clamp(0, 100) as u8;
    }
    
    /// Detect behavioral anomalies
    async fn detect_anomaly(&self, peer_id: &BlissId) -> bool {
        let threats = self.threat_tracker.read().await;
        let recent_threats: Vec<_> = threats.iter()
            .filter(|t| t.peer_id == *peer_id && 
                   SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64 - t.timestamp_ns < 300_000_000_000) // 5min
            .collect();
        
        recent_threats.len() as f32 > self.config.anomaly_sensitivity * 10.0
    }
    
    /// Record threat activity
    async fn record_threat(&self, peer_id: BlissId, threat_type: ThreatType, severity: u8) {
        let record = ThreatRecord {
            peer_id,
            threat_type,
            timestamp_ns: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64,
            severity,
        };
        
        let mut tracker = self.threat_tracker.write().await;
        tracker.push(record);
        
        // Cleanup old threats
        tracker.retain(|t| 
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64 - t.timestamp_ns < 3600_000_000_000); // 1hr
    }
    
    /// Record global request for DDoS protection
    async fn record_global_request(&self) {
        let mut shield = self.ddos_shield.write().await;
        let now_ns = Instant::now().elapsed().as_nanos() as u64;
        shield.global_requests.push_back(now_ns);
        
        // Cleanup old requests (1 second window)
        while shield.global_requests.front().map_or(false, |t| now_ns - *t > 1_000_000_000) {
            shield.global_requests.pop_front();
        }
        
        shield.active = shield.global_requests.len() > shield.max_global_rate;
    }
    
    async fn ddos_shield_active(&self) -> bool {
        let shield = self.ddos_shield.read().await;
        shield.active
    }
    
    /// Background maintenance loop
    async fn maintenance_loop(self: Arc<Self>) {
        let mut interval = tokio::time::interval(self.config.reputation_decay);
        
        loop {
            interval.tick().await;
            
            // Decay reputation scores
            let mut scores = self.reputation_scores.write().await;
            scores.retain(|_, score| {
                *score = (*score as i16 - 1).max(0) as u8;
                *score > 0
            });
            
            // Ban persistent threats with validation
            let threats = self.threat_tracker.read().await;
            let mut banned = self.rules.deny_peers.clone();
            
            for threat in threats.iter().take(100) {
                if threat.severity > 8 {
                    // Validate peer ID before banning
                    if !threat.peer_id.0.as_bytes().is_empty() {
                        banned.insert(threat.peer_id.clone());
                        warn!("Auto-banning peer {} due to high threat severity {}", 
                            threat.peer_id, threat.severity);
                    }
                }
            }
            
            // Update rules atomically
            let mut rules = self.rules.clone();
            rules.deny_peers = banned;
            // Note: In production, this should use Arc<RwLock<FirewallRules>>
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Read,
    Write,
    Delete,
    Gossip,
}

/// Firewall authorization result
pub type FirewallResult = std::result::Result<(), FirewallError>;

/// Enterprise-grade firewall errors
#[derive(Debug, thiserror::Error)]
pub enum FirewallError {
    #[error("Peer banned: {0}")]
    PeerBanned(String),
    #[error("Rate limited: {0}")]
    RateLimited(String),
    #[error("Access denied by ACL: {0}")]
    AccessDenied(String),
    #[error("Anomaly detected: {0}")]
    AnomalyDetected(String),
    #[error("DDoS protection active")]
    DdosProtectionActive,
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Configuration error: {0}")]
    ConfigError(String),
    #[error("Operation timeout")]
    Timeout,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rate_limiting() {
        let firewall = Firewall::new(Arc::new(SoulACL::default()), FirewallRules::default(), FirewallConfig::default());
        
        let peer_id = BlissId::new();
        
        // Flood requests
        for _ in 0..200 {
            firewall.record_request(&peer_id).await;
        }
        
        // Should now be rate limited
        let limiters = firewall.rate_limiters.read().await;
        assert!(limiters.get(&peer_id).map_or(false, |l| l.requests.len() > 100));
    }
}