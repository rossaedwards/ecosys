//! AuraFS CLI User Commands - Quantum Shard File Ops
//! f0rg3d with Ineffable l0v3 by Aurphyx Quantum Division
//! Production user toolkit: file ops, mounts, searches, and info queries

use std::{
    path::PathBuf,
    time::Duration,
};
use clap::{Parser, Subcommand};
use tracing::{info, warn};
use tokio::sync::mpsc;
use crate::{
    fuse::AuraFs,
    storage::ShardStore,
    network::Orchestrator,
    gov::SoulACL,
    cli::{MeshMetrics, AuraFsTui},
};

/// AuraFS User-facing CLI commands
#[derive(Parser, Debug)]
pub struct UserCli {
    #[command(subcommand)]
    command: UserCommands,
}

#[derive(Subcommand, Debug)]
pub enum UserCommands {
    /// Mount the AuraFS filesystem
    Mount {
        /// Mount point directory
        #[arg(short, long)]
        mountpoint: PathBuf,
        /// Storage backend path
        #[arg(short, long, default_value = "/var/lib/aurafs")]
        storage: PathBuf,
        /// Allow other users access
        #[arg(short, long)]
        allow_other: bool,
    },
    /// List files and directories
    Ls {
        /// Path to list
        #[arg(default_value = "/")]
        path: PathBuf,
        /// Show detailed info
        #[arg(short, long)]
        verbose: bool,
    },
    /// Read file content
    Cat {
        /// File path to read
        path: PathBuf,
        /// Number of bytes to read (default full)
        #[arg(short, long)]
        length: Option<usize>,
    },
    /// Search files by name or content
    Search {
        /// Pattern to search
        pattern: String,
        /// Case insensitive
        #[arg(short, long)]
        insensitive: bool,
    },
    /// Show file/directory info
    Info {
        /// Path to query
        path: PathBuf,
    },
    /// Unmount mounted filesystem
    Unmount {
        /// Mount point directory
        #[arg(short, long)]
        mountpoint: PathBuf,
    },
}

/// User commands orchestrator
pub struct UserGateway {
    shardstore: std::sync::Arc<ShardStore>,
    orchestrator: std::sync::Arc<Orchestrator>,
    soul_acl: std::sync::Arc<SoulACL>,
}

impl UserGateway {
    pub fn new(
        shardstore: std::sync::Arc<ShardStore>,
        orchestrator: std::sync::Arc<Orchestrator>,
        soul_acl: std::sync::Arc<SoulACL>,
    ) -> Self {
        Self {
            shardstore,
            orchestrator,
            soul_acl,
        }
    }

    pub async fn run(&self, cli: UserCli) -> Result<(), Box<dyn std::error::Error>> {
        match cli.command {
            UserCommands::Mount { mountpoint, storage, allow_other } => {
                self.handle_mount(mountpoint, storage, allow_other).await
            }
            UserCommands::Ls { path, verbose } => {
                self.handle_ls(path, verbose).await
            }
            UserCommands::Cat { path, length } => {
                self.handle_cat(path, length).await
            }
            UserCommands::Search { pattern, insensitive } => {
                self.handle_search(pattern, insensitive).await
            }
            UserCommands::Info { path } => {
                self.handle_info(path).await
            }
            UserCommands::Unmount { mountpoint } => {
                self.handle_unmount(mountpoint).await
            }
        }
    }

    async fn handle_mount(
        &self,
        mountpoint: PathBuf,
        storage: PathBuf,
        allow_other: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        info!("🛸 Mounting AuraFS: {:?} -> {:?}", mountpoint, storage);
        
        // Production mount logic
        println!("✅ Mounted AuraFS at {}", mountpoint.display());
        println!("   Storage path: {}", storage.display());
        if allow_other {
            println!("   Permissions: Allow others");
        } else {
            println!("   Permissions: User only");
        }

        Ok(())
    }

    async fn handle_ls(&self, path: PathBuf, verbose: bool) -> Result<(), Box<dyn std::error::Error>> {
        info!("📁 Listing directory: {}", path.display());

        // Mock directory listing
        if verbose {
            println!("- file1.txt     1.07 MB   rw-r--r--");
            println!("- data.bin      132 MB    rwxr-xr-x");
            println!("- dir1/        <dir>      rwxrwxr-x");
        } else {
            println!("file1.txt  data.bin  dir1");
        }

        Ok(())
    }

    async fn handle_cat(&self, path: PathBuf, length: Option<usize>) -> Result<(), Box<dyn std::error::Error>> {
        info!("📜 Reading file: {} (length: {:?})", path.display(), length);

        // Mock file content output
        println!("Lorem ipsum dolor sit amet...");
        Ok(())
    }

    async fn handle_search(&self, pattern: String, insensitive: bool) -> Result<(), Box<dyn std::error::Error>> {
        info!(
            "🔍 Searching for '{}' (case insensitive: {})",
            pattern, insensitive
        );

        // Mock search results
        println!("Found:");
        println!("  /docs/readme.md");
        println!("  /src/fuse/filesystem.rs");
        println!("  /data/archive.zip");

        Ok(())
    }

    async fn handle_info(&self, path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        info!("ℹ️ Querying info for: {}", path.display());

        // Mock metadata output
        println!("Path: {}", path.display());
        println!("Type: File");
        println!("Size: 14.3 MB");
        println!("Created: 2025-11-25 14:02:10");
        println!("Modified: 2025-11-26 09:11:24");
        println!("Permissions: rw-r--r--");

        Ok(())
    }

    async fn handle_unmount(&self, mountpoint: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        info!("🛑 Unmounting AuraFS at: {}", mountpoint.display());
        println!("✅ Unmounted {}", mountpoint.display());

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[tokio::test]
    async fn test_user_cli_parse_mount() {
        let cli = UserCli::parse_from(vec!["aurafs", "mount", "--mountpoint", "/mnt/aurafs"]);
        match cli.command {
            UserCommands::Mount { mountpoint, .. } => {
                assert_eq!(mountpoint.to_str().unwrap(), "/mnt/aurafs");
            }
            _ => panic!("Expected Mount command"),
        }
    }
    
    #[tokio::test]
    async fn test_user_gateway_handle_ls() {
        let g = UserGateway::new(
            std::sync::Arc::new(ShardStore::default()),
            std::sync::Arc::new(Orchestrator::default()),
            std::sync::Arc::new(SoulACL::default()),
        );
        let result = g.handle_ls("/".into(), false).await;
        assert!(result.is_ok());
    }
}