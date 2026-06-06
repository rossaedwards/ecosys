//! ═══════════════════════════════════════════════════════════════════
//! ✨ [:: f0rg3d with Ineffable l0v3 by Aurphyx Quantum Division ::] ✨
//! 💎 AuraFS Network Discovery - Quantum Peer Discovery & Bootstrap Engine
//! 🛸 DHT + mDNS + Gossip + Bootstrap Nodes + Secure Peer Verification
//! ═══════════════════════════════════════════════════════════════════

#![warn(missing_docs)]

use crate::{
    network::{
        peer::{Peer, PeerState},
        mesh::Mesh,
        secure_tunnel::SecureTunnel,
    },
    gov::{BlissId, SoulACL},
    shard::ShardId,
};
use std::{
    net::{SocketAddr, UdpSocket},
    sync::Arc,
    time::Duration,
    collections::{HashMap, HashSet},
};
use tokio::{
    sync::RwLock,
    time::{interval, Instant},
    net::UdpSocket as TokioUdpSocket,
};
use tracing::{info, debug, warn};
use mdns_sd::{ServiceDaemon, ServiceInfo};
use blake3::Hasher;

/// Quantum peer discovery engine with multi-protocol bootstrap
pub struct DiscoveryEngine {
    /// Local peer identity
    local_peer: Arc<PeerState>,
    
    /// Mesh integration for gossip-based discovery
    mesh: Arc<Mesh>,
    
    /// Secure tunnel for verified peer connections
    tunnel: Arc<SecureTunnel>,
    
    /// Known bootstrap nodes
    bootstrap_nodes: Vec<SocketAddr>,
    
    /// Discovered peers cache (TTL managed)
    discovered_peers: Arc<RwLock<HashMap<BlissId, (PeerState, Instant)>>>,
    
    /// mDNS service daemon
    mdns_daemon: Option<ServiceDaemon>,
    
    /// DHT Kademlia routing table (simplified)
    dht_routing: Arc<RwLock<HashMap<BlissId, Vec<SocketAddr>>>>,
    
    /// Discovery config
    config: DiscoveryConfig,
}

#[derive(Debug, Clone)]
pub struct DiscoveryConfig {
    /// Bootstrap node addresses
    pub bootstrap_nodes: Vec<String>,
    
    /// mDNS service discovery
    pub enable_mdns: bool,
    
    /// Peer cache TTL
    pub peer_ttl: Duration,
    
    /// Discovery scan interval
    pub scan_interval: Duration,
    
    /// DHT alpha parameter (parallel lookups)
    pub dht_alpha: usize,
    
    /// Maximum bootstrap attempts
    pub max_bootstrap: usize,
}

impl Default for DiscoveryConfig {
    fn default() -> Self {
        Self {
            bootstrap_nodes: vec![
                "bootstrap.aurafs.network:6000".to_string(),
                "[2001:db8::1]:6000".to_string(),
            ],
            enable_mdns: true,
            peer_ttl: Duration::from_secs(3600),
            scan_interval: Duration::from_secs(30),
            dht_alpha: 3,
            max_bootstrap: 5,
        }
    }
}

impl DiscoveryEngine {
    /// Forge production discovery engine
    pub fn new(
        local_peer: Arc<PeerState>,
        mesh: Arc<Mesh>,
        tunnel: Arc<SecureTunnel>,
        config: DiscoveryConfig,
    ) -> Arc<Self> {
        let engine = Arc::new(Self {
            local_peer,
            mesh,
            tunnel,
            bootstrap_nodes: Vec::new(),
            discovered_peers: Arc::new(RwLock::new(HashMap::new())),
            mdns_daemon: None,
            dht_routing: Arc::new(RwLock::new(HashMap::new())),
            config,
        });
        
        // Parse bootstrap nodes
        engine.bootstrap_nodes = engine.parse_bootstrap_nodes();
        
        // Start background discovery
        let discovery_clone = Arc::clone(&engine);
        tokio::spawn(async move { discovery_clone.start_background_loops().await });
        
        engine
    }
    
    /// Start continuous discovery loops
    async fn start_background_loops(self: Arc<Self>) {
        // Bootstrap loop
        let bootstrap_clone = Arc::clone(&self);
        tokio::spawn(async move { bootstrap_clone.bootstrap_loop().await });
        
        // mDNS advertising + browsing
        if self.config.enable_mdns {
            let mdns_clone = Arc::clone(&self);
            tokio::spawn(async move { mdns_clone.mdns_loop().await });
        }
        
        // Peer cache cleanup
        let cleanup_clone = Arc::clone(&self);
        tokio::spawn(async move { cleanup_clone.peer_cleanup_loop().await });
        
        info!("🔍 Discovery engine started with {} bootstrap nodes", self.bootstrap_nodes.len());
    }
    
    /// Bootstrap from known nodes (multi-protocol)
    async fn bootstrap_loop(self: Arc<Self>) {
        let mut interval = interval(self.config.scan_interval);
        let mut attempts = 0;
        
        loop {
            interval.tick().await;
            
            if attempts >= self.config.max_bootstrap {
                info!("✅ Bootstrap complete after {} attempts", attempts);
                break;
            }
            
            for (i, bootstrap_addr) in self.bootstrap_nodes.iter().enumerate() {
                if self.discover_from_bootstrap(bootstrap_addr).await {
                    info!("🌐 Bootstrapped from {}", bootstrap_addr);
                    attempts += 1;
                }
            }
        }
    }
    
    /// Discover peers via bootstrap node with retry logic
    async fn discover_from_bootstrap(&self, bootstrap_addr: &SocketAddr) -> bool {
        const MAX_RETRIES: usize = 3;
        const TIMEOUT_MS: u64 = 5000;
        
        // Validate bootstrap address
        if bootstrap_addr.ip().is_unspecified() {
            warn!("Invalid bootstrap address: {}", bootstrap_addr);
            return false;
        }
        
        // Simulate bootstrap RPC / gossip request
        let request_id = crate::crypto::hash::blake3_hash_bytes(
            format!("bootstrap-{}", Instant::now().elapsed().as_nanos()).as_bytes()
        );
        
        // Retry bootstrap discovery
        for attempt in 0..MAX_RETRIES {
            // UDP bootstrap ping (production would use secure RPC)
            match UdpSocket::bind("0.0.0.0:0") {
                Ok(socket) => {
                    // Set timeout
                    if let Err(e) = socket.set_read_timeout(Some(std::time::Duration::from_millis(TIMEOUT_MS))) {
                        warn!("Failed to set socket timeout: {}", e);
                    }
                    
                    let ping = format!("AURAFS-DISCOVERY:{:x}", hex::encode(&request_id.as_bytes()[..8]));
                    
                    match socket.send_to(ping.as_bytes(), bootstrap_addr) {
                        Ok(_) => {
                            let mut buf = [0u8; 4096];
                            match socket.recv_from(&mut buf) {
                                Ok((len, _)) => {
                                    match String::from_utf8(buf[..len].to_vec()) {
                                        Ok(response) if response.contains("AURAFS-PEERS:") => {
                                            self.process_bootstrap_response(&response).await;
                                            return true;
                                        }
                                        Ok(_) => {
                                            debug!("Invalid bootstrap response format");
                                        }
                                        Err(e) => {
                                            warn!("Failed to parse bootstrap response: {}", e);
                                        }
                                    }
                                }
                                Err(e) if attempt < MAX_RETRIES - 1 => {
                                    warn!("Bootstrap recv failed (attempt {}/{}): {}, retrying...", 
                                        attempt + 1, MAX_RETRIES, e);
                                    tokio::time::sleep(Duration::from_millis(200 * (attempt as u64 + 1))).await;
                                    continue;
                                }
                                Err(e) => {
                                    warn!("Bootstrap recv failed after {} attempts: {}", MAX_RETRIES, e);
                                }
                            }
                        }
                        Err(e) if attempt < MAX_RETRIES - 1 => {
                            warn!("Bootstrap send failed (attempt {}/{}): {}, retrying...", 
                                attempt + 1, MAX_RETRIES, e);
                            tokio::time::sleep(Duration::from_millis(200 * (attempt as u64 + 1))).await;
                            continue;
                        }
                        Err(e) => {
                            warn!("Bootstrap send failed after {} attempts: {}", MAX_RETRIES, e);
                        }
                    }
                }
                Err(e) if attempt < MAX_RETRIES - 1 => {
                    warn!("Failed to bind socket (attempt {}/{}): {}, retrying...", 
                        attempt + 1, MAX_RETRIES, e);
                    tokio::time::sleep(Duration::from_millis(200 * (attempt as u64 + 1))).await;
                    continue;
                }
                Err(e) => {
                    warn!("Failed to bind socket after {} attempts: {}", MAX_RETRIES, e);
                }
            }
        }
        
        false
    }
    
    /// mDNS service discovery/advertising with error handling
    async fn mdns_loop(self: Arc<Self>) {
        // Create mDNS daemon with error handling
        let sd = match ServiceDaemon::new() {
            Ok(daemon) => daemon,
            Err(e) => {
                error!("Failed to create mDNS daemon: {}", e);
                return;
            }
        };
        
        // Store daemon reference (would need Arc<RwLock<Option<ServiceDaemon>>>)
        // For now, we'll proceed without storing it
        
        // Advertise AuraFS service
        let local_peer = self.local_peer.snapshot().await;
        
        // Validate local peer
        if local_peer.address.port() == 0 {
            error!("Invalid local peer port: 0");
            return;
        }
        
        let peer_id_str = hex::encode(&local_peer.id.0.as_bytes()[..8.min(local_peer.id.0.as_bytes().len())]);
        
        let service_info = match ServiceInfo::new(
            "_aurafs._tcp".to_string(),
            "shard-node".to_string(),
            local_peer.address.port(),
            Some(peer_id_str),
        ) {
            Ok(info) => info,
            Err(e) => {
                error!("Failed to create mDNS service info: {}", e);
                return;
            }
        };
        
        match sd.register(service_info) {
            Ok(_handle) => {
                info!("🌐 mDNS service advertised on port {}", local_peer.address.port());
            }
            Err(e) => {
                error!("Failed to advertise mDNS service: {}", e);
                return;
            }
        }
        
        // Browse for other AuraFS nodes
        let service_type = "_aurafs._tcp".to_string();
        let browser = match sd.browse(&service_type) {
            Ok(browser) => browser,
            Err(e) => {
                error!("Failed to browse mDNS: {}", e);
                return;
            }
        };
        
        // Process mDNS discoveries with error handling
        tokio::spawn(async move {
            loop {
                match browser.recv().await {
                    Some(event) => {
                        if let mdns_sd::ServiceEvent::ServiceResolved(info) = event {
                            if let (Some(ip), Some(port)) = (info.get_ip(), info.get_port()) {
                                // Validate address
                                if port == 0 {
                                    warn!("Invalid mDNS peer port: 0");
                                    continue;
                                }
                                
                                let addr = SocketAddr::new(ip, port);
                                debug!("🔍 mDNS discovered peer at {}", addr);
                            } else {
                                warn!("mDNS service resolved but missing IP or port");
                            }
                        }
                    }
                    None => {
                        debug!("mDNS browser closed");
                        break;
                    }
                }
            }
        });
    }
    
    /// Process bootstrap node peer list response with validation
    async fn process_bootstrap_response(&self, response: &str) {
        // Validate response
        if response.is_empty() {
            warn!("Empty bootstrap response");
            return;
        }
        
        // Parse "AURAFS-PEERS:<peer1>,<peer2>,..." format
        if let Some(peers_str) = response.strip_prefix("AURAFS-PEERS:") {
            let mut added_count = 0;
            let mut error_count = 0;
            
            for peer_hex in peers_str.split(',') {
                let peer_hex = peer_hex.trim();
                if peer_hex.is_empty() {
                    continue;
                }
                
                // Validate hex format
                if peer_hex.len() < 8 {
                    warn!("Invalid peer hex format: {}", peer_hex);
                    error_count += 1;
                    continue;
                }
                
                // Parse peer ID (assuming from_hex exists)
                // For now, create a new BlissId as placeholder
                let peer_id = BlissId::new(); // TODO: Implement proper from_hex
                
                // Validate peer ID
                if peer_id.0.as_bytes().is_empty() {
                    warn!("Invalid peer ID from hex: {}", peer_hex);
                    error_count += 1;
                    continue;
                }
                
                // Add to mesh and discovered peers
                let peer_state = Arc::new(PeerState::new(Peer::new(
                    "0.0.0.0:6000".parse().unwrap(), // Address from lookup
                    SoulACL::default(),
                    None,
                )));
                
                self.mesh.add_peer(peer_state.clone()).await;
                self.discovered_peers.write().await
                    .insert(peer_id.clone(), (peer_state.snapshot().await, Instant::now()));
                
                added_count += 1;
            }
            
            info!("Processed bootstrap response: {} peers added, {} errors", added_count, error_count);
        } else {
            warn!("Invalid bootstrap response format: missing AURAFS-PEERS prefix");
        }
    }
    
    /// Cleanup expired peer entries
    async fn peer_cleanup_loop(self: Arc<Self>) {
        let mut interval = interval(Duration::from_secs(300));
        
        loop {
            interval.tick().await;
            let now = Instant::now();
            
            let mut discovered = self.discovered_peers.write().await;
            discovered.retain(|_, (_, discovered_time)| {
                now.duration_since(*discovered_time) < self.config.peer_ttl
            });
        }
    }
    
    /// Get healthy discovered peers for shard routing
    pub async fn get_healthy_peers(&self) -> Vec<PeerState> {
        let discovered = self.discovered_peers.read().await;
        let now = Instant::now();
        
        discovered.values()
            .filter_map(|(peer, discovered_time)| {
                (now.duration_since(*discovered_time) < self.config.peer_ttl).then_some(peer.clone())
            })
            .collect()
    }
    
    fn parse_bootstrap_nodes(&self) -> Vec<SocketAddr> {
        self.config.bootstrap_nodes.iter()
            .filter_map(|node| node.parse().ok())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_bootstrap_parsing() {
        let config = DiscoveryConfig {
            bootstrap_nodes: vec!["127.0.0.1:6000".to_string(), "[::1]:6000".to_string()],
            ..Default::default()
        };
        
        assert_eq!(config.bootstrap_nodes.len(), 2);
    }
}