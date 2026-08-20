#!/usr/bin/env python3
"""
vap_tagger.py — V.A.P. v3.1 Sidecar Generator
Aurphyx SUXS / rAE
Produces a .vap.json file alongside any audio file.
Usage: python vap_tagger.py <audio_file.mp3>
Requires: librosa, numpy, scipy
"""

import json
import os
import sys

import librosa
import numpy as np


def hex_to_rgb_norm(hex_str):
    h = hex_str.lstrip("#")
    return [int(h[i : i + 2], 16) / 255.0 for i in (0, 2, 4)]


def analyze(filepath: str) -> dict:
    y, sr = librosa.load(filepath, sr=44100, mono=True)

    # ── PHASE I: Physical (DSP) ──────────────────────────────────────
    # Pillar 1: Structural
    tempo, beats = librosa.beat.beat_track(y=y, sr=sr)
    bpm_raw = float(tempo)

    onset_env = librosa.onset.onset_strength(y=y, sr=sr)
    syncopation = float(np.std(onset_env) / (np.mean(onset_env) + 1e-6))
    syncopation = min(syncopation / 3.0, 1.0)  # normalize to 0-1

    # Pillar 3: Timbral — Spectral Centroid
    centroid = librosa.feature.spectral_centroid(y=y, sr=sr)
    centroid_mean = float(np.mean(centroid))

    # Saturation Index (THD proxy via spectral flatness)
    flatness = librosa.feature.spectral_flatness(y=y)
    saturation = float(np.mean(flatness))

    # Dynamic range (LRA proxy)
    rms = librosa.feature.rms(y=y)
    lra = float(20 * np.log10(np.max(rms) / (np.mean(rms) + 1e-9) + 1e-9))

    # Pillar 2: Tonal — Key detection
    chroma = librosa.feature.chroma_cqt(y=y, sr=sr)
    key_idx = int(np.argmax(np.mean(chroma, axis=1)))
    keys = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"]
    key_str = keys[key_idx]

    # ── PHASE II: Psychological ──────────────────────────────────────
    # Pillar 5: Affective — Thayer Model
    # Arousal: RMS amplitude + BPM + spectral density
    rms_norm = float(np.clip(np.mean(rms) * 10, 0, 1))
    bpm_norm = float(np.clip(bpm_raw / 180.0, 0, 1))
    cent_norm = float(np.clip(centroid_mean / 5000.0, 0, 1))
    arousal = rms_norm * 0.5 + bpm_norm * 0.3 + cent_norm * 0.2

    # Valence: major/minor proxy via chroma energy distribution
    major_sum = float(sum(np.mean(chroma, axis=1)[[0, 2, 4, 5, 7, 9, 11]]))
    minor_sum = float(sum(np.mean(chroma, axis=1)[[0, 2, 3, 5, 7, 8, 10]]))
    valence = float(
        np.clip((major_sum - minor_sum) / (major_sum + minor_sum + 1e-6), -1, 1)
    )

    # ── PHASE III: I/O (Photometric pillar) ─────────────────────────
    # Derive primary hex from spectral centroid wavelength per VAP spec
    if centroid_mean < 200:
        primary_hex, palette_temp = "#8B0000", 0.9  # Deep Red
    elif centroid_mean < 2000:
        primary_hex, palette_temp = "#FF8C00", 0.7  # Orange/Amber
    else:
        primary_hex, palette_temp = "#4B0082", 0.2  # Blue/Violet

    secondary_hex = "#7B14C8"  # Aurphyx brand violet
    brightness_floor = 0.05
    brightness_ceiling = 1.0
    strobe_threshold = 0.85 if bpm_raw > 130 else 1.0  # no strobe for slow tracks
    fog_density = max(0.0, 0.3 - arousal * 0.4)  # less fog = more energy
    visual_noise = min(saturation, 0.6)

    vap_object = {
        "VAP_VERSION": "3.1",
        "IDENTITY": {
            "TITLE": os.path.basename(filepath),
            "ARTIST": "Unknown",
            "SOURCE_DNA": filepath,
        },
        "PILLARS": {
            "STRUCTURAL": {
                "BPM_RAW": bpm_raw,
                "GROOVE_QUANTIZATION": "MACHINE_LOCK"
                if syncopation < 0.2
                else "HUMAN_SWING",
                "SYNCOPATION_INDEX": syncopation,
            },
            "TONAL": {
                "KEY": key_str,
                "DISSONANCE_RATING": float(
                    np.clip(1.0 - (major_sum / (major_sum + minor_sum + 1e-6)), 0, 1)
                ),
            },
            "TIMBRAL": {
                "SPECTRAL_CENTROID_HZ": centroid_mean,
                "SATURATION_INDEX": saturation,
                "DYNAMIC_RANGE_LRA": lra,
                "FIDELITY": "HI-FI" if lra > 10 else "LO-FI",
            },
            "AFFECTIVE": {
                "VALENCE": valence,
                "AROUSAL": arousal,
                "DOMINANCE": float(
                    np.clip(bpm_norm * 0.7 + (1.0 - valence) * 0.3, 0, 1)
                ),
            },
            "PHOTOMETRIC": {
                "PRIMARY_HEX": primary_hex,
                "SECONDARY_HEX": secondary_hex,
                "PALETTE_TEMP": palette_temp,
                "BRIGHTNESS_FLOOR": brightness_floor,
                "BRIGHTNESS_CEILING": brightness_ceiling,
                "STROBE_TRIGGER": strobe_threshold,
                "FADE_MODE": "SMOOTH",
                "FOG_DENSITY": fog_density,
                "VISUAL_NOISE": visual_noise,
            },
            "KINETIC": {
                "TARGET_HR_ZONE": f"{int(bpm_raw * 0.8)}-{int(bpm_raw * 1.2)}",
                "MET_SCORE": 1.0
                if bpm_raw < 60
                else (3.0 if bpm_raw < 100 else (5.0 if bpm_raw < 140 else 8.0)),
                "ENTRAINMENT_FACTOR": float(np.clip(arousal * 100, 0, 100)),
            },
        },
    }

    return vap_object


if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python vap_tagger.py <audio_file>")
        sys.exit(1)

    audio_path = sys.argv[1]
    vap_data = analyze(audio_path)
    out_path = audio_path.rsplit(".", 1)[0] + ".vap.json"

    with open(out_path, "w") as f:
        json.dump(vap_data, f, indent=2)

    print(f"✅ V.A.P. v3.1 sidecar written: {out_path}")
    print(f"   BPM: {vap_data['PILLARS']['STRUCTURAL']['BPM_RAW']:.1f}")
    print(f"   Key: {vap_data['PILLARS']['TONAL']['KEY']}")
    print(f"   Valence: {vap_data['PILLARS']['AFFECTIVE']['VALENCE']:.2f}")
    print(f"   Arousal: {vap_data['PILLARS']['AFFECTIVE']['AROUSAL']:.2f}")
    print(f"   Primary Color: {vap_data['PILLARS']['PHOTOMETRIC']['PRIMARY_HEX']}")
