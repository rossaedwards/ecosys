package org.aurphyx.vibeaudioplayer

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.lifecycle.viewmodel.compose.viewModel
import org.aurphyx.vibeaudioplayer.audio.PlayerViewModel
import org.aurphyx.vibeaudioplayer.ui.VibePlayerApp
import org.aurphyx.vibeaudioplayer.ui.theme.VibeAudioPlayerTheme

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()
        setContent {
            VibeAudioPlayerTheme {
                val vm: PlayerViewModel = viewModel()
                VibePlayerApp(vm)
            }
        }
    }
}
