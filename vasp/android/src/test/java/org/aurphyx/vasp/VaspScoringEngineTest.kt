package org.aurphyx.vasp

import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Assert.assertTrue
import org.junit.Test
import org.aurphyx.vasp.player.toPlayerProfile

class VaspScoringEngineTest {
    private val engine = VaspScoringEngine()
    private val profile = engine.generate(RawAnalysis.afterDark())

    @Test
    fun versionIs369() {
        assertEquals("3.69", profile.vaspVersion)
    }

    @Test
    fun identityFromFixture() {
        assertEquals("After Dark", profile.identity.title)
        assertEquals("Mr.Kitty", profile.identity.artist)
        assertNull(profile.identity.isrc)
    }

    @Test
    fun kickIsBoomSub() {
        val kick = profile.pillars.structural.percussiveDna.kickTransient
        assertEquals("Boom (Sub)", kick.profile)
        assertEquals("Soft", kick.attack)
        assertEquals("Long", kick.decay)
    }

    @Test
    fun spectralIsBrightAiry() {
        assertEquals("Bright/Airy", profile.pillars.timbral.spectralPhysics.spectralCentroid)
    }

    @Test
    fun thayerMatchesPythonEngine() {
        val thayer = profile.pillars.affective.thayerCoordinates
        assertEquals(-0.35, thayer.valence, 0.001)
        assertEquals(0.75, thayer.arousal, 0.001)
        assertEquals("Aggressive", thayer.dominance)
    }

    @Test
    fun photometricIndigo() {
        assertEquals("#4B0082", profile.pillars.photometric.chromaticMap.primaryHex)
        assertEquals("Cool", profile.pillars.photometric.chromaticMap.paletteTemperature)
        assertNull(profile.pillars.photometric.chromaticMap.secondaryHex)
    }

    @Test
    fun kineticMetAndHrZone() {
        assertEquals(6.0, profile.pillars.kinetic.energyExpenditure.metScore!!, 0.001)
        assertEquals("115-145", profile.pillars.kinetic.biometricEntrainment.targetHrZone)
        assertEquals(0.75, profile.pillars.kinetic.motorResponse.drive!!, 0.001)
        assertEquals(0.94, profile.pillars.kinetic.motorResponse.headNod!!, 0.001)
    }

    @Test
    fun missingCatalogStaysUnknown() {
        assertEquals("unknown", profile.pillars.genealogical.eraAnchoring.culturalEra)
        assertTrue(profile.pillars.genealogical.dnaSampling.sampleLineage.isEmpty())
        assertNull(profile.pillars.genealogical.eraAnchoring.releaseDate)
        assertEquals("unknown", profile.pillars.linguistic.semanticContent.explicitFilter)
    }

    @Test
    fun jsonRoundTripKeepsNinePillars() {
        val json = profile.encodeToString()
        val decoded = VaspJson.decode(json)
        assertEquals(profile.vaspVersion, decoded.vaspVersion)
        assertEquals(profile.identity.title, decoded.identity.title)
        assertEquals(
            profile.pillars.affective.thayerCoordinates.valence,
            decoded.pillars.affective.thayerCoordinates.valence,
            0.001,
        )
        Vasp.REQUIRED_PILLARS.forEach { key ->
            assertTrue("missing $key", json.contains("\"$key\""))
        }
    }

    @Test
    fun playerReadoutMatchesTechSpecFields() {
        val player = profile.toPlayerProfile()
        assertEquals("Aggressive", player.dominance)
        assertEquals(135, player.bpmPerceived)
        assertEquals("unknown Minor", player.keySignature)
        assertEquals(1.0f, player.spatialWidthRatio)
        assertEquals("Crisp Digital", player.texturalDensity)
        assertEquals("unknown", player.lyricProminence)
        assertEquals("#4B0082", player.primaryHex)
        assertEquals("#4B0082", player.secondaryHex)
        assertEquals("unknown", player.macroSetting)
        assertEquals("unknown", player.weather)
    }
}
