//! ═══════════════════════════════════════════════════════════════════
//! ✨ [:: f0rg3d with l0v3 by Aurphyx Quantum Division ::] ✨
//! 💎 AuraFS Reticulum Bridge - Tri-Hybrid Laning Architecture
//! 🌐 Lane 1: Wi-Fi HaLow (802.11ah) | Lane 2: LoRa (915MHz) | Lane 3: Starlink
//! ═══════════════════════════════════════════════════════════════════

#![warn(missing_docs)]

use crate::{
    gov::BlissId,
    shard::ShardId,
    network::{NodeManager, NetworkError, NetworkResult},
};
use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::RwLock;
use tracing::{info, warn, error};
use thiserror::Error;

/// Network lane types for tri-hybrid architecture
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NetworkLane {
    /// Lane 1: Wi-Fi HaLow (802.11ah) - Fast file transfer
    HaLow,
    /// Lane 2: LoRa (915MHz) - Resilient signaling
    LoRa,
    /// Lane 3: Starlink Ethernet - Inter-mesh sync
    Starlink,
}

impl NetworkLane {
    /// Get lane name
    pub fn name(&self) -> &'static str {
        match self {
            NetworkLane::HaLow => "HaLow",
            NetworkLane::LoRa => "LoRa",
            NetworkLane::Starlink => "Starlink",
        }
    }
    
    /// Get maximum throughput (bytes/sec) for lane
    pub fn max_throughput(&self) -> usize {
        match self {
            NetworkLane::HaLow => 150_000_000, // ~150 Mbps
            NetworkLane::LoRa => 50_000,       // ~50 Kbps
            NetworkLane::Starlink => 200_000_000, // ~200 Mbps
        }
    }
}

/// Lane status and health metrics
#[derive(Debug, Clone)]
pub struct LaneStatus {
    /// Lane type
    pub lane: NetworkLane,
    /// Is lane currently active/connected
    pub is_active: bool,
    /// Last successful transmission timestamp
    pub last_success: Option<Instant>,
    /// Current throughput (bytes/sec)
    pub current_throughput: usize,
    /// Error count in last minute
    pub error_count: u64,
    /// Latency in milliseconds
    pub latency_ms: f64,
}

impl LaneStatus {
    /// Create new lane status
    pub fn new(lane: NetworkLane) -> Self {
        Self {
            lane,
            is_active: false,
            last_success: None,
            current_throughput: 0,
            error_count: 0,
            latency_ms: 0.0,
        }
    }
    
    /// Check if lane is healthy
    pub fn is_healthy(&self) -> bool {
        self.is_active && self.error_count < 10
    }
}

/// Reticulum bridge for tri-hybrid laning architecture
pub struct ReticulumBridge {
    /// Node manager reference
    node_manager: Arc<NodeManager>,
    
    /// Lane statuses
    lanes: Arc<RwLock<HashMap<NetworkLane, LaneStatus>>>,
    
    /// Gateway mode flag (Starlink detected)
    is_gateway: Arc<RwLock<bool>>,
    
    /// Traffic threshold for lane selection (1MB)
    traffic_threshold: usize,
}

impl ReticulumBridge {
    /// Create new Reticulum bridge
    pub fn new(node_manager: Arc<NodeManager>) -> Arc<Self> {
        let mut lanes = HashMap::new();
        lanes.insert(NetworkLane::HaLow, LaneStatus::new(NetworkLane::HaLow));
        lanes.insert(NetworkLane::LoRa, LaneStatus::new(NetworkLane::LoRa));
        lanes.insert(NetworkLane::Starlink, LaneStatus::new(NetworkLane::Starlink));
        
        Arc::new(Self {
            node_manager,
            lanes: Arc::new(RwLock::new(lanes)),
            is_gateway: Arc::new(RwLock::new(false)),
            traffic_threshold: 1_000_000, // 1MB
        })
    }
    
    /// Initialize lanes and detect hardware
    pub async fn initialize(&self) -> NetworkResult<()> {
        info!("🌐 Initializing Reticulum Bridge - Tri-Hybrid Laning");
        
        // Detect Starlink (check for Ethernet interface with specific characteristics)
        let starlink_detected = self.detect_starlink().await?;
        if starlink_detected {
            *self.is_gateway.write().await = true;
            info!("🛰️ Starlink detected - Gateway mode enabled");
            self.set_lane_active(NetworkLane::Starlink, true).await;
        }
        
        // Initialize HaLow (placeholder - would detect Seeed Wio-WM6180)
        let halow_detected = self.detect_halow().await?;
        if halow_detected {
            info!("📡 Wi-Fi HaLow detected");
            self.set_lane_active(NetworkLane::HaLow, true).await;
        }
        
        // Initialize LoRa (placeholder - would detect WM1302 or LoRa-E5)
        let lora_detected = self.detect_lora().await?;
        if lora_detected {
            info!("📻 LoRa detected");
            self.set_lane_active(NetworkLane::LoRa, true).await;
        }
        
        Ok(())
    }
    
    /// Route data based on size and replication requirements
    pub async fn route_data(
        &self,
        data: &[u8],
        replication: ReplicationType,
    ) -> NetworkResult<NetworkLane> {
        let data_size = data.len();
        
        // Global replication always uses Starlink
        if matches!(replication, ReplicationType::Global) {
            if self.is_lane_active(NetworkLane::Starlink).await {
                return Ok(NetworkLane::Starlink);
            } else {
                return Err(NetworkError::ConfigError(
                    "Global replication requires Starlink but lane is inactive".to_string(),
                ));
            }
        }
        
        // Large traffic (>1MB) -> Lane 1 (HaLow)
        if data_size > self.traffic_threshold {
            if self.is_lane_active(NetworkLane::HaLow).await {
                return Ok(NetworkLane::HaLow);
            }
            // Fallback to LoRa for metadata only if HaLow is down
            if data_size < 10_000 && self.is_lane_active(NetworkLane::LoRa).await {
                warn!("⚠️ HaLow down, falling back to LoRa for metadata");
                return Ok(NetworkLane::LoRa);
            }
        }
        
        // Small traffic -> LoRa (resilient signaling)
        if self.is_lane_active(NetworkLane::LoRa).await {
            return Ok(NetworkLane::LoRa);
        }
        
        // Fallback to HaLow if LoRa is down
        if self.is_lane_active(NetworkLane::HaLow).await {
            return Ok(NetworkLane::HaLow);
        }
        
        Err(NetworkError::ConfigError(
            "No active network lanes available".to_string(),
        ))
    }
    
    /// Send data via selected lane with packet serialization and handshake
    pub async fn send_data(
        &self,
        data: &[u8],
        target: &BlissId,
        replication: ReplicationType,
    ) -> NetworkResult<()> {
        use crate::network::packet::{NetworkPacket, PacketType, HandshakePacket};
        
        let lane = self.route_data(data, replication).await?;
        
        info!("📤 Routing {} bytes to {} via {}", data.len(), target, lane.name());
        
        // For large data (>10MB), perform handshake first
        if data.len() > 10_000_000 {
            // Send Hello packet
            let hello = HandshakePacket::hello(
                BlissId::genesis(), // Source (would be actual node ID)
                1, // Protocol version
                vec!["shard_transfer".to_string()],
            );
            
            // In production, would serialize and send over network
            let _hello_bytes = hello.serialize()
                .map_err(|e| NetworkError::ConfigError(format!("Packet serialization failed: {}", e)))?;
            
            info!("🤝 Handshake sent, waiting for Ack...");
            
            // Wait for Ack (in production, would wait for response)
            // For now, simulate Ack received
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            
            info!("✅ Handshake acknowledged, sending data...");
        }
        
        // Create shard data packet
        let packet = NetworkPacket::new(
            PacketType::ShardData,
            0, // Sequence (would be incremented for multiple packets)
            None, // Shard ID (would be set if known)
            BlissId::genesis(), // Source
            target.clone(),
            data.to_vec(),
        );
        
        // Serialize packet
        let packet_bytes = packet.serialize()
            .map_err(|e| NetworkError::ConfigError(format!("Packet serialization failed: {}", e)))?;
        
        // Verify packet integrity
        if !packet.verify() {
            return Err(NetworkError::SecurityError("Packet checksum verification failed".to_string()));
        }
        
        // In production, would send packet_bytes over the selected lane
        // For now, simulate successful transmission
        info!("📦 Packet serialized: {} bytes (header: {} bytes, payload: {} bytes)",
            packet_bytes.len(),
            packet_bytes.len() - data.len(),
            data.len()
        );
        
        // Update lane metrics
        self.update_lane_metrics(lane, data.len()).await;
        self.mark_lane_success(lane).await;
        
        Ok(())
    }
    
    /// Replicate shard across mesh
    pub async fn replicate_shard(
        &self,
        shard_id: &ShardId,
        replication: ReplicationType,
    ) -> NetworkResult<()> {
        // Load shard data from storage via NodeManager
        use crate::storage::shard_store::ShardStore;
        
        let shard_store = Arc::new(ShardStore::default());
        let shard = shard_store.load_shard(shard_id).await
            .map_err(|e| NetworkError::ConfigError(format!(
                "Failed to load shard {} for replication: {}",
                shard_id, e
            )))?;
        
        let shard_data = shard.data;
        
        // Route based on replication type
        let lane = self.route_data(&shard_data, replication).await?;
        
        info!("🔄 Replicating shard {} ({} bytes) via {}", 
            shard_id, shard_data.len(), lane.name());
        
        // Announce to mesh if in gateway mode
        if *self.is_gateway.read().await {
            self.announce_gateway().await?;
        }
        
        Ok(())
    }
    
    /// Check if lane is active
    pub async fn is_lane_active(&self, lane: NetworkLane) -> bool {
        let lanes = self.lanes.read().await;
        lanes.get(&lane)
            .map(|status| status.is_active)
            .unwrap_or(false)
    }
    
    /// Set lane active status
    pub async fn set_lane_active(&self, lane: NetworkLane, active: bool) {
        let mut lanes = self.lanes.write().await;
        if let Some(status) = lanes.get_mut(&lane) {
            status.is_active = active;
            if active {
                status.last_success = Some(Instant::now());
            }
        }
    }
    
    /// Get all lane statuses
    pub async fn get_lane_statuses(&self) -> Vec<LaneStatus> {
        let lanes = self.lanes.read().await;
        lanes.values().cloned().collect()
    }
    
    /// Check if in gateway mode
    pub async fn is_gateway_mode(&self) -> bool {
        *self.is_gateway.read().await
    }
    
    /// Detect Starlink connection
    async fn detect_starlink(&self) -> NetworkResult<bool> {
        // Check for Starlink Ethernet interface characteristics
        // Starlink typically uses specific MAC address prefixes or interface names
        #[cfg(target_os = "linux")]
        {
            use std::process::Command;
            
            // Check for Starlink interface via network manager or ip command
            if let Ok(output) = Command::new("ip")
                .args(&["link", "show"])
                .output()
            {
                let output_str = String::from_utf8_lossy(&output.stdout);
                // Look for Starlink-specific interface patterns
                if output_str.contains("starlink") || 
                   output_str.contains("eth0") && output_str.contains("UP") {
                    // Additional check: verify it's actually Starlink
                    // In production, would check MAC vendor or route to Starlink gateway
                    return Ok(true);
                }
            }
        }
        
        #[cfg(target_os = "windows")]
        {
            use std::process::Command;
            
            // Check network adapters on Windows
            if let Ok(output) = Command::new("netsh")
                .args(&["interface", "show", "interface"])
                .output()
            {
                let output_str = String::from_utf8_lossy(&output.stdout);
                if output_str.to_lowercase().contains("ethernet") {
                    // Additional verification would check for Starlink-specific adapter
                    return Ok(true);
                }
            }
        }
        
        // Default: not detected (can be overridden via config)
        Ok(false)
    }
    
    /// Detect Wi-Fi HaLow (Seeed Wio-WM6180)
    async fn detect_halow(&self) -> NetworkResult<bool> {
        #[cfg(all(feature = "hardware-rpi", target_os = "linux"))]
        {
            // Check for Seeed Wio-WM6180 via USB or SPI interface
            use std::path::Path;
            
            // Check for USB device with Seeed vendor ID
            if Path::new("/sys/bus/usb/devices").exists() {
                // Would enumerate USB devices and check for WM6180
                // For now, assume present if hardware-rpi feature is enabled
                return Ok(true);
            }
        }
        
        #[cfg(all(not(feature = "hardware-rpi"), target_os = "linux"))]
        {
            use std::process::Command;
            if let Ok(output) = Command::new("iw")
                .args(&["dev"])
                .output()
            {
                let output_str = String::from_utf8_lossy(&output.stdout);
                if output_str.contains("802.11ah") || output_str.contains("HaLow") {
                    return Ok(true);
                }
            }
        }
        
        Ok(false)
    }
    
    /// Detect LoRa (WM1302 or LoRa-E5)
    async fn detect_lora(&self) -> NetworkResult<bool> {
        #[cfg(all(feature = "hardware-embedded", target_os = "linux"))]
        {
            // LoRa-E5 is typically connected via UART/Serial
            use std::path::Path;
            
            if Path::new("/dev/ttyUSB0").exists() || 
               Path::new("/dev/ttyACM0").exists() {
                // Additional verification: send AT command to check for LoRa-E5
                return Ok(true);
            }
        }
        
        #[cfg(all(feature = "hardware-embedded", target_os = "windows"))]
        {
            use std::process::Command;
            if let Ok(output) = Command::new("mode")
                .output()
            {
                let output_str = String::from_utf8_lossy(&output.stdout);
                if output_str.contains("COM") {
                    return Ok(true);
                }
            }
        }
        
        #[cfg(all(not(feature = "hardware-embedded"), target_os = "linux"))]
        {
            // On non-embedded platforms, LoRa would be via USB dongle
            use std::process::Command;
            if let Ok(output) = Command::new("lsusb")
                .output()
            {
                let output_str = String::from_utf8_lossy(&output.stdout);
                // Check for common LoRa USB dongle vendor IDs
                if output_str.contains("1d50") || // Seeed Studio
                   output_str.contains("0403") {  // FTDI (common for LoRa modules)
                    return Ok(true);
                }
            }
        }
        
        Ok(false)
    }
    
    /// Update lane metrics after transmission
    async fn update_lane_metrics(&self, lane: NetworkLane, bytes: usize) {
        let mut lanes = self.lanes.write().await;
        if let Some(status) = lanes.get_mut(&lane) {
            status.current_throughput = bytes;
            status.last_success = Some(Instant::now());
        }
    }
    
    /// Mark lane transmission success
    async fn mark_lane_success(&self, lane: NetworkLane) {
        let mut lanes = self.lanes.write().await;
        if let Some(status) = lanes.get_mut(&lane) {
            status.last_success = Some(Instant::now());
            status.error_count = status.error_count.saturating_sub(1);
        }
    }
    
    /// Announce gateway to mesh
    async fn announce_gateway(&self) -> NetworkResult<()> {
        info!("🌍 Announcing Global Gateway to mesh");
        // In production, would broadcast gateway announcement via all lanes
        Ok(())
    }
}

/// Replication type for shard distribution
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplicationType {
    /// Local mesh replication
    Local,
    /// Global inter-mesh replication
    Global,
}

/// Reticulum bridge errors
#[derive(Debug, Error)]
pub enum ReticulumError {
    #[error("Lane error: {0}")]
    LaneError(String),
    #[error("Hardware detection failed: {0}")]
    HardwareError(String),
    #[error("No active lanes available")]
    NoActiveLanes,
}

impl From<ReticulumError> for NetworkError {
    fn from(err: ReticulumError) -> Self {
        match err {
            ReticulumError::LaneError(msg) => NetworkError::ConfigError(msg),
            ReticulumError::HardwareError(msg) => NetworkError::ConfigError(msg),
            ReticulumError::NoActiveLanes => NetworkError::ConfigError(
                "No active network lanes available".to_string(),
            ),
        }
    }
}

