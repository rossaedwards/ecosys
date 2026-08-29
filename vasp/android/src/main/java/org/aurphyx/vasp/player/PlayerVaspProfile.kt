package org.aurphyx.vasp.player

import org.aurphyx.vasp.model.VaspObject

/**
 * Flattened 9-readout profile for Vibe Audio Player (`VAP_TechSpec.md` §4.2).
 *
 * This is a UI contraction of nested VASP 3.69 JSON, not a second schema.
 * Missing catalog facts stay `"unknown"`; photometric fallback reuses primary hex.
 */
data class PlayerVaspProfile(
    val dominance: String,
    val bpmPerceived: Int,
    val keySignature: String,
    val spatialWidthRatio: Float,
    val texturalDensity: String,
    val lyricProminence: String,
    val primaryHex: String,
    val secondaryHex: String,
    val macroSetting: String,
    val weather: String,
)

fun VaspObject.toPlayerProfile(): PlayerVaspProfile {
    val thayer = pillars.affective.thayerCoordinates
    val bpm = pillars.structural.temporalDynamics.bpmRaw ?: 0.0
    val perceived = pillars.structural.temporalDynamics.bpmPerceived
        ?.toDoubleOrNull()
        ?.toInt()
        ?: bpm.toInt()
    val primary = pillars.photometric.chromaticMap.primaryHex ?: "#000000"
    val secondary = pillars.photometric.chromaticMap.secondaryHex ?: primary
    val spatial = pillars.timbral.productionAesthetic.spatialWidth
        ?.toFloatOrNull()
        ?.coerceIn(0f, 2f)
        ?: 1.0f
    val surface = pillars.timbral.textureGrain.surface
    val textural = when {
        !surface.isNullOrEmpty() && surface != "unknown" -> surface
        else -> when (pillars.timbral.spectralPhysics.spectralCentroid) {
            "Dark/Muddy" -> "Dense Analog"
            "Warm/Body" -> "Warm Tape"
            "Bright/Airy" -> "Crisp Digital"
            else -> "unknown"
        }
    }
    val lyric = pillars.linguistic.vocalTexture.position
        ?.takeUnless { it == "unknown" }
        ?: "unknown"
    return PlayerVaspProfile(
        dominance = thayer.dominance ?: "unknown",
        bpmPerceived = perceived,
        keySignature = pillars.tonal.harmonicProfile.keySignature ?: "unknown",
        spatialWidthRatio = spatial,
        texturalDensity = textural,
        lyricProminence = lyric,
        primaryHex = primary,
        secondaryHex = secondary,
        macroSetting = pillars.contextual.scenarioEngine.macroSetting ?: "unknown",
        weather = pillars.contextual.meteorologicalMatch.weather ?: "unknown",
    )
}
