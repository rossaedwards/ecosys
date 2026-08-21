//! Full-file PCM decode via Symphonia (interleaved f32).

use crate::formats::MediaFormat;
use std::fs::File;
use std::path::{Path, PathBuf};
use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::{DecoderOptions, CODEC_TYPE_NULL};
use symphonia::core::errors::Error as SymphoniaError;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DecodeError {
    #[error("I/O: {0}")]
    Io(#[from] std::io::Error),
    #[error("unsupported media: {0}")]
    Unsupported(String),
    #[error("symphonia: {0}")]
    Symphonia(String),
    #[error("no audio track")]
    NoAudioTrack,
}

/// Decoded, seekable PCM track held in memory (MVP; streaming decode later).
#[derive(Debug, Clone)]
pub struct DecodedTrack {
    pub path: PathBuf,
    pub format: MediaFormat,
    pub sample_rate: u32,
    pub channels: u16,
    /// Interleaved f32 samples in [-1, 1]
    pub samples: Vec<f32>,
    pub duration_sec: f64,
}

impl DecodedTrack {
    pub fn frames(&self) -> usize {
        if self.channels == 0 {
            return 0;
        }
        self.samples.len() / self.channels as usize
    }

    pub fn frame_at_sec(&self, sec: f64) -> usize {
        let f = (sec * self.sample_rate as f64).round() as isize;
        f.clamp(0, self.frames() as isize) as usize
    }
}

/// Decode an entire media file to interleaved f32 PCM.
pub fn decode_file(path: impl AsRef<Path>) -> Result<DecodedTrack, DecodeError> {
    let path = path.as_ref();
    let format = MediaFormat::from_path(path)
        .ok_or_else(|| DecodeError::Unsupported(path.display().to_string()))?;

    let file = File::open(path)?;
    let mss = MediaSourceStream::new(Box::new(file), Default::default());
    let mut hint = Hint::new();
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        hint.with_extension(ext);
    }

    let probed = symphonia::default::get_probe()
        .format(
            &hint,
            mss,
            &FormatOptions {
                enable_gapless: true,
                ..Default::default()
            },
            &MetadataOptions::default(),
        )
        .map_err(|e| DecodeError::Symphonia(e.to_string()))?;

    let mut format_reader = probed.format;
    let track = format_reader
        .tracks()
        .iter()
        .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
        .ok_or(DecodeError::NoAudioTrack)?
        .clone();

    let track_id = track.id;
    let sample_rate = track
        .codec_params
        .sample_rate
        .ok_or_else(|| DecodeError::Symphonia("missing sample rate".into()))?;
    let channels = track
        .codec_params
        .channels
        .map(|c| c.count() as u16)
        .unwrap_or(2)
        .max(1);

    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &DecoderOptions::default())
        .map_err(|e| DecodeError::Symphonia(e.to_string()))?;

    let mut samples: Vec<f32> = Vec::new();
    if let Some(n_frames) = track.codec_params.n_frames {
        samples.reserve((n_frames as usize).saturating_mul(channels as usize));
    }

    let mut sample_buf: Option<SampleBuffer<f32>> = None;

    loop {
        let packet = match format_reader.next_packet() {
            Ok(p) => p,
            Err(SymphoniaError::ResetRequired) => {
                decoder.reset();
                continue;
            }
            Err(SymphoniaError::IoError(e)) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                break;
            }
            Err(SymphoniaError::IoError(_)) => break,
            Err(e) => {
                let msg = e.to_string().to_ascii_lowercase();
                if msg.contains("end of stream") || msg.contains("eof") {
                    break;
                }
                return Err(DecodeError::Symphonia(e.to_string()));
            }
        };

        if packet.track_id() != track_id {
            continue;
        }

        let decoded = match decoder.decode(&packet) {
            Ok(buf) => buf,
            Err(SymphoniaError::DecodeError(_)) => continue,
            Err(SymphoniaError::IoError(_)) => break,
            Err(e) => {
                let msg = e.to_string().to_ascii_lowercase();
                if msg.contains("end of stream") {
                    break;
                }
                continue;
            }
        };

        if sample_buf.is_none() {
            let spec = *decoded.spec();
            let capacity = decoded.capacity() as u64;
            sample_buf = Some(SampleBuffer::<f32>::new(capacity, spec));
        }

        if let Some(buf) = sample_buf.as_mut() {
            buf.copy_interleaved_ref(decoded);
            samples.extend_from_slice(buf.samples());
        }
    }

    // Normalize channel count if decoder interleaved with different layout
    let frames = samples.len() / channels as usize;
    let duration_sec = if sample_rate > 0 {
        frames as f64 / sample_rate as f64
    } else {
        0.0
    };

    Ok(DecodedTrack {
        path: path.to_path_buf(),
        format,
        sample_rate,
        channels,
        samples,
        duration_sec,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::process::Command;

    #[test]
    fn decode_sine_wav_if_ffmpeg() {
        let dir = std::env::temp_dir().join("vmp_decode_test");
        let _ = std::fs::create_dir_all(&dir);
        let wav = dir.join("sine.wav");
        let status = Command::new("ffmpeg")
            .args([
                "-y",
                "-f",
                "lavfi",
                "-i",
                "sine=frequency=440:duration=0.25",
                "-ar",
                "44100",
                wav.to_str().unwrap(),
            ])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status();

        let Ok(st) = status else {
            eprintln!("ffmpeg missing — skip decode integration");
            return;
        };
        if !st.success() {
            eprintln!("ffmpeg failed — skip");
            return;
        }

        let track = decode_file(&wav).expect("decode wav");
        assert!(track.sample_rate > 0);
        assert!(track.frames() > 100);
        assert!(track.duration_sec > 0.1);
        let _ = std::fs::remove_file(&wav);
    }

    #[test]
    fn unsupported_ext_errors() {
        let p = std::env::temp_dir().join("vmp_not_audio.txt");
        let mut f = File::create(&p).unwrap();
        writeln!(f, "hello").unwrap();
        assert!(decode_file(&p).is_err());
    }
}
