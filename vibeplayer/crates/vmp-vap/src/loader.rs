//! VAP load chain: sidecar → (future ID3/Vorbis) → defaults.

use crate::error::VapResult;
use crate::types::VapObject;
use std::path::{Path, PathBuf};

/// Where the VAP payload was recovered from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadSource {
    Sidecar,
    /// Reserved for ID3v2 TXXX VAP_OBJECT
    Id3,
    /// Reserved for Vorbis COMMENT VAP_OBJECT=
    Vorbis,
    Defaults,
}

#[derive(Debug, Clone)]
pub struct LoadOutcome {
    pub source: LoadSource,
    pub object: VapObject,
    pub path: Option<PathBuf>,
}

/// Locates and parses V.A.P. metadata for an audio file.
pub struct VapLoader;

impl VapLoader {
    /// Resolve `<audio>.vap.json` next to the media file.
    pub fn sidecar_path(audio: impl AsRef<Path>) -> PathBuf {
        let audio = audio.as_ref();
        let mut p = audio.to_path_buf();
        // track.flac → track.flac.vap.json OR track.vap.json
        let with_suffix = PathBuf::from(format!("{}.vap.json", audio.display()));
        if with_suffix.exists() {
            return with_suffix;
        }
        if let Some(stem) = audio.file_stem().and_then(|s| s.to_str()) {
            if let Some(parent) = audio.parent() {
                let alt = parent.join(format!("{stem}.vap.json"));
                if alt.exists() {
                    return alt;
                }
                return alt;
            }
        }
        p.set_extension("vap.json");
        p
    }

    /// Full load priority chain. Never hard-fails for missing VAP (spec §3.2).
    pub fn load_for_audio(audio: impl AsRef<Path>) -> LoadOutcome {
        let audio = audio.as_ref();
        let side = Self::sidecar_path(audio);
        if side.exists() {
            if let Ok(object) = VapObject::from_path(&side) {
                return LoadOutcome {
                    source: LoadSource::Sidecar,
                    object,
                    path: Some(side),
                };
            }
        }

        // ID3 / Vorbis hooks — stubs until tag crates land in Phase 2.
        let (artist, title) = guess_identity(audio);
        LoadOutcome {
            source: LoadSource::Defaults,
            object: VapObject::defaults(&artist, &title),
            path: None,
        }
    }

    pub fn load_path(path: impl AsRef<Path>) -> VapResult<VapObject> {
        VapObject::from_path(path)
    }
}

fn guess_identity(audio: &Path) -> (String, String) {
    let title = audio
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Untitled")
        .to_string();
    ("Unknown".into(), title)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sidecar_path_prefers_stem() {
        let p = Path::new("/music/track.flac");
        let s = VapLoader::sidecar_path(p);
        assert!(s.to_string_lossy().contains("track"));
        assert!(s.to_string_lossy().ends_with(".vap.json"));
    }
}
