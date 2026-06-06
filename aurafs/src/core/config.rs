//! ═══════════════════════════════════════════════════════════════════
//! ⚙️ AuraFS Core Config - Enterprise Configuration Management
//! ✨ f0rg3d with Ineffable l0v3 by Aurphyx Quantum Division ✨
//! Centralized configuration with validation, hot-reload support,
//! environment variable overrides, and enterprise defaults.
//! ═══════════════════════════════════════════════════════════════════

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use notify::{Watcher, RecommendedWatcher, RecursiveMode, Event};

use crate::core::{Result, AuraFSError, ErrorCode, ErrorPhase, internal, client};

/// Default configuration values
pub mod defaults {
    use std::time::Duration;
    
    pub const MAX_SHARD_SIZE_BYTES: usize = 100 * 1024 * 1024; // 100MB
    pub const MAX_BIOMETRIC_SIZE_BYTES: usize = 1024 * 1024; // 1MB
    pub const DEFAULT_REPLICATION_FACTOR: u64 = 3;
    pub const DEFAULT_TIMEOUT_SECS: u64 = 30;
    pub const CRYPTO_TIMEOUT_SECS: u64 = 5;
    pub const MAX_RETRIES: usize = 3;
    pub const RETRY_BASE_DELAY_MS: u64 = 10;
    pub const HEALTH_CHECK_INTERVAL_SECS: u64 = 30;
    pub const METRICS_FLUSH_INTERVAL_SECS: u64 = 10;
    pub const MAX_CONCURRENT_OPERATIONS: usize = 1000;
    pub const CACHE_TTL_SECS: u64 = 300; // 5 minutes
    pub const MAX_CHILDREN_PER_SHARD: usize = 1_000_000;
    pub const MAX_BLISSID_RECORDS: usize = 10_000_000;
    pub const PROOF_EXPIRATION_MAX_HOURS: u64 = 365 * 24; // 1 year
    pub const CIRCUIT_BREAKER_THRESHOLD: u32 = 5;
    pub const CIRCUIT_BREAKER_TIMEOUT_SECS: u64 = 60;
    pub const RATE_LIMIT_WINDOW_SECS: u64 = 60;
    pub const RATE_LIMIT_MAX_REQUESTS: u32 = 1000;
    
    pub fn default_timeout() -> Duration {
        Duration::from_secs(DEFAULT_TIMEOUT_SECS)
    }
    
    pub fn crypto_timeout() -> Duration {
        Duration::from_secs(CRYPTO_TIMEOUT_SECS)
    }
}

/// Cryptographic configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoConfig {
    /// Maximum message size for signing (bytes)
    pub max_sign_message_size: usize,
    /// Maximum hash input size (bytes)
    pub max_hash_input_size: usize,
    /// Timeout for cryptographic operations
    pub operation_timeout_secs: u64,
    /// Number of retries for entropy operations
    pub entropy_retries: usize,
    /// Enable constant-time operations where applicable
    pub constant_time_enabled: bool,
}

impl Default for CryptoConfig {
    fn default() -> Self {
        Self {
            max_sign_message_size: 10 * 1024 * 1024, // 10MB
            max_hash_input_size: 100 * 1024 * 1024, // 100MB
            operation_timeout_secs: defaults::CRYPTO_TIMEOUT_SECS,
            entropy_retries: defaults::MAX_RETRIES,
            constant_time_enabled: true,
        }
    }
}

/// Shard configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShardConfig {
    /// Maximum shard size (bytes)
    pub max_size_bytes: usize,
    /// Default replication factor
    pub default_replication_factor: u64,
    /// Maximum replication factor
    pub max_replication_factor: u64,
    /// Maximum children per shard (fractal hierarchy)
    pub max_children_per_shard: usize,
    /// Enable checksum verification on load
    pub verify_checksums: bool,
    /// Enable Merkle proof generation
    pub enable_merkle_proofs: bool,
}

impl Default for ShardConfig {
    fn default() -> Self {
        Self {
            max_size_bytes: defaults::MAX_SHARD_SIZE_BYTES,
            default_replication_factor: defaults::DEFAULT_REPLICATION_FACTOR,
            max_replication_factor: 1000,
            max_children_per_shard: defaults::MAX_CHILDREN_PER_SHARD,
            verify_checksums: true,
            enable_merkle_proofs: true,
        }
    }
}

/// Identity/BlissID configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityConfig {
    /// Maximum biometric data size (bytes)
    pub max_biometric_size: usize,
    /// Maximum records in memory cache
    pub max_cached_records: usize,
    /// Proof expiration maximum (hours)
    pub max_proof_expiration_hours: u64,
    /// Enable signature verification on registration
    pub verify_signatures: bool,
    /// Persistence backend type
    pub persistence_backend: PersistenceBackend,
    /// Database path (for SQLite/RocksDB)
    pub database_path: Option<PathBuf>,
}

impl Default for IdentityConfig {
    fn default() -> Self {
        Self {
            max_biometric_size: defaults::MAX_BIOMETRIC_SIZE_BYTES,
            max_cached_records: defaults::MAX_BLISSID_RECORDS,
            max_proof_expiration_hours: defaults::PROOF_EXPIRATION_MAX_HOURS,
            verify_signatures: true,
            persistence_backend: PersistenceBackend::InMemory,
            database_path: None,
        }
    }
}

/// Persistence backend types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PersistenceBackend {
    /// In-memory (development/testing)
    InMemory,
    /// SQLite database
    SQLite,
    /// RocksDB (high-performance)
    RocksDB,
    /// PostgreSQL (enterprise)
    PostgreSQL,
}

/// Resilience configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResilienceConfig {
    /// Maximum retry attempts
    pub max_retries: usize,
    /// Base delay between retries (ms)
    pub retry_base_delay_ms: u64,
    /// Maximum delay between retries (ms)
    pub retry_max_delay_ms: u64,
    /// Circuit breaker failure threshold
    pub circuit_breaker_threshold: u32,
    /// Circuit breaker timeout (seconds)
    pub circuit_breaker_timeout_secs: u64,
    /// Enable circuit breaker
    pub circuit_breaker_enabled: bool,
}

impl Default for ResilienceConfig {
    fn default() -> Self {
        Self {
            max_retries: defaults::MAX_RETRIES,
            retry_base_delay_ms: defaults::RETRY_BASE_DELAY_MS,
            retry_max_delay_ms: 5000,
            circuit_breaker_threshold: defaults::CIRCUIT_BREAKER_THRESHOLD,
            circuit_breaker_timeout_secs: defaults::CIRCUIT_BREAKER_TIMEOUT_SECS,
            circuit_breaker_enabled: true,
        }
    }
}

/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// Rate limit window (seconds)
    pub window_secs: u64,
    /// Maximum requests per window
    pub max_requests_per_window: u32,
    /// Enable rate limiting
    pub enabled: bool,
    /// Per-soul rate limiting
    pub per_soul_enabled: bool,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            window_secs: defaults::RATE_LIMIT_WINDOW_SECS,
            max_requests_per_window: defaults::RATE_LIMIT_MAX_REQUESTS,
            enabled: true,
            per_soul_enabled: true,
        }
    }
}

/// Observability configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityConfig {
    /// Enable metrics collection
    pub metrics_enabled: bool,
    /// Metrics flush interval (seconds)
    pub metrics_flush_interval_secs: u64,
    /// Enable distributed tracing
    pub tracing_enabled: bool,
    /// Tracing sample rate (0.0 - 1.0)
    pub tracing_sample_rate: f64,
    /// OpenTelemetry endpoint
    pub otlp_endpoint: Option<String>,
    /// Enable audit logging
    pub audit_logging_enabled: bool,
    /// Health check interval (seconds)
    pub health_check_interval_secs: u64,
}

impl Default for ObservabilityConfig {
    fn default() -> Self {
        Self {
            metrics_enabled: true,
            metrics_flush_interval_secs: defaults::METRICS_FLUSH_INTERVAL_SECS,
            tracing_enabled: true,
            tracing_sample_rate: 1.0,
            otlp_endpoint: None,
            audit_logging_enabled: true,
            health_check_interval_secs: defaults::HEALTH_CHECK_INTERVAL_SECS,
        }
    }
}

/// Master configuration for AuraFS Core
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreConfig {
    /// Cryptographic settings
    pub crypto: CryptoConfig,
    /// Shard settings
    pub shard: ShardConfig,
    /// Identity settings
    pub identity: IdentityConfig,
    /// Resilience settings
    pub resilience: ResilienceConfig,
    /// Rate limiting settings
    pub rate_limit: RateLimitConfig,
    /// Observability settings
    pub observability: ObservabilityConfig,
    /// Default operation timeout (seconds)
    pub default_timeout_secs: u64,
    /// Maximum concurrent operations
    pub max_concurrent_operations: usize,
    /// Data directory path
    pub data_dir: PathBuf,
    /// Enable debug mode
    pub debug_mode: bool,
    /// Environment name (dev, staging, prod)
    pub environment: String,
}

impl Default for CoreConfig {
    fn default() -> Self {
        Self {
            crypto: CryptoConfig::default(),
            shard: ShardConfig::default(),
            identity: IdentityConfig::default(),
            resilience: ResilienceConfig::default(),
            rate_limit: RateLimitConfig::default(),
            observability: ObservabilityConfig::default(),
            default_timeout_secs: defaults::DEFAULT_TIMEOUT_SECS,
            max_concurrent_operations: defaults::MAX_CONCURRENT_OPERATIONS,
            data_dir: PathBuf::from("./data"),
            debug_mode: false,
            environment: "development".to_string(),
        }
    }
}

impl CoreConfig {
    /// Create new configuration with defaults
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Load configuration from file with validation
    pub fn from_file(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Err(client(
                AuraFSError::Config {
                    message: format!("Configuration file not found: {}", path.display()),
                    key: None,
                },
                ErrorPhase::Config,
                ErrorCode::NotFound,
            ));
        }
        
        let contents = std::fs::read_to_string(path)
            .map_err(|e| internal(
                AuraFSError::Config {
                    message: format!("Failed to read config file: {}", e),
                    key: None,
                },
                ErrorPhase::Config,
            ))?;
        
        let config: CoreConfig = toml::from_str(&contents)
            .map_err(|e| client(
                AuraFSError::Config {
                    message: format!("Failed to parse config file: {}", e),
                    key: None,
                },
                ErrorPhase::Config,
                ErrorCode::InvalidInput,
            ))?;
        
        config.validate()?;
        Ok(config)
    }
    
    /// Load from environment variables (override file config)
    pub fn with_env_overrides(mut self) -> Self {
        // Crypto overrides
        if let Ok(val) = std::env::var("AURAFS_CRYPTO_TIMEOUT_SECS") {
            if let Ok(secs) = val.parse() {
                self.crypto.operation_timeout_secs = secs;
            }
        }
        
        // Shard overrides
        if let Ok(val) = std::env::var("AURAFS_MAX_SHARD_SIZE_BYTES") {
            if let Ok(size) = val.parse() {
                self.shard.max_size_bytes = size;
            }
        }
        
        // Resilience overrides
        if let Ok(val) = std::env::var("AURAFS_MAX_RETRIES") {
            if let Ok(retries) = val.parse() {
                self.resilience.max_retries = retries;
            }
        }
        
        // Observability overrides
        if let Ok(endpoint) = std::env::var("AURAFS_OTLP_ENDPOINT") {
            self.observability.otlp_endpoint = Some(endpoint);
        }
        
        if let Ok(val) = std::env::var("AURAFS_TRACING_SAMPLE_RATE") {
            if let Ok(rate) = val.parse() {
                self.observability.tracing_sample_rate = rate;
            }
        }
        
        // General overrides
        if let Ok(dir) = std::env::var("AURAFS_DATA_DIR") {
            self.data_dir = PathBuf::from(dir);
        }
        
        if let Ok(env) = std::env::var("AURAFS_ENVIRONMENT") {
            self.environment = env;
        }
        
        if std::env::var("AURAFS_DEBUG").is_ok() {
            self.debug_mode = true;
        }
        
        self
    }
    
    /// Validate configuration
    pub fn validate(&self) -> Result<()> {
        // Crypto validation
        if self.crypto.max_sign_message_size == 0 {
            return Err(client(
                AuraFSError::Config {
                    message: "max_sign_message_size must be > 0".to_string(),
                    key: Some("crypto.max_sign_message_size".to_string()),
                },
                ErrorPhase::Config,
                ErrorCode::InvalidInput,
            ));
        }
        
        // Shard validation
        if self.shard.max_size_bytes == 0 {
            return Err(client(
                AuraFSError::Config {
                    message: "max_shard_size must be > 0".to_string(),
                    key: Some("shard.max_size_bytes".to_string()),
                },
                ErrorPhase::Config,
                ErrorCode::InvalidInput,
            ));
        }
        
        if self.shard.default_replication_factor == 0 {
            return Err(client(
                AuraFSError::Config {
                    message: "default_replication_factor must be > 0".to_string(),
                    key: Some("shard.default_replication_factor".to_string()),
                },
                ErrorPhase::Config,
                ErrorCode::InvalidInput,
            ));
        }
        
        // Resilience validation
        if self.resilience.retry_base_delay_ms == 0 {
            return Err(client(
                AuraFSError::Config {
                    message: "retry_base_delay_ms must be > 0".to_string(),
                    key: Some("resilience.retry_base_delay_ms".to_string()),
                },
                ErrorPhase::Config,
                ErrorCode::InvalidInput,
            ));
        }
        
        // Rate limit validation
        if self.rate_limit.enabled && self.rate_limit.max_requests_per_window == 0 {
            return Err(client(
                AuraFSError::Config {
                    message: "max_requests_per_window must be > 0 when rate limiting is enabled".to_string(),
                    key: Some("rate_limit.max_requests_per_window".to_string()),
                },
                ErrorPhase::Config,
                ErrorCode::InvalidInput,
            ));
        }
        
        // Observability validation
        if self.observability.tracing_sample_rate < 0.0 || self.observability.tracing_sample_rate > 1.0 {
            return Err(client(
                AuraFSError::Config {
                    message: "tracing_sample_rate must be between 0.0 and 1.0".to_string(),
                    key: Some("observability.tracing_sample_rate".to_string()),
                },
                ErrorPhase::Config,
                ErrorCode::InvalidInput,
            ));
        }
        
        Ok(())
    }
    
    /// Save configuration to file
    pub fn save_to_file(&self, path: &Path) -> Result<()> {
        let contents = toml::to_string_pretty(self)
            .map_err(|e| internal(
                AuraFSError::Config {
                    message: format!("Failed to serialize config: {}", e),
                    key: None,
                },
                ErrorPhase::Config,
            ))?;
        
        std::fs::write(path, contents)
            .map_err(|e| internal(
                AuraFSError::Config {
                    message: format!("Failed to write config file: {}", e),
                    key: None,
                },
                ErrorPhase::Config,
            ))?;
        
        Ok(())
    }
    
    /// Get operation timeout as Duration
    pub fn timeout(&self) -> Duration {
        Duration::from_secs(self.default_timeout_secs)
    }
    
    /// Get crypto timeout as Duration
    pub fn crypto_timeout(&self) -> Duration {
        Duration::from_secs(self.crypto.operation_timeout_secs)
    }
    
    /// Check if production environment
    pub fn is_production(&self) -> bool {
        self.environment == "production" || self.environment == "prod"
    }
}

/// Configuration manager with hot-reload support
pub struct ConfigManager {
    config: Arc<RwLock<CoreConfig>>,
    config_path: Option<PathBuf>,
    _watcher: Option<RecommendedWatcher>,
}

impl ConfigManager {
    /// Create new configuration manager
    pub fn new(config: CoreConfig) -> Self {
        Self {
            config: Arc::new(RwLock::new(config)),
            config_path: None,
            _watcher: None,
        }
    }
    
    /// Create from file with hot-reload support
    pub fn from_file_with_reload(path: PathBuf) -> Result<Self> {
        let config = CoreConfig::from_file(&path)?;
        let config_arc = Arc::new(RwLock::new(config));
        
        // Set up file watcher for hot-reload
        let config_clone = Arc::clone(&config_arc);
        let path_clone = path.clone();
        
        let mut watcher = notify::recommended_watcher(move |res: notify::Result<Event>| {
            if let Ok(event) = res {
                if event.kind.is_modify() {
                    tracing::info!("Configuration file changed, reloading...");
                    if let Ok(new_config) = CoreConfig::from_file(&path_clone) {
                        if let Ok(mut guard) = futures::executor::block_on(async {
                            config_clone.write().await
                        }.into()) {
                            *guard = new_config;
                            tracing::info!("Configuration reloaded successfully");
                        }
                    } else {
                        tracing::warn!("Failed to reload configuration, keeping current config");
                    }
                }
            }
        }).map_err(|e| internal(
            AuraFSError::Config {
                message: format!("Failed to create file watcher: {}", e),
                key: None,
            },
            ErrorPhase::Config,
        ))?;
        
        watcher.watch(&path, RecursiveMode::NonRecursive)
            .map_err(|e| internal(
                AuraFSError::Config {
                    message: format!("Failed to watch config file: {}", e),
                    key: None,
                },
                ErrorPhase::Config,
            ))?;
        
        Ok(Self {
            config: config_arc,
            config_path: Some(path),
            _watcher: Some(watcher),
        })
    }
    
    /// Get current configuration (read lock)
    pub async fn get(&self) -> tokio::sync::RwLockReadGuard<'_, CoreConfig> {
        self.config.read().await
    }
    
    /// Update configuration (write lock)
    pub async fn update<F>(&self, f: F) -> Result<()>
    where
        F: FnOnce(&mut CoreConfig),
    {
        let mut guard = self.config.write().await;
        f(&mut *guard);
        guard.validate()?;
        
        // Persist to file if path is set
        if let Some(ref path) = self.config_path {
            guard.save_to_file(path)?;
        }
        
        Ok(())
    }
    
    /// Get shared config reference
    pub fn shared(&self) -> Arc<RwLock<CoreConfig>> {
        Arc::clone(&self.config)
    }
}

// ======================================================================
// TESTS
// ======================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config() {
        let config = CoreConfig::default();
        assert!(config.validate().is_ok());
    }
    
    #[test]
    fn test_config_validation_failures() {
        let mut config = CoreConfig::default();
        config.crypto.max_sign_message_size = 0;
        assert!(config.validate().is_err());
        
        config = CoreConfig::default();
        config.observability.tracing_sample_rate = 2.0;
        assert!(config.validate().is_err());
    }
    
    #[test]
    fn test_env_overrides() {
        std::env::set_var("AURAFS_ENVIRONMENT", "production");
        std::env::set_var("AURAFS_DEBUG", "1");
        
        let config = CoreConfig::default().with_env_overrides();
        assert_eq!(config.environment, "production");
        assert!(config.debug_mode);
        
        // Cleanup
        std::env::remove_var("AURAFS_ENVIRONMENT");
        std::env::remove_var("AURAFS_DEBUG");
    }
    
    #[test]
    fn test_is_production() {
        let mut config = CoreConfig::default();
        config.environment = "production".to_string();
        assert!(config.is_production());
        
        config.environment = "prod".to_string();
        assert!(config.is_production());
        
        config.environment = "development".to_string();
        assert!(!config.is_production());
    }
}
