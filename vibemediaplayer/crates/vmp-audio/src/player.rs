//! Real-time playback engine — decode once, stream to cpal with EQ + volume.

use crate::decode::{decode_file, DecodedTrack};
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::sync::Arc;
use thiserror::Error;
use vmp_dsp::{EqMode, EqStateSnapshot, GraphicEq};

#[derive(Debug, Error)]
pub enum PlayerError {
    #[error("decode: {0}")]
    Decode(#[from] crate::decode::DecodeError),
    #[error("no track loaded")]
    NoTrack,
    #[error("playback backend: {0}")]
    Backend(String),
    #[error("{0}")]
    Message(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerStatus {
    pub path: Option<String>,
    pub playing: bool,
    pub position_sec: f64,
    pub duration_sec: f64,
    pub volume: f32,
    pub sample_rate: u32,
    pub channels: u16,
    pub ended: bool,
    pub backend: String,
}

pub struct Shared {
    pub track: Mutex<Option<DecodedTrack>>,
    pub frame: AtomicU64,
    pub playing: AtomicBool,
    pub volume: Mutex<f32>,
    pub eq: Mutex<GraphicEq>,
    pub ended: AtomicBool,
    pub out_sample_rate: AtomicU64,
    pub out_channels: AtomicU64,
    /// Rendered-PCM tap for external consumers (e.g. a live visualizer).
    /// Fed the exact interleaved, post-EQ/volume output buffer each render
    /// call. Non-blocking on the audio thread: a full/absent receiver just
    /// means the caller isn't keeping up, never stalls playback.
    pub pcm_tap: Mutex<Option<SyncSender<Vec<f32>>>>,
}

/// Holds the cpal output stream. Stream types are marked !Send on some platforms;
/// we only touch the stream from the controlling process and share PCM via `Shared`.
#[cfg(feature = "playback")]
#[allow(dead_code)] // kept alive so the ALSA/cpal callback stream is not dropped
struct StreamHolder(Option<cpal::Stream>);

#[cfg(feature = "playback")]
// SAFETY: Stream is only closed on drop of PlayerEngine; audio callback uses Arc<Shared> only.
unsafe impl Send for StreamHolder {}
#[cfg(feature = "playback")]
unsafe impl Sync for StreamHolder {}

/// Thread-safe player. With `playback` feature, opens a cpal output stream.
pub struct PlayerEngine {
    shared: Arc<Shared>,
    #[cfg(feature = "playback")]
    _stream: StreamHolder,
    backend: String,
}

impl Default for PlayerEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl PlayerEngine {
    pub fn new() -> Self {
        let shared = Arc::new(Shared {
            track: Mutex::new(None),
            frame: AtomicU64::new(0),
            playing: AtomicBool::new(false),
            volume: Mutex::new(0.75),
            eq: Mutex::new(GraphicEq::new_10(48000.0)),
            ended: AtomicBool::new(false),
            pcm_tap: Mutex::new(None),
            out_sample_rate: AtomicU64::new(48000),
            out_channels: AtomicU64::new(2),
        });

        #[cfg(feature = "playback")]
        {
            let (stream, backend) = match start_cpal_stream(shared.clone()) {
                Ok((st, be, sr, ch)) => {
                    shared.out_sample_rate.store(sr as u64, Ordering::SeqCst);
                    shared.out_channels.store(ch as u64, Ordering::SeqCst);
                    *shared.eq.lock() = GraphicEq::new_10(sr as f32);
                    (st, be)
                }
                Err(e) => {
                    eprintln!("[vmp-audio] cpal unavailable ({e}) — control/decode still work");
                    (None, format!("null ({e})"))
                }
            };
            return Self {
                shared,
                _stream: StreamHolder(stream),
                backend,
            };
        }

        #[cfg(not(feature = "playback"))]
        {
            Self {
                shared,
                backend: "decode-only".into(),
            }
        }
    }

    pub fn load(&self, path: impl AsRef<Path>) -> Result<PlayerStatus, PlayerError> {
        let track = decode_file(path)?;
        self.shared.frame.store(0, Ordering::SeqCst);
        self.shared.playing.store(false, Ordering::SeqCst);
        self.shared.ended.store(false, Ordering::SeqCst);
        *self.shared.track.lock() = Some(track);
        Ok(self.status())
    }

    pub fn play(&self) {
        self.shared.ended.store(false, Ordering::SeqCst);
        self.shared.playing.store(true, Ordering::SeqCst);
    }

    pub fn pause(&self) {
        self.shared.playing.store(false, Ordering::SeqCst);
    }

    pub fn toggle(&self) {
        if self.shared.playing.load(Ordering::SeqCst) {
            self.pause();
        } else {
            self.play();
        }
    }

    pub fn stop(&self) {
        self.pause();
        self.shared.frame.store(0, Ordering::SeqCst);
        self.shared.ended.store(false, Ordering::SeqCst);
    }

    pub fn seek_sec(&self, sec: f64) {
        let track = self.shared.track.lock();
        if let Some(t) = track.as_ref() {
            let f = t.frame_at_sec(sec) as u64;
            self.shared.frame.store(f, Ordering::SeqCst);
            self.shared.ended.store(false, Ordering::SeqCst);
        }
    }

    pub fn set_volume(&self, v: f32) {
        *self.shared.volume.lock() = v.clamp(0.0, 1.5);
    }

    pub fn set_eq_band(&self, index: usize, gain_db: f32) {
        self.shared.eq.lock().set_band(index, gain_db);
    }

    pub fn set_eq_mode(&self, mode: EqMode) {
        self.shared.eq.lock().set_mode(mode);
    }

    pub fn eq_snapshot(&self) -> EqStateSnapshot {
        self.shared.eq.lock().snapshot()
    }

    pub fn path(&self) -> Option<PathBuf> {
        self.shared
            .track
            .lock()
            .as_ref()
            .map(|t| t.path.clone())
    }

    pub fn status(&self) -> PlayerStatus {
        let track = self.shared.track.lock();
        let (duration, sr, ch, path) = if let Some(t) = track.as_ref() {
            (
                t.duration_sec,
                t.sample_rate,
                t.channels,
                Some(t.path.display().to_string()),
            )
        } else {
            (0.0, 0, 0, None)
        };
        drop(track);
        // `frame` is source-domain sample frame index
        let frame = self.shared.frame.load(Ordering::SeqCst) as f64;
        let position_sec = if sr > 0 { frame / sr as f64 } else { 0.0 };
        PlayerStatus {
            path,
            playing: self.shared.playing.load(Ordering::SeqCst),
            position_sec,
            duration_sec: duration,
            volume: *self.shared.volume.lock(),
            sample_rate: sr,
            channels: ch,
            ended: self.shared.ended.load(Ordering::SeqCst),
            backend: self.backend.clone(),
        }
    }

    pub fn backend(&self) -> &str {
        &self.backend
    }

    /// Software pull (tests / null backend) — advances the playhead.
    pub fn render_soft(&self, out: &mut [f32], out_channels: usize) {
        render_frames(&self.shared, out, out_channels);
    }

    /// Subscribe to the live rendered-PCM tap (interleaved, post-EQ/volume,
    /// at `status().sample_rate`/`channels` — the output device's negotiated
    /// format, not the source track's). Replaces any previous subscriber.
    /// The audio thread never blocks on this: a receiver that isn't drained
    /// just stops getting buffers once its bounded capacity fills.
    pub fn subscribe_pcm(&self, capacity: usize) -> Receiver<Vec<f32>> {
        let (tx, rx) = sync_channel(capacity.max(1));
        *self.shared.pcm_tap.lock() = Some(tx);
        rx
    }
}

/// Fill an output buffer from shared state (device callback or software pull).
///
/// `Shared::frame` is the **source** frame cursor (track sample-rate domain).
/// Each output frame advances the cursor by `track_sr / out_sr`.
pub fn render_frames(shared: &Shared, out: &mut [f32], out_channels: usize) {
    if !shared.playing.load(Ordering::Relaxed) {
        out.fill(0.0);
        return;
    }

    let track_guard = shared.track.lock();
    let Some(track) = track_guard.as_ref() else {
        out.fill(0.0);
        return;
    };

    let in_ch = track.channels as usize;
    let vol = *shared.volume.lock();
    let mut eq = shared.eq.lock();
    let total_frames = track.frames();
    let mut src_pos = shared.frame.load(Ordering::Relaxed) as f64;
    let out_channels = out_channels.max(1);
    let frames_needed = out.len() / out_channels;

    let out_sr = shared.out_sample_rate.load(Ordering::Relaxed).max(1) as f64;
    let step = if track.sample_rate > 0 {
        track.sample_rate as f64 / out_sr
    } else {
        1.0
    };

    for i in 0..frames_needed {
        let src_frame = src_pos as usize;
        if src_frame >= total_frames {
            shared.playing.store(false, Ordering::Relaxed);
            shared.ended.store(true, Ordering::Relaxed);
            for j in (i * out_channels)..out.len() {
                out[j] = 0.0;
            }
            break;
        }
        // Linear interpolation between source frames for cleaner resample
        let frac = src_pos - src_frame as f64;
        let next = (src_frame + 1).min(total_frames.saturating_sub(1));
        let base0 = src_frame * in_ch;
        let base1 = next * in_ch;
        for c in 0..out_channels {
            let c0 = c.min(in_ch.saturating_sub(1));
            let s0 = track.samples.get(base0 + c0).copied().unwrap_or(0.0);
            let s1 = track.samples.get(base1 + c0).copied().unwrap_or(s0);
            let mut s = s0 + (s1 - s0) * frac as f32;
            s = eq.process_sample_mut(s) * vol;
            out[i * out_channels + c] = s.clamp(-1.0, 1.0);
        }
        src_pos += step;
    }
    shared.frame.store(src_pos.max(0.0) as u64, Ordering::Relaxed);

    if let Some(tap) = shared.pcm_tap.lock().as_ref() {
        // try_send: a full or dropped receiver must never stall the audio callback.
        let _ = tap.try_send(out.to_vec());
    }
}

#[cfg(feature = "playback")]
fn start_cpal_stream(
    shared: Arc<Shared>,
) -> Result<(Option<cpal::Stream>, String, u32, u16), PlayerError> {
    use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .ok_or_else(|| PlayerError::Backend("no default output device".into()))?;
    let name = device.name().unwrap_or_else(|_| "default".into());
    let config = device
        .default_output_config()
        .map_err(|e| PlayerError::Backend(e.to_string()))?;
    let sample_rate = config.sample_rate().0;
    let channels = config.channels();
    let stream_config: cpal::StreamConfig = config.clone().into();
    let err_fn = |e| eprintln!("[vmp-audio] stream error: {e}");

    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => {
            let sh = shared.clone();
            let ch = channels as usize;
            device
                .build_output_stream(
                    &stream_config,
                    move |data: &mut [f32], _| render_frames(&sh, data, ch),
                    err_fn,
                    None,
                )
                .map_err(|e| PlayerError::Backend(e.to_string()))?
        }
        cpal::SampleFormat::I16 => {
            let sh = shared.clone();
            let ch = channels as usize;
            device
                .build_output_stream(
                    &stream_config,
                    move |data: &mut [i16], _| {
                        let mut f = vec![0.0f32; data.len()];
                        render_frames(&sh, &mut f, ch);
                        for (o, s) in data.iter_mut().zip(f.iter()) {
                            *o = (s.clamp(-1.0, 1.0) * i16::MAX as f32) as i16;
                        }
                    },
                    err_fn,
                    None,
                )
                .map_err(|e| PlayerError::Backend(e.to_string()))?
        }
        other => {
            return Err(PlayerError::Backend(format!(
                "unsupported sample format {other:?}"
            )))
        }
    };

    stream
        .play()
        .map_err(|e| PlayerError::Backend(e.to_string()))?;

    Ok((Some(stream), format!("cpal:{name}"), sample_rate, channels))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    #[test]
    fn soft_render_advances() {
        let dir = std::env::temp_dir().join("vmp_player_test");
        let _ = std::fs::create_dir_all(&dir);
        let wav = dir.join("t.wav");
        let ok = Command::new("ffmpeg")
            .args([
                "-y",
                "-f",
                "lavfi",
                "-i",
                "sine=frequency=440:duration=0.5",
                "-ar",
                "44100",
                wav.to_str().unwrap(),
            ])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false);
        if !ok {
            return;
        }

        let eng = PlayerEngine::new();
        eng.load(&wav).unwrap();
        // Pause device stream so soft render owns the playhead for the assertion
        eng.pause();
        eng.play();
        let mut buf = vec![0.0f32; 4096];
        eng.render_soft(&mut buf, 2);
        assert!(
            buf.iter().any(|s| s.abs() > 0.0001),
            "expected non-silent PCM from sine wav"
        );
        let st = eng.status();
        assert!(st.position_sec > 0.0, "playhead should advance");
        eng.stop();
    }

    #[test]
    fn pcm_tap_receives_rendered_buffers() {
        let dir = std::env::temp_dir().join("vmp_player_test");
        let _ = std::fs::create_dir_all(&dir);
        let wav = dir.join("t_tap.wav");
        let ok = Command::new("ffmpeg")
            .args([
                "-y",
                "-f",
                "lavfi",
                "-i",
                "sine=frequency=440:duration=0.5",
                "-ar",
                "44100",
                wav.to_str().unwrap(),
            ])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false);
        if !ok {
            return;
        }

        let eng = PlayerEngine::new();
        eng.load(&wav).unwrap();
        eng.pause();
        let rx = eng.subscribe_pcm(8);
        eng.play();

        let mut buf = vec![0.0f32; 4096];
        eng.render_soft(&mut buf, 2);

        let tapped = rx.try_recv().expect("tap should receive a buffer after render_soft");
        assert_eq!(tapped.len(), buf.len());
        assert!(tapped.iter().any(|s| s.abs() > 0.0001));
        eng.stop();
    }

    #[test]
    fn backend_reports_cpal_when_playback_enabled() {
        let eng = PlayerEngine::new();
        #[cfg(feature = "playback")]
        {
            assert!(
                eng.backend().starts_with("cpal:") || eng.backend().starts_with("null"),
                "backend={}",
                eng.backend()
            );
        }
        #[cfg(not(feature = "playback"))]
        {
            assert_eq!(eng.backend(), "decode-only");
        }
    }
}
