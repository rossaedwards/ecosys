package org.aurphyx.vibeaudioplayer.ui.theme

import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.darkColorScheme
import androidx.compose.runtime.Composable

private val VibeDark = darkColorScheme(
    primary = VibeAccent,
    onPrimary = VibeVoid,
    secondary = VibePrimary,
    onSecondary = VibeFg,
    background = VibeVoid,
    onBackground = VibeFg,
    surface = VibeSurface,
    onSurface = VibeFg,
    surfaceVariant = VibeSurface2,
    onSurfaceVariant = VibeMuted,
    outline = VibeMuted.copy(alpha = 0.24f),
)

@Composable
fun VibeAudioPlayerTheme(content: @Composable () -> Unit) {
    MaterialTheme(
        colorScheme = VibeDark,
        typography = Typography,
        content = content,
    )
}
