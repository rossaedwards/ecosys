//! Dockable modules (VLC / WinAmp style floating panels).

use serde::{Deserialize, Serialize};

/// Identifiers for every movable chrome module.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModuleId {
    Transport,
    Playlist,
    Equalizer,
    Devices,
    PillarTabs,
    MetadataEditor,
    Visualizer,
    Spectrum,
    SkinBrowser,
    AgoraPlugins,
    VinylDecks,
    VinylMixer,
    VinylLibrary,
}

impl ModuleId {
    pub fn label(self) -> &'static str {
        match self {
            Self::Transport => "Transport",
            Self::Playlist => "Playlist",
            Self::Equalizer => "Equalizer",
            Self::Devices => "Devices",
            Self::PillarTabs => "V.A.P. Pillars",
            Self::MetadataEditor => "Metadata Editor",
            Self::Visualizer => "Visualizer",
            Self::Spectrum => "Spectrum",
            Self::SkinBrowser => "Skinz",
            Self::AgoraPlugins => "Agora Plugins",
            Self::VinylDecks => "Vinyl Decks",
            Self::VinylMixer => "Mixer",
            Self::VinylLibrary => "DJ Library",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleFrame {
    pub id: ModuleId,
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
    pub z: i32,
    pub visible: bool,
    pub docked: bool,
    /// Dock zone: left | right | top | bottom | float
    pub zone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleLayout {
    pub name: String,
    pub modules: Vec<ModuleFrame>,
}

impl ModuleLayout {
    pub fn default_player() -> Self {
        Self {
            name: "default_player".into(),
            modules: vec![
                ModuleFrame {
                    id: ModuleId::PillarTabs,
                    x: 0.0,
                    y: 48.0,
                    w: 56.0,
                    h: 520.0,
                    z: 1,
                    visible: true,
                    docked: true,
                    zone: "left".into(),
                },
                ModuleFrame {
                    id: ModuleId::MetadataEditor,
                    x: 64.0,
                    y: 48.0,
                    w: 640.0,
                    h: 360.0,
                    z: 2,
                    visible: true,
                    docked: true,
                    zone: "center".into(),
                },
                ModuleFrame {
                    id: ModuleId::Equalizer,
                    x: 720.0,
                    y: 48.0,
                    w: 320.0,
                    h: 220.0,
                    z: 3,
                    visible: true,
                    docked: false,
                    zone: "float".into(),
                },
                ModuleFrame {
                    id: ModuleId::Playlist,
                    x: 64.0,
                    y: 420.0,
                    w: 480.0,
                    h: 160.0,
                    z: 4,
                    visible: true,
                    docked: true,
                    zone: "bottom".into(),
                },
                ModuleFrame {
                    id: ModuleId::Transport,
                    x: 0.0,
                    y: 600.0,
                    w: 1040.0,
                    h: 64.0,
                    z: 10,
                    visible: true,
                    docked: true,
                    zone: "bottom".into(),
                },
                ModuleFrame {
                    id: ModuleId::Devices,
                    x: 720.0,
                    y: 280.0,
                    w: 320.0,
                    h: 180.0,
                    z: 5,
                    visible: false,
                    docked: false,
                    zone: "float".into(),
                },
                ModuleFrame {
                    id: ModuleId::Visualizer,
                    x: 560.0,
                    y: 420.0,
                    w: 240.0,
                    h: 160.0,
                    z: 6,
                    visible: true,
                    docked: false,
                    zone: "float".into(),
                },
                ModuleFrame {
                    id: ModuleId::SkinBrowser,
                    x: 400.0,
                    y: 200.0,
                    w: 280.0,
                    h: 160.0,
                    z: 7,
                    visible: false,
                    docked: false,
                    zone: "float".into(),
                },
            ],
        }
    }

    pub fn vinyl_vibez() -> Self {
        Self {
            name: "vinyl_vibez_mixxx".into(),
            modules: vec![
                ModuleFrame {
                    id: ModuleId::VinylDecks,
                    x: 40.0,
                    y: 60.0,
                    w: 900.0,
                    h: 280.0,
                    z: 2,
                    visible: true,
                    docked: true,
                    zone: "center".into(),
                },
                ModuleFrame {
                    id: ModuleId::VinylMixer,
                    x: 340.0,
                    y: 350.0,
                    w: 300.0,
                    h: 160.0,
                    z: 3,
                    visible: true,
                    docked: true,
                    zone: "center".into(),
                },
                ModuleFrame {
                    id: ModuleId::VinylLibrary,
                    x: 40.0,
                    y: 520.0,
                    w: 900.0,
                    h: 160.0,
                    z: 4,
                    visible: true,
                    docked: true,
                    zone: "bottom".into(),
                },
                ModuleFrame {
                    id: ModuleId::Equalizer,
                    x: 960.0,
                    y: 60.0,
                    w: 280.0,
                    h: 220.0,
                    z: 5,
                    visible: true,
                    docked: false,
                    zone: "float".into(),
                },
                ModuleFrame {
                    id: ModuleId::MetadataEditor,
                    x: 960.0,
                    y: 300.0,
                    w: 280.0,
                    h: 240.0,
                    z: 6,
                    visible: true,
                    docked: false,
                    zone: "float".into(),
                },
            ],
        }
    }

    pub fn move_module(&mut self, id: ModuleId, x: f64, y: f64) {
        let max_z = self.modules.iter().map(|m| m.z).max().unwrap_or(0);
        if let Some(m) = self.modules.iter_mut().find(|m| m.id == id) {
            m.x = x.max(0.0);
            m.y = y.max(0.0);
            m.docked = false;
            m.zone = "float".into();
            m.z = max_z + 1;
        }
    }

    pub fn resize_module(&mut self, id: ModuleId, w: f64, h: f64) {
        if let Some(m) = self.modules.iter_mut().find(|m| m.id == id) {
            m.w = w.max(120.0);
            m.h = h.max(80.0);
        }
    }

    pub fn set_visible(&mut self, id: ModuleId, visible: bool) {
        if let Some(m) = self.modules.iter_mut().find(|m| m.id == id) {
            m.visible = visible;
        }
    }

    pub fn toggle_visible(&mut self, id: ModuleId) {
        if let Some(m) = self.modules.iter_mut().find(|m| m.id == id) {
            m.visible = !m.visible;
        }
    }
}
