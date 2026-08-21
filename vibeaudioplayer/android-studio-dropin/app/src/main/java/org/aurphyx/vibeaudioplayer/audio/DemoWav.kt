package org.aurphyx.vibeaudioplayer.audio

import android.content.Context
import java.io.File
import kotlin.math.exp
import kotlin.math.sin

object DemoWav {
    data class Spec(val id: String, val bpm: Int, val rootHz: Float, val drive: Float, val bright: Float, val hats: Float)

    val SPECS = listOf(
        Spec("demo-night-drive", 128, 110f, 0.86f, 0.62f, 1f),
        Spec("demo-orbital-lattice", 96, 146.83f, 0.55f, 0.42f, 0.5f),
        Spec("demo-forward-current", 140, 185f, 0.92f, 0.78f, 1f),
    )

    fun ensure(context: Context): Map<String, File> {
        val dir = File(context.cacheDir, "demos").apply { mkdirs() }
        return SPECS.associate { spec ->
            val file = File(dir, "${spec.id}.wav")
            if (!file.exists() || file.length() < 1000) write(file, spec)
            spec.id to file
        }
    }

    private fun write(file: File, spec: Spec) {
        val sr = 22050
        val seconds = 8
        val n = sr * seconds
        val pcm = ShortArray(n)
        val beat = 60.0 / spec.bpm
        for (i in 0 until n) {
            val t = i / sr.toDouble()
            val phase = (t % beat) / beat
            val kick = exp(-(phase * 7.2) * (phase * 7.2)) * spec.drive
            val hatPhase = (t * spec.bpm / 60.0 * 4.0)
            val hat = if (hatPhase % 1.0 < 0.08) spec.hats * 0.18 * (1.0 - (hatPhase % 1.0) / 0.08) else 0.0
            val bass = sin(2 * Math.PI * spec.rootHz * 0.5 * t) * 0.22 * spec.drive
            val pad = sin(2 * Math.PI * spec.rootHz * t) * 0.05 * spec.bright
            val noise = (Math.random() * 2 - 1) * hat
            val kickTone = sin(2 * Math.PI * (150 - phase * 110) * t) * kick * 0.55
            val sample = (kickTone + bass + pad + noise).coerceIn(-0.95, 0.95)
            pcm[i] = (sample * 32767).toInt().toShort()
        }
        val dataBytes = n * 2
        val out = ByteArray(44 + dataBytes)
        fun str(off: Int, s: String) = s.forEachIndexed { i, c -> out[off + i] = c.code.toByte() }
        fun le16(off: Int, v: Int) {
            out[off] = (v and 0xFF).toByte()
            out[off + 1] = ((v shr 8) and 0xFF).toByte()
        }
        fun le32(off: Int, v: Int) {
            out[off] = (v and 0xFF).toByte()
            out[off + 1] = ((v shr 8) and 0xFF).toByte()
            out[off + 2] = ((v shr 16) and 0xFF).toByte()
            out[off + 3] = ((v shr 24) and 0xFF).toByte()
        }
        str(0, "RIFF")
        le32(4, 36 + dataBytes)
        str(8, "WAVE")
        str(12, "fmt ")
        le32(16, 16)
        le16(20, 1)
        le16(22, 1)
        le32(24, sr)
        le32(28, sr * 2)
        le16(32, 2)
        le16(34, 16)
        str(36, "data")
        le32(40, dataBytes)
        var p = 44
        for (s in pcm) {
            out[p++] = (s.toInt() and 0xFF).toByte()
            out[p++] = ((s.toInt() shr 8) and 0xFF).toByte()
        }
        file.writeBytes(out)
    }
}
