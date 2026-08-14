//! Probe media files with Symphonia (duration, codec, sample rate).

use crate::formats::MediaFormat;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::{Path, PathBuf};
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProbeError {
    #[error("I/O: {0}")]
    Io(#[from] std::io::Error),
    #[error("unsupported format for path: {0}")]
    Unsupported(String),
    #[error("symphonia: {0}")]
    Symphonia(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaProbe {
    pub path: PathBuf,
    pub format: MediaFormat,
    pub format_label: String,
    pub duration_sec: Option<f64>,
    pub sample_rate: Option<u32>,
    pub channels: Option<u16>,
    pub codec: Option<String>,
    pub bit_rate: Option<u64>,
    pub playable: bool,
    pub notes: Vec<String>,
}

/// Probe a media file for playback parameters (does not fully decode PCM).
pub fn probe_media(path: impl AsRef<Path>) -> Result<MediaProbe, ProbeError> {
    let path = path.as_ref();
    let format = MediaFormat::from_path(path).ok_or_else(|| {
        ProbeError::Unsupported(path.display().to_string())
    })?;

    let mut notes = Vec::new();
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
            &FormatOptions::default(),
            &MetadataOptions::default(),
        )
        .map_err(|e| ProbeError::Symphonia(e.to_string()))?;

    let fmt = probed.format;
    let track = fmt
        .default_track()
        .ok_or_else(|| ProbeError::Symphonia("no default track".into()))?;

    let sample_rate = track.codec_params.sample_rate;
    let channels = track
        .codec_params
        .channels
        .map(|c| c.count() as u16);
    let bit_rate = track.codec_params.bits_per_sample.map(|b| b as u64);

    let duration_sec = match (
        track.codec_params.n_frames,
        track.codec_params.sample_rate,
    ) {
        (Some(frames), Some(sr)) if sr > 0 => Some(frames as f64 / sr as f64),
        _ => {
            notes.push("duration unknown until full decode".into());
            None
        }
    };

    let codec = Some(format!("{:?}", track.codec_params.codec));

    if format.is_container() {
        notes.push("MP4/M4V: audio track used for playback; video not rendered in VMP core yet".into());
    }

    Ok(MediaProbe {
        path: path.to_path_buf(),
        format,
        format_label: format.label().into(),
        duration_sec,
        sample_rate,
        channels,
        codec,
        bit_rate,
        playable: true,
        notes,
    })
}

/// Recursively collect playable media paths under a folder.
pub fn scan_folder(root: impl AsRef<Path>) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut out = Vec::new();
    scan_folder_inner(root.as_ref(), &mut out)?;
    out.sort();
    Ok(out)
}

fn scan_folder_inner(dir: &Path, out: &mut Vec<PathBuf>) -> Result<(), std::io::Error> {
    if !dir.is_dir() {
        return Ok(());
    }
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let p = entry.path();
        if p.is_dir() {
            scan_folder_inner(&p, out)?;
        } else if MediaFormat::from_path(&p).is_some() {
            out.push(p);
        }
    }
    Ok(())
}
