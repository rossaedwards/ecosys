package org.aurphyx.vasp.model

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import org.aurphyx.vasp.Vasp

@Serializable
data class VaspObject(
    @SerialName("VASP_VERSION") val vaspVersion: String = Vasp.VERSION,
    @SerialName("IDENTITY") val identity: Identity,
    @SerialName("PILLARS") val pillars: Pillars,
)

@Serializable
data class Identity(
    @SerialName("TITLE") val title: String,
    @SerialName("ARTIST") val artist: String,
    @SerialName("ISRC") val isrc: String? = null,
    @SerialName("SOURCE_DNA") val sourceDna: String? = null,
)

@Serializable
data class Pillars(
    @SerialName("STRUCTURAL") val structural: StructuralPillar,
    @SerialName("TONAL") val tonal: TonalPillar,
    @SerialName("TIMBRAL") val timbral: TimbralPillar,
    @SerialName("LINGUISTIC") val linguistic: LinguisticPillar,
    @SerialName("AFFECTIVE") val affective: AffectivePillar,
    @SerialName("CONTEXTUAL") val contextual: ContextualPillar,
    @SerialName("PHOTOMETRIC") val photometric: PhotometricPillar,
    @SerialName("KINETIC") val kinetic: KineticPillar,
    @SerialName("GENEALOGICAL") val genealogical: GenealogicalPillar,
)

@Serializable
data class StructuralPillar(
    @SerialName("TEMPORAL_DYNAMICS") val temporalDynamics: TemporalDynamics,
    @SerialName("ARRANGEMENT_ARCHITECTURE") val arrangementArchitecture: ArrangementArchitecture,
    @SerialName("PERCUSSIVE_DNA") val percussiveDna: PercussiveDna,
)

@Serializable
data class TemporalDynamics(
    @SerialName("BPM_RAW") val bpmRaw: Double? = null,
    @SerialName("BPM_PERCEIVED") val bpmPerceived: String? = "unknown",
    @SerialName("GROOVE_QUANTIZATION") val grooveQuantization: String? = "unknown",
    @SerialName("TIME_SIGNATURE") val timeSignature: String? = "unknown",
)

@Serializable
data class ArrangementArchitecture(
    @SerialName("SECTIONAL_MARKERS") val sectionalMarkers: List<String> = emptyList(),
    @SerialName("MIX_WINDOW_INDEX") val mixWindowIndex: Double? = null,
    @SerialName("BREAKDOWN_DEPTH") val breakdownDepth: Double? = null,
)

@Serializable
data class PercussiveDna(
    @SerialName("KICK_TRANSIENT") val kickTransient: KickTransient,
    @SerialName("SYNCOPATION_INDEX") val syncopationIndex: Double? = null,
    @SerialName("GHOST_NOTE_DENSITY") val ghostNoteDensity: Double? = null,
)

@Serializable
data class KickTransient(
    @SerialName("ATTACK") val attack: String? = null,
    @SerialName("DECAY") val decay: String? = null,
    @SerialName("PROFILE") val profile: String? = null,
)

@Serializable
data class TonalPillar(
    @SerialName("HARMONIC_PROFILE") val harmonicProfile: HarmonicProfile,
    @SerialName("MELODIC_CONTOUR") val melodicContour: MelodicContour,
    @SerialName("TUNING_STANDARD") val tuningStandard: TuningStandard,
)

@Serializable
data class HarmonicProfile(
    @SerialName("KEY_SIGNATURE") val keySignature: String? = "unknown",
    @SerialName("CHORD_COMPLEXITY") val chordComplexity: String? = "unknown",
    @SerialName("DISSONANCE_RATING") val dissonanceRating: Double? = null,
)

@Serializable
data class MelodicContour(
    @SerialName("RANGE_SPAN") val rangeSpan: Double? = null,
    @SerialName("HOOK_STRENGTH") val hookStrength: Double? = null,
    @SerialName("MELODIC_MOTION") val melodicMotion: String? = "unknown",
)

@Serializable
data class TuningStandard(
    @SerialName("REFERENCE_PITCH") val referencePitch: String? = "unknown",
    @SerialName("MICROTONALITY") val microtonality: String? = "unknown",
)

@Serializable
data class TimbralPillar(
    @SerialName("SPECTRAL_PHYSICS") val spectralPhysics: SpectralPhysics,
    @SerialName("PRODUCTION_AESTHETIC") val productionAesthetic: ProductionAesthetic,
    @SerialName("TEXTURE_GRAIN") val textureGrain: TextureGrain,
)

@Serializable
data class SpectralPhysics(
    @SerialName("FREQUENCY_BALANCE") val frequencyBalance: FrequencyBalance = FrequencyBalance(),
    @SerialName("SPECTRAL_SATURATION") val spectralSaturation: Double? = null,
    @SerialName("SPECTRAL_CENTROID") val spectralCentroid: String? = null,
)

@Serializable
data class FrequencyBalance(
    @SerialName("SUB_DOMINANT") val subDominant: Double? = null,
    @SerialName("MID_FORWARD") val midForward: Double? = null,
    @SerialName("AIR_BRILLIANCE") val airBrilliance: Double? = null,
)

@Serializable
data class ProductionAesthetic(
    @SerialName("FIDELITY_SCORE") val fidelityScore: String? = "unknown",
    @SerialName("DYNAMIC_RANGE_LRA") val dynamicRangeLra: Double? = null,
    @SerialName("SPATIAL_WIDTH") val spatialWidth: String? = "unknown",
)

@Serializable
data class TextureGrain(
    @SerialName("SURFACE") val surface: String? = "unknown",
    @SerialName("ARTIFACTS") val artifacts: String? = null,
)

@Serializable
data class LinguisticPillar(
    @SerialName("SEMANTIC_CONTENT") val semanticContent: SemanticContent,
    @SerialName("VOCAL_TEXTURE") val vocalTexture: VocalTexture,
    @SerialName("LANGUAGE_PROFILE") val languageProfile: LanguageProfile,
)

@Serializable
data class SemanticContent(
    @SerialName("EXPLICIT_FILTER") val explicitFilter: String? = "unknown",
    @SerialName("TOPIC_CLUSTERS") val topicClusters: List<String> = emptyList(),
    @SerialName("NARRATIVE_ARC") val narrativeArc: String? = "unknown",
)

@Serializable
data class VocalTexture(
    @SerialName("POSITION") val position: String? = "unknown",
    @SerialName("DELIVERY_STYLE") val deliveryStyle: String? = "unknown",
    @SerialName("PROCESSING") val processing: String? = "unknown",
)

@Serializable
data class LanguageProfile(
    @SerialName("PRIMARY_LANGUAGE") val primaryLanguage: String? = "unknown",
    @SerialName("DIALECT_SLANG") val dialectSlang: String? = "unknown",
)

@Serializable
data class AffectivePillar(
    @SerialName("THAYER_COORDINATES") val thayerCoordinates: ThayerCoordinates,
    @SerialName("EMOTIONAL_COMPLEXITY") val emotionalComplexity: EmotionalComplexity,
    @SerialName("TENSION_ARC") val tensionArc: TensionArc,
)

@Serializable
data class ThayerCoordinates(
    @SerialName("VALENCE") val valence: Double,
    @SerialName("AROUSAL") val arousal: Double,
    @SerialName("DOMINANCE") val dominance: String? = null,
)

@Serializable
data class EmotionalComplexity(
    @SerialName("MOOD_STABILITY") val moodStability: String? = "unknown",
    @SerialName("CATHARSIS_POTENTIAL") val catharsisPotential: Double? = null,
    @SerialName("NOSTALGIA_TRIGGER") val nostalgiaTrigger: Double? = null,
)

@Serializable
data class TensionArc(
    @SerialName("BUILD_UP_VELOCITY") val buildUpVelocity: Double? = null,
    @SerialName("RESOLUTION_STATE") val resolutionState: String? = "unknown",
)

@Serializable
data class ContextualPillar(
    @SerialName("SCENARIO_ENGINE") val scenarioEngine: ScenarioEngine,
    @SerialName("INTENT_VECTORS") val intentVectors: IntentVectors,
    @SerialName("METEOROLOGICAL_MATCH") val meteorologicalMatch: MeteorologicalMatch,
)

@Serializable
data class ScenarioEngine(
    @SerialName("MACRO_SETTING") val macroSetting: String? = "unknown",
    @SerialName("MICRO_ACTIVITY") val microActivity: String? = "unknown",
    @SerialName("SOCIAL_SETTING") val socialSetting: String? = "unknown",
)

@Serializable
data class IntentVectors(
    @SerialName("FUNCTIONAL_GOAL") val functionalGoal: String? = "unknown",
    @SerialName("TIME_OF_DAY") val timeOfDay: String? = "unknown",
)

@Serializable
data class MeteorologicalMatch(
    @SerialName("WEATHER") val weather: String? = "unknown",
    @SerialName("TEMPERATURE") val temperature: String? = "unknown",
)

@Serializable
data class PhotometricPillar(
    @SerialName("CHROMATIC_MAP") val chromaticMap: ChromaticMap,
    @SerialName("LUMEN_DYNAMICS") val lumenDynamics: LumenDynamics,
    @SerialName("VISUAL_TEXTURE") val visualTexture: VisualTexture,
)

@Serializable
data class ChromaticMap(
    @SerialName("PRIMARY_HEX") val primaryHex: String? = null,
    @SerialName("SECONDARY_HEX") val secondaryHex: String? = null,
    @SerialName("PALETTE_TEMPERATURE") val paletteTemperature: String? = null,
)

@Serializable
data class LumenDynamics(
    @SerialName("BRIGHTNESS_FLOOR") val brightnessFloor: Double? = null,
    @SerialName("BRIGHTNESS_CEILING") val brightnessCeiling: Double? = null,
    @SerialName("STROBE_TRIGGER") val strobeTrigger: Double? = null,
    @SerialName("FADE_RATE") val fadeRate: String? = "unknown",
)

@Serializable
data class VisualTexture(
    @SerialName("FOG_DENSITY") val fogDensity: Double? = null,
    @SerialName("LASER_COMPATIBILITY") val laserCompatibility: Boolean? = null,
    @SerialName("VISUAL_NOISE") val visualNoise: String? = "unknown",
)

@Serializable
data class KineticPillar(
    @SerialName("BIOMETRIC_ENTRAINMENT") val biometricEntrainment: BiometricEntrainment,
    @SerialName("MOTOR_RESPONSE") val motorResponse: MotorResponse,
    @SerialName("ENERGY_EXPENDITURE") val energyExpenditure: EnergyExpenditure,
)

@Serializable
data class BiometricEntrainment(
    @SerialName("TARGET_HR_ZONE") val targetHrZone: String? = null,
    @SerialName("HRV_IMPACT") val hrvImpact: String? = null,
    @SerialName("BREATH_RATE") val breathRate: Double? = null,
)

@Serializable
data class MotorResponse(
    @SerialName("DRIVE") val drive: Double? = null,
    @SerialName("SWAY") val sway: Double? = null,
    @SerialName("HEAD_NOD") val headNod: Double? = null,
)

@Serializable
data class EnergyExpenditure(
    @SerialName("MET_SCORE") val metScore: Double? = null,
)

@Serializable
data class GenealogicalPillar(
    @SerialName("ERA_ANCHORING") val eraAnchoring: EraAnchoring,
    @SerialName("DNA_SAMPLING") val dnaSampling: DnaSampling,
    @SerialName("TRIBE_ALIGNMENT") val tribeAlignment: TribeAlignment,
)

@Serializable
data class EraAnchoring(
    @SerialName("RELEASE_DATE") val releaseDate: String? = null,
    @SerialName("CULTURAL_ERA") val culturalEra: String? = "unknown",
    @SerialName("TIMELESSNESS_SCORE") val timelessnessScore: Double? = null,
)

@Serializable
data class DnaSampling(
    @SerialName("SAMPLE_LINEAGE") val sampleLineage: List<String> = emptyList(),
    @SerialName("INTERPOLATION") val interpolation: List<String> = emptyList(),
    @SerialName("GENRE_TREE") val genreTree: String? = "unknown",
)

@Serializable
data class TribeAlignment(
    @SerialName("SUBCULTURE_ID") val subcultureId: String? = "unknown",
    @SerialName("AUTHENTICITY_SCORE") val authenticityScore: Double? = null,
    @SerialName("VIRAL_VELOCITY") val viralVelocity: String? = "unknown",
)
