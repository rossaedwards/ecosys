import kotlin.math.max
import kotlin.math.min
import kotlin.math.round

/**
 * Reference Implementation of VASP Logic Architecture.
 * This engine converts raw audio analysis data into standardized
 * Vibe Audio Metadata.
 */
class VASPScoringEngine {
    val version: String = "3.69"

    companion object {
        val REQUIRED_PILLARS = listOf(
            "STRUCTURAL",
            "TONAL",
            "TIMBRAL",
            "LINGUISTIC",
            "AFFECTIVE",
            "CONTEXTUAL",
            "PHOTOMETRIC",
            "KINETIC",
            "GENEALOGICAL",
        )
    }

    private fun get(rawData: Map<String, Any?>, key: String, default: Any? = null): Any? {
        if (key in rawData && rawData[key] != "") {
            return rawData[key]
        }
        return default
    }

    private fun listOrEmpty(rawData: Map<String, Any?>, key: String): List<Any?> {
        val value = get(rawData, key) ?: return emptyList()
        @Suppress("UNCHECKED_CAST")
        return value as List<Any?>
    }

    private fun asDouble(value: Any?): Double? = when (value) {
        null -> null
        is Number -> value.toDouble()
        else -> null
    }

    private fun round2(value: Double): Double = round(value * 100.0) / 100.0

    // --- PHASE I: PHYSICAL ANALYSIS (DSP Layer) ---

    /** Logic for Subset 1.3: Percussive DNA */
    fun calculateKickProfile(attackMs: Double): String = when {
        attackMs < 10 -> "Sharp (Click)"
        attackMs in 10.0..30.0 -> "Punch (Thud)"
        else -> "Boom (Sub)"
    }

    /** Logic for Subset 3.1: Spectral Physics -> Tonal Color */
    fun calculateSpectralColor(centroidHz: Double): String = when {
        centroidHz < 200 -> "Dark/Muddy"
        centroidHz in 200.0..2000.0 -> "Warm/Body"
        else -> "Bright/Airy"
    }

    // --- PHASE II: PSYCHOLOGICAL ANALYSIS (NLP/ML Layer) ---

    /**
     * Logic for Subset 5.1: Affective (Thayer Model)
     * keyMode: "Major" or "Minor"
     * sentimentScore: -1.0 to 1.0 (from NLP)
     * rmsAmplitude: 0.0 to 1.0 (Loudness)
     */
    fun calculateThayerCoordinates(
        keyMode: String,
        sentimentScore: Double,
        rmsAmplitude: Double,
    ): Map<String, Any?> {
        val baseValence = if (keyMode == "Major") 0.5 else -0.5
        var valence = (baseValence + sentimentScore) / 2.0
        valence = max(-1.0, min(1.0, valence))
        val arousal = rmsAmplitude
        return linkedMapOf(
            "valence" to round2(valence),
            "arousal" to round2(arousal),
            "mood_quadrant" to getQuadrant(valence, arousal),
        )
    }

    private fun getQuadrant(valence: Double, arousal: Double): String = when {
        valence > 0 && arousal > 0.5 -> "Euphoria/Joy"
        valence > 0 && arousal <= 0.5 -> "Calm/Content"
        valence <= 0 && arousal > 0.5 -> "Anger/Fear"
        else -> "Depression/Melancholy"
    }

    /**
     * Deterministic TONAL heuristic (Intro Specs Pillar 2).
     * Uses key_mode / key_signature / dissonance when present.
     * Does not invent a root note, chord vocabulary, or tuning standard.
     */
    fun calculateTonalProfile(rawData: Map<String, Any?>): Map<String, Any?> {
        val keyMode = get(rawData, "key_mode") as String?
        var keySignature = get(rawData, "key_signature") as String?
        keySignature = when {
            keySignature == null && keyMode != null -> "unknown $keyMode"
            keySignature == null -> "unknown"
            else -> keySignature
        }

        val dissonance: Any? = when {
            "dissonance_rating" in rawData -> rawData["dissonance_rating"]
            keyMode == "Minor" -> 0.45
            keyMode == "Major" -> 0.15
            else -> null
        }

        return linkedMapOf(
            "HARMONIC_PROFILE" to linkedMapOf(
                "KEY_SIGNATURE" to keySignature,
                "CHORD_COMPLEXITY" to get(rawData, "chord_complexity", "unknown"),
                "DISSONANCE_RATING" to dissonance,
            ),
            "MELODIC_CONTOUR" to linkedMapOf(
                "RANGE_SPAN" to get(rawData, "range_span"),
                "HOOK_STRENGTH" to get(rawData, "hook_strength"),
                "MELODIC_MOTION" to get(rawData, "melodic_motion", "unknown"),
            ),
            "TUNING_STANDARD" to linkedMapOf(
                "REFERENCE_PITCH" to get(rawData, "reference_pitch", "unknown"),
                "MICROTONALITY" to get(rawData, "microtonality", "unknown"),
            ),
        )
    }

    /**
     * Deterministic LINGUISTIC heuristic (Intro Specs Pillar 4).
     * Uses explicit_filter, language, and lyric fields when present.
     * Does not infer profanity, topic, or dialect from sentiment alone.
     */
    fun calculateLinguisticProfile(rawData: Map<String, Any?>): Map<String, Any?> = linkedMapOf(
        "SEMANTIC_CONTENT" to linkedMapOf(
            "EXPLICIT_FILTER" to get(rawData, "explicit_filter", "unknown"),
            "TOPIC_CLUSTERS" to listOrEmpty(rawData, "topic_clusters"),
            "NARRATIVE_ARC" to get(rawData, "narrative_arc", "unknown"),
        ),
        "VOCAL_TEXTURE" to linkedMapOf(
            "POSITION" to get(rawData, "vocal_position", "unknown"),
            "DELIVERY_STYLE" to get(rawData, "delivery_style", "unknown"),
            "PROCESSING" to get(rawData, "vocal_processing", "unknown"),
        ),
        "LANGUAGE_PROFILE" to linkedMapOf(
            "PRIMARY_LANGUAGE" to get(rawData, "primary_language", "unknown"),
            "DIALECT_SLANG" to get(rawData, "dialect_slang", "unknown"),
        ),
    )

    /**
     * Deterministic CONTEXTUAL heuristic (Intro Specs Pillar 6).
     * Maps BPM + arousal + sentiment onto scenario clusters when those
     * DSP/NLP values exist. Catalog-only fields stay unknown.
     */
    fun calculateContextualProfile(rawData: Map<String, Any?>, arousal: Double?): Map<String, Any?> {
        val bpm = asDouble(get(rawData, "bpm"))
        val sentiment = asDouble(get(rawData, "sentiment_score", 0.0))

        val macro: Any?
        val micro: Any?
        val social: Any?
        val goal: Any?
        val timeOfDay: Any?

        if (get(rawData, "macro_setting") != null) {
            macro = rawData["macro_setting"]
            micro = get(rawData, "micro_activity", "unknown")
            social = get(rawData, "social_setting", "unknown")
            goal = get(rawData, "functional_goal", "unknown")
            timeOfDay = get(rawData, "time_of_day", "unknown")
        } else if (bpm != null && arousal != null && bpm >= 140 && arousal >= 0.8) {
            macro = "Gym"
            micro = "Heavy Lifting"
            social = "Crowd/Mass"
            goal = "Hype"
            timeOfDay = "unknown"
        } else if (bpm != null && sentiment != null && bpm <= 130 && sentiment <= 0) {
            macro = "Car"
            micro = "Night Drive"
            social = "Solo"
            goal = "unknown"
            timeOfDay = "Late Night"
        } else if (arousal != null && arousal < 0.3) {
            macro = "Bedroom"
            micro = "Sleep"
            social = "Solo"
            goal = "Sleep"
            timeOfDay = "unknown"
        } else {
            macro = "unknown"
            micro = "unknown"
            social = "unknown"
            goal = "unknown"
            timeOfDay = "unknown"
        }

        return linkedMapOf(
            "SCENARIO_ENGINE" to linkedMapOf(
                "MACRO_SETTING" to macro,
                "MICRO_ACTIVITY" to micro,
                "SOCIAL_SETTING" to social,
            ),
            "INTENT_VECTORS" to linkedMapOf(
                "FUNCTIONAL_GOAL" to goal,
                "TIME_OF_DAY" to timeOfDay,
            ),
            "METEOROLOGICAL_MATCH" to linkedMapOf(
                "WEATHER" to get(rawData, "weather", "unknown"),
                "TEMPERATURE" to get(rawData, "temperature", "unknown"),
            ),
        )
    }

    /**
     * Deterministic GENEALOGICAL heuristic (Intro Specs Pillar 9).
     * Passes through catalog facts from raw_data. Missing samples,
     * release date, cultural era, and tribe alignment are null,
     * empty arrays, or unknown — never invented.
     */
    fun calculateGenealogicalProfile(rawData: Map<String, Any?>): Map<String, Any?> = linkedMapOf(
        "ERA_ANCHORING" to linkedMapOf(
            "RELEASE_DATE" to get(rawData, "release_date"),
            "CULTURAL_ERA" to get(rawData, "cultural_era", "unknown"),
            "TIMELESSNESS_SCORE" to get(rawData, "timelessness_score"),
        ),
        "DNA_SAMPLING" to linkedMapOf(
            "SAMPLE_LINEAGE" to listOrEmpty(rawData, "sample_lineage"),
            "INTERPOLATION" to listOrEmpty(rawData, "interpolation"),
            "GENRE_TREE" to get(rawData, "genre_tree", "unknown"),
        ),
        "TRIBE_ALIGNMENT" to linkedMapOf(
            "SUBCULTURE_ID" to get(rawData, "subculture_id", "unknown"),
            "AUTHENTICITY_SCORE" to get(rawData, "authenticity_score"),
            "VIRAL_VELOCITY" to get(rawData, "viral_velocity", "unknown"),
        ),
    )

    // --- PHASE III: ENVIRONMENTAL ANALYSIS (I/O Layer) ---

    /**
     * Logic for Subset 7.1: Photometric (Chromatic Map)
     * Maps Audio Frequency to Visual Wavelength (Approximation)
     */
    fun calculatePhotometricHex(dominantFreqHz: Double): String = when {
        dominantFreqHz < 60 -> "#8B0000"      // Deep Red (Sub Bass)
        dominantFreqHz < 250 -> "#FF8C00"     // Dark Orange (Low Mids)
        dominantFreqHz < 2000 -> "#008080"    // Teal (Mids/Vocals)
        else -> "#4B0082"                     // Indigo/UV (Highs/Air)
    }

    fun calculatePaletteTemperature(dominantFreqHz: Double): String =
        if (dominantFreqHz < 250) "Warm" else "Cool"

    /** Logic for Subset 8.1: Kinetic (Biometrics) */
    fun calculateKineticMet(bpm: Double): Double = when {
        bpm < 60 -> 1.0   // Rest
        bpm < 100 -> 3.0  // Light Activity
        bpm < 140 -> 6.0  // Moderate Activity
        else -> 8.0       // High Intensity (Sprint)
    }

    // --- MASTER GENERATOR ---

    /** Orchestrates the analysis phases to build the full VASP object. */
    fun generateVaspProfile(rawData: Map<String, Any?>): String {
        // Phase I
        val attackMs = (rawData.getValue("attack_ms") as Number).toDouble()
        val centroidHz = (rawData.getValue("centroid_hz") as Number).toDouble()
        val bpm = (rawData.getValue("bpm") as Number).toDouble()
        val kickProfile = calculateKickProfile(attackMs)
        val spectralTone = calculateSpectralColor(centroidHz)

        val (attackLabel, decayLabel) = when {
            kickProfile.startsWith("Sharp") -> "Sharp" to "Short"
            kickProfile.startsWith("Punch") -> "Soft" to "Short"
            else -> "Soft" to "Long"
        }

        // Phase II
        val affective = calculateThayerCoordinates(
            rawData.getValue("key_mode") as String,
            (rawData.getValue("sentiment_score") as Number).toDouble(),
            (rawData.getValue("rms_amplitude") as Number).toDouble(),
        )
        val valence = affective.getValue("valence") as Double
        val arousal = affective.getValue("arousal") as Double
        val dominance = when {
            valence <= 0 && arousal > 0.5 -> "Aggressive"
            valence > 0 && arousal > 0.5 -> "Empowering"
            else -> "Vulnerable"
        }

        val tonal = calculateTonalProfile(rawData)
        val linguistic = calculateLinguisticProfile(rawData)
        val contextual = calculateContextualProfile(rawData, arousal)
        val genealogical = calculateGenealogicalProfile(rawData)

        // Phase III
        val dominantFreqHz = (rawData.getValue("dominant_freq_hz") as Number).toDouble()
        val chromaHex = calculatePhotometricHex(dominantFreqHz)
        val paletteTemp = calculatePaletteTemperature(dominantFreqHz)
        val metScore = calculateKineticMet(bpm)

        val vaspObject = linkedMapOf(
            "VASP_VERSION" to version,
            "IDENTITY" to linkedMapOf(
                "TITLE" to (get(rawData, "title") ?: get(rawData, "TITLE") ?: "unknown"),
                "ARTIST" to (get(rawData, "artist") ?: get(rawData, "ARTIST") ?: "unknown"),
                "ISRC" to (get(rawData, "isrc") ?: get(rawData, "ISRC")),
                "SOURCE_DNA" to (get(rawData, "source_dna") ?: get(rawData, "SOURCE_DNA")),
            ),
            "PILLARS" to linkedMapOf(
                "STRUCTURAL" to linkedMapOf(
                    "TEMPORAL_DYNAMICS" to linkedMapOf(
                        "BPM_RAW" to rawData.getValue("bpm"),
                        "BPM_PERCEIVED" to get(rawData, "bpm_perceived", "unknown"),
                        "GROOVE_QUANTIZATION" to get(rawData, "groove_quantization", "unknown"),
                        "TIME_SIGNATURE" to get(rawData, "time_signature", "unknown"),
                    ),
                    "ARRANGEMENT_ARCHITECTURE" to linkedMapOf(
                        "SECTIONAL_MARKERS" to listOrEmpty(rawData, "sectional_markers"),
                        "MIX_WINDOW_INDEX" to get(rawData, "mix_window_index"),
                        "BREAKDOWN_DEPTH" to get(rawData, "breakdown_depth"),
                    ),
                    "PERCUSSIVE_DNA" to linkedMapOf(
                        "KICK_TRANSIENT" to linkedMapOf(
                            "ATTACK" to attackLabel,
                            "DECAY" to decayLabel,
                            "PROFILE" to kickProfile,
                        ),
                        "SYNCOPATION_INDEX" to get(rawData, "syncopation_index"),
                        "GHOST_NOTE_DENSITY" to get(rawData, "ghost_note_density"),
                    ),
                ),
                "TONAL" to tonal,
                "TIMBRAL" to linkedMapOf(
                    "SPECTRAL_PHYSICS" to linkedMapOf(
                        "FREQUENCY_BALANCE" to linkedMapOf(
                            "SUB_DOMINANT" to get(rawData, "sub_dominant"),
                            "MID_FORWARD" to get(rawData, "mid_forward"),
                            "AIR_BRILLIANCE" to get(rawData, "air_brilliance"),
                        ),
                        "SPECTRAL_SATURATION" to get(rawData, "spectral_saturation"),
                        "SPECTRAL_CENTROID" to spectralTone,
                    ),
                    "PRODUCTION_AESTHETIC" to linkedMapOf(
                        "FIDELITY_SCORE" to get(rawData, "fidelity_score", "unknown"),
                        "DYNAMIC_RANGE_LRA" to get(rawData, "dynamic_range_lra"),
                        "SPATIAL_WIDTH" to get(rawData, "spatial_width", "unknown"),
                    ),
                    "TEXTURE_GRAIN" to linkedMapOf(
                        "SURFACE" to get(rawData, "texture_surface", "unknown"),
                        "ARTIFACTS" to get(rawData, "texture_artifacts"),
                    ),
                ),
                "LINGUISTIC" to linguistic,
                "AFFECTIVE" to linkedMapOf(
                    "THAYER_COORDINATES" to linkedMapOf(
                        "VALENCE" to valence,
                        "AROUSAL" to arousal,
                        "DOMINANCE" to dominance,
                    ),
                    "EMOTIONAL_COMPLEXITY" to linkedMapOf(
                        "MOOD_STABILITY" to get(rawData, "mood_stability", "unknown"),
                        "CATHARSIS_POTENTIAL" to get(rawData, "catharsis_potential"),
                        "NOSTALGIA_TRIGGER" to get(rawData, "nostalgia_trigger"),
                    ),
                    "TENSION_ARC" to linkedMapOf(
                        "BUILD_UP_VELOCITY" to get(rawData, "build_up_velocity"),
                        "RESOLUTION_STATE" to get(rawData, "resolution_state", "unknown"),
                    ),
                ),
                "CONTEXTUAL" to contextual,
                "PHOTOMETRIC" to linkedMapOf(
                    "CHROMATIC_MAP" to linkedMapOf(
                        "PRIMARY_HEX" to chromaHex,
                        "SECONDARY_HEX" to get(rawData, "secondary_hex"),
                        "PALETTE_TEMPERATURE" to paletteTemp,
                    ),
                    "LUMEN_DYNAMICS" to linkedMapOf(
                        "BRIGHTNESS_FLOOR" to get(rawData, "brightness_floor"),
                        "BRIGHTNESS_CEILING" to get(rawData, "brightness_ceiling"),
                        "STROBE_TRIGGER" to get(rawData, "strobe_trigger"),
                        "FADE_RATE" to get(rawData, "fade_rate", "unknown"),
                    ),
                    "VISUAL_TEXTURE" to linkedMapOf(
                        "FOG_DENSITY" to get(rawData, "fog_density"),
                        "LASER_COMPATIBILITY" to get(rawData, "laser_compatibility"),
                        "VISUAL_NOISE" to get(rawData, "visual_noise", "unknown"),
                    ),
                ),
                "KINETIC" to linkedMapOf(
                    "BIOMETRIC_ENTRAINMENT" to linkedMapOf(
                        "TARGET_HR_ZONE" to "${(bpm - 20).toInt()}-${(bpm + 10).toInt()}",
                        "HRV_IMPACT" to if (arousal > 0.7) "Low HRV" else "High HRV",
                        "BREATH_RATE" to get(rawData, "breath_rate"),
                    ),
                    "MOTOR_RESPONSE" to linkedMapOf(
                        "DRIVE" to round2(min(1.0, metScore / 8.0)),
                        "SWAY" to get(rawData, "sway"),
                        "HEAD_NOD" to round2(min(1.0, max(0.0, (bpm - 60) / 80.0))),
                    ),
                    "ENERGY_EXPENDITURE" to linkedMapOf(
                        "MET_SCORE" to metScore,
                    ),
                ),
                "GENEALOGICAL" to genealogical,
            ),
        )

        @Suppress("UNCHECKED_CAST")
        val pillars = vaspObject.getValue("PILLARS") as Map<String, Any?>
        val missing = REQUIRED_PILLARS.filter { it !in pillars }
        if (missing.isNotEmpty()) {
            throw IllegalArgumentException("Incomplete VASP profile, missing pillars: $missing")
        }
        return toJson(vaspObject, indent = 2)
    }
}

/** Minimal JSON encoder matching Python json.dumps(indent=2) for Maps/Lists/primitives. */
private fun toJson(value: Any?, indent: Int = 0, level: Int = 0): String {
    val pad = " ".repeat(level)
    val inner = " ".repeat(level + indent)
    return when (value) {
        null -> "null"
        is Boolean -> value.toString()
        is Int, is Long, is Short, is Byte -> value.toString()
        is Double, is Float -> formatJsonNumber((value as Number).toDouble())
        is Number -> formatJsonNumber(value.toDouble())
        is String -> "\"${escapeJson(value)}\""
        is Map<*, *> -> {
            if (value.isEmpty()) return "{}"
            val entries = value.entries.joinToString(",\n") { (k, v) ->
                """$inner"${escapeJson(k.toString())}": ${toJson(v, indent, level + indent)}"""
            }
            "{\n$entries\n$pad}"
        }
        is List<*> -> {
            if (value.isEmpty()) return "[]"
            val items = value.joinToString(",\n") { item ->
                "$inner${toJson(item, indent, level + indent)}"
            }
            "[\n$items\n$pad]"
        }
        else -> "\"${escapeJson(value.toString())}\""
    }
}

private fun formatJsonNumber(d: Double): String {
    if (!d.isFinite()) return d.toString()
    if (d == 0.0) return if (1.0 / d < 0) "-0.0" else "0.0"
    val asLong = d.toLong()
    return if (d == asLong.toDouble()) "$asLong.0" else d.toString()
}

private fun escapeJson(s: String): String = buildString(s.length) {
    for (c in s) {
        when (c) {
            '\\' -> append("\\\\")
            '"' -> append("\\\"")
            '\n' -> append("\\n")
            '\r' -> append("\\r")
            '\t' -> append("\\t")
            else -> append(c)
        }
    }
}

// --- SIMULATION ---
// Mock Data (representing "After Dark" by Mr.Kitty)
fun main() {
    val mockAudioAnalysis = linkedMapOf<String, Any?>(
        "title" to "After Dark",
        "artist" to "Mr.Kitty",
        "bpm" to 135,
        "attack_ms" to 45,            // Slow attack (Synth wash)
        "centroid_hz" to 2200,        // Bright/Airy
        "key_mode" to "Minor",
        "sentiment_score" to -0.2,    // Melancholic
        "rms_amplitude" to 0.75,      // Steady loud
        "dominant_freq_hz" to 3000,   // High synth lead focus
    )

    val engine = VASPScoringEngine()
    val vaspProfile = engine.generateVaspProfile(mockAudioAnalysis)
    println("--- Vibe Audio Generated Metadata ---")
    println(vaspProfile)
}
