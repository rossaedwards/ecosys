//! ═══════════════════════════════════════════════════════════════
//! ✨ [:: f0rg3d with l0v3 by Aurphyx Quantum Division ::] ✨
//! 💎 AuraFS RNS Bridge Manager - Production Process Manager
//! 🌐 Python Bridge Process Management + Auto-Restart + Health Monitoring
//! ═══════════════════════════════════════════════════════════════

#![warn(missing_docs)]

use crate::network::{NetworkError, NetworkResult};
use std::{
    path::{Path, PathBuf},
    process::Stdio,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::{
    process::{Child, Command},
    sync::{RwLock, Mutex},
    task::JoinHandle,
    time::sleep,
};
use tracing::{info, warn, error, debug};

/// Maximum restart attempts before reporting failure
const MAX_RESTART_ATTEMPTS: u32 = 10;

/// Initial restart delay (exponential backoff start)
const INITIAL_RESTART_DELAY: Duration = Duration::from_secs(2);

/// Maximum restart delay (cap exponential backoff)
const MAX_RESTART_DELAY: Duration = Duration::from_secs(60);

/// Interval to check if process is still alive
const HEALTH_CHECK_INTERVAL: Duration = Duration::from_secs(5);

/// Process status enum
#[derive(Debug, Clone, PartialEq)]
pub enum ProcessStatus {
    /// Process is not running
    Stopped,
    /// Process is starting up
    Starting,
    /// Process is running healthy
    Running,
    /// Process failed and is waiting to restart
    Failed,
    /// Process failed permanently after max retries
    Fatal,
}

/// Manager for the Python RNS Bridge process
pub struct RNSBridgeManager {
    script_path: PathBuf,
    process: Arc<Mutex<Option<Child>>>,
    status: Arc<RwLock<ProcessStatus>>,
    monitor_handle: Arc<RwLock<Option<JoinHandle<()>>>>,
    restart_count: Arc<RwLock<u32>>,
    last_restart_time: Arc<RwLock<Option<Instant>>>,
}

impl RNSBridgeManager {
    /// Create a new bridge manager
    pub fn new<P: AsRef<Path>>(script_path: P) -> NetworkResult<Self> {
        let path = script_path.as_ref().to_path_buf();
        
        if !path.exists() {
            return Err(NetworkError::ConfigError(format!(
                "RNS bridge script not found at: {}",
                path.display()
            )));
        }

        Ok(Self {
            script_path: path,
            process: Arc::new(Mutex::new(None)),
            status: Arc::new(RwLock::new(ProcessStatus::Stopped)),
            monitor_handle: Arc::new(RwLock::new(None)),
            restart_count: Arc::new(RwLock::new(0)),
            last_restart_time: Arc::new(RwLock::new(None)),
        })
    }

    /// Start the bridge process and monitoring task
    pub async fn start(&self) -> NetworkResult<()> {
        let mut status = self.status.write().await;
        if *status == ProcessStatus::Running {
            warn!("⚠️  RNS Bridge is already running");
            return Ok(());
        }

        info!("🚀 Starting RNS Bridge process...");
        *status = ProcessStatus::Starting;
        
        // Spawn the process
        self.spawn_process().await?;
        
        // Start monitoring task
        self.start_monitor().await;
        
        *status = ProcessStatus::Running;
        info!("✅ RNS Bridge process started successfully");
        
        Ok(())
    }

    /// Internal method to spawn the Python process
    async fn spawn_process(&self) -> NetworkResult<()> {
        let mut cmd = Command::new("python3");
        
        // On Windows it might be just "python"
        #[cfg(windows)]
        let mut cmd = Command::new("python");

        cmd.arg(&self.script_path);
        
        // Configure output redirection - we want to see logs but not block
        cmd.stdout(Stdio::inherit());
        cmd.stderr(Stdio::inherit());
        
        // Set environment variables if needed
        cmd.env("PYTHONUNBUFFERED", "1"); // Force unbuffered output for logs

        match cmd.spawn() {
            Ok(child) => {
                let mut process_guard = self.process.lock().await;
                *process_guard = Some(child);
                Ok(())
            }
            Err(e) => {
                let msg = format!("Failed to spawn RNS bridge: {}", e);
                error!("❌ {}", msg);
                *self.status.write().await = ProcessStatus::Failed;
                Err(NetworkError::IoError(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    msg,
                )))
            }
        }
    }

    /// Start the background health monitor
    async fn start_monitor(&self) {
        let process_clone = self.process.clone();
        let status_clone = self.status.clone();
        let script_path_clone = self.script_path.clone();
        let restart_count_clone = self.restart_count.clone();
        
        let handle = tokio::spawn(async move {
            let mut current_delay = INITIAL_RESTART_DELAY;
            
            loop {
                // Check if we should stop monitoring
                {
                    let status = status_clone.read().await;
                    if *status == ProcessStatus::Stopped || *status == ProcessStatus::Fatal {
                        break;
                    }
                }

                sleep(HEALTH_CHECK_INTERVAL).await;

                let mut needs_restart = false;
                {
                    let mut process_guard = process_clone.lock().await;
                    if let Some(child) = process_guard.as_mut() {
                        match child.try_wait() {
                            Ok(Some(status)) => {
                                warn!("⚠️  RNS Bridge process exited with: {}", status);
                                needs_restart = true;
                            }
                            Ok(None) => {
                                // Process still running, all good
                                *status_clone.write().await = ProcessStatus::Running;
                                // Reset backoff if we've been running for a while
                                current_delay = INITIAL_RESTART_DELAY;
                            }
                            Err(e) => {
                                error!("❌ Error checking process status: {}", e);
                                needs_restart = true;
                            }
                        }
                    } else {
                        needs_restart = true;
                    }
                }

                if needs_restart {
                    let mut restarts = restart_count_clone.write().await;
                    if *restarts >= MAX_RESTART_ATTEMPTS {
                        error!("❌ Maximum restart attempts ({}) reached. Giving up.", MAX_RESTART_ATTEMPTS);
                        *status_clone.write().await = ProcessStatus::Fatal;
                        break;
                    }

                    *restarts += 1;
                    warn!("🔄 Restarting RNS Bridge (Attempt {}/{}) in {:?}...", 
                          *restarts, MAX_RESTART_ATTEMPTS, current_delay);
                    
                    *status_clone.write().await = ProcessStatus::Failed;
                    
                    // Wait before restart (exponential backoff)
                    sleep(current_delay).await;
                    current_delay = std::cmp::min(current_delay * 2, MAX_RESTART_DELAY);

                    // Re-spawn logic inline to avoid complex borrowing
                    let mut cmd = Command::new("python3");
                    #[cfg(windows)]
                    let mut cmd = Command::new("python");
                    
                    cmd.arg(&script_path_clone)
                       .stdout(Stdio::inherit())
                       .stderr(Stdio::inherit())
                       .env("PYTHONUNBUFFERED", "1");

                    match cmd.spawn() {
                        Ok(child) => {
                            let mut process_guard = process_clone.lock().await;
                            *process_guard = Some(child);
                            *status_clone.write().await = ProcessStatus::Starting;
                            info!("✅ RNS Bridge successfully restarted");
                        }
                        Err(e) => {
                            error!("❌ Failed to restart RNS bridge: {}", e);
                        }
                    }
                }
            }
            debug!("🛑 Health monitor task exiting");
        });

        let mut monitor_guard = self.monitor_handle.write().await;
        *monitor_guard = Some(handle);
    }

    /// Stop the bridge process
    pub async fn stop(&self) -> NetworkResult<()> {
        info!("🛑 Stopping RNS Bridge...");
        *self.status.write().await = ProcessStatus::Stopped;
        
        let mut process_guard = self.process.lock().await;
        if let Some(mut child) = process_guard.take() {
            // Try graceful kill first
            #[cfg(unix)]
            {
                // Send SIGTERM on Unix
                unsafe {
                    libc::kill(child.id().unwrap() as i32, libc::SIGTERM);
                }
            }
            
            // On Windows or as fallback, force kill if needed
            // tokio's kill() is basically SIGKILL
            if let Err(e) = child.kill().await {
                // It might have already exited
                debug!("Process kill result (may be already dead): {}", e);
            }
            
            // Wait for process to exit to prevent zombies
            let _ = child.wait().await;
            info!("✅ RNS Bridge process stopped");
        }
        
        // Abort monitor task
        if let Some(handle) = self.monitor_handle.write().await.take() {
            handle.abort();
        }
        
        Ok(())
    }
    
    /// Check if process is running
    pub async fn is_running(&self) -> bool {
        let status = self.status.read().await;
        *status == ProcessStatus::Running || *status == ProcessStatus::Starting
    }
    
    /// Get process status
    pub async fn get_status(&self) -> ProcessStatus {
        self.status.read().await.clone()
    }
}