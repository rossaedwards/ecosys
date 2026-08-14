//! `vmp` — headless Vibe Media Player CLI.

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use vmp_audio::{
    decode_file, list_devices, load_media_tags, probe_media, save_vap_for_media, scan_folder,
    PlayerEngine, OPEN_DIALOG_FILTER,
};
use vmp_v01d::{binding_for_vinyl_vibez, info as v01d_info};
use vmp_vinyl::VinylEngine;

#[derive(Parser)]
#[command(name = "vmp", about = "Vibe Media Player — paradigm-shifting experiential media")]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show version / stack
    Version,
    /// Probe a media file (format, duration, codec)
    Probe { path: PathBuf },
    /// Decode to PCM stats (no output device required)
    Decode { path: PathBuf },
    /// Play a file (cpal if --features playback; else soft-render progress)
    Play {
        path: PathBuf,
        /// Seconds to play (0 = full track)
        #[arg(long, default_value_t = 0.0)]
        seconds: f64,
    },
    /// Load / print V.A.P. metadata
    Vap {
        path: PathBuf,
        /// Write pretty JSON to stdout
        #[arg(long)]
        json: bool,
    },
    /// Save VAP sidecar (+ optional embed)
    VapSave {
        path: PathBuf,
        #[arg(long)]
        embed: bool,
    },
    /// Scan folder for media
    Scan { dir: PathBuf },
    /// Print open-dialog filter string
    Filters,
    /// List ALSA/cpal input & output devices
    Devices,
    /// v01d / Vinyl Vibez binding plan
    V01d,
    /// Dual-deck Vinyl Vibez mix (Mixxx symbiont engine)
    Vinyl {
        /// Deck A path
        deck_a: PathBuf,
        /// Deck B path (optional — mono A if omitted)
        deck_b: Option<PathBuf>,
        /// Seconds to render/play
        #[arg(long, default_value_t = 2.0)]
        seconds: f64,
        /// Crossfader 0=A .. 1=B
        #[arg(long, default_value_t = 0.5)]
        xfade: f32,
        /// Rate % on deck B
        #[arg(long, default_value_t = 0.0)]
        rate_b: f64,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match cli.cmd {
        Commands::Version => {
            println!("Vibe Media Player CLI");
            println!("  vmp-audio + vmp-vap + v01d ({})", v01d_info().version);
            #[cfg(feature = "playback")]
            {
                println!("  playback: cpal ENABLED (ALSA/CoreAudio/WASAPI)");
                let inv = list_devices();
                println!(
                    "  host: {} · {} out · {} in",
                    inv.host,
                    inv.outputs.len(),
                    inv.inputs.len()
                );
            }
            #[cfg(not(feature = "playback"))]
            println!("  playback: soft-render (build with --features playback for cpal)");
            println!(
                "  formats: {}",
                OPEN_DIALOG_FILTER.lines().next().unwrap_or("")
            );
        }
        Commands::Probe { path } => {
            let p = probe_media(&path)?;
            println!("{}", serde_json::to_string_pretty(&p)?);
        }
        Commands::Decode { path } => {
            let t = decode_file(&path)?;
            println!(
                "decoded {} · {} Hz · {} ch · {} frames · {:.2}s · {}",
                t.format.label(),
                t.sample_rate,
                t.channels,
                t.frames(),
                t.duration_sec,
                path.display()
            );
        }
        Commands::Play { path, seconds } => {
            let engine = PlayerEngine::new();
            let st = engine.load(&path)?;
            println!(
                "playing {} ({:.1}s) via {}",
                st.path.as_deref().unwrap_or("?"),
                st.duration_sec,
                st.backend
            );
            engine.play();
            let limit = if seconds > 0.0 {
                seconds
            } else {
                st.duration_sec + 0.25
            };
            let start = std::time::Instant::now();
            let mut buf = vec![0.0f32; 2048];
            while start.elapsed().as_secs_f64() < limit {
                // Keep playhead advancing when cpal stream is absent
                if engine.status().backend.starts_with("decode")
                    || engine.status().backend.starts_with("null")
                {
                    engine.render_soft(&mut buf, 2);
                }
                let s = engine.status();
                if s.ended {
                    break;
                }
                eprint!(
                    "\r  {:>6.1}s / {:>6.1}s   ",
                    s.position_sec, s.duration_sec
                );
                thread::sleep(Duration::from_millis(20));
            }
            engine.stop();
            eprintln!("\ndone.");
        }
        Commands::Vap { path, json } => {
            let bundle = load_media_tags(&path)?;
            if json {
                println!("{}", bundle.vap.to_pretty_json()?);
            } else {
                println!(
                    "{} — {} [{}] source={}",
                    bundle.vap.identity.artist,
                    bundle.vap.identity.title,
                    bundle.format.label(),
                    bundle.vap_source
                );
                println!("  sidecar: {}", bundle.vap_sidecar_path.display());
                println!("  can_embed: {}", bundle.can_embed);
                if let Some(bpm) = bundle.vap.bpm() {
                    println!("  bpm: {bpm}");
                }
            }
        }
        Commands::VapSave { path, embed } => {
            let bundle = load_media_tags(&path)?;
            let report = save_vap_for_media(&path, &bundle.vap, embed)?;
            for m in report.messages {
                println!("· {m}");
            }
        }
        Commands::Scan { dir } => {
            let files = scan_folder(&dir)?;
            for f in &files {
                println!("{}", f.display());
            }
            println!("--- {} files ---", files.len());
        }
        Commands::Filters => {
            println!("{OPEN_DIALOG_FILTER}");
        }
        Commands::Devices => {
            let inv = list_devices();
            println!("host: {}", inv.host);
            println!("outputs:");
            for d in &inv.outputs {
                let mark = if d.is_default { "*" } else { " " };
                println!(
                    "  {mark} {}  {}  sr={:?} ch={:?}",
                    d.id, d.name, d.default_sample_rate, d.max_channels
                );
            }
            println!("inputs:");
            for d in &inv.inputs {
                let mark = if d.is_default { "*" } else { " " };
                println!(
                    "  {mark} {}  {}  sr={:?} ch={:?}",
                    d.id, d.name, d.default_sample_rate, d.max_channels
                );
            }
        }
        Commands::V01d => {
            println!("{}", serde_json::to_string_pretty(&v01d_info())?);
            println!(
                "{}",
                serde_json::to_string_pretty(&binding_for_vinyl_vibez())?
            );
        }
        Commands::Vinyl {
            deck_a,
            deck_b,
            seconds,
            xfade,
            rate_b,
        } => {
            let mut eng = VinylEngine::new();
            eng.output_sample_rate = 48_000;
            eng.load('A', &deck_a)?;
            if let Some(b) = deck_b {
                eng.load('B', &b)?;
                eng.play('B', true);
                eng.set_rate_percent('B', rate_b);
            }
            eng.play('A', true);
            eng.set_crossfader(xfade);

            let snap = eng.snapshot();
            println!(
                "Vinyl Vibez · {} | A={:.1}s B={:.1}s xfade={xfade}",
                snap.origin,
                snap.deck_a.duration_sec,
                snap.deck_b.duration_sec
            );

            // Soft-render + optional cpal via PlayerEngine-like loop
            let out_ch = 2usize;
            let sr = eng.output_sample_rate as f64;
            let total_frames = (seconds * sr) as usize;
            let block = 1024usize;
            let mut buf = vec![0.0f32; block * out_ch];
            let mut done = 0usize;

            #[cfg(feature = "playback")]
            let device_player = {
                // Stream mixed blocks through a one-shot decode path:
                // use soft process only; cpal full duplex later.
                None::<()>
            };

            while done < total_frames {
                let n = (total_frames - done).min(block);
                let slice_len = n * out_ch;
                eng.process(&mut buf[..slice_len], out_ch);
                // Peak meter
                let peak = buf[..slice_len]
                    .iter()
                    .map(|s| s.abs())
                    .fold(0.0f32, f32::max);
                eprint!(
                    "\r  vinyl {:>5.2}s  peak={:.3}  A@{:.2}s B@{:.2}s   ",
                    done as f64 / sr,
                    peak,
                    eng.deck_a.position_sec(),
                    eng.deck_b.position_sec()
                );
                done += n;
                // Pace roughly real-time when playing through speakers via external tools;
                // here we soft-render for engine verification.
                let _ = device_player;
                std::thread::sleep(std::time::Duration::from_millis(
                    ((n as f64 / sr) * 1000.0) as u64,
                ));
            }
            eprintln!("\ndone.");
        }
    }
    Ok(())
}
