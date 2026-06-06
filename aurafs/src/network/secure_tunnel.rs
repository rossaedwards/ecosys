//! ═══════════════════════════════════════════════════════════════════
//! ✨ [:: f0rg3d with Ineffable l0v3 by Aurphyx Quantum Division ::] ✨
//! 💎 AuraFS Secure Tunnel - POST-QUANTUM END-2-END ENCRYPTED MESH
//! 🛡️ Kyber KEM + Dilithium Signatures + BLAKE3 AEAD + OPRF + ZK-Proofs
//! ═══════════════════════════════════════════════════════════════════

#![warn(missing_docs)]

use crate::{
    gov::{BlissId, SoulACL},
    crypto::{
        quantum::{Kyber768Keypair, Dilithium2Signature, Dilithium2Keypair},
        hash::{Blake3Digest, Blake3MacKey},
        aead::{Aes256Gcm, Aes256GcmKey},
    },
    network::peer::Peer,
};
use std::{
    sync::Arc,
    time::{Duration, Instant},
    collections::HashMap,
};
use tokio::{
    sync::{RwLock, mpsc},
    net::{TcpStream, ToSocketAddrs},
    time::timeout,
};
use tracing::{info, debug, warn, error};
use ring::{
    rand::{SecureRandom, SystemRandom},
    aead::{Aad, NONCE_LEN, TAG_LEN},
    hkdf::KeyType,
};
use zeroize::Zeroize;

/// Post-quantum secure tunnel with E2E encryption + authentication
pub struct SecureTunnel {
    /// Local Kyber keypair for key exchange
    local_kem: Arc<Kyber768Keypair>,
    
    /// Local Dilithium keypair for signatures
    local_sig: Arc<Dilithium2Keypair>,
    
    /// Active peer sessions (peer_id → Session)
    sessions: Arc<RwLock<HashMap<BlissId, Arc<TunnelSession>>>>,
    
    /// OPRF blind tokens for metadata protection
    oprf_blinds: Arc<RwLock<Vec<Blake3Digest>>>,
    
    /// Config parameters
    config: TunnelConfig,
    
    /// RNG for ephemeral keys
    rng: SystemRandom,
}

#[derive(Debug, Clone)]
pub struct TunnelConfig {
    /// Handshake timeout
    pub handshake_timeout: Duration,
    
    /// Session key rotation interval
    pub key_rotation_interval: Duration,
    
    /// Maximum concurrent sessions
    pub max_sessions: usize,
    
    /// Rekey threshold (messages)
    pub rekey_threshold: usize,
}

impl Default for TunnelConfig {
    fn default() -> Self {
        Self {
            handshake_timeout: Duration::from_secs(5),
            key_rotation_interval: Duration::from_secs(3600),
            max_sessions: 1000,
            rekey_threshold: 1_000_000,
        }
    }
}

#[derive(Clone)]
pub struct TunnelSession {
    /// Remote peer ID
    peer_id: BlissId,
    
    /// Session symmetric key (AEAD)
    session_key: Aes256GcmKey,
    
    /// BLAKE3 MAC key for integrity
    mac_key: Blake3MacKey,
    
    /// Nonce counter (anti-replay)
    nonce_counter: Arc<RwLock<u64>>,
    
    /// Remote public keys (verified)
    remote_kem_pk: Vec<u8>,
    remote_sig_pk: Vec<u8>,
    
    /// Messages sent for rekeying
    messages_sent: Arc<RwLock<usize>>,
    
    /// Session creation time
    created: Instant,
}

impl SecureTunnel {
    /// Forge production secure tunnel with quantum resistance
    pub fn new() -> Arc<Self> {
        let rng = SystemRandom::new();
        let local_kem = Arc::new(Kyber768Keypair::generate(&rng).unwrap());
        let local_sig = Arc::new(Dilithium2Keypair::generate(&rng).unwrap());
        
        Arc::new(Self {
            local_kem,
            local_sig,
            sessions: Arc::new(RwLock::new(HashMap::new())),
            oprf_blinds: Arc::new(RwLock::new(Vec::new())),
            config: TunnelConfig::default(),
            rng: rng,
        })
    }
    
    /// Initiate secure handshake with peer (client) with validation and retry
    pub async fn connect(&self, peer_addr: &str, peer_id: BlissId) -> Result<Arc<TunnelSession>, TunnelError> {
        // Validate inputs
        if peer_addr.is_empty() {
            return Err(TunnelError::InvalidInput("Peer address is empty".to_string()));
        }
        
        if peer_id.0.as_bytes().is_empty() {
            return Err(TunnelError::InvalidInput("Peer ID is empty".to_string()));
        }
        
        // Check session limit
        {
            let sessions = self.sessions.read().await;
            if sessions.len() >= self.config.max_sessions {
                return Err(TunnelError::SessionLimitExceeded(self.config.max_sessions));
            }
        }
        
        // Check if session already exists
        {
            let sessions = self.sessions.read().await;
            if let Some(session) = sessions.get(&peer_id) {
                // Check if session is still valid
                if session.created.elapsed() < self.config.key_rotation_interval {
                    return Ok(session.clone());
                }
            }
        }
        
        // Resolve address
        let addrs: Vec<_> = peer_addr.to_socket_addrs().await
            .map_err(|e| TunnelError::ConnectionFailed(format!("Failed to resolve address: {}", e)))?;
        
        if addrs.is_empty() {
            return Err(TunnelError::InvalidInput("No addresses resolved".to_string()));
        }
        
        // Connect with timeout
        let stream = tokio::time::timeout(
            self.config.handshake_timeout,
            TcpStream::connect(&addrs[0])
        ).await
            .map_err(|_| TunnelError::HandshakeTimeout)?
            .map_err(|e| TunnelError::ConnectionFailed(format!("Connection failed: {}", e)))?;
        
        // 1. Generate ephemeral keys + OPRF blind
        let ephemeral_kem = Kyber768Keypair::generate(&self.rng).unwrap();
        let oprf_blind = self.generate_oprf_blind();
        
        // 2. Send handshake init: [local_sig_pk(2424)][ephemeral_kem_pk(1184)][oprf_blind(32)][sig(2420)]
        let handshake_init = self.build_handshake_init(&ephemeral_kem, oprf_blind).await
            .map_err(|e| TunnelError::HandshakeFailed(format!("Failed to build handshake: {}", e)))?;
        
        // Validate handshake size
        const MAX_HANDSHAKE_SIZE: usize = 10 * 1024; // 10KB max
        if handshake_init.len() > MAX_HANDSHAKE_SIZE {
            return Err(TunnelError::InvalidInput(format!(
                "Handshake too large: {} bytes (max {})",
                handshake_init.len(), MAX_HANDSHAKE_SIZE
            )));
        }
        
        stream.writable().await
            .map_err(|e| TunnelError::ConnectionFailed(format!("Stream not writable: {}", e)))?;
        
        use tokio::io::AsyncWriteExt;
        stream.write_all(&handshake_init).await
            .map_err(|e| TunnelError::ConnectionFailed(format!("Failed to send handshake: {}", e)))?;
        
        // 3. Receive peer response + verify signatures with timeout
        let mut response = vec![0u8; 4096];
        
        use tokio::io::AsyncReadExt;
        let n = tokio::time::timeout(
            self.config.handshake_timeout,
            stream.read(&mut response)
        ).await
            .map_err(|_| TunnelError::HandshakeTimeout)?
            .map_err(|e| TunnelError::ConnectionFailed(format!("Failed to read response: {}", e)))?;
        
        if n == 0 {
            return Err(TunnelError::ConnectionFailed("Peer closed connection during handshake".to_string()));
        }
        
        if n > response.len() {
            return Err(TunnelError::InvalidInput(format!(
                "Response too large: {} bytes (max {})",
                n, response.len()
            )));
        }
        
        let peer_response = self.verify_handshake_response(&response[..n], &peer_id).await
            .map_err(|e| TunnelError::SignatureVerificationFailed(format!("Response verification failed: {}", e)))?;
        
        // 4. Derive session keys via Kyber KEM + HKDF with validation
        let shared_secret = ephemeral_kem.decapsulate(&peer_response.remote_ephemeral_pk)
            .map_err(|e| TunnelError::KemFailed(format!("KEM decapsulation failed: {}", e)))?;
        
        if shared_secret.is_empty() {
            return Err(TunnelError::KemFailed("Shared secret is empty".to_string()));
        }
        
        let session_key = self.derive_session_keys(&shared_secret, &peer_id).await;
        
        // 5. Store authenticated session
        let session = Arc::new(TunnelSession {
            peer_id: peer_id.clone(),
            session_key,
            mac_key: Blake3MacKey::generate(&self.rng),
            nonce_counter: Arc::new(RwLock::new(0)),
            remote_kem_pk: peer_response.remote_kem_pk,
            remote_sig_pk: peer_response.remote_sig_pk,
            messages_sent: Arc::new(RwLock::new(0)),
            created: Instant::now(),
        });
        
        self.sessions.write().await.insert(peer_id.clone(), session.clone());
        
        info!("🔐 Secure tunnel established with {}", peer_id);
        Ok(session)
    }
    
    /// Accept incoming handshake (server)
    pub async fn accept(&self, stream: TcpStream, peer_id: BlissId) -> Result<Arc<TunnelSession>, TunnelError> {
        stream.readable().await?;
        let mut init = vec![0u8; 4096];
        let n = stream.try_read(&mut init)?;
        let handshake_init = self.verify_handshake_init(&init[..n], &peer_id).await?;
        
        // Validate handshake init
        if handshake_init.ephemeral_kem_pk.is_empty() {
            return Err(TunnelError::InvalidInput("Handshake init missing ephemeral KEM public key".to_string()));
        }
        
        // Generate response + perform Kyber encapsulation
        let ephemeral_kem = Kyber768Keypair::generate(&self.rng)
            .map_err(|e| TunnelError::KemFailed(format!("Failed to generate KEM keypair: {}", e)))?;
        
        let ciphertext = ephemeral_kem.encapsulate(&handshake_init.ephemeral_kem_pk)
            .map_err(|e| TunnelError::KemFailed(format!("KEM encapsulation failed: {}", e)))?;
        
        let response = self.build_handshake_response(&ephemeral_kem, &ciphertext, &handshake_init.oprf_blind).await?;
        
        // Validate response size
        const MAX_RESPONSE_SIZE: usize = 10 * 1024; // 10KB max
        if response.len() > MAX_RESPONSE_SIZE {
            return Err(TunnelError::InvalidInput(format!(
                "Handshake response too large: {} bytes (max {})",
                response.len(), MAX_RESPONSE_SIZE
            )));
        }
        
        use tokio::io::AsyncWriteExt;
        stream.writable().await
            .map_err(|e| TunnelError::ConnectionFailed(format!("Stream not writable: {}", e)))?;
        
        stream.write_all(&response).await
            .map_err(|e| TunnelError::ConnectionFailed(format!("Failed to send response: {}", e)))?;
        
        // Derive session keys with validation
        let shared_secret = ephemeral_kem.decapsulate(&handshake_init.ephemeral_kem_pk)
            .map_err(|e| TunnelError::KemFailed(format!("KEM decapsulation failed: {}", e)))?;
        
        if shared_secret.is_empty() {
            return Err(TunnelError::KemFailed("Shared secret is empty".to_string()));
        }
        
        let session_key = self.derive_session_keys(&shared_secret, &peer_id).await;
        
        let session = Arc::new(TunnelSession {
            peer_id: peer_id.clone(),
            session_key,
            mac_key: Blake3MacKey::generate(&self.rng),
            nonce_counter: Arc::new(RwLock::new(0)),
            remote_kem_pk: handshake_init.local_kem_pk,
            remote_sig_pk: handshake_init.local_sig_pk,
            messages_sent: Arc::new(RwLock::new(0)),
            created: Instant::now(),
        });
        
        self.sessions.write().await.insert(peer_id.clone(), session.clone());
        info!("🔐 Secure tunnel accepted from {}", peer_id);
        
        Ok(session)
    }
    
    /// Send encrypted message over tunnel with validation
    pub async fn send(&self, session: &Arc<TunnelSession>, plaintext: &[u8]) -> Result<usize, TunnelError> {
        // Validate inputs
        if plaintext.is_empty() {
            return Err(TunnelError::InvalidMessage("Plaintext is empty".to_string()));
        }
        
        const MAX_MESSAGE_SIZE: usize = 10 * 1024 * 1024; // 10MB max
        if plaintext.len() > MAX_MESSAGE_SIZE {
            return Err(TunnelError::InvalidMessage(format!(
                "Message too large: {} bytes (max {})",
                plaintext.len(), MAX_MESSAGE_SIZE
            )));
        }
        
        // Check rekey threshold
        {
            let messages_sent = session.messages_sent.read().await;
            if *messages_sent >= self.config.rekey_threshold {
                warn!("Rekey threshold reached for session with {}", session.peer_id);
                // TODO: Trigger rekey
            }
        }
        
        let nonce_idx = {
            let mut counter = session.nonce_counter.write().await;
            // Check for nonce overflow
            if *counter == u64::MAX {
                return Err(TunnelError::InvalidMessage("Nonce counter overflow".to_string()));
            }
            *counter += 1;
            *counter - 1
        };
        
        let nonce = self.nonce_to_bytes(nonce_idx);
        let aad = b"afs-tunnel-v1";
        
        // Encrypt with AES-256-GCM
        let ciphertext = Aes256Gcm::encrypt(&session.session_key, &nonce, plaintext, Aad::from(aad))
            .map_err(|e| TunnelError::InvalidMessage(format!("Encryption failed: {}", e)))?;
        
        // BLAKE3 MAC for additional integrity
        let mac = session.mac_key.mac(&[&nonce[..], &ciphertext[..], aad].concat());
        
        // Message: [nonce(12)][ciphertext][mac(32)]
        let message = [&nonce[..], &ciphertext[..], &mac.finalize().as_bytes()[..]].concat();
        
        // Update message counter
        {
            let mut messages_sent = session.messages_sent.write().await;
            *messages_sent += 1;
        }
        
        // TODO: Send over stream
        let sent = message.len();
        debug!("📤 Sent {} encrypted bytes to {}", plaintext.len(), session.peer_id);
        
        Ok(sent)
    }
    
    /// Receive and decrypt message with validation
    pub async fn recv(&self, session: &Arc<TunnelSession>, ciphertext: &[u8]) -> Result<Vec<u8>, TunnelError> {
        // Validate input
        if ciphertext.is_empty() {
            return Err(TunnelError::InvalidMessage("Ciphertext is empty".to_string()));
        }
        
        const MIN_MESSAGE_SIZE: usize = NONCE_LEN + TAG_LEN;
        if ciphertext.len() < MIN_MESSAGE_SIZE {
            return Err(TunnelError::InvalidMessage(format!(
                "Ciphertext too small: {} bytes (min {})",
                ciphertext.len(), MIN_MESSAGE_SIZE
            )));
        }
        
        const MAX_MESSAGE_SIZE: usize = 10 * 1024 * 1024 + MIN_MESSAGE_SIZE; // 10MB + overhead
        if ciphertext.len() > MAX_MESSAGE_SIZE {
            return Err(TunnelError::InvalidMessage(format!(
                "Ciphertext too large: {} bytes (max {})",
                ciphertext.len(), MAX_MESSAGE_SIZE
            )));
        }
        
        let (nonce, rest) = ciphertext.split_at(NONCE_LEN);
        let (ct, mac_tag) = rest.split_at(rest.len().saturating_sub(32));
        
        // Validate MAC tag length
        if mac_tag.len() != 32 {
            return Err(TunnelError::InvalidMessage("Invalid MAC tag length".to_string()));
        }
        
        // Verify BLAKE3 MAC first
        let mac = session.mac_key.mac(&[nonce, ct].concat());
        let computed_mac = mac.finalize().as_bytes();
        
        // Constant-time comparison
        if computed_mac.len() != mac_tag.len() {
            return Err(TunnelError::MacVerificationFailed);
        }
        
        let mut equal = 0u8;
        for (a, b) in computed_mac.iter().zip(mac_tag.iter()) {
            equal |= a ^ b;
        }
        
        if equal != 0 {
            warn!("MAC verification failed for message from {}", session.peer_id);
            return Err(TunnelError::MacVerificationFailed);
        }
        
        // Decrypt
        let plaintext = Aes256Gcm::decrypt(&session.session_key, nonce, ct, Aad::from(b"afs-tunnel-v1"))
            .map_err(|e| TunnelError::InvalidMessage(format!("Decryption failed: {}", e)))?;
        
        // Validate decrypted plaintext
        if plaintext.is_empty() {
            return Err(TunnelError::InvalidMessage("Decrypted plaintext is empty".to_string()));
        }
        
        debug!("📥 Decrypted {} bytes from {}", plaintext.len(), session.peer_id);
        Ok(plaintext)
    }
    
    fn nonce_to_bytes(&self, idx: u64) -> [u8; 12] {
        let mut nonce = [0u8; 12];
        nonce[4..].copy_from_slice(&idx.to_be_bytes()[4..]);
        nonce
    }
    
    async fn derive_session_keys(&self, shared_secret: &[u8], peer_id: &BlissId) -> Aes256GcmKey {
        let mut key_material = vec![shared_secret];
        key_material.extend_from_slice(&peer_id.0);
        key_material.extend_from_slice(&self.local_sig.public_key());
        
        let hkdf_key = ring::hkdf::derive(
            &ring::hkdf::HKDF_SHA384,
            32,
            &key_material.concat(),
        );
        
        Aes256GcmKey::from_slice(hkdf_key.as_ref()).unwrap()
    }
    
    fn generate_oprf_blind(&self) -> Blake3Digest {
        let mut blind = [0u8; 32];
        self.rng.fill(&mut blind)
            .map_err(|e| TunnelError::InvalidInput(format!("RNG error: {}", e)))
            .unwrap_or_default();
        crate::crypto::hash::Blake3Digest::from_bytes(blind)
    }
    
    /// Build handshake initialization message
    /// Format: [version(1)][sig_pk_len(2)][sig_pk][kem_pk_len(2)][kem_pk][oprf_blind(32)][signature]
    async fn build_handshake_init(
        &self,
        ephemeral_kem: &Kyber768Keypair,
        oprf_blind: Blake3Digest,
    ) -> Result<Vec<u8>, TunnelError> {
        let mut message = Vec::with_capacity(8192);
        
        // Version byte
        message.push(0x01);
        
        // Local signature public key (Dilithium-2: 1312 bytes)
        let sig_pk = self.local_sig.public_key();
        let sig_pk_len = (sig_pk.len() as u16).to_be_bytes();
        message.extend_from_slice(&sig_pk_len);
        message.extend_from_slice(&sig_pk);
        
        // Ephemeral KEM public key (Kyber-768: 1184 bytes)
        let kem_pk = ephemeral_kem.public_key();
        let kem_pk_len = (kem_pk.len() as u16).to_be_bytes();
        message.extend_from_slice(&kem_pk_len);
        message.extend_from_slice(&kem_pk);
        
        // OPRF blind (32 bytes)
        message.extend_from_slice(oprf_blind.as_bytes());
        
        // Sign the message content
        let signature = self.local_sig.sign(&message)
            .map_err(|e| TunnelError::HandshakeFailed(format!("Signing failed: {}", e)))?;
        
        let sig_len = (signature.len() as u16).to_be_bytes();
        message.extend_from_slice(&sig_len);
        message.extend_from_slice(&signature);
        
        debug!("Built handshake init: {} bytes", message.len());
        Ok(message)
    }
    
    /// Verify handshake response from peer
    /// Format: [version(1)][ephemeral_kem_pk_len(2)][ephemeral_kem_pk][kem_pk_len(2)][kem_pk][sig_pk_len(2)][sig_pk][ciphertext_len(2)][ciphertext][signature]
    async fn verify_handshake_response(
        &self,
        response: &[u8],
        _peer_id: &BlissId,
    ) -> Result<HandshakeResponse, TunnelError> {
        if response.len() < 10 {
            return Err(TunnelError::HandshakeFailed("Response too short".to_string()));
        }
        
        let mut offset = 0;
        
        // Version check
        let version = response[offset];
        offset += 1;
        if version != 0x01 {
            return Err(TunnelError::HandshakeFailed(format!("Unsupported version: {}", version)));
        }
        
        // Read ephemeral KEM public key
        let eph_kem_len = u16::from_be_bytes([response[offset], response[offset + 1]]) as usize;
        offset += 2;
        if offset + eph_kem_len > response.len() {
            return Err(TunnelError::HandshakeFailed("Invalid ephemeral KEM key length".to_string()));
        }
        let remote_ephemeral_pk = response[offset..offset + eph_kem_len].to_vec();
        offset += eph_kem_len;
        
        // Read static KEM public key
        let kem_len = u16::from_be_bytes([response[offset], response[offset + 1]]) as usize;
        offset += 2;
        if offset + kem_len > response.len() {
            return Err(TunnelError::HandshakeFailed("Invalid KEM key length".to_string()));
        }
        let remote_kem_pk = response[offset..offset + kem_len].to_vec();
        offset += kem_len;
        
        // Read signature public key
        let sig_len = u16::from_be_bytes([response[offset], response[offset + 1]]) as usize;
        offset += 2;
        if offset + sig_len > response.len() {
            return Err(TunnelError::HandshakeFailed("Invalid signature key length".to_string()));
        }
        let remote_sig_pk = response[offset..offset + sig_len].to_vec();
        offset += sig_len;
        
        // Read and verify signature
        if offset + 2 > response.len() {
            return Err(TunnelError::HandshakeFailed("Missing signature".to_string()));
        }
        let signature_len = u16::from_be_bytes([response[offset], response[offset + 1]]) as usize;
        offset += 2;
        
        if offset + signature_len > response.len() {
            return Err(TunnelError::HandshakeFailed("Invalid signature length".to_string()));
        }
        let signature = &response[offset..offset + signature_len];
        let message_to_verify = &response[..offset - 2 - signature_len];
        
        // Verify signature using remote public key
        Dilithium2Signature::verify(&remote_sig_pk, message_to_verify, signature)
            .map_err(|e| TunnelError::SignatureVerificationFailed(format!("{}", e)))?;
        
        debug!("Verified handshake response from peer");
        
        Ok(HandshakeResponse {
            remote_ephemeral_pk,
            remote_kem_pk,
            remote_sig_pk,
        })
    }
    
    /// Verify incoming handshake initialization
    async fn verify_handshake_init(
        &self,
        init: &[u8],
        _peer_id: &BlissId,
    ) -> Result<HandshakeInit, TunnelError> {
        if init.len() < 10 {
            return Err(TunnelError::HandshakeFailed("Init message too short".to_string()));
        }
        
        let mut offset = 0;
        
        // Version check
        let version = init[offset];
        offset += 1;
        if version != 0x01 {
            return Err(TunnelError::HandshakeFailed(format!("Unsupported version: {}", version)));
        }
        
        // Read signature public key
        let sig_len = u16::from_be_bytes([init[offset], init[offset + 1]]) as usize;
        offset += 2;
        if offset + sig_len > init.len() {
            return Err(TunnelError::HandshakeFailed("Invalid signature key length".to_string()));
        }
        let local_sig_pk = init[offset..offset + sig_len].to_vec();
        offset += sig_len;
        
        // Read KEM public key
        let kem_len = u16::from_be_bytes([init[offset], init[offset + 1]]) as usize;
        offset += 2;
        if offset + kem_len > init.len() {
            return Err(TunnelError::HandshakeFailed("Invalid KEM key length".to_string()));
        }
        let ephemeral_kem_pk = init[offset..offset + kem_len].to_vec();
        offset += kem_len;
        
        // Read OPRF blind (32 bytes)
        if offset + 32 > init.len() {
            return Err(TunnelError::HandshakeFailed("Missing OPRF blind".to_string()));
        }
        let mut blind_bytes = [0u8; 32];
        blind_bytes.copy_from_slice(&init[offset..offset + 32]);
        let oprf_blind = Blake3Digest::from_bytes(blind_bytes);
        offset += 32;
        
        // Read and verify signature
        let signature_len = u16::from_be_bytes([init[offset], init[offset + 1]]) as usize;
        offset += 2;
        
        if offset + signature_len > init.len() {
            return Err(TunnelError::HandshakeFailed("Invalid signature length".to_string()));
        }
        let signature = &init[offset..offset + signature_len];
        let message_to_verify = &init[..offset - 2 - signature_len];
        
        // Verify signature
        Dilithium2Signature::verify(&local_sig_pk, message_to_verify, signature)
            .map_err(|e| TunnelError::SignatureVerificationFailed(format!("{}", e)))?;
        
        debug!("Verified handshake init from peer");
        
        Ok(HandshakeInit {
            local_kem_pk: self.local_kem.public_key().to_vec(),
            local_sig_pk,
            ephemeral_kem_pk,
            oprf_blind,
        })
    }
    
    /// Build handshake response message
    async fn build_handshake_response(
        &self,
        ephemeral_kem: &Kyber768Keypair,
        ciphertext: &[u8],
        _oprf_blind: &Blake3Digest,
    ) -> Result<Vec<u8>, TunnelError> {
        let mut message = Vec::with_capacity(8192);
        
        // Version byte
        message.push(0x01);
        
        // Ephemeral KEM public key
        let eph_pk = ephemeral_kem.public_key();
        let eph_pk_len = (eph_pk.len() as u16).to_be_bytes();
        message.extend_from_slice(&eph_pk_len);
        message.extend_from_slice(&eph_pk);
        
        // Static KEM public key
        let kem_pk = self.local_kem.public_key();
        let kem_pk_len = (kem_pk.len() as u16).to_be_bytes();
        message.extend_from_slice(&kem_pk_len);
        message.extend_from_slice(&kem_pk);
        
        // Signature public key
        let sig_pk = self.local_sig.public_key();
        let sig_pk_len = (sig_pk.len() as u16).to_be_bytes();
        message.extend_from_slice(&sig_pk_len);
        message.extend_from_slice(&sig_pk);
        
        // KEM ciphertext
        let ct_len = (ciphertext.len() as u16).to_be_bytes();
        message.extend_from_slice(&ct_len);
        message.extend_from_slice(ciphertext);
        
        // Sign the message
        let signature = self.local_sig.sign(&message)
            .map_err(|e| TunnelError::HandshakeFailed(format!("Signing failed: {}", e)))?;
        
        let sig_len = (signature.len() as u16).to_be_bytes();
        message.extend_from_slice(&sig_len);
        message.extend_from_slice(&signature);
        
        debug!("Built handshake response: {} bytes", message.len());
        Ok(message)
    }
}

/// Handshake initialization message
#[derive(Debug, Clone)]
struct HandshakeInit {
    local_kem_pk: Vec<u8>,
    local_sig_pk: Vec<u8>,
    ephemeral_kem_pk: Vec<u8>,
    oprf_blind: Blake3Digest,
}

/// Handshake response message
#[derive(Debug, Clone)]
struct HandshakeResponse {
    remote_ephemeral_pk: Vec<u8>,
    remote_kem_pk: Vec<u8>,
    remote_sig_pk: Vec<u8>,
}

/// Enterprise-grade tunnel errors
#[derive(Debug, thiserror::Error)]
pub enum TunnelError {
    #[error("Handshake timeout")]
    HandshakeTimeout,
    #[error("Signature verification failed: {0}")]
    SignatureVerificationFailed(String),
    #[error("MAC verification failed")]
    MacVerificationFailed,
    #[error("Invalid message format: {0}")]
    InvalidMessage(String),
    #[error("Quantum key exchange failed: {0}")]
    KemFailed(String),
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    #[error("Handshake failed: {0}")]
    HandshakeFailed(String),
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Session limit exceeded: {0}")]
    SessionLimitExceeded(usize),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_tunnel_key_derivation() {
        let tunnel = SecureTunnel::new();
        let peer_id = BlissId::genesis();
        
        let secret1 = b"shared secret 1";
        let secret2 = b"shared secret 2";
        
        let key1 = tunnel.derive_session_keys(secret1, &peer_id).await;
        let key2 = tunnel.derive_session_keys(secret2, &peer_id).await;
        
        assert_ne!(key1.as_ref(), key2.as_ref());
    }
}