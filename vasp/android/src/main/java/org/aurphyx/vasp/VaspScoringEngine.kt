package org.aurphyx.vasp

import kotlin.math.max
import kotlin.math.min
import kotlin.math.round
import org.aurphyx.vasp.model.AffectivePillar
import org.aurphyx.vasp.model.ArrangementArchitecture
import org.aurphyx.vasp.model.BiometricEntrainment
import org.aurphyx.vasp.model.ChromaticMap
import org.aurphyx.vasp.model.ContextualPillar
import org.aurphyx.vasp.model.DnaSampling
import org.aurphyx.vasp.model.EmotionalComplexity
import org.aurphyx.vasp.model.EnergyExpenditure
import org.aurphyx.vasp.model.EraAnchoring
import org.aurphyx.vasp.model.FrequencyBalance
import org.aurphyx.vasp.model.GenealogicalPillar
import org.aurphyx.vasp.model.HarmonicProfile
import org.aurphyx.vasp.model.Identity
import org.aurphyx.vasp.model.IntentVectors
import org.aurphyx.vasp.model.KickTransient
import org.aurphyx.vasp.model.KineticPillar
import org.aurphyx.vasp.model.LanguageProfile
import org.aurphyx.vasp.model.LinguisticPillar
import org.aurphyx.vasp.model.LumenDynamics
import org.aurphyx.vasp.model.MelodicContour
import org.aurphyx.vasp.model.MeteorologicalMatch
import org.aurphyx.vasp.model.MotorResponse
import org.aurphyx.vasp.model.PercussiveDna
import org.aurphyx.vasp.model.PhotometricPillar
import org.aurphyx.vasp.model.Pillars
import org.aurphyx.vasp.model.ProductionAesthetic
import org.aurphyx.vasp.model.ScenarioEngine
import org.aurphyx.vasp.model.SemanticContent
import org.aurphyx.vasp.model.SpectralPhysics
import org.aurphyx.vasp.model.StructuralPillar
import org.aurphyx.vasp.model.TemporalDynamics
import org.aurphyx.vasp.model.TensionArc
import org.aurphyx.vasp.model.TextureGrain
import org.aurphyx.vasp.model.ThayerCoordinates
import org.aurphyx.vasp.model.TimbralPillar
import org.aurphyx.vasp.model.TonalPillar
import org.aurphyx.vasp.model.TribeAlignment
import org.aurphyx.vasp.model.TuningStandard
import org.aurphyx.vasp.model.VaspObject
import org.aurphyx.vasp.model.VisualTexture
import org.aurphyx.vasp.model.VocalTexture
import org.aurphyx.vasp.player.PlayerVaspProfile

/**
 * Reference implementation of VASP Logic Architecture for Android.
 * Converts [RawAnalysis] into a complete 9-pillar [VaspObject].
 */
class VaspScoringEngine(
    val version: String = Vasp.VERSION,
) {
    data class ThayerResult(
        val valence: Double,
        val arousal: Double,
        val moodQuadrant: String,
    )

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

    /**
     * Logic for Subset 5.1: Affective (Thayer Model)
     * [keyMode]: `"Major"` or `"Minor"`
     * [sentimentScore]: -1.0 to 1.0 (from NLP)
     * [rmsAmplitude]: 0.0 to 1.0 (Loudness)
     */
    fun calculateThayerCoordinates(
        keyMode: String,
        sentimentScore: Double,
        rmsAmplitude: Double,
    ): ThayerResult {
        val baseValence = if (keyMode == "Major") 0.5 else -0.5
        val valence = max(-1.0, min(1.0, (baseValence + sentimentScore) / 2.0))
        val arousal = rmsAmplitude
        return ThayerResult(
            valence = round2(valence),
            arousal = round2(arousal),
            moodQuadrant = quadrant(valence, arousal),
        )
    }

    /**
     * Logic for Subset 7.1: Photometric (Chromatic Map)
     * Maps audio frequency to visual wavelength (approximation).
     */
    fun calculatePhotometricHex(dominantFreqHz: Double): String = when {
        dominantFreqHz < 60 -> "#8B0000"
        dominantFreqHz < 250 -> "#FF8C00"
        dominantFreqHz < 2000 -> "#008080"
        else -> "#4B0082"
    }

    fun calculatePaletteTemperature(dominantFreqHz: Double): String =
        if (dominantFreqHz < 250) "Warm" else "Cool"

    /** Logic for Subset 8.1: Kinetic (Biometrics) */
    fun calculateKineticMet(bpm: Double): Double = when {
        bpm < 60 -> 1.0
        bpm < 100 -> 3.0
        bpm < 140 -> 6.0
        else -> 8.0
    }

    fun generate(raw: RawAnalysis): VaspObject {
        val kickProfile = calculateKickProfile(raw.attackMs)
        val spectralTone = calculateSpectralColor(raw.centroidHz)
        val (attackLabel, decayLabel) = when {
            kickProfile.startsWith("Sharp") -> "Sharp" to "Short"
            kickProfile.startsWith("Punch") -> "Soft" to "Short"
            else -> "Soft" to "Long"
        }

        val thayer = calculateThayerCoordinates(raw.keyMode, raw.sentimentScore, raw.rmsAmplitude)
        val dominance = when {
            thayer.valence <= 0 && thayer.arousal > 0.5 -> "Aggressive"
            thayer.valence > 0 && thayer.arousal > 0.5 -> "Empowering"
            else -> "Vulnerable"
        }

        val chromaHex = calculatePhotometricHex(raw.dominantFreqHz)
        val paletteTemp = calculatePaletteTemperature(raw.dominantFreqHz)
        val metScore = calculateKineticMet(raw.bpm)

        val obj = VaspObject(
            vaspVersion = version,
            identity = Identity(
                title = present(raw.title) ?: "unknown",
                artist = present(raw.artist) ?: "unknown",
                isrc = present(raw.isrc),
                sourceDna = present(raw.sourceDna),
            ),
            pillars = Pillars(
                structural = StructuralPillar(
                    temporalDynamics = TemporalDynamics(
                        bpmRaw = raw.bpm,
                        bpmPerceived = present(raw.bpmPerceived) ?: "unknown",
                        grooveQuantization = present(raw.grooveQuantization) ?: "unknown",
                        timeSignature = present(raw.timeSignature) ?: "unknown",
                    ),
                    arrangementArchitecture = ArrangementArchitecture(
                        sectionalMarkers = raw.sectionalMarkers,
                        mixWindowIndex = raw.mixWindowIndex,
                        breakdownDepth = raw.breakdownDepth,
                    ),
                    percussiveDna = PercussiveDna(
                        kickTransient = KickTransient(
                            attack = attackLabel,
                            decay = decayLabel,
                            profile = kickProfile,
                        ),
                        syncopationIndex = raw.syncopationIndex,
                        ghostNoteDensity = raw.ghostNoteDensity,
                    ),
                ),
                tonal = calculateTonalProfile(raw),
                timbral = TimbralPillar(
                    spectralPhysics = SpectralPhysics(
                        frequencyBalance = FrequencyBalance(
                            subDominant = raw.subDominant,
                            midForward = raw.midForward,
                            airBrilliance = raw.airBrilliance,
                        ),
                        spectralSaturation = raw.spectralSaturation,
                        spectralCentroid = spectralTone,
                    ),
                    productionAesthetic = ProductionAesthetic(
                        fidelityScore = present(raw.fidelityScore) ?: "unknown",
                        dynamicRangeLra = raw.dynamicRangeLra,
                        spatialWidth = present(raw.spatialWidth) ?: "unknown",
                    ),
                    textureGrain = TextureGrain(
                        surface = present(raw.textureSurface) ?: "unknown",
                        artifacts = present(raw.textureArtifacts),
                    ),
                ),
                linguistic = calculateLinguisticProfile(raw),
                affective = AffectivePillar(
                    thayerCoordinates = ThayerCoordinates(
                        valence = thayer.valence,
                        arousal = thayer.arousal,
                        dominance = dominance,
                    ),
                    emotionalComplexity = EmotionalComplexity(
                        moodStability = present(raw.moodStability) ?: "unknown",
                        catharsisPotential = raw.catharsisPotential,
                        nostalgiaTrigger = raw.nostalgiaTrigger,
                    ),
                    tensionArc = TensionArc(
                        buildUpVelocity = raw.buildUpVelocity,
                        resolutionState = present(raw.resolutionState) ?: "unknown",
                    ),
                ),
                contextual = calculateContextualProfile(raw, thayer.arousal),
                photometric = PhotometricPillar(
                    chromaticMap = ChromaticMap(
                        primaryHex = chromaHex,
                        secondaryHex = present(raw.secondaryHex),
                        paletteTemperature = paletteTemp,
                    ),
                    lumenDynamics = LumenDynamics(
                        brightnessFloor = raw.brightnessFloor,
                        brightnessCeiling = raw.brightnessCeiling,
                        strobeTrigger = raw.strobeTrigger,
                        fadeRate = present(raw.fadeRate) ?: "unknown",
                    ),
                    visualTexture = VisualTexture(
                        fogDensity = raw.fogDensity,
                        laserCompatibility = raw.laserCompatibility,
                        visualNoise = present(raw.visualNoise) ?: "unknown",
                    ),
                ),
                kinetic = KineticPillar(
                    biometricEntrainment = BiometricEntrainment(
                        targetHrZone = "${(raw.bpm - 20).toInt()}-${(raw.bpm + 10).toInt()}",
                        hrvImpact = if (thayer.arousal > 0.7) "Low HRV" else "High HRV",
                        breathRate = raw.breathRate,
                    ),
                    motorResponse = MotorResponse(
                        drive = round2(min(1.0, metScore / 8.0)),
                        sway = raw.sway,
                        headNod = round2(min(1.0, max(0.0, (raw.bpm - 60) / 80.0))),
                    ),
                    energyExpenditure = EnergyExpenditure(metScore = metScore),
                ),
                genealogical = calculateGenealogicalProfile(raw),
            ),
        )
        return obj
    }

    fun generateJson(raw: RawAnalysis): String = generate(raw).encodeToString()

    /** Flattened UI readout for Vibe Audio Player `TrackItem.vaspProfile`. */
    fun generatePlayerProfile(raw: RawAnalysis): PlayerVaspProfile =
        generate(raw).toPlayerProfile()

    private fun calculateTonalProfile(raw: RawAnalysis): TonalPillar {
        val keySignature = when {
            present(raw.keySignature) != null -> raw.keySignature
            present(raw.keyMode) != null -> "unknown ${raw.keyMode}"
            else -> "unknown"
        }
        val dissonance = when {
            raw.dissonanceRating != null -> raw.dissonanceRating
            raw.keyMode == "Minor" -> 0.45
            raw.keyMode == "Major" -> 0.15
            else -> null
        }
        return TonalPillar(
            harmonicProfile = HarmonicProfile(
                keySignature = keySignature,
                chordComplexity = present(raw.chordComplexity) ?: "unknown",
                dissonanceRating = dissonance,
            ),
            melodicContour = MelodicContour(
                rangeSpan = raw.rangeSpan,
                hookStrength = raw.hookStrength,
                melodicMotion = present(raw.melodicMotion) ?: "unknown",
            ),
            tuningStandard = TuningStandard(
                referencePitch = present(raw.referencePitch) ?: "unknown",
                microtonality = present(raw.microtonality) ?: "unknown",
            ),
        )
    }

    private fun calculateLinguisticProfile(raw: RawAnalysis): LinguisticPillar = LinguisticPillar(
        semanticContent = SemanticContent(
            explicitFilter = present(raw.explicitFilter) ?: "unknown",
            topicClusters = raw.topicClusters,
            narrativeArc = present(raw.narrativeArc) ?: "unknown",
        ),
        vocalTexture = VocalTexture(
            position = present(raw.vocalPosition) ?: "unknown",
            deliveryStyle = present(raw.deliveryStyle) ?: "unknown",
            processing = present(raw.vocalProcessing) ?: "unknown",
        ),
        languageProfile = LanguageProfile(
            primaryLanguage = present(raw.primaryLanguage) ?: "unknown",
            dialectSlang = present(raw.dialectSlang) ?: "unknown",
        ),
    )

    private fun calculateContextualProfile(raw: RawAnalysis, arousal: Double): ContextualPillar {
        val macro: String?
        val micro: String?
        val social: String?
        val goal: String?
        val timeOfDay: String?
        if (present(raw.macroSetting) != null) {
            macro = raw.macroSetting
            micro = present(raw.microActivity) ?: "unknown"
            social = present(raw.socialSetting) ?: "unknown"
            goal = present(raw.functionalGoal) ?: "unknown"
            timeOfDay = present(raw.timeOfDay) ?: "unknown"
        } else if (raw.bpm >= 140 && arousal >= 0.8) {
            macro = "Gym"
            micro = "Heavy Lifting"
            social = "Crowd/Mass"
            goal = "Hype"
            timeOfDay = "unknown"
        } else if (raw.bpm <= 130 && raw.sentimentScore <= 0) {
            macro = "Car"
            micro = "Night Drive"
            social = "Solo"
            goal = "unknown"
            timeOfDay = "Late Night"
        } else if (arousal < 0.3) {
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
        return ContextualPillar(
            scenarioEngine = ScenarioEngine(
                macroSetting = macro,
                microActivity = micro,
                socialSetting = social,
            ),
            intentVectors = IntentVectors(
                functionalGoal = goal,
                timeOfDay = timeOfDay,
            ),
            meteorologicalMatch = MeteorologicalMatch(
                weather = present(raw.weather) ?: "unknown",
                temperature = present(raw.temperature) ?: "unknown",
            ),
        )
    }

    private fun calculateGenealogicalProfile(raw: RawAnalysis): GenealogicalPillar = GenealogicalPillar(
        eraAnchoring = EraAnchoring(
            releaseDate = present(raw.releaseDate),
            culturalEra = present(raw.culturalEra) ?: "unknown",
            timelessnessScore = raw.timelessnessScore,
        ),
        dnaSampling = DnaSampling(
            sampleLineage = raw.sampleLineage,
            interpolation = raw.interpolation,
            genreTree = present(raw.genreTree) ?: "unknown",
        ),
        tribeAlignment = TribeAlignment(
            subcultureId = present(raw.subcultureId) ?: "unknown",
            authenticityScore = raw.authenticityScore,
            viralVelocity = present(raw.viralVelocity) ?: "unknown",
        ),
    )

    private fun quadrant(valence: Double, arousal: Double): String = when {
        valence > 0 && arousal > 0.5 -> "Euphoria/Joy"
        valence > 0 && arousal <= 0.5 -> "Calm/Content"
        valence <= 0 && arousal > 0.5 -> "Anger/Fear"
        else -> "Depression/Melancholy"
    }

    private fun present(value: String?): String? =
        if (value.isNullOrEmpty()) null else value

    private fun round2(value: Double): Double = round(value * 100.0) / 100.0
}
