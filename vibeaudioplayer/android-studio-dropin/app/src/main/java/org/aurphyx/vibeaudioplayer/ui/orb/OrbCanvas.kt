package org.aurphyx.vibeaudioplayer.ui.orb

import androidx.compose.foundation.Canvas
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.graphics.Brush
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.StrokeCap
import androidx.compose.ui.graphics.drawscope.Stroke
import kotlin.math.abs
import kotlin.math.cos
import kotlin.math.min
import kotlin.math.sin
import org.aurphyx.vibeaudioplayer.data.OrbFrame
import org.aurphyx.vibeaudioplayer.data.SceneSettings

@Composable
fun OrbCanvas(frame: OrbFrame, settings: SceneSettings, modifier: Modifier = Modifier) {
    Canvas(modifier.fillMaxSize()) {
        val w = size.width
        val h = size.height
        val cx = w / 2f
        val cy = h * 0.42f
        val scale = min(w, h) * 0.46f
        drawRect(Color(0xFF07060C))

        val bloomR = scale * (1.6f + frame.bloom * 0.5f)
        drawCircle(
            brush = Brush.radialGradient(
                colors = listOf(
                    frame.primary.copy(alpha = 0.28f + frame.bloom * 0.2f),
                    frame.secondary.copy(alpha = 0.12f),
                    Color.Transparent,
                ),
                center = Offset(cx, cy),
                radius = bloomR,
            ),
            radius = bloomR,
            center = Offset(cx, cy),
        )

        val mNode = 2f + frame.syncopation * 6f
        val nNode = mNode + 1f + frame.groove * 2f
        val cols = if (settings.reducedMotion) 28 else 48
        val rows = cols
        val stepX = w / cols
        val stepY = h / rows
        for (j in 0 until rows) {
            for (i in 0 until cols) {
                val nx = i / (cols - 1f) * 2f - 1f
                val ny = j / (rows - 1f) * 2f - 1f
                val px = nx * (w / h)
                val v = abs(chladni(px * 0.8f, ny * 0.8f, mNode, nNode))
                if (v > 0.055f) continue
                val a = (1f - v / 0.055f) * (0.16f + frame.arousal * 0.32f)
                drawRect(
                    color = frame.primary.copy(alpha = a),
                    topLeft = Offset(cx + nx * scale * 1.35f, cy + ny * scale * 1.35f),
                    size = androidx.compose.ui.geometry.Size(stepX.coerceAtLeast(2f), stepY.coerceAtLeast(2f)),
                )
            }
        }

        val pulseR = (0.35f + frame.arousal * 0.25f + sin(frame.time * frame.bpmNorm * 6.28f) * 0.05f) * scale
        drawCircle(
            color = frame.primary.copy(alpha = 0.55f + frame.chrom.getOrElse(1) { 0f } * 0.3f),
            radius = pulseR,
            center = Offset(cx, cy),
            style = Stroke(width = 2.5f + (1f - frame.groove) * 2f),
        )

        if (settings.spectrum) {
            val bands = arrayOf(
                Color(0.85f, 0.05f, 0.05f) to frame.chrom.getOrElse(0) { 0f },
                Color(1f, 0.55f, 0f) to frame.chrom.getOrElse(1) { 0f },
                Color(0.1f, 0.75f, 0.55f) to frame.chrom.getOrElse(2) { 0f },
                Color(0.3f, 0.15f, 0.95f) to frame.chrom.getOrElse(3) { 0f },
            )
            bands.forEachIndexed { b, (col, mag) ->
                val a0 = -Math.PI.toFloat() / 2f + b * (Math.PI.toFloat() / 2f) * 0.92f
                drawArc(
                    color = col.copy(alpha = 0.25f + mag * 0.7f),
                    startAngle = Math.toDegrees(a0.toDouble()).toFloat(),
                    sweepAngle = 76f,
                    useCenter = false,
                    topLeft = Offset(cx - scale * 0.92f, cy - scale * 0.92f),
                    size = androidx.compose.ui.geometry.Size(scale * 1.84f, scale * 1.84f),
                    style = Stroke(width = 3f + mag * 8f, cap = StrokeCap.Round),
                )
            }
        }

        val orbR = scale * (0.22f + frame.chrom.getOrElse(1) { 0f } * 0.04f + frame.arousal * 0.04f + frame.beatPulse * 0.03f)
        drawCircle(
            brush = Brush.radialGradient(
                colors = listOf(Color.White.copy(alpha = 0.75f), frame.secondary, frame.primary, Color.Transparent),
                center = Offset(cx - orbR * 0.2f, cy - orbR * 0.25f),
                radius = orbR * 1.5f,
            ),
            radius = orbR * 1.45f,
            center = Offset(cx, cy),
        )

        for (i in 0 until 9) {
            val a = i / 9f * Math.PI.toFloat() * 2f - Math.PI.toFloat() / 2f + frame.time * 0.04f
            val rad = scale * (0.58f + frame.arousal * 0.04f)
            val mag = frame.chrom[i % 4]
            drawCircle(
                color = lerp(frame.secondary, frame.primary, i / 8f).copy(alpha = 0.35f + mag * 0.55f),
                radius = 5f + mag * 6f,
                center = Offset(cx + cos(a) * rad, cy + sin(a) * rad),
            )
        }
    }
}

private fun chladni(x: Float, y: Float, m: Float, n: Float): Float {
    val pi = Math.PI.toFloat()
    return cos(m * pi * x) * cos(n * pi * y) - cos(n * pi * x) * cos(m * pi * y)
}

private fun lerp(a: Color, b: Color, t: Float) = Color(
    a.red + (b.red - a.red) * t,
    a.green + (b.green - a.green) * t,
    a.blue + (b.blue - a.blue) * t,
    1f,
)
