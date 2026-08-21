//! Read/write V.A.P. metadata on multi-format files via lofty + sidecar.

use crate::formats::{MediaFormat, TagFamily};
use lofty::config::WriteOptions;
use lofty::file::{AudioFile, TaggedFileExt};
use lofty::probe::Probe;
use lofty::tag::{Accessor, ItemKey, Tag, TagItem, TagType};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use thiserror::Error;
use vmp_vap::{LoadOutcome, LoadSource, VapLoader, VapObject};

/// Custom key for embedded VAP JSON blob.
pub const VAP_TAG_KEY: &str = "VAP_OBJECT";
pub const VAP_TAG_DESCRIPTION: &str = "Vibe Audio Protocol v3.1";

#[derive(Debug, Error)]
pub enum TagError {
    #[error("I/O: {0}")]
    Io(#[from] std::io::Error),
    #[error("lofty: {0}")]
    Lofty(String),
    #[error("VAP: {0}")]
    Vap(String),
    #[error("JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("{0}")]
    Message(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BibliographicTags {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub genre: Option<String>,
    pub year: Option<u32>,
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaTagBundle {
    pub path: PathBuf,
    pub format: MediaFormat,
    pub tag_family: TagFamily,
    pub bibliographic: BibliographicTags,
    pub vap: VapObject,
    pub vap_source: String,
    pub vap_sidecar_path: PathBuf,
    pub can_embed: bool,
}

/// Load VAP + bibliographic tags for any supported media path.
pub fn load_media_tags(path: impl AsRef<Path>) -> Result<MediaTagBundle, TagError> {
    let path = path.as_ref();
    let format = MediaFormat::from_path(path).unwrap_or(MediaFormat::Unknown);
    let tag_family = format.tag_family();
    let sidecar = VapLoader::sidecar_path(path);

    // 1) Sidecar first
    if sidecar.exists() {
        if let Ok(vap) = VapObject::from_path(&sidecar) {
            let bibliographic = read_biblio(path).unwrap_or_else(|_| BibliographicTags {
                title: Some(vap.identity.title.clone()),
                artist: Some(vap.identity.artist.clone()),
                album: None,
                genre: None,
                year: None,
                comment: None,
            });
            return Ok(MediaTagBundle {
                path: path.to_path_buf(),
                format,
                tag_family,
                bibliographic,
                vap,
                vap_source: format!("{:?}", LoadSource::Sidecar),
                vap_sidecar_path: sidecar,
                can_embed: format.supports_native_vap_embed(),
            });
        }
    }

    // 2) Embedded VAP_OBJECT via lofty
    if let Ok(Some(vap)) = read_embedded_vap(path) {
        let bibliographic = read_biblio(path).unwrap_or_else(|_| BibliographicTags {
            title: Some(vap.identity.title.clone()),
            artist: Some(vap.identity.artist.clone()),
            album: None,
            genre: None,
            year: None,
            comment: None,
        });
        return Ok(MediaTagBundle {
            path: path.to_path_buf(),
            format,
            tag_family,
            bibliographic,
            vap,
            vap_source: "Embedded".into(),
            vap_sidecar_path: sidecar,
            can_embed: format.supports_native_vap_embed(),
        });
    }

    // 3) Defaults + biblio from tags
    let bibliographic = read_biblio(path).unwrap_or(BibliographicTags {
        title: path
            .file_stem()
            .and_then(|s| s.to_str())
            .map(|s| s.to_string()),
        artist: Some("Unknown".into()),
        album: None,
        genre: None,
        year: None,
        comment: None,
    });
    let title = bibliographic
        .title
        .clone()
        .unwrap_or_else(|| "Untitled".into());
    let artist = bibliographic
        .artist
        .clone()
        .unwrap_or_else(|| "Unknown".into());
    let vap = VapObject::defaults(&artist, &title);

    Ok(MediaTagBundle {
        path: path.to_path_buf(),
        format,
        tag_family,
        bibliographic,
        vap,
        vap_source: format!("{:?}", LoadSource::Defaults),
        vap_sidecar_path: sidecar,
        can_embed: format.supports_native_vap_embed(),
    })
}

/// Save VAP to sidecar always; optionally embed into the media file tags.
pub fn save_vap_for_media(
    path: impl AsRef<Path>,
    vap: &VapObject,
    embed: bool,
) -> Result<SaveReport, TagError> {
    let path = path.as_ref();
    let format = MediaFormat::from_path(path).unwrap_or(MediaFormat::Unknown);
    let sidecar = VapLoader::sidecar_path(path);
    vap.save_path(&sidecar)
        .map_err(|e| TagError::Vap(e.to_string()))?;

    let mut report = SaveReport {
        sidecar: Some(sidecar),
        embedded: false,
        format,
        messages: vec!["Wrote .vap.json sidecar".into()],
    };

    if embed && format.supports_native_vap_embed() {
        match write_embedded_vap(path, vap) {
            Ok(()) => {
                report.embedded = true;
                report
                    .messages
                    .push(format!("Embedded VAP_OBJECT via {:?}", format.tag_family()));
            }
            Err(e) => {
                report
                    .messages
                    .push(format!("Embed skipped/failed: {e} (sidecar OK)"));
            }
        }
    } else if embed {
        report
            .messages
            .push("Format prefers sidecar-only for VAP".into());
    }

    // Sync TITLE/ARTIST into standard tags when possible
    if let Err(e) = write_biblio_identity(path, &vap.identity.title, &vap.identity.artist) {
        report
            .messages
            .push(format!("Bibliographic tag sync note: {e}"));
    }

    Ok(report)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveReport {
    pub sidecar: Option<PathBuf>,
    pub embedded: bool,
    pub format: MediaFormat,
    pub messages: Vec<String>,
}

fn read_biblio(path: &Path) -> Result<BibliographicTags, TagError> {
    let tagged = Probe::open(path)
        .map_err(|e| TagError::Lofty(e.to_string()))?
        .read()
        .map_err(|e| TagError::Lofty(e.to_string()))?;

    let tag = tagged.primary_tag().or_else(|| tagged.first_tag());
    Ok(match tag {
        Some(t) => BibliographicTags {
            title: t.title().map(|s| s.to_string()),
            artist: t.artist().map(|s| s.to_string()),
            album: t.album().map(|s| s.to_string()),
            genre: t.genre().map(|s| s.to_string()),
            year: t.year(),
            comment: t.comment().map(|s| s.to_string()),
        },
        None => BibliographicTags {
            title: None,
            artist: None,
            album: None,
            genre: None,
            year: None,
            comment: None,
        },
    })
}

fn read_embedded_vap(path: &Path) -> Result<Option<VapObject>, TagError> {
    let tagged = match Probe::open(path).and_then(|p| p.read()) {
        Ok(t) => t,
        Err(_) => return Ok(None),
    };

    for tag in tagged.tags() {
        // Description / comment style
        if let Some(item) = tag.get(&ItemKey::Description) {
            if let Some(text) = item.value().text() {
                if text.trim_start().starts_with('{') {
                    if let Ok(v) = VapObject::from_str(text) {
                        return Ok(Some(v));
                    }
                }
            }
        }
        if let Some(item) = tag.get(&ItemKey::Comment) {
            if let Some(text) = item.value().text() {
                if text.contains("VAP_VERSION") {
                    if let Ok(v) = VapObject::from_str(text) {
                        return Ok(Some(v));
                    }
                }
            }
        }
        // Unknown items often hold freeform keys
        for item in tag.items() {
            let key = format!("{:?}", item.key());
            if key.contains("VAP") || key.contains("Unknown") {
                if let Some(text) = item.value().text() {
                    if text.contains("VAP_VERSION") {
                        if let Ok(v) = VapObject::from_str(text) {
                            return Ok(Some(v));
                        }
                    }
                }
            }
        }
    }
    Ok(None)
}

fn write_embedded_vap(path: &Path, vap: &VapObject) -> Result<(), TagError> {
    let json = vap
        .to_pretty_json()
        .map_err(|e| TagError::Vap(e.to_string()))?;

    let mut tagged = Probe::open(path)
        .map_err(|e| TagError::Lofty(e.to_string()))?
        .read()
        .map_err(|e| TagError::Lofty(e.to_string()))?;

    let tag_type = preferred_tag_type(path).unwrap_or_else(|| tagged.primary_tag_type());

    if tagged.primary_tag().is_none() {
        tagged.insert_tag(Tag::new(tag_type));
    }

    let tag = tagged
        .primary_tag_mut()
        .ok_or_else(|| TagError::Message("failed to get primary tag".into()))?;

    // Store as comment + description for maximum reader compatibility
    tag.insert_text(ItemKey::Comment, json.clone());
    tag.insert_text(ItemKey::Description, json.clone());
    // Freeform / unknown custom key
    tag.insert(TagItem::new(
        ItemKey::Unknown(VAP_TAG_KEY.to_string()),
        lofty::tag::ItemValue::Text(json),
    ));

    tagged
        .save_to_path(path, WriteOptions::default())
        .map_err(|e| TagError::Lofty(e.to_string()))?;
    Ok(())
}

fn write_biblio_identity(path: &Path, title: &str, artist: &str) -> Result<(), TagError> {
    let mut tagged = Probe::open(path)
        .map_err(|e| TagError::Lofty(e.to_string()))?
        .read()
        .map_err(|e| TagError::Lofty(e.to_string()))?;

    let tag_type = preferred_tag_type(path).unwrap_or_else(|| tagged.primary_tag_type());

    if tagged.primary_tag().is_none() {
        tagged.insert_tag(Tag::new(tag_type));
    }
    let tag = tagged
        .primary_tag_mut()
        .ok_or_else(|| TagError::Message("no primary tag".into()))?;
    tag.set_title(title.to_string());
    tag.set_artist(artist.to_string());
    tagged
        .save_to_path(path, WriteOptions::default())
        .map_err(|e| TagError::Lofty(e.to_string()))?;
    Ok(())
}

fn preferred_tag_type(path: &Path) -> Option<TagType> {
    match MediaFormat::from_path(path)? {
        MediaFormat::Mp3 => Some(TagType::Id3v2),
        MediaFormat::Flac | MediaFormat::OggVorbis | MediaFormat::Opus => {
            Some(TagType::VorbisComments)
        }
        MediaFormat::M4a | MediaFormat::Mp4 | MediaFormat::M4v | MediaFormat::Aac => {
            Some(TagType::Mp4Ilst)
        }
        MediaFormat::Aiff | MediaFormat::Wav => Some(TagType::Id3v2),
        _ => None,
    }
}

/// Convenience: same as VapLoader but with tag embed path filled in.
pub fn load_vap_chain(path: impl AsRef<Path>) -> LoadOutcome {
    match load_media_tags(path.as_ref()) {
        Ok(bundle) => {
            let source = if bundle.vap_source.contains("Sidecar") {
                LoadSource::Sidecar
            } else if bundle.vap_source.contains("Embedded") {
                LoadSource::Id3
            } else {
                LoadSource::Defaults
            };
            LoadOutcome {
                source,
                object: bundle.vap,
                path: Some(bundle.vap_sidecar_path),
            }
        }
        Err(_) => VapLoader::load_for_audio(path),
    }
}
