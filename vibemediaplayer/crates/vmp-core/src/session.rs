//! Player session: File menu actions, VAP edit, modules, Vinyl Vibez.

use crate::modules::{ModuleId, ModuleLayout};
use crate::playlist::{Playlist, PlaylistItem, RecentMedia};
use crate::AppMode;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use vmp_audio::{
    load_media_tags, probe_media, save_vap_for_media, scan_folder, MediaFormat, MediaProbe,
    MediaTagBundle, SaveReport,
};
use vmp_dsp::{EqMode, EqStateSnapshot, GraphicEq};
use vmp_vap::{PillarId, VapObject};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransportState {
    pub playing: bool,
    pub position_sec: f64,
    pub duration_sec: f64,
    pub volume: f32,
    pub shuffle: bool,
    pub repeat: bool,
    pub crossfade: bool,
}

impl Default for TransportState {
    fn default() -> Self {
        Self {
            playing: false,
            position_sec: 0.0,
            duration_sec: 0.0,
            volume: 0.75,
            shuffle: false,
            repeat: true,
            crossfade: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub id: String,
    pub name: String,
    pub is_input: bool,
    pub is_output: bool,
    pub default_sample_rate: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceMatrix {
    pub inputs: Vec<DeviceInfo>,
    pub outputs: Vec<DeviceInfo>,
    pub selected_input: Option<String>,
    pub selected_output: Option<String>,
    pub sample_rate: u32,
    pub buffer_size: u32,
}

impl Default for DeviceMatrix {
    fn default() -> Self {
        Self::stub_default()
    }
}

impl DeviceMatrix {
    pub fn stub_default() -> Self {
        Self {
            inputs: vec![DeviceInfo {
                id: "default_in".into(),
                name: "System Default Input".into(),
                is_input: true,
                is_output: false,
                default_sample_rate: Some(48000),
            }],
            outputs: vec![
                DeviceInfo {
                    id: "default_out".into(),
                    name: "System Default Output".into(),
                    is_input: false,
                    is_output: true,
                    default_sample_rate: Some(48000),
                },
                DeviceInfo {
                    id: "vibe_cable".into(),
                    name: "Vibe Cable (virtual — Phase 5)".into(),
                    is_input: false,
                    is_output: true,
                    default_sample_rate: Some(48000),
                },
            ],
            selected_input: Some("default_in".into()),
            selected_output: Some("default_out".into()),
            sample_rate: 48000,
            buffer_size: 256,
        }
    }
}

/// Dual-deck state for Vinyl Vibez / Mixxx surface.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VinylDeck {
    pub name: String,
    pub loaded_path: Option<String>,
    pub playing: bool,
    pub pitch_percent: f32,
    pub bpm: Option<f64>,
    pub position_sec: f64,
    pub cue_points: Vec<f64>,
}

impl VinylDeck {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            loaded_path: None,
            playing: false,
            pitch_percent: 0.0,
            bpm: None,
            position_sec: 0.0,
            cue_points: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VinylMixer {
    pub crossfader: f32,
    pub gain_a: f32,
    pub gain_b: f32,
    pub filter_a: f32,
    pub filter_b: f32,
    pub sync_enabled: bool,
    pub quantize: bool,
}

impl Default for VinylMixer {
    fn default() -> Self {
        Self {
            crossfader: 0.5,
            gain_a: 0.8,
            gain_b: 0.8,
            filter_a: 0.5,
            filter_b: 0.5,
            sync_enabled: false,
            quantize: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSnapshot {
    pub app_mode: AppMode,
    pub transport: TransportState,
    pub playlist: Playlist,
    pub recent: RecentMedia,
    pub vap: Option<VapObject>,
    pub vap_source: Option<String>,
    pub media_path: Option<String>,
    pub media_format: Option<String>,
    pub probe: Option<MediaProbe>,
    pub active_pillar: u8,
    pub eq: EqStateSnapshot,
    pub devices: DeviceMatrix,
    pub skin_id: String,
    pub layout: ModuleLayout,
    pub stream_url: Option<String>,
    pub deck_a: VinylDeck,
    pub deck_b: VinylDeck,
    pub mixer: VinylMixer,
    pub dirty: bool,
    pub status: String,
}

pub struct PlayerSession {
    pub app_mode: AppMode,
    pub transport: TransportState,
    pub playlist: Playlist,
    pub recent: RecentMedia,
    pub vap: Option<VapObject>,
    pub vap_source: Option<String>,
    pub media_path: Option<PathBuf>,
    pub media_format: Option<MediaFormat>,
    pub probe: Option<MediaProbe>,
    pub active_pillar: PillarId,
    pub eq: GraphicEq,
    pub devices: DeviceMatrix,
    pub skin_id: String,
    pub layout: ModuleLayout,
    pub stream_url: Option<String>,
    pub deck_a: VinylDeck,
    pub deck_b: VinylDeck,
    pub mixer: VinylMixer,
    pub dirty: bool,
    pub status: String,
}

impl Default for PlayerSession {
    fn default() -> Self {
        Self::new()
    }
}

impl PlayerSession {
    pub fn new() -> Self {
        Self {
            app_mode: AppMode::Player,
            transport: TransportState::default(),
            playlist: Playlist::new("Now Playing"),
            recent: RecentMedia::new(25),
            vap: None,
            vap_source: None,
            media_path: None,
            media_format: None,
            probe: None,
            active_pillar: PillarId::Structural,
            eq: GraphicEq::new_10(48000.0),
            devices: DeviceMatrix::stub_default(),
            skin_id: "soul_cosmic".into(),
            layout: ModuleLayout::default_player(),
            stream_url: None,
            deck_a: VinylDeck::new("Deck A"),
            deck_b: VinylDeck::new("Deck B"),
            mixer: VinylMixer::default(),
            dirty: false,
            status: "Ready".into(),
        }
    }

    pub fn set_app_mode(&mut self, mode: AppMode) {
        self.app_mode = mode;
        self.layout = match mode {
            AppMode::Player => ModuleLayout::default_player(),
            AppMode::VinylVibez => ModuleLayout::vinyl_vibez(),
        };
        self.status = format!("Mode: {}", mode.label());
    }

    pub fn open_file(&mut self, path: impl AsRef<Path>) -> Result<(), String> {
        let path = path.as_ref();
        self.load_media(path)?;
        self.playlist.clear();
        self.push_current_to_playlist();
        self.playlist.current_index = Some(0);
        self.recent.push(path.display().to_string());
        self.status = format!("Opened {}", path.display());
        Ok(())
    }

    pub fn open_many(&mut self, paths: &[PathBuf]) -> Result<(), String> {
        if paths.is_empty() {
            return Err("no files".into());
        }
        self.playlist.clear();
        for p in paths {
            if MediaFormat::from_path(p).is_none() {
                continue;
            }
            let _ = self.enrich_playlist_item(p);
        }
        if let Some(first) = paths.first() {
            self.load_media(first)?;
            self.recent.push(first.display().to_string());
        }
        self.playlist.current_index = Some(0);
        self.status = format!("Opened {} files", self.playlist.items.len());
        Ok(())
    }

    pub fn open_folder(&mut self, dir: impl AsRef<Path>) -> Result<(), String> {
        let files = scan_folder(dir).map_err(|e| e.to_string())?;
        if files.is_empty() {
            return Err("no media files in folder".into());
        }
        self.open_many(&files)
    }

    pub fn open_stream(&mut self, url: &str) -> Result<(), String> {
        if !(url.starts_with("http://")
            || url.starts_with("https://")
            || url.starts_with("rtsp://")
            || url.starts_with("icy://"))
        {
            return Err("unsupported stream URL scheme".into());
        }
        self.stream_url = Some(url.to_string());
        self.media_path = None;
        self.vap = Some(VapObject::defaults("Stream", url));
        self.vap_source = Some("Stream".into());
        self.playlist.add(PlaylistItem {
            path: String::new(),
            title: url.to_string(),
            artist: "Network Stream".into(),
            duration_label: None,
            duration_sec: None,
            bpm: None,
            key: None,
            format: Some("Stream".into()),
            uri: Some(url.to_string()),
        });
        self.status = format!("Stream ready: {url}");
        Ok(())
    }

    pub fn open_disc_stub(&mut self, device: &str) -> Result<(), String> {
        // Full CDDA later; record intent + status for UI
        self.status = format!("Open Disc: {device} (CDDA backend Phase 1.5)");
        Err(format!(
            "Disc playback for '{device}' is staged — enable cdda feature next"
        ))
    }

    pub fn open_network_device_stub(&mut self, device: &str) -> Result<(), String> {
        self.status = format!("Network device: {device}");
        Err(format!(
            "Network device '{device}' discovery staged (UPnP/DLNA Phase 2)"
        ))
    }

    fn load_media(&mut self, path: &Path) -> Result<(), String> {
        let format = MediaFormat::from_path(path)
            .ok_or_else(|| format!("unsupported format: {}", path.display()))?;

        let bundle: MediaTagBundle = load_media_tags(path).map_err(|e| e.to_string())?;
        let probe = probe_media(path).ok();

        self.media_path = Some(path.to_path_buf());
        self.media_format = Some(format);
        self.vap = Some(bundle.vap);
        self.vap_source = Some(bundle.vap_source);
        self.probe = probe.clone();
        if let Some(p) = &probe {
            if let Some(d) = p.duration_sec {
                self.transport.duration_sec = d;
            }
        }
        self.transport.position_sec = 0.0;
        self.dirty = false;
        Ok(())
    }

    fn enrich_playlist_item(&mut self, path: &Path) -> Result<(), String> {
        let bundle = load_media_tags(path).map_err(|e| e.to_string())?;
        let probe = probe_media(path).ok();
        let format = MediaFormat::from_path(path).map(|f| f.label().to_string());
        let duration_sec = probe.as_ref().and_then(|p| p.duration_sec);
        let duration_label = duration_sec.map(|d| {
            let m = (d as u64) / 60;
            let s = (d as u64) % 60;
            format!("{m}:{s:02}")
        });
        let bpm = bundle.vap.bpm();
        let key = bundle.vap.pillars.tonal.as_ref().and_then(|t| {
            t.pointer("/HARMONIC_PROFILE/KEY_SIGNATURE")
                .or_else(|| t.pointer("/HARMONIC_PROFILE/KEY")) // pre-3.69-cleanup documents
                .or_else(|| t.get("KEY_SIGNATURE"))
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
        });
        let title = bundle.vap.identity.title.clone();
        let artist = bundle.vap.identity.artist.clone();
        self.playlist.add(PlaylistItem {
            path: path.display().to_string(),
            title,
            artist,
            duration_label,
            duration_sec,
            bpm,
            key,
            format,
            uri: None,
        });
        Ok(())
    }

    fn push_current_to_playlist(&mut self) {
        let Some(path) = self.media_path.clone() else {
            return;
        };
        let _ = self.enrich_playlist_item(&path);
    }

    pub fn save_vap(&mut self, embed: bool) -> Result<SaveReport, String> {
        let path = self
            .media_path
            .as_ref()
            .ok_or("no media file loaded — cannot embed tags")?;
        let vap = self.vap.as_ref().ok_or("no VAP object")?;
        let report = save_vap_for_media(path, vap, embed).map_err(|e| e.to_string())?;
        self.dirty = false;
        self.status = report.messages.join("; ");
        Ok(report)
    }

    pub fn set_vap(&mut self, vap: VapObject) {
        self.vap = Some(vap);
        self.dirty = true;
    }

    pub fn create_playlist(&mut self, name: &str) {
        self.playlist = Playlist::new(name);
        self.status = format!("Created playlist '{name}'");
    }

    pub fn save_playlist(&self, path: impl AsRef<Path>) -> Result<(), String> {
        self.playlist.save_m3u(path).map_err(|e| e.to_string())
    }

    pub fn load_playlist_file(&mut self, path: impl AsRef<Path>) -> Result<(), String> {
        self.playlist = Playlist::load_m3u(path).map_err(|e| e.to_string())?;
        let first_path = self
            .playlist
            .items
            .first()
            .filter(|i| !i.path.is_empty())
            .map(|i| i.path.clone());
        let first_uri = self
            .playlist
            .items
            .first()
            .and_then(|i| i.uri.clone());
        if let Some(p) = first_path {
            let _ = self.load_media(Path::new(&p));
        } else if let Some(uri) = first_uri {
            let _ = self.open_stream(&uri);
        }
        self.status = format!("Loaded playlist '{}'", self.playlist.name);
        Ok(())
    }

    pub fn convert_export_stub(&self, target_format: &str) -> Result<String, String> {
        let src = self
            .media_path
            .as_ref()
            .ok_or("load a file before Convert/Export")?;
        Ok(format!(
            "Queued convert {} → {target_format} (ffmpeg/symphonia export pipeline Phase 2)",
            src.display()
        ))
    }

    pub fn load_to_deck(&mut self, deck: char, path: impl AsRef<Path>) -> Result<(), String> {
        let path = path.as_ref();
        let bundle = load_media_tags(path).map_err(|e| e.to_string())?;
        let d = if deck == 'B' || deck == 'b' {
            &mut self.deck_b
        } else {
            &mut self.deck_a
        };
        d.loaded_path = Some(path.display().to_string());
        d.bpm = bundle.vap.bpm();
        d.position_sec = 0.0;
        d.playing = false;
        self.status = format!("Loaded {} → Deck {deck}", path.display());
        Ok(())
    }

    pub fn set_pillar_tab(&mut self, index: u8) {
        self.active_pillar = match index {
            2 => PillarId::Tonal,
            3 => PillarId::Timbral,
            4 => PillarId::Linguistic,
            5 => PillarId::Affective,
            6 => PillarId::Contextual,
            7 => PillarId::Photometric,
            8 => PillarId::Kinetic,
            9 => PillarId::Genealogical,
            _ => PillarId::Structural,
        };
    }

    pub fn set_eq_band(&mut self, index: usize, gain_db: f32) {
        self.eq.set_band(index, gain_db);
    }

    pub fn set_eq_mode(&mut self, mode: EqMode) {
        self.eq.set_mode(mode);
    }

    pub fn toggle_play(&mut self) {
        self.transport.playing = !self.transport.playing;
    }

    pub fn move_module(&mut self, id: ModuleId, x: f64, y: f64) {
        self.layout.move_module(id, x, y);
    }

    pub fn snapshot(&self) -> SessionSnapshot {
        SessionSnapshot {
            app_mode: self.app_mode,
            transport: self.transport.clone(),
            playlist: self.playlist.clone(),
            recent: self.recent.clone(),
            vap: self.vap.clone(),
            vap_source: self.vap_source.clone(),
            media_path: self.media_path.as_ref().map(|p| p.display().to_string()),
            media_format: self.media_format.map(|f| f.label().to_string()),
            probe: self.probe.clone(),
            active_pillar: self.active_pillar.index(),
            eq: self.eq.snapshot(),
            devices: self.devices.clone(),
            skin_id: self.skin_id.clone(),
            layout: self.layout.clone(),
            stream_url: self.stream_url.clone(),
            deck_a: self.deck_a.clone(),
            deck_b: self.deck_b.clone(),
            mixer: self.mixer.clone(),
            dirty: self.dirty,
            status: self.status.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vinyl_mode_switches_layout() {
        let mut s = PlayerSession::new();
        s.set_app_mode(AppMode::VinylVibez);
        assert!(s
            .layout
            .modules
            .iter()
            .any(|m| m.id == ModuleId::VinylDecks));
    }
}
