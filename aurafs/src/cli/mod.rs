//! AuraFS CLI Module
//! f0rg3d in l0v3 by Ross Edwards & Aurphyx 💎

// Core CLI modules (implemented)
pub mod banner;
pub mod aurafs_cli;
pub mod admin;
pub mod user;
pub mod tui;

// Re-exports for convenience
pub use aurafs_cli::{Cli, CliGateway};
pub use admin::AdminGateway;
pub use user::UserGateway;
pub use banner::{print_banner, print_info};

// TODO: These modules are planned but not yet implemented
// pub mod core_commands;
// pub mod ai_cli;
// pub mod quantum_cli;
// pub mod game_cli;
// pub mod security_cli;
// pub mod enterprise_cli;

/// CLI initialization complete marker
pub fn cli_init_complete() {
    println!("AuraFS CLI initialized successfully");
}
