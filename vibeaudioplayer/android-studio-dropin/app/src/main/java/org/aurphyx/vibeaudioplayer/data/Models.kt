package org.aurphyx.vibeaudioplayer.data

import android.net.Uri
import androidx.compose.ui.graphics.Color

enum class Tab { Orb, Library, Vasp, Scene, About }

data class VaspField(val value: String, val status: String = "known")

data class PillarCard(
    val key: String,
    val label: String,
    val archetype: String,
    val purpose: String,
    val fields: List<Pair<String, VaspField>>,
)

data class VaspProfile(
    val title: String,
    val artist: String,
    val bpm: Int,
    val primary: Long,
    val secondary: Long,
    val valence: Float,
    val arousal: Float,
    val groove: Float,
    val syncopation: Float,
    val dissonance: Float,
    val fog: Float,
    val brightness: Float,
    val entrainment: Float,
    val scene: String,
    val mood: String,
    val key: String,
    val pillars: List<PillarCard>,
) {
    val primaryColor get() = Color(primary)
    val secondaryColor get() = Color(secondary)
}

data class Track(
    val id: String,
    val title: String,
    val artist: String,
    val kind: Kind,
    val uri: Uri?,
    val mime: String? = null,
    val vasp: VaspProfile,
    val unsupported: Boolean = false,
) {
    enum class Kind { DEMO, FILE }
}

data class SceneSettings(
    val colorIntensity: Float = 0.82f,
    val motionIntensity: Float = 0.78f,
    val particles: Boolean = true,
    val spectrum: Boolean = true,
    val beatPulse: Boolean = true,
    val reducedMotion: Boolean = false,
    val readableType: Boolean = false,
)

data class OrbFrame(
    val time: Float,
    val chrom: FloatArray,
    val centroid: Float,
    val saturation: Float,
    val syncopation: Float,
    val bpmNorm: Float,
    val groove: Float,
    val dissonance: Float,
    val valence: Float,
    val arousal: Float,
    val fog: Float,
    val primary: Color,
    val secondary: Color,
    val brightnessFloor: Float,
    val brightnessCeiling: Float,
    val visualNoise: Float,
    val entrainment: Float,
    val bloom: Float,
    val beatPulse: Float,
)
