//! Multi-format audio for Vibe Media Player.
//!
//! - Probe + decode: Symphonia (MP3, M4A/AAC, FLAC, Ogg, WAV, AIFF, MP4, …)
//! - Tags: lofty (VAP sidecar + native embed)
//! - Playback: cpal stream + EQ (`playback` feature)

mod decode;
mod devices;
mod formats;
mod player;
mod probe;
mod tags;

pub use decode::{decode_file, DecodeError, DecodedTrack};
pub use devices::{list_devices, AudioDeviceInfo, DeviceInventory};
pub use formats::*;
pub use player::{render_frames, PlayerEngine, PlayerError, PlayerStatus};
pub use probe::*;
pub use tags::{
    load_media_tags, load_vap_chain, save_vap_for_media, BibliographicTags, MediaTagBundle,
    SaveReport, TagError, VAP_TAG_DESCRIPTION, VAP_TAG_KEY,
};

/// Human-readable accept list for file dialogs.
pub const OPEN_DIALOG_FILTER: &str = "\
Media (*.mp3 *.m4a *.aac *.flac *.ogg *.oga *.opus *.wav *.aiff *.aif *.mp4 *.m4v *.caf);;\
Lossless (*.flac *.wav *.aiff *.aif);;\
Lossy (*.mp3 *.m4a *.aac *.ogg *.opus);;\
Video containers (*.mp4 *.m4v);;\
All (*)";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_extensions() {
        assert!(MediaFormat::from_path("x.mp3").unwrap().is_audio());
        assert!(MediaFormat::from_path("x.flac").unwrap().is_audio());
        assert!(MediaFormat::from_path("x.m4a").unwrap().is_audio());
        assert!(MediaFormat::from_path("x.ogg").unwrap().is_audio());
        assert!(MediaFormat::from_path("x.wav").unwrap().is_audio());
        assert!(MediaFormat::from_path("x.mp4").unwrap().is_container());
        assert!(MediaFormat::from_path("x.txt").is_none());
    }

    #[test]
    fn dialog_filter_mentions_flac() {
        assert!(OPEN_DIALOG_FILTER.contains("flac"));
    }
}
