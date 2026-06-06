//! TTS CLI Commands
//! f0rg3d in l0v3 by Ross Edwards & Aurphyx 💎

use clap::{Parser, Subcommand};

/// TTS CLI
#[derive(Parser)]
#[command(name = "aurafs-tts")]
#[command(about = "AuraFS Text-to-Speech CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// CLI commands
#[derive(Subcommand)]
pub enum Commands {
    /// Synthesize text to audio
    Speak {
        /// Text to speak
        text: String,
        /// Voice ID
        #[arg(short, long)]
        voice: Option<String>,
        /// Output file
        #[arg(short, long)]
        output: Option<String>,
    },
    /// List available voices
    Voices {
        /// Filter by language
        #[arg(short, long)]
        language: Option<String>,
    },
    /// Manage voice packs
    Packs {
        #[command(subcommand)]
        action: PackAction,
    },
    /// Start TTS API server
    Serve {
        /// Port to listen on
        #[arg(short, long, default_value = "8080")]
        port: u16,
    },
}

/// Voice pack actions
#[derive(Subcommand)]
pub enum PackAction {
    /// List installed packs
    List,
    /// Install a voice pack
    Install {
        /// Pack URL or ID
        pack: String,
    },
    /// Uninstall a voice pack
    Uninstall {
        /// Pack ID
        pack: String,
    },
}
