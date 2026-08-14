//! Vibe Media Player core — session, playlist, dockable modules, File menu ops.

mod modules;
mod playlist;
mod session;

pub use modules::*;
pub use playlist::*;
pub use session::*;

pub use vmp_audio::{
    load_media_tags, probe_media, save_vap_for_media, scan_folder, MediaFormat, MediaProbe,
    MediaTagBundle, OPEN_DIALOG_FILTER,
};
pub use vmp_dsp::{EqMode, EqStateSnapshot, GraphicEq};
pub use vmp_v01d::{binding_for_vinyl_vibez, info as v01d_info, mode_from_ui};
pub use vmp_vap::{LoadSource, PillarId, VapObject, PILLAR_TABS};

/// Application surface mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum AppMode {
    /// Classic modular media player
    #[default]
    Player,
    /// Vinyl Vibez — evolves into Mixxx-class DJ
    VinylVibez,
}

impl AppMode {
    pub fn label(self) -> &'static str {
        match self {
            Self::Player => "Vibe Media Player",
            Self::VinylVibez => "Vinyl Vibez → Mixxx",
        }
    }
}

/// File menu command identifiers (VLC-class).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FileMenuAction {
    OpenFile,
    OpenManyFiles,
    OpenFolder,
    OpenDisc,
    OpenRecentMedia,
    Stream,
    ConvertExport,
    CreatePlaylist,
    SavePlaylist,
    EditPlaylist,
    OpenNetworkDevice,
    SaveAndQuit,
    Quit,
}

impl FileMenuAction {
    pub fn all() -> &'static [FileMenuAction] {
        &[
            Self::OpenFile,
            Self::OpenManyFiles,
            Self::OpenFolder,
            Self::OpenDisc,
            Self::OpenRecentMedia,
            Self::Stream,
            Self::ConvertExport,
            Self::CreatePlaylist,
            Self::SavePlaylist,
            Self::EditPlaylist,
            Self::OpenNetworkDevice,
            Self::SaveAndQuit,
            Self::Quit,
        ]
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::OpenFile => "Open File…",
            Self::OpenManyFiles => "Open Many Files…",
            Self::OpenFolder => "Open Folder…",
            Self::OpenDisc => "Open Disc…",
            Self::OpenRecentMedia => "Open Recent Media",
            Self::Stream => "Stream…",
            Self::ConvertExport => "Convert / Export…",
            Self::CreatePlaylist => "Create Playlist",
            Self::SavePlaylist => "Save Playlist…",
            Self::EditPlaylist => "Edit Playlist…",
            Self::OpenNetworkDevice => "Open Network Device…",
            Self::SaveAndQuit => "Save & Quit",
            Self::Quit => "Quit",
        }
    }

    pub fn shortcut(self) -> Option<&'static str> {
        match self {
            Self::OpenFile => Some("Ctrl+O"),
            Self::OpenManyFiles => Some("Ctrl+Shift+O"),
            Self::OpenFolder => Some("Ctrl+F"),
            Self::Stream => Some("Ctrl+N"),
            Self::SavePlaylist => Some("Ctrl+S"),
            Self::SaveAndQuit => Some("Ctrl+Q"),
            Self::Quit => Some("Alt+F4"),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_menu_has_thirteen() {
        assert_eq!(FileMenuAction::all().len(), 13);
    }

    #[test]
    fn modules_default_layout() {
        let layout = ModuleLayout::default_player();
        assert!(layout.modules.iter().any(|m| m.id == ModuleId::Transport));
        assert!(layout.modules.iter().any(|m| m.id == ModuleId::PillarTabs));
    }
}
