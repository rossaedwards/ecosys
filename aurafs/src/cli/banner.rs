// ═══════════════════════════════════════════════════════════════════
// ✨ [:: f0rg3d with l0v3 by Aurphyx ::] ✨
// 💎 AuraFS - Quantum-Secure Distributed Filesystem
// 🌐 Infinite Storage, Zero Boundaries
// ═══════════════════════════════════════════════════════════════════

//! CLI Banner - Diamond-grade terminal art for AuraFS
//!
//! Displays the legendary AuraFS quantum filesystem banner
//! with dynamic taglines, version info, and distributed storage branding.

use colored::*;
use rand::Rng;
use std::io:{self, Write};

/// Main AuraFS banner - The legendary quantum filesystem intro
pub fn print_banner() {
    print_aurafs_logo();
    print_version_info();
    print_quantum_tagline();
    println!();
}

/// Print the AuraFS ASCII logo
fn print_aurafs_logo() {
    let logo = r#"
╔══════════════════════════════════════════════════════════════════════╗
║                                                                      ║
║      █████╗ ██╗   ██╗██████╗  █████╗ ███████╗███████╗              ║
║     ██╔══██╗██║   ██║██╔══██╗██╔══██╗██╔════╝██╔════╝              ║
║     ███████║██║   ██║██████╔╝███████║█████╗  ███████╗              ║
║     ██╔══██║██║   ██║██╔══██╗██╔══██║██╔══╝  ╚════██║              ║
║     ██║  ██║╚██████╔╝██║  ██║██║  ██║██║     ███████║              ║
║     ╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝     ╚══════╝              ║
║                                                                      ║
║         🌌 QUANTUM-SECURE DISTRIBUTED FILESYSTEM 🌌                 ║
║            "Infinite Storage, Zero Boundaries"                      ║
║                                                                      ║
╚══════════════════════════════════════════════════════════════════════╝
"#;

    println!("{}", logo.bright_magenta().bold());
}

/// Print version and build information
fn print_version_info() {
    let version = env!("CARGO_PKG_VERSION");
    let authors = "Aurphyx Quantum Division";
    
    println!(
        "{}",
        format!("   💎 v{} - f0rg3d with l0v3 by {}", version, authors)
            .bright_cyan()
    );
}

/// Print a random quantum-themed tagline
fn print_quantum_tagline() {
    let taglines = [
        "⚛️  Entangled storage across infinite dimensions",
        "🌠 Where your data transcends spacetime",
        "🔮 Quantum coherence meets distributed architecture",
        "✨ Every byte protected by quantum cryptography",
        "🌊 Flowing seamlessly through the storage continuum",
        "💫 Redefining the boundaries of persistent memory",
        "🧬 Evolutionary storage for the quantum age",
        "🌟 One filesystem to unite them all",
        "⚡ Latency approaching the speed of causality",
        "🎨 Painting reality with quantum bits",
    ];
    
    let mut rng = rand::thread_rng();
    let tagline = taglines[rng.gen_range(0..taglines.len())];
    
    println!("{}\n", tagline.bright_green().italic());
}

/// Print a compact banner for sub-commands
pub fn print_compact_banner() {
    println!("{}", "╔═════ AuraFS ═════╗".bright_magenta().bold());
    println!("{}", "╚═══════════════════╝".bright_magenta().bold());
}

/// Print storage operation indicators
pub fn print_operation(operation: &str, path: &str) {
    print!(
        "{} {} {}... ",
        "⚛️".bright_magenta(),
        operation.bright_white(),
        path.bright_cyan()
    );
    io::stdout().flush().unwrap();
}

/// Print success message
pub fn print_success(message: &str) {
    println!("{} {}", "✓".bright_green().bold(), message.bright_white());
}

/// Print error message
pub fn print_error(message: &str) {
    eprintln!("{} {}", "✗".bright_red().bold(), message.bright_white());
}

/// Print warning message
pub fn print_warning(message: &str) {
    println!("{} {}", "⚠".bright_yellow().bold(), message.bright_white());
}

/// Print info message
pub fn print_info(message: &str) {
    println!("{} {}", "ℹ".bright_blue().bold(), message.bright_white());
}

/// Print quantum encryption status banner
pub fn print_encryption_banner(status: &str) {
    let banner = match status {
        "active" => {
            r#"
┌──────────────────────────────────────┐
│  🔐 QUANTUM ENCRYPTION ACTIVE 🔐    │
│   Post-Quantum Lattice Cryptography  │
└──────────────────────────────────────┘
"#
        }
        "syncing" => {
            r#"
┌──────────────────────────────────────┐
│   🔄 QUANTUM KEY EXCHANGE 🔄        │
│      Establishing Entanglement       │
└──────────────────────────────────────┘
"#
        }
        "distributed" => {
            r#"
┌──────────────────────────────────────┐
│  🌐 DISTRIBUTED MODE ENABLED 🌐     │
│    Multi-Node Quantum Coherence      │
└──────────────────────────────────────┘
"#
        }
        _ => {
            r#"
┌──────────────────────────────────────┐
│       AURAFS INITIALIZED 💎         │
└──────────────────────────────────────┘
"#
        }
    };

    println!("{}", banner.bright_cyan());
}

/// Print filesystem stats banner
pub fn print_stats_banner(
    total_nodes: usize,
    total_storage: u64,
    quantum_shards: usize,
    uptime_hours: u64,
) {
    let banner = format!(
        r#"

╔══════════════════════════════════════════════════════════════════════╗
║                                                                      ║
║                    ⚛️  FILESYSTEM STATUS ⚛️                         ║
║                                                                      ║
║  🖥️  Active Nodes:      {:<46}║
║  💾 Total Storage:      {:<46}║
║  🧬 Quantum Shards:     {:<46}║
║  ⏱️  Uptime:            {:<46}║
║                                                                      ║
║            🌌 Quantum coherence maintained 🌌                       ║
║                                                                      ║
╚══════════════════════════════════════════════════════════════════════╝

"#,
        total_nodes,
        format_bytes(total_storage),
        quantum_shards,
        format!("{} hours", uptime_hours)
    );

    println!("{}", banner.bright_green().bold());
}

/// Print data transfer progress
pub fn print_transfer_progress(
    operation: &str,
    filename: &str,
    bytes_transferred: u64,
    total_bytes: u64,
) {
    let percentage = (bytes_transferred as f64 / total_bytes as f64 * 100.0) as u8;
    let bar_width = 40;
    let filled = (bar_width as f64 * (percentage as f64 / 100.0)) as usize;
    let empty = bar_width - filled;

    let bar = format!(
        "[{}{}]",
        "█".repeat(filled).bright_green(),
        "░".repeat(empty).bright_black()
    );

    print!(
        "\r  {} {} {} {}% ({} / {})    ",
        "⚛️".bright_magenta(),
        operation.bright_white(),
        filename.bright_cyan(),
        format!("{:3}", percentage).bright_yellow(),
        format_bytes(bytes_transferred),
        format_bytes(total_bytes)
    );
    print!("{}", bar);
    io::stdout().flush().unwrap();
}

/// Print quantum shard distribution map
pub fn print_shard_map(shards: &[(String, u8)]) {
    println!("\n{}", "═══ QUANTUM SHARD DISTRIBUTION ═══".bright_cyan().bold());
    
    for (node, shard_count) in shards {
        let bar = "█".repeat(*shard_count as usize).bright_magenta();
        println!("  {} {} {}", "📡".bright_yellow(), node.bright_white(), bar);
    }
    
    println!();
}

/// Print mount status
pub fn print_mount_status(mount_point: &str, status: &str) {
    let symbol = match status {
        "mounted" => "✓".bright_green(),
        "unmounted" => "⊘".bright_yellow(),
        "error" => "✗".bright_red(),
        _ => "?".bright_white(),
    };

    println!(
        "{} Mount: {} [{}]",
        symbol.bold(),
        mount_point.bright_cyan(),
        status.bright_white()
    );
}

/// Print Aurphyx signature
pub fn print_aurphyx_signature() {
    println!(
        "{}",
        "   ✨ [:: f0rg3d with l0v3 by Aurphyx ::] ✨\n"
            .bright_magenta()
            .italic()
    );
}

/// Print help hint
pub fn print_help_hint() {
    println!(
        "{}",
        "   💡 Run with --help for quantum storage commands".bright_blue().italic()
    );
    println!();
}

/// Animated quantum initialization sequence
pub fn print_quantum_init_animation() {
    use std::thread;
    use std::time::Duration;

    let frames = [
        "⚛️ ", "🌀", "💫", "✨", "🔮", "⚡", "🌊", "💎",
    ];

    print!("   Initializing quantum filesystem ");
    io::stdout().flush().unwrap();

    for _ in 0..15 {
        for frame in &frames {
            print!("\r   Initializing quantum filesystem {} ", frame.bright_magenta());
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(80));
        }
    }

    println!("\r   Initializing quantum filesystem ✓ ".bright_green());
}

/// Format bytes for human-readable display
fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB", "PB", "EB"];
    
    if bytes == 0 {
        return "0 B".to_string();
    }
    
    let base = 1024_f64;
    let exp = (bytes as f64).log(base).floor() as usize;
    let exp = exp.min(UNITS.len() - 1);
    
    let value = bytes as f64 / base.powi(exp as i32);
    
    format!("{:.2} {}", value, UNITS[exp])
}

/// Print a celebratory sync completion banner
pub fn print_sync_complete(files_synced: usize, elapsed_ms: u128) {
    let banner = format!(
        r#"

╔══════════════════════════════════════════════════════════════════════╗
║                                                                      ║
║                  ⚛️  QUANTUM SYNC COMPLETE! ⚛️                      ║
║                                                                      ║
║  📦 Files Synced:    {:<48}║
║  ⏱️  Time Elapsed:   {:<48}║
║                                                                      ║
║          🌌 Data entangled across all nodes 🌌                     ║
║                                                                      ║
╚══════════════════════════════════════════════════════════════════════╝

"#,
        files_synced,
        format!("{}ms", elapsed_ms)
    );

    println!("{}", banner.bright_green().bold());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_banner_prints_without_panic() {
        print_compact_banner();
        print_aurphyx_signature();
    }

    #[test]
    fn test_encryption_banners() {
        print_encryption_banner("active");
        print_encryption_banner("syncing");
        print_encryption_banner("distributed");
        print_encryption_banner("unknown");
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(0), "0 B");
        assert_eq!(format_bytes(1024), "1.00 KB");
        assert_eq!(format_bytes(1048576), "1.00 MB");
        assert_eq!(format_bytes(1073741824), "1.00 GB");
    }

    #[test]
    fn test_mount_status() {
        print_mount_status("/mnt/aurafs", "mounted");
        print_mount_status("/mnt/aurafs", "unmounted");
        print_mount_status("/mnt/aurafs", "error");
    }
}