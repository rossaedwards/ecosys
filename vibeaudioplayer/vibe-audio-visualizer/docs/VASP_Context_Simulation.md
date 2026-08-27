```python
import json


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


class VAPContextEngine:
    """
    Simulates the 'Contextual Override' layer of the Vibe Audio
    Protocol.
    This engine filters and modifies track metadata based on active
    scenarios.
    """

    def __init__(self):
        self.active_context = None
        self.override_rules = {
            "GYM_PEAK": {
                "min_bpm": 120,
                "min_arousal": 0.7,
                "min_met": 6.0,
                "lighting_modifier": "FORCE_RED_SHIFT",
                "narrative_filter": ["relax", "sleep", "sad"]
            },
            "NIGHT_DRIVE": {
                "max_bpm": 130,
                "min_valence": -0.5,
                "lighting_modifier": "FORCE_COOL_SHIFT",
                "narrative_filter": ["hype", "scream"]
            }
        }

    def set_context(self, context_tag):
        if context_tag in self.override_rules:
            self.active_context = context_tag
            print("STATUS: Context set to [{}]".format(context_tag))
        else:
            print("ERROR: Unknown Context")

    def require_complete_profile(self, track_vap):
        pillars = track_vap.get("PILLARS", {})
        missing = [key for key in REQUIRED_PILLARS if key not in pillars]
        if missing:
            raise ValueError(
                "Incomplete VASP profile, missing pillars: {}".format(missing)
            )
        return True

    def apply_override(self, track_vap):
        """
        Evaluates a track's VASP profile against the active context
        rules.
        Returns a modified 'Contextual Profile' with compatibility
        scores.
        """
        self.require_complete_profile(track_vap)
        if not self.active_context:
            return {
                "status": "PASSTHROUGH",
                "compatibility": 1.0,
                "data": track_vap,
            }
        rules = self.override_rules[self.active_context]
        score = 1.0
        modifications = []
        rejection_reasons = []

        # 1. KINETIC CHECK (Heart Rate / BPM)
        track_bpm = track_vap["PILLARS"]["STRUCTURAL"]["TEMPORAL_DYNAMICS"]["BPM_RAW"]
        track_met = track_vap["PILLARS"]["KINETIC"]["ENERGY_EXPENDITURE"]["MET_SCORE"]

        if "min_bpm" in rules and track_bpm < rules["min_bpm"]:
            score -= 0.4
            rejection_reasons.append(
                "BPM {} too low for {}".format(track_bpm, self.active_context)
            )

        if "min_met" in rules and track_met < rules["min_met"]:
            score -= 0.3
            rejection_reasons.append(
                "MET Score {} insufficient for workout".format(track_met)
            )

        # 2. AFFECTIVE CHECK (Energy/Arousal)
        track_arousal = track_vap["PILLARS"]["AFFECTIVE"]["THAYER_COORDINATES"]["AROUSAL"]
        if "min_arousal" in rules and track_arousal < rules["min_arousal"]:
            score -= 0.3
            rejection_reasons.append("Energy level too low")

        # 3. PHOTOMETRIC OVERRIDE (Lighting)
        chromatic = track_vap["PILLARS"]["PHOTOMETRIC"]["CHROMATIC_MAP"]
        final_hex = chromatic["PRIMARY_HEX"]
        if rules["lighting_modifier"] == "FORCE_RED_SHIFT":
            if chromatic["PALETTE_TEMPERATURE"] == "Cool":
                final_hex = "#FF4500"
                modifications.append("Lighting forced to High-Energy Red")

        # 4. FINAL VERDICT
        status = "ACCEPTED"
        if score < 0.6:
            status = "REJECTED"
        elif score < 0.8:
            status = "BORDERLINE"
        return {
            "context": self.active_context,
            "track_title": track_vap["IDENTITY"]["TITLE"],
            "status": status,
            "compatibility_score": round(max(0, score), 2),
            "overrides_applied": modifications,
            "rejection_log": rejection_reasons,
            "output_vap": {
                "final_hex": final_hex,
                "target_hr": rules.get("min_bpm", 0),
            },
        }


# --- TEST DATA ---
# Track 1: High Energy (Matches Gym)
track_a = {
    "VASP_VERSION": "3.69",
    "IDENTITY": {"TITLE": "Work", "ARTIST": "Bear Grillz"},
    "PILLARS": {
        "STRUCTURAL": {
            "TEMPORAL_DYNAMICS": {
                "BPM_RAW": 150,
                "BPM_PERCEIVED": "1.0x",
                "GROOVE_QUANTIZATION": "Machine Lock",
                "TIME_SIGNATURE": "4/4"
            },
            "ARRANGEMENT_ARCHITECTURE": {
                "SECTIONAL_MARKERS": [],
                "MIX_WINDOW_INDEX": None,
                "BREAKDOWN_DEPTH": None
            },
            "PERCUSSIVE_DNA": {
                "KICK_TRANSIENT": {"ATTACK": "Sharp", "DECAY": "Short", "PROFILE": "Sharp (Click)"},
                "SYNCOPATION_INDEX": 0.2,
                "GHOST_NOTE_DENSITY": None
            }
        },
        "TONAL": {
            "HARMONIC_PROFILE": {
                "KEY_SIGNATURE": "unknown",
                "CHORD_COMPLEXITY": "unknown",
                "DISSONANCE_RATING": None
            },
            "MELODIC_CONTOUR": {
                "RANGE_SPAN": None,
                "HOOK_STRENGTH": None,
                "MELODIC_MOTION": "unknown"
            },
            "TUNING_STANDARD": {
                "REFERENCE_PITCH": "unknown",
                "MICROTONALITY": "unknown"
            }
        },
        "TIMBRAL": {
            "SPECTRAL_PHYSICS": {
                "FREQUENCY_BALANCE": {
                    "SUB_DOMINANT": None,
                    "MID_FORWARD": None,
                    "AIR_BRILLIANCE": None
                },
                "SPECTRAL_SATURATION": None,
                "SPECTRAL_CENTROID": "unknown"
            },
            "PRODUCTION_AESTHETIC": {
                "FIDELITY_SCORE": "unknown",
                "DYNAMIC_RANGE_LRA": None,
                "SPATIAL_WIDTH": "unknown"
            },
            "TEXTURE_GRAIN": {"SURFACE": "unknown", "ARTIFACTS": None}
        },
        "LINGUISTIC": {
            "SEMANTIC_CONTENT": {
                "EXPLICIT_FILTER": "unknown",
                "TOPIC_CLUSTERS": [],
                "NARRATIVE_ARC": "unknown"
            },
            "VOCAL_TEXTURE": {
                "POSITION": "unknown",
                "DELIVERY_STYLE": "unknown",
                "PROCESSING": "unknown"
            },
            "LANGUAGE_PROFILE": {
                "PRIMARY_LANGUAGE": "unknown",
                "DIALECT_SLANG": "unknown"
            }
        },
        "AFFECTIVE": {
            "THAYER_COORDINATES": {
                "VALENCE": 0.1,
                "AROUSAL": 0.95,
                "DOMINANCE": "Empowering"
            },
            "EMOTIONAL_COMPLEXITY": {
                "MOOD_STABILITY": "unknown",
                "CATHARSIS_POTENTIAL": None,
                "NOSTALGIA_TRIGGER": None
            },
            "TENSION_ARC": {
                "BUILD_UP_VELOCITY": None,
                "RESOLUTION_STATE": "unknown"
            }
        },
        "CONTEXTUAL": {
            "SCENARIO_ENGINE": {
                "MACRO_SETTING": "Gym",
                "MICRO_ACTIVITY": "Heavy Lifting",
                "SOCIAL_SETTING": "Crowd/Mass"
            },
            "INTENT_VECTORS": {
                "FUNCTIONAL_GOAL": "Hype",
                "TIME_OF_DAY": "unknown"
            },
            "METEOROLOGICAL_MATCH": {
                "WEATHER": "unknown",
                "TEMPERATURE": "unknown"
            }
        },
        "PHOTOMETRIC": {
            "CHROMATIC_MAP": {
                "PRIMARY_HEX": "#FF0000",
                "SECONDARY_HEX": None,
                "PALETTE_TEMPERATURE": "Warm"
            },
            "LUMEN_DYNAMICS": {
                "BRIGHTNESS_FLOOR": None,
                "BRIGHTNESS_CEILING": None,
                "STROBE_TRIGGER": None,
                "FADE_RATE": "unknown"
            },
            "VISUAL_TEXTURE": {
                "FOG_DENSITY": None,
                "LASER_COMPATIBILITY": None,
                "VISUAL_NOISE": "unknown"
            }
        },
        "KINETIC": {
            "BIOMETRIC_ENTRAINMENT": {
                "TARGET_HR_ZONE": "130-160",
                "HRV_IMPACT": "Low HRV",
                "BREATH_RATE": None
            },
            "MOTOR_RESPONSE": {"DRIVE": 1.0, "SWAY": None, "HEAD_NOD": 0.9},
            "ENERGY_EXPENDITURE": {"MET_SCORE": 9.0}
        },
        "GENEALOGICAL": {
            "ERA_ANCHORING": {
                "RELEASE_DATE": None,
                "CULTURAL_ERA": "unknown",
                "TIMELESSNESS_SCORE": None
            },
            "DNA_SAMPLING": {
                "SAMPLE_LINEAGE": [],
                "INTERPOLATION": [],
                "GENRE_TREE": "unknown"
            },
            "TRIBE_ALIGNMENT": {
                "SUBCULTURE_ID": "unknown",
                "AUTHENTICITY_SCORE": None,
                "VIRAL_VELOCITY": "unknown"
            }
        }
    }
}

# Track 2: Low Energy (Clashes with Gym)
track_b = {
    "VASP_VERSION": "3.69",
    "IDENTITY": {"TITLE": "Dreams", "ARTIST": "The Cranberries"},
    "PILLARS": {
        "STRUCTURAL": {
            "TEMPORAL_DYNAMICS": {
                "BPM_RAW": 128,
                "BPM_PERCEIVED": "1.0x",
                "GROOVE_QUANTIZATION": "Human Swing",
                "TIME_SIGNATURE": "4/4"
            },
            "ARRANGEMENT_ARCHITECTURE": {
                "SECTIONAL_MARKERS": [],
                "MIX_WINDOW_INDEX": None,
                "BREAKDOWN_DEPTH": None
            },
            "PERCUSSIVE_DNA": {
                "KICK_TRANSIENT": {"ATTACK": "Soft", "DECAY": "Short", "PROFILE": "Punch (Thud)"},
                "SYNCOPATION_INDEX": 0.15,
                "GHOST_NOTE_DENSITY": None
            }
        },
        "TONAL": {
            "HARMONIC_PROFILE": {
                "KEY_SIGNATURE": "unknown",
                "CHORD_COMPLEXITY": "Triadic",
                "DISSONANCE_RATING": 0.1
            },
            "MELODIC_CONTOUR": {
                "RANGE_SPAN": None,
                "HOOK_STRENGTH": None,
                "MELODIC_MOTION": "unknown"
            },
            "TUNING_STANDARD": {
                "REFERENCE_PITCH": "unknown",
                "MICROTONALITY": "unknown"
            }
        },
        "TIMBRAL": {
            "SPECTRAL_PHYSICS": {
                "FREQUENCY_BALANCE": {
                    "SUB_DOMINANT": None,
                    "MID_FORWARD": None,
                    "AIR_BRILLIANCE": None
                },
                "SPECTRAL_SATURATION": None,
                "SPECTRAL_CENTROID": "unknown"
            },
            "PRODUCTION_AESTHETIC": {
                "FIDELITY_SCORE": "unknown",
                "DYNAMIC_RANGE_LRA": None,
                "SPATIAL_WIDTH": "Stereo"
            },
            "TEXTURE_GRAIN": {"SURFACE": "unknown", "ARTIFACTS": None}
        },
        "LINGUISTIC": {
            "SEMANTIC_CONTENT": {
                "EXPLICIT_FILTER": "Clean",
                "TOPIC_CLUSTERS": [],
                "NARRATIVE_ARC": "unknown"
            },
            "VOCAL_TEXTURE": {
                "POSITION": "unknown",
                "DELIVERY_STYLE": "Sung",
                "PROCESSING": "Dry"
            },
            "LANGUAGE_PROFILE": {
                "PRIMARY_LANGUAGE": "en",
                "DIALECT_SLANG": "unknown"
            }
        },
        "AFFECTIVE": {
            "THAYER_COORDINATES": {
                "VALENCE": 0.6,
                "AROUSAL": 0.4,
                "DOMINANCE": "Vulnerable"
            },
            "EMOTIONAL_COMPLEXITY": {
                "MOOD_STABILITY": "unknown",
                "CATHARSIS_POTENTIAL": None,
                "NOSTALGIA_TRIGGER": None
            },
            "TENSION_ARC": {
                "BUILD_UP_VELOCITY": None,
                "RESOLUTION_STATE": "unknown"
            }
        },
        "CONTEXTUAL": {
            "SCENARIO_ENGINE": {
                "MACRO_SETTING": "Car",
                "MICRO_ACTIVITY": "Commute",
                "SOCIAL_SETTING": "Solo"
            },
            "INTENT_VECTORS": {
                "FUNCTIONAL_GOAL": "unknown",
                "TIME_OF_DAY": "unknown"
            },
            "METEOROLOGICAL_MATCH": {
                "WEATHER": "unknown",
                "TEMPERATURE": "unknown"
            }
        },
        "PHOTOMETRIC": {
            "CHROMATIC_MAP": {
                "PRIMARY_HEX": "#00FFFF",
                "SECONDARY_HEX": None,
                "PALETTE_TEMPERATURE": "Cool"
            },
            "LUMEN_DYNAMICS": {
                "BRIGHTNESS_FLOOR": None,
                "BRIGHTNESS_CEILING": None,
                "STROBE_TRIGGER": None,
                "FADE_RATE": "unknown"
            },
            "VISUAL_TEXTURE": {
                "FOG_DENSITY": None,
                "LASER_COMPATIBILITY": None,
                "VISUAL_NOISE": "unknown"
            }
        },
        "KINETIC": {
            "BIOMETRIC_ENTRAINMENT": {
                "TARGET_HR_ZONE": "108-138",
                "HRV_IMPACT": "High HRV",
                "BREATH_RATE": None
            },
            "MOTOR_RESPONSE": {"DRIVE": 0.5, "SWAY": None, "HEAD_NOD": 0.4},
            "ENERGY_EXPENDITURE": {"MET_SCORE": 4.0}
        },
        "GENEALOGICAL": {
            "ERA_ANCHORING": {
                "RELEASE_DATE": None,
                "CULTURAL_ERA": "unknown",
                "TIMELESSNESS_SCORE": None
            },
            "DNA_SAMPLING": {
                "SAMPLE_LINEAGE": [],
                "INTERPOLATION": [],
                "GENRE_TREE": "unknown"
            },
            "TRIBE_ALIGNMENT": {
                "SUBCULTURE_ID": "unknown",
                "AUTHENTICITY_SCORE": None,
                "VIRAL_VELOCITY": "unknown"
            }
        }
    }
}

# --- RUN SIMULATION ---
engine = VAPContextEngine()
print("--- SIMULATION START: CONTEXT [GYM_PEAK] ---")
engine.set_context("GYM_PEAK")
print("\n")
# Process Track A
result_a = engine.apply_override(track_a)
print("TRACK: {}".format(result_a["track_title"]))
print(json.dumps(result_a, indent=2))
print("\n" + "-" * 30 + "\n")
# Process Track B
result_b = engine.apply_override(track_b)
print("TRACK: {}".format(result_b["track_title"]))
print(json.dumps(result_b, indent=2))
```
