```python
import json
import math


class VASPScoringEngine:
    """
    Reference Implementation of the Vibe Audio Standard and Protocol v3.69
    Logic Architecture.
    This engine converts raw audio analysis data into standardized
    Vibe Audio Metadata.
    """

    REQUIRED_PILLARS = [
        "STRUCTURAL",
        "TONAL",
        "TIMBRAL",
        "LINGUISTIC",
        "AFFECTIVE",
        "CONTEXTUAL",
        "PHOTOMETRIC",
        "KINETIC",
        "GENEALOGICAL",
    ]

    def __init__(self):
        self.version = "3.69"

    def _get(self, raw_data, key, default=None):
        if key in raw_data and raw_data[key] not in ("",):
            return raw_data[key]
        return default

    def _list_or_empty(self, raw_data, key):
        value = self._get(raw_data, key)
        if value is None:
            return []
        return value

    # --- PHASE I: PHYSICAL ANALYSIS (DSP Layer) ---
    def calculate_kick_profile(self, attack_ms):
        """Logic for Subset 1.3: Percussive DNA"""
        if attack_ms < 10:
            return "Sharp (Click)"
        elif 10 <= attack_ms <= 30:
            return "Punch (Thud)"
        else:
            return "Boom (Sub)"

    def calculate_spectral_color(self, centroid_hz):
        """Logic for Subset 3.1: Spectral Physics -> Tonal Color"""
        if centroid_hz < 200:
            return "Dark/Muddy"
        elif 200 <= centroid_hz <= 2000:
            return "Warm/Body"
        else:
            return "Bright/Airy"

    # --- PHASE II: PSYCHOLOGICAL ANALYSIS (NLP/ML Layer) ---
    def calculate_thayer_coordinates(self, key_mode, sentiment_score, rms_amplitude):
        """
        Logic for Subset 5.1: Affective (Thayer Model)
        key_mode: 'Major' or 'Minor'
        sentiment_score: -1.0 to 1.0 (from NLP)
        rms_amplitude: 0.0 to 1.0 (Loudness)
        """
        # Calculate Valence (Positivity)
        base_valence = 0.5 if key_mode == "Major" else -0.5
        valence = (base_valence + sentiment_score) / 2
        valence = max(-1.0, min(1.0, valence))  # Clamp values
        # Calculate Arousal (Energy)
        arousal = rms_amplitude
        return {
            "valence": round(valence, 2),
            "arousal": round(arousal, 2),
            "mood_quadrant": self._get_quadrant(valence, arousal),
        }

    def _get_quadrant(self, valence, arousal):
        if valence > 0 and arousal > 0.5:
            return "Euphoria/Joy"
        if valence > 0 and arousal <= 0.5:
            return "Calm/Content"
        if valence <= 0 and arousal > 0.5:
            return "Anger/Fear"
        return "Depression/Melancholy"

    def calculate_tonal_profile(self, raw_data):
        """
        Deterministic TONAL heuristic (Intro Specs Pillar 2).
        Uses key_mode / key_signature / dissonance when present.
        Does not invent a root note, chord vocabulary, or tuning standard.
        """
        key_signature = self._get(raw_data, "key_signature")
        key_mode = self._get(raw_data, "key_mode")
        if key_signature is None and key_mode is not None:
            key_signature = "unknown {}".format(key_mode)
        elif key_signature is None:
            key_signature = "unknown"

        if "dissonance_rating" in raw_data:
            dissonance = raw_data["dissonance_rating"]
        elif key_mode == "Minor":
            dissonance = 0.45
        elif key_mode == "Major":
            dissonance = 0.15
        else:
            dissonance = None

        return {
            "HARMONIC_PROFILE": {
                "KEY_SIGNATURE": key_signature,
                "CHORD_COMPLEXITY": self._get(raw_data, "chord_complexity", "unknown"),
                "DISSONANCE_RATING": dissonance,
            },
            "MELODIC_CONTOUR": {
                "RANGE_SPAN": self._get(raw_data, "range_span"),
                "HOOK_STRENGTH": self._get(raw_data, "hook_strength"),
                "MELODIC_MOTION": self._get(raw_data, "melodic_motion", "unknown"),
            },
            "TUNING_STANDARD": {
                "REFERENCE_PITCH": self._get(raw_data, "reference_pitch", "unknown"),
                "MICROTONALITY": self._get(raw_data, "microtonality", "unknown"),
            },
        }

    def calculate_linguistic_profile(self, raw_data):
        """
        Deterministic LINGUISTIC heuristic (Intro Specs Pillar 4).
        Uses explicit_filter, language, and lyric fields when present.
        Does not infer profanity, topic, or dialect from sentiment alone.
        """
        return {
            "SEMANTIC_CONTENT": {
                "EXPLICIT_FILTER": self._get(raw_data, "explicit_filter", "unknown"),
                "TOPIC_CLUSTERS": self._list_or_empty(raw_data, "topic_clusters"),
                "NARRATIVE_ARC": self._get(raw_data, "narrative_arc", "unknown"),
            },
            "VOCAL_TEXTURE": {
                "POSITION": self._get(raw_data, "vocal_position", "unknown"),
                "DELIVERY_STYLE": self._get(raw_data, "delivery_style", "unknown"),
                "PROCESSING": self._get(raw_data, "vocal_processing", "unknown"),
            },
            "LANGUAGE_PROFILE": {
                "PRIMARY_LANGUAGE": self._get(raw_data, "primary_language", "unknown"),
                "DIALECT_SLANG": self._get(raw_data, "dialect_slang", "unknown"),
            },
        }

    def calculate_contextual_profile(self, raw_data, arousal):
        """
        Deterministic CONTEXTUAL heuristic (Intro Specs Pillar 6).
        Maps BPM + arousal + sentiment onto scenario clusters when those
        DSP/NLP values exist. Catalog-only fields stay unknown.
        """
        bpm = self._get(raw_data, "bpm")
        sentiment = self._get(raw_data, "sentiment_score", 0.0)
        if self._get(raw_data, "macro_setting") is not None:
            macro = raw_data["macro_setting"]
            micro = self._get(raw_data, "micro_activity", "unknown")
            social = self._get(raw_data, "social_setting", "unknown")
            goal = self._get(raw_data, "functional_goal", "unknown")
            time_of_day = self._get(raw_data, "time_of_day", "unknown")
        elif bpm is not None and arousal is not None and bpm >= 140 and arousal >= 0.8:
            macro = "Gym"
            micro = "Heavy Lifting"
            social = "Crowd/Mass"
            goal = "Hype"
            time_of_day = "unknown"
        elif bpm is not None and sentiment is not None and bpm <= 130 and sentiment <= 0:
            macro = "Car"
            micro = "Night Drive"
            social = "Solo"
            goal = "unknown"
            time_of_day = "Late Night"
        elif arousal is not None and arousal < 0.3:
            macro = "Bedroom"
            micro = "Sleep"
            social = "Solo"
            goal = "Sleep"
            time_of_day = "unknown"
        else:
            macro = "unknown"
            micro = "unknown"
            social = "unknown"
            goal = "unknown"
            time_of_day = "unknown"
        return {
            "SCENARIO_ENGINE": {
                "MACRO_SETTING": macro,
                "MICRO_ACTIVITY": micro,
                "SOCIAL_SETTING": social,
            },
            "INTENT_VECTORS": {
                "FUNCTIONAL_GOAL": goal,
                "TIME_OF_DAY": time_of_day,
            },
            "METEOROLOGICAL_MATCH": {
                "WEATHER": self._get(raw_data, "weather", "unknown"),
                "TEMPERATURE": self._get(raw_data, "temperature", "unknown"),
            },
        }

    def calculate_genealogical_profile(self, raw_data):
        """
        Deterministic GENEALOGICAL heuristic (Intro Specs Pillar 9).
        Passes through catalog facts from raw_data. Missing samples,
        release date, cultural era, and tribe alignment are null,
        empty arrays, or unknown — never invented.
        """
        return {
            "ERA_ANCHORING": {
                "RELEASE_DATE": self._get(raw_data, "release_date"),
                "CULTURAL_ERA": self._get(raw_data, "cultural_era", "unknown"),
                "TIMELESSNESS_SCORE": self._get(raw_data, "timelessness_score"),
            },
            "DNA_SAMPLING": {
                "SAMPLE_LINEAGE": self._list_or_empty(raw_data, "sample_lineage"),
                "INTERPOLATION": self._list_or_empty(raw_data, "interpolation"),
                "GENRE_TREE": self._get(raw_data, "genre_tree", "unknown"),
            },
            "TRIBE_ALIGNMENT": {
                "SUBCULTURE_ID": self._get(raw_data, "subculture_id", "unknown"),
                "AUTHENTICITY_SCORE": self._get(raw_data, "authenticity_score"),
                "VIRAL_VELOCITY": self._get(raw_data, "viral_velocity", "unknown"),
            },
        }

    # --- PHASE III: ENVIRONMENTAL ANALYSIS (I/O Layer) ---
    def calculate_photometric_hex(self, dominant_freq_hz):
        """
        Logic for Subset 7.1: Photometric (Chromatic Map)
        Maps Audio Frequency to Visual Wavelength (Approximation)
        """
        if dominant_freq_hz < 60:
            return "#8B0000"  # Deep Red (Sub Bass)
        elif 60 <= dominant_freq_hz < 250:
            return "#FF8C00"  # Dark Orange (Low Mids)
        elif 250 <= dominant_freq_hz < 2000:
            return "#008080"  # Teal (Mids/Vocals)
        else:
            return "#4B0082"  # Indigo/UV (Highs/Air)

    def calculate_palette_temperature(self, dominant_freq_hz):
        if dominant_freq_hz < 250:
            return "Warm"
        return "Cool"

    def calculate_kinetic_met(self, bpm):
        """Logic for Subset 8.1: Kinetic (Biometrics)"""
        if bpm < 60:
            return 1.0  # Rest
        elif 60 <= bpm < 100:
            return 3.0  # Light Activity
        elif 100 <= bpm < 140:
            return 6.0  # Moderate Activity
        else:
            return 8.0  # High Intensity (Sprint)

    # --- MASTER GENERATOR ---
    def generate_vap_profile(self, raw_data):
        """
        Orchestrates the analysis phases to build the full VASP object.
        """
        # Phase I
        kick_profile = self.calculate_kick_profile(raw_data["attack_ms"])
        spectral_tone = self.calculate_spectral_color(raw_data["centroid_hz"])
        bpm = raw_data["bpm"]
        if kick_profile.startswith("Sharp"):
            attack_label = "Sharp"
            decay_label = "Short"
        elif kick_profile.startswith("Punch"):
            attack_label = "Soft"
            decay_label = "Short"
        else:
            attack_label = "Soft"
            decay_label = "Long"

        # Phase II
        affective = self.calculate_thayer_coordinates(
            raw_data["key_mode"],
            raw_data["sentiment_score"],
            raw_data["rms_amplitude"],
        )
        valence = affective["valence"]
        arousal = affective["arousal"]
        if valence <= 0 and arousal > 0.5:
            dominance = "Aggressive"
        elif valence > 0 and arousal > 0.5:
            dominance = "Empowering"
        else:
            dominance = "Vulnerable"

        tonal = self.calculate_tonal_profile(raw_data)
        linguistic = self.calculate_linguistic_profile(raw_data)
        contextual = self.calculate_contextual_profile(raw_data, arousal)
        genealogical = self.calculate_genealogical_profile(raw_data)

        # Phase III
        chroma_hex = self.calculate_photometric_hex(raw_data["dominant_freq_hz"])
        palette_temp = self.calculate_palette_temperature(raw_data["dominant_freq_hz"])
        met_score = self.calculate_kinetic_met(bpm)

        vap_object = {
            "VAP_VERSION": self.version,
            "IDENTITY": {
                "TITLE": self._get(raw_data, "title") or self._get(raw_data, "TITLE") or "unknown",
                "ARTIST": self._get(raw_data, "artist") or self._get(raw_data, "ARTIST") or "unknown",
                "ISRC": self._get(raw_data, "isrc") or self._get(raw_data, "ISRC"),
                "SOURCE_DNA": self._get(raw_data, "source_dna") or self._get(raw_data, "SOURCE_DNA"),
            },
            "PILLARS": {
                "STRUCTURAL": {
                    "TEMPORAL_DYNAMICS": {
                        "BPM_RAW": bpm,
                        "BPM_PERCEIVED": self._get(raw_data, "bpm_perceived", "unknown"),
                        "GROOVE_QUANTIZATION": self._get(raw_data, "groove_quantization", "unknown"),
                        "TIME_SIGNATURE": self._get(raw_data, "time_signature", "unknown"),
                    },
                    "ARRANGEMENT_ARCHITECTURE": {
                        "SECTIONAL_MARKERS": self._list_or_empty(raw_data, "sectional_markers"),
                        "MIX_WINDOW_INDEX": self._get(raw_data, "mix_window_index"),
                        "BREAKDOWN_DEPTH": self._get(raw_data, "breakdown_depth"),
                    },
                    "PERCUSSIVE_DNA": {
                        "KICK_TRANSIENT": {
                            "ATTACK": attack_label,
                            "DECAY": decay_label,
                            "PROFILE": kick_profile,
                        },
                        "SYNCOPATION_INDEX": self._get(raw_data, "syncopation_index"),
                        "GHOST_NOTE_DENSITY": self._get(raw_data, "ghost_note_density"),
                    },
                },
                "TONAL": tonal,
                "TIMBRAL": {
                    "SPECTRAL_PHYSICS": {
                        "FREQUENCY_BALANCE": {
                            "SUB_DOMINANT": self._get(raw_data, "sub_dominant"),
                            "MID_FORWARD": self._get(raw_data, "mid_forward"),
                            "AIR_BRILLIANCE": self._get(raw_data, "air_brilliance"),
                        },
                        "SPECTRAL_SATURATION": self._get(raw_data, "spectral_saturation"),
                        "SPECTRAL_CENTROID": spectral_tone,
                    },
                    "PRODUCTION_AESTHETIC": {
                        "FIDELITY_SCORE": self._get(raw_data, "fidelity_score", "unknown"),
                        "DYNAMIC_RANGE_LRA": self._get(raw_data, "dynamic_range_lra"),
                        "SPATIAL_WIDTH": self._get(raw_data, "spatial_width", "unknown"),
                    },
                    "TEXTURE_GRAIN": {
                        "SURFACE": self._get(raw_data, "texture_surface", "unknown"),
                        "ARTIFACTS": self._get(raw_data, "texture_artifacts"),
                    },
                },
                "LINGUISTIC": linguistic,
                "AFFECTIVE": {
                    "THAYER_COORDINATES": {
                        "VALENCE": valence,
                        "AROUSAL": arousal,
                        "DOMINANCE": dominance,
                    },
                    "EMOTIONAL_COMPLEXITY": {
                        "MOOD_STABILITY": self._get(raw_data, "mood_stability", "unknown"),
                        "CATHARSIS_POTENTIAL": self._get(raw_data, "catharsis_potential"),
                        "NOSTALGIA_TRIGGER": self._get(raw_data, "nostalgia_trigger"),
                    },
                    "TENSION_ARC": {
                        "BUILD_UP_VELOCITY": self._get(raw_data, "build_up_velocity"),
                        "RESOLUTION_STATE": self._get(raw_data, "resolution_state", "unknown"),
                    },
                },
                "CONTEXTUAL": contextual,
                "PHOTOMETRIC": {
                    "CHROMATIC_MAP": {
                        "PRIMARY_HEX": chroma_hex,
                        "SECONDARY_HEX": self._get(raw_data, "secondary_hex"),
                        "PALETTE_TEMPERATURE": palette_temp,
                    },
                    "LUMEN_DYNAMICS": {
                        "BRIGHTNESS_FLOOR": self._get(raw_data, "brightness_floor"),
                        "BRIGHTNESS_CEILING": self._get(raw_data, "brightness_ceiling"),
                        "STROBE_TRIGGER": self._get(raw_data, "strobe_trigger"),
                        "FADE_RATE": self._get(raw_data, "fade_rate", "unknown"),
                    },
                    "VISUAL_TEXTURE": {
                        "FOG_DENSITY": self._get(raw_data, "fog_density"),
                        "LASER_COMPATIBILITY": self._get(raw_data, "laser_compatibility"),
                        "VISUAL_NOISE": self._get(raw_data, "visual_noise", "unknown"),
                    },
                },
                "KINETIC": {
                    "BIOMETRIC_ENTRAINMENT": {
                        "TARGET_HR_ZONE": "{}-{}".format(int(bpm - 20), int(bpm + 10)),
                        "HRV_IMPACT": "Low HRV" if arousal > 0.7 else "High HRV",
                        "BREATH_RATE": self._get(raw_data, "breath_rate"),
                    },
                    "MOTOR_RESPONSE": {
                        "DRIVE": round(min(1.0, met_score / 8.0), 2),
                        "SWAY": self._get(raw_data, "sway"),
                        "HEAD_NOD": round(min(1.0, max(0.0, (bpm - 60) / 80.0)), 2),
                    },
                    "ENERGY_EXPENDITURE": {
                        "MET_SCORE": met_score,
                    },
                },
                "GENEALOGICAL": genealogical,
            },
        }
        missing = [k for k in self.REQUIRED_PILLARS if k not in vap_object["PILLARS"]]
        if missing:
            raise ValueError("Incomplete VASP profile, missing pillars: {}".format(missing))
        return json.dumps(vap_object, indent=2)


# --- SIMULATION ---
# Mock Data (representing "After Dark" by Mr.Kitty)
mock_audio_analysis = {
    "title": "After Dark",
    "artist": "Mr.Kitty",
    "bpm": 135,
    "attack_ms": 45,            # Slow attack (Synth wash)
    "centroid_hz": 2200,        # Bright/Airy
    "key_mode": "Minor",
    "sentiment_score": -0.2,    # Melancholic
    "rms_amplitude": 0.75,      # Steady loud
    "dominant_freq_hz": 3000    # High synth lead focus
}

# Run Engine
engine = VASPScoringEngine()
vap_profile = engine.generate_vap_profile(mock_audio_analysis)
print("--- Vibe Audio v3.69 Generated Metadata ---")
print(vap_profile)
```
