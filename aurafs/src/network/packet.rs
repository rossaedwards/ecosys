//! ═══════════════════════════════════════════════════════════════════
//! ✨ [:: f0rg3d with l0v3 by Aurphyx Quantum Division ::] ✨
//! 💎 AuraFS Network Packet Protocol - Production Packet Serialization
//! 📡 Magic Bytes + Shard ID + Payload + Handshake Support
//! ═══════════════════════════════════════════════════════════════════

#![warn(missing_docs)]

use crate::{
    shard::ShardId,
    gov::BlissId,
    crypto::hash::{Hash, hash},
};
use serde::{Serialize, Deserialize};
use std::io::{self, Read, Write};
use thiserror::Error;

/// Magic bytes for AuraFS packet identification
pub const AURAFS_MAGIC: [u8; 4] = [0x41, 0x55, 0x52, 0x41]; // "AURA"

/// Packet types for tri-hybrid network
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PacketType {
    /// Handshake initiation
    Hello = 0x01,
    /// Handshake acknowledgment
    Ack = 0x02,
    /// Shard data packet
    ShardData = 0x03,
    /// Heartbeat packet
    Heartbeat = 0x04,
    /// Error packet
    Error = 0x05,
}

/// Production packet header structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PacketHeader {
    /// Magic bytes for packet identification
    pub magic: [u8; 4],
    /// Packet type
    pub packet_type: PacketType,
    /// Packet sequence number
    pub sequence: u64,
    /// Shard ID (if applicable)
    pub shard_id: Option<ShardId>,
    /// Source BlissId
    pub source: BlissId,
    /// Destination BlissId
    pub destination: BlissId,
    /// Payload size in bytes
    pub payload_size: u64,
    /// Checksum of payload
    pub checksum: Hash,
}

/// Complete network packet
#[derive(Debug, Clone)]
pub struct NetworkPacket {
    /// Packet header
    pub header: PacketHeader,
    /// Payload data
    pub payload: Vec<u8>,
}

impl NetworkPacket {
    /// Create new packet
    pub fn new(
        packet_type: PacketType,
        sequence: u64,
        shard_id: Option<ShardId>,
        source: BlissId,
        destination: BlissId,
        payload: Vec<u8>,
    ) -> Self {
        let checksum = hash(&payload);
        
        let header = PacketHeader {
            magic: AURAFS_MAGIC,
            packet_type,
            sequence,
            shard_id,
            source,
            destination,
            payload_size: payload.len() as u64,
            checksum,
        };
        
        Self { header, payload }
    }
    
    /// Serialize packet to bytes
    pub fn serialize(&self) -> Result<Vec<u8>, PacketError> {
        use byteorder::{BigEndian, WriteBytesExt};
        
        let mut buffer = Vec::new();
        
        // Write magic bytes
        buffer.write_all(&self.header.magic)?;
        
        // Write packet type
        buffer.write_u8(self.header.packet_type as u8)?;
        
        // Write sequence
        buffer.write_u64::<BigEndian>(self.header.sequence)?;
        
        // Write shard ID (if present)
        if let Some(ref shard_id) = self.header.shard_id {
            buffer.write_u8(1)?; // Present flag
            buffer.write_all(&shard_id.0.as_bytes())?;
        } else {
            buffer.write_u8(0)?; // Not present
        }
        
        // Write source BlissId (64 hex chars)
        let source_hex = self.header.source.to_hex();
        let source_bytes = source_hex.as_bytes();
        if source_bytes.len() != 64 {
            return Err(PacketError::Serialization(
                format!("Invalid BlissId hex length: expected 64, got {}", source_bytes.len())
            ));
        }
        buffer.write_all(source_bytes)?;
        
        // Write destination BlissId (64 hex chars)
        let dest_hex = self.header.destination.to_hex();
        let dest_bytes = dest_hex.as_bytes();
        if dest_bytes.len() != 64 {
            return Err(PacketError::Serialization(
                format!("Invalid BlissId hex length: expected 64, got {}", dest_bytes.len())
            ));
        }
        buffer.write_all(dest_bytes)?;
        
        // Write payload size
        buffer.write_u64::<BigEndian>(self.header.payload_size)?;
        
        // Write checksum
        buffer.write_all(self.header.checksum.as_bytes())?;
        
        // Write payload
        buffer.write_all(&self.payload)?;
        
        Ok(buffer)
    }
    
    /// Deserialize packet from bytes
    pub fn deserialize(data: &[u8]) -> Result<Self, PacketError> {
        use byteorder::{BigEndian, ReadBytesExt};
        use std::io::Cursor;
        
        let mut cursor = Cursor::new(data);
        
        // Read magic bytes
        let mut magic = [0u8; 4];
        cursor.read_exact(&mut magic)?;
        if magic != AURAFS_MAGIC {
            return Err(PacketError::InvalidMagic);
        }
        
        // Read packet type
        let packet_type_byte = cursor.read_u8()?;
        let packet_type = match packet_type_byte {
            0x01 => PacketType::Hello,
            0x02 => PacketType::Ack,
            0x03 => PacketType::ShardData,
            0x04 => PacketType::Heartbeat,
            0x05 => PacketType::Error,
            _ => return Err(PacketError::InvalidPacketType(packet_type_byte)),
        };
        
        // Read sequence
        let sequence = cursor.read_u64::<BigEndian>()?;
        
        // Read shard ID
        let shard_id = if cursor.read_u8()? == 1 {
            let mut shard_bytes = vec![0u8; 32]; // ShardId is typically 32 bytes
            cursor.read_exact(&mut shard_bytes)?;
            Some(ShardId::from_content(&shard_bytes))
        } else {
            None
        };
        
        // Read source BlissId (64 hex chars = 32 bytes when decoded)
        let mut source_hex = vec![0u8; 64]; // BlissId hex string length
        cursor.read_exact(&mut source_hex)?;
        let source_str = String::from_utf8_lossy(&source_hex);
        let source = BlissId::from_hex(&source_str)
            .map_err(|_| PacketError::InvalidBlissId)?;
        
        // Read destination BlissId
        let mut dest_hex = vec![0u8; 64];
        cursor.read_exact(&mut dest_hex)?;
        let dest_str = String::from_utf8_lossy(&dest_hex);
        let destination = BlissId::from_hex(&dest_str)
            .map_err(|_| PacketError::InvalidBlissId)?;
        
        // Read payload size
        let payload_size = cursor.read_u64::<BigEndian>()?;
        
        // Read checksum
        let mut checksum_bytes = [0u8; 32]; // Hash size
        cursor.read_exact(&mut checksum_bytes)?;
        let checksum = Hash::from_bytes(checksum_bytes)
            .map_err(|_| PacketError::InvalidChecksum)?;
        
        // Read payload
        let mut payload = vec![0u8; payload_size as usize];
        cursor.read_exact(&mut payload)?;
        
        // Verify checksum
        let computed_checksum = hash(&payload);
        if computed_checksum.as_bytes() != checksum.as_bytes() {
            return Err(PacketError::ChecksumMismatch);
        }
        
        let header = PacketHeader {
            magic,
            packet_type,
            sequence,
            shard_id,
            source,
            destination,
            payload_size,
            checksum,
        };
        
        Ok(Self { header, payload })
    }
    
    /// Verify packet integrity
    pub fn verify(&self) -> bool {
        let computed_checksum = hash(&self.payload);
        computed_checksum.as_bytes() == self.header.checksum.as_bytes()
    }
}

/// Handshake packet for connection establishment
#[derive(Debug, Clone)]
pub struct HandshakePacket {
    /// Node's BlissId
    pub node_id: BlissId,
    /// Protocol version
    pub version: u8,
    /// Supported features
    pub features: Vec<String>,
}

impl HandshakePacket {
    /// Create Hello packet
    pub fn hello(node_id: BlissId, version: u8, features: Vec<String>) -> NetworkPacket {
        let payload = serde_json::to_vec(&(version, features)).unwrap_or_default();
        NetworkPacket::new(
            PacketType::Hello,
            0,
            None,
            node_id.clone(),
            BlissId::genesis(), // Broadcast
            payload,
        )
    }
    
    /// Create Ack packet
    pub fn ack(node_id: BlissId, version: u8) -> NetworkPacket {
        let payload = vec![version];
        NetworkPacket::new(
            PacketType::Ack,
            0,
            None,
            node_id.clone(),
            BlissId::genesis(), // Broadcast
            payload,
        )
    }
}

/// Packet errors
#[derive(Debug, Error)]
pub enum PacketError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Invalid magic bytes")]
    InvalidMagic,
    #[error("Invalid packet type: {0}")]
    InvalidPacketType(u8),
    #[error("Invalid BlissId")]
    InvalidBlissId,
    #[error("Invalid checksum")]
    InvalidChecksum,
    #[error("Checksum mismatch")]
    ChecksumMismatch,
    #[error("Serialization error: {0}")]
    Serialization(String),
}

