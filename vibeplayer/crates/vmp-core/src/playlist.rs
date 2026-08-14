//! Playlist create / save / load (M3U-compatible).

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use thiserror::Error;
use vmp_audio::MediaFormat;

#[derive(Debug, Error)]
pub enum PlaylistError {
    #[error("I/O: {0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Message(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistItem {
    pub path: String,
    pub title: String,
    pub artist: String,
    pub duration_label: Option<String>,
    pub duration_sec: Option<f64>,
    pub bpm: Option<f64>,
    pub key: Option<String>,
    pub format: Option<String>,
    /// Object URL / stream URI when not a local path
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Playlist {
    pub name: String,
    pub items: Vec<PlaylistItem>,
    pub current_index: Option<usize>,
}

impl Playlist {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            items: Vec::new(),
            current_index: None,
        }
    }

    pub fn clear(&mut self) {
        self.items.clear();
        self.current_index = None;
    }

    pub fn add(&mut self, item: PlaylistItem) {
        self.items.push(item);
        if self.current_index.is_none() {
            self.current_index = Some(0);
        }
    }

    pub fn add_paths(&mut self, paths: &[PathBuf]) {
        for p in paths {
            let title = p
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Untitled")
                .to_string();
            let format = MediaFormat::from_path(p).map(|f| f.label().to_string());
            self.add(PlaylistItem {
                path: p.display().to_string(),
                title,
                artist: "Unknown".into(),
                duration_label: None,
                duration_sec: None,
                bpm: None,
                key: None,
                format,
                uri: None,
            });
        }
    }

    pub fn save_m3u(&self, path: impl AsRef<Path>) -> Result<(), PlaylistError> {
        let mut body = String::from("#EXTM3U\n");
        body.push_str(&format!("#PLAYLIST:{}\n", self.name));
        for item in &self.items {
            let dur = item.duration_sec.map(|d| d as i64).unwrap_or(-1);
            body.push_str(&format!(
                "#EXTINF:{},{} - {}\n",
                dur, item.artist, item.title
            ));
            if let Some(uri) = &item.uri {
                body.push_str(uri);
            } else {
                body.push_str(&item.path);
            }
            body.push('\n');
        }
        std::fs::write(path, body)?;
        Ok(())
    }

    pub fn load_m3u(path: impl AsRef<Path>) -> Result<Self, PlaylistError> {
        let raw = std::fs::read_to_string(path.as_ref())?;
        let mut pl = Playlist::new(
            path.as_ref()
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Playlist"),
        );
        let mut pending_title: Option<String> = None;
        for line in raw.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("#EXTM3U") {
                continue;
            }
            if let Some(rest) = line.strip_prefix("#PLAYLIST:") {
                pl.name = rest.to_string();
                continue;
            }
            if let Some(rest) = line.strip_prefix("#EXTINF:") {
                // duration,title
                if let Some((_, title)) = rest.split_once(',') {
                    pending_title = Some(title.trim().to_string());
                }
                continue;
            }
            if line.starts_with('#') {
                continue;
            }
            let title = pending_title
                .take()
                .unwrap_or_else(|| {
                    Path::new(line)
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or(line)
                        .to_string()
                });
            let (artist, title) = if let Some((a, t)) = title.split_once(" - ") {
                (a.to_string(), t.to_string())
            } else {
                ("Unknown".into(), title)
            };
            let is_uri = line.contains("://");
            pl.add(PlaylistItem {
                path: if is_uri {
                    String::new()
                } else {
                    line.to_string()
                },
                title,
                artist,
                duration_label: None,
                duration_sec: None,
                bpm: None,
                key: None,
                format: MediaFormat::from_path(line).map(|f| f.label().to_string()),
                uri: if is_uri {
                    Some(line.to_string())
                } else {
                    None
                },
            });
        }
        Ok(pl)
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecentMedia {
    pub paths: Vec<String>,
    pub max: usize,
}

impl RecentMedia {
    pub fn new(max: usize) -> Self {
        Self {
            paths: Vec::new(),
            max,
        }
    }

    pub fn push(&mut self, path: impl Into<String>) {
        let path = path.into();
        self.paths.retain(|p| p != &path);
        self.paths.insert(0, path);
        self.paths.truncate(self.max.max(1));
    }
}
