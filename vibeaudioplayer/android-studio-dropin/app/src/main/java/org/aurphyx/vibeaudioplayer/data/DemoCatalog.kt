package org.aurphyx.vibeaudioplayer.data

object DemoCatalog {
    private fun f(v: String, status: String = "known") = VaspField(v, status)

    private fun nine(
        structural: List<Pair<String, VaspField>>,
        tonal: List<Pair<String, VaspField>>,
        timbral: List<Pair<String, VaspField>>,
        linguistic: List<Pair<String, VaspField>>,
        affective: List<Pair<String, VaspField>>,
        contextual: List<Pair<String, VaspField>>,
        photometric: List<Pair<String, VaspField>>,
        kinetic: List<Pair<String, VaspField>>,
        genealogical: List<Pair<String, VaspField>>,
    ) = listOf(
        PillarCard("STRUCTURAL", "Structural", "The Skeleton", "Tempo, groove, kick pulse, arrangement", structural),
        PillarCard("TONAL", "Tonal", "The Flesh", "Key, harmony, dissonance, contour", tonal),
        PillarCard("TIMBRAL", "Timbral", "The Skin", "Spectral balance, texture, production", timbral),
        PillarCard("LINGUISTIC", "Linguistic", "The Voice", "Lyrics, language, vocal style", linguistic),
        PillarCard("AFFECTIVE", "Affective", "The Heart", "Valence, arousal, mood, tension", affective),
        PillarCard("CONTEXTUAL", "Contextual", "The Scene", "Scenario, setting, atmosphere", contextual),
        PillarCard("PHOTOMETRIC", "Photometric", "The Eye", "Palette, brightness, light behavior", photometric),
        PillarCard("KINETIC", "Kinetic", "The Body", "Entrainment, movement energy, MET", kinetic),
        PillarCard("GENEALOGICAL", "Genealogical", "The Roots", "Genre, lineage, era, tribe", genealogical),
    )

    val nightDrive = VaspProfile(
        title = "Night Drive Protocol",
        artist = "Aurphyx Demo",
        bpm = 128,
        primary = 0xFF4B0082,
        secondary = 0xFF008080,
        valence = -0.22f,
        arousal = 0.82f,
        groove = 0.08f,
        syncopation = 0.18f,
        dissonance = 0.38f,
        fog = 0.58f,
        brightness = 0.62f,
        entrainment = 76f,
        scene = "Night drive",
        mood = "Focused, nocturnal, energetic",
        key = "A Minor",
        pillars = nine(
            listOf("BPM" to f("128"), "Groove" to f("Machine-lock groove"), "Kick pulse" to f("Strong kick pulse")),
            listOf("Key" to f("A Minor"), "Dissonance" to f("Moderate dissonance")),
            listOf("Spectral" to f("Bright and airy"), "Texture" to f("Glassy electronic texture")),
            listOf("Lyrics" to f("Instrumental"), "Content" to f("Clean content tier")),
            listOf("Valence" to f("Low-to-neutral valence"), "Arousal" to f("High arousal"), "Mood" to f("Focused, nocturnal, energetic")),
            listOf("Scenario" to f("Night drive"), "Atmosphere" to f("Wet asphalt, sodium-to-neon")),
            listOf("Primary" to f("#4B0082"), "Secondary" to f("#008080"), "Brightness" to f("0.62")),
            listOf("Entrainment" to f("Strong beat entrainment"), "MET" to f("6")),
            listOf("Genre" to f("Electronic"), "Lineage" to f("Synthwave-inspired")),
        ),
    )

    val orbital = VaspProfile(
        title = "Orbital Lattice",
        artist = "Aurphyx Demo",
        bpm = 96,
        primary = 0xFF1B1464,
        secondary = 0xFF5B8DEF,
        valence = 0.28f,
        arousal = 0.34f,
        groove = 0.42f,
        syncopation = 0.22f,
        dissonance = 0.12f,
        fog = 0.32f,
        brightness = 0.44f,
        entrainment = 38f,
        scene = "Deep space drift",
        mood = "Contemplative, weightless",
        key = "D Minor",
        pillars = nine(
            listOf("BPM" to f("96"), "Groove" to f("Slow-orbit pulse"), "Kick pulse" to f("Rounded kick, wide decay")),
            listOf("Key" to f("D Minor"), "Dissonance" to f("Low dissonance")),
            listOf("Spectral" to f("Dark, sub-heavy"), "Texture" to f("Velvet drone + glass chime")),
            listOf("Lyrics" to f("Instrumental"), "Content" to f("Clean content tier")),
            listOf("Valence" to f("Neutral-positive valence"), "Arousal" to f("Mid-low arousal"), "Mood" to f("Contemplative, weightless")),
            listOf("Scenario" to f("Deep space drift"), "Atmosphere" to f("Vacuum hush, distant stars")),
            listOf("Primary" to f("#1B1464"), "Secondary" to f("#5B8DEF"), "Brightness" to f("0.44")),
            listOf("Entrainment" to f("Breath-paced pulse"), "MET" to f("3")),
            listOf("Genre" to f("Ambient electronic"), "Lineage" to f("Orbital / IDM adjacent")),
        ),
    )

    val forward = VaspProfile(
        title = "Forward Current",
        artist = "Aurphyx Demo",
        bpm = 140,
        primary = 0xFF0D7377,
        secondary = 0xFF7C6CFF,
        valence = 0.45f,
        arousal = 0.94f,
        groove = 0.08f,
        syncopation = 0.28f,
        dissonance = 0.55f,
        fog = 0.5f,
        brightness = 0.74f,
        entrainment = 86f,
        scene = "Rain circuit",
        mood = "Urgent, kinetic, lucid",
        key = "F# Minor",
        pillars = nine(
            listOf("BPM" to f("140"), "Groove" to f("High-energy lock"), "Kick pulse" to f("Hard four-on-the-floor")),
            listOf("Key" to f("F# Minor"), "Dissonance" to f("Bright tension")),
            listOf("Spectral" to f("Crisp highs, present mids"), "Texture" to f("Electric current, metallic hats")),
            listOf("Lyrics" to f("Instrumental"), "Content" to f("Clean content tier")),
            listOf("Valence" to f("Neutral-high valence"), "Arousal" to f("Very high arousal"), "Mood" to f("Urgent, kinetic, lucid")),
            listOf("Scenario" to f("Rain circuit"), "Atmosphere" to f("Wet glass, sodium flares")),
            listOf("Primary" to f("#0D7377"), "Secondary" to f("#7C6CFF"), "Brightness" to f("0.74")),
            listOf("Entrainment" to f("Body-lock beat"), "MET" to f("7")),
            listOf("Genre" to f("Electronic"), "Lineage" to f("Techno-adjacent synthwave")),
        ),
    )

    fun fileProfile(title: String): VaspProfile {
        val h = title.hashCode()
        val hue = (h ushr 1) % 360
        fun hsl(hDeg: Int, s: Float, l: Float): Long {
            val sat = s
            val lig = l
            fun k(n: Int) = (n + hDeg / 30f) % 12f
            val a = sat * minOf(lig, 1f - lig)
            fun f(n: Int): Float {
                val x = minOf(k(n) - 3f, minOf(9f - k(n), 1f))
                return lig - a * maxOf(-1f, x)
            }
            val r = (f(0) * 255).toInt().coerceIn(0, 255)
            val g = (f(8) * 255).toInt().coerceIn(0, 255)
            val b = (f(4) * 255).toInt().coerceIn(0, 255)
            return 0xFF000000L or (r.toLong() shl 16) or (g.toLong() shl 8) or b.toLong()
        }
        val primary = hsl((hue + 260) % 360, 0.72f, 0.28f)
        val secondary = hsl((hue + 175) % 360, 0.64f, 0.38f)
        return VaspProfile(
            title = title,
            artist = "Local file",
            bpm = 120,
            primary = primary,
            secondary = secondary,
            valence = 0f,
            arousal = 0.55f,
            groove = 0.28f,
            syncopation = 0.2f,
            dissonance = 0.22f,
            fog = 0.24f,
            brightness = 0.6f,
            entrainment = 55f,
            scene = "Local playback",
            mood = "Listening",
            key = "Pending",
            pillars = nine(
                listOf("BPM" to f("Pending", "pending"), "Groove" to f("Pending analysis", "pending")),
                listOf("Key" to f("Pending", "pending"), "Dissonance" to f("Pending", "pending")),
                listOf("Spectral" to f("Pending", "pending"), "Fidelity" to f("Source file")),
                listOf("Lyrics" to f("Unknown", "unknown"), "Language" to f("Unknown", "unknown")),
                listOf("Valence" to f("Pending", "pending"), "Arousal" to f("Pending", "pending")),
                listOf("Scenario" to f("Local playback"), "Setting" to f("On device")),
                listOf("Primary" to f("#${primary.toString(16).takeLast(6).uppercase()}"), "Secondary" to f("#${secondary.toString(16).takeLast(6).uppercase()}")),
                listOf("Entrainment" to f("Pending", "pending"), "MET" to f("—", "pending")),
                listOf("Lineage" to f("Local library"), "Tribe" to f("Personal collection")),
            ),
        )
    }

    fun demos(): List<Track> = listOf(
        Track("demo-night-drive", nightDrive.title, nightDrive.artist, Track.Kind.DEMO, null, "audio/wav", nightDrive),
        Track("demo-orbital-lattice", orbital.title, orbital.artist, Track.Kind.DEMO, null, "audio/wav", orbital),
        Track("demo-forward-current", forward.title, forward.artist, Track.Kind.DEMO, null, "audio/wav", forward),
    )
}
