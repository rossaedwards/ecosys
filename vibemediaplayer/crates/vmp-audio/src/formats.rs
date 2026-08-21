//! Supported media formats and extension maps.

use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MediaFormat {
    Mp3,
    M4a,
    Aac,
    Flac,
    OggVorbis,
    Opus,
    Wav,
    Aiff,
    Mp4,
    M4v,
    Caf,
    Unknown,
}

impl MediaFormat {
    pub fn from_path(path: impl AsRef<Path>) -> Option<Self> {
        let ext = path
            .as_ref()
            .extension()
            .and_then(|e| e.to_str())?
            .to_ascii_lowercase();
        Some(match ext.as_str() {
            "mp3" => Self::Mp3,
            "m4a" => Self::M4a,
            "aac" => Self::Aac,
            "flac" => Self::Flac,
            "ogg" | "oga" => Self::OggVorbis,
            "opus" => Self::Opus,
            "wav" => Self::Wav,
            "aiff" | "aif" => Self::Aiff,
            "mp4" => Self::Mp4,
            "m4v" => Self::M4v,
            "caf" => Self::Caf,
            _ => return None,
        })
    }

    pub fn is_audio(self) -> bool {
        matches!(
            self,
            Self::Mp3
                | Self::M4a
                | Self::Aac
                | Self::Flac
                | Self::OggVorbis
                | Self::Opus
                | Self::Wav
                | Self::Aiff
                | Self::Caf
        )
    }

    pub fn is_container(self) -> bool {
        matches!(self, Self::Mp4 | Self::M4v)
    }

    /// Whether VAP can be embedded in native tags (vs sidecar-only).
    pub fn supports_native_vap_embed(self) -> bool {
        matches!(
            self,
            Self::Mp3
                | Self::Flac
                | Self::OggVorbis
                | Self::Opus
                | Self::M4a
                | Self::Mp4
                | Self::Aiff
                | Self::Wav
        )
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Mp3 => "MP3",
            Self::M4a => "M4A/AAC",
            Self::Aac => "AAC",
            Self::Flac => "FLAC",
            Self::OggVorbis => "Ogg Vorbis",
            Self::Opus => "Opus",
            Self::Wav => "WAV",
            Self::Aiff => "AIFF",
            Self::Mp4 => "MP4",
            Self::M4v => "M4V",
            Self::Caf => "CAF",
            Self::Unknown => "Unknown",
        }
    }

    /// Extensions accepted when scanning folders.
    pub fn all_extensions() -> &'static [&'static str] {
        &[
            "mp3", "m4a", "aac", "flac", "ogg", "oga", "opus", "wav", "aiff", "aif", "mp4", "m4v",
            "caf",
        ]
    }
}

/// Tag family used for VAP_OBJECT storage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TagFamily {
    Id3v2,
    VorbisComment,
    Mp4Freeform,
    RiffInfo,
    SidecarOnly,
}

impl MediaFormat {
    pub fn tag_family(self) -> TagFamily {
        match self {
            Self::Mp3 | Self::Aiff | Self::Wav => TagFamily::Id3v2,
            Self::Flac | Self::OggVorbis | Self::Opus => TagFamily::VorbisComment,
            Self::M4a | Self::Aac | Self::Mp4 | Self::M4v | Self::Caf => TagFamily::Mp4Freeform,
            Self::Unknown => TagFamily::SidecarOnly,
        }
    }
}
