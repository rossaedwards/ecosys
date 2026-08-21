package org.aurphyx.vibeaudioplayer.audio

import android.app.Application
import android.content.ComponentName
import android.content.Intent
import android.net.Uri
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableFloatStateOf
import androidx.compose.runtime.mutableLongStateOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.setValue
import androidx.core.content.ContextCompat
import androidx.lifecycle.AndroidViewModel
import androidx.lifecycle.viewModelScope
import androidx.media3.common.MediaItem
import androidx.media3.common.MediaMetadata
import androidx.media3.common.PlaybackException
import androidx.media3.common.Player
import androidx.media3.session.MediaController
import androidx.media3.session.SessionToken
import kotlin.math.exp
import kotlin.math.sin
import kotlinx.coroutines.delay
import kotlinx.coroutines.isActive
import kotlinx.coroutines.launch
import org.aurphyx.vibeaudioplayer.data.DemoCatalog
import org.aurphyx.vibeaudioplayer.data.LibraryStore
import org.aurphyx.vibeaudioplayer.data.OrbFrame
import org.aurphyx.vibeaudioplayer.data.SceneSettings
import org.aurphyx.vibeaudioplayer.data.Tab
import org.aurphyx.vibeaudioplayer.data.Track
import org.aurphyx.vibeaudioplayer.data.VaspProfile

class PlayerViewModel(app: Application) : AndroidViewModel(app) {
    private val store = LibraryStore(app)
    private var controller: MediaController? = null
    private val controllerFuture = MediaController.Builder(
        app,
        SessionToken(app, ComponentName(app, PlaybackService::class.java)),
    ).buildAsync()

    var tab by mutableStateOf(Tab.Orb)
        private set
    var library by mutableStateOf(DemoCatalog.demos())
        private set
    var currentId by mutableStateOf("demo-night-drive")
        private set
    var playing by mutableStateOf(false)
        private set
    var positionMs by mutableLongStateOf(0L)
        private set
    var durationMs by mutableLongStateOf(0L)
        private set
    var volume by mutableFloatStateOf(0.85f)
        private set
    var settings by mutableStateOf(SceneSettings())
        private set
    var notice by mutableStateOf<String?>(null)
        private set
    var showPermission by mutableStateOf(true)
        private set
    var showSplash by mutableStateOf(true)
        private set
    var activePillar by mutableStateOf("PHOTOMETRIC")
        private set
    var frame by mutableStateOf(sampleFrame())
        private set

    val current: Track
        get() = library.find { it.id == currentId } ?: library.first()

    val profile: VaspProfile
        get() = current.vasp

    private val listener = object : Player.Listener {
        override fun onIsPlayingChanged(isPlaying: Boolean) {
            playing = isPlaying
        }

        override fun onMediaItemTransition(mediaItem: MediaItem?, reason: Int) {
            mediaItem?.mediaId?.let { currentId = it }
        }

        override fun onPlayerError(error: PlaybackException) {
            markUnsupported(currentId)
            notice = "Format not supported on this device."
            controller?.seekToNextMediaItem()
            controller?.prepare()
            controller?.play()
        }

        override fun onPlaybackStateChanged(playbackState: Int) {
            durationMs = controller?.duration?.coerceAtLeast(0) ?: 0L
        }
    }

    init {
        library = DemoCatalog.demos() + store.loadFiles()
        controllerFuture.addListener(
            {
                val c = runCatching { controllerFuture.get() }.getOrNull() ?: return@addListener
                controller = c
                c.addListener(listener)
                c.volume = volume
                syncFromPlayer()
            },
            ContextCompat.getMainExecutor(app),
        )
        viewModelScope.launch {
            delay(1600)
            showSplash = false
        }
        viewModelScope.launch {
            while (isActive) {
                val c = controller
                if (c != null) {
                    positionMs = c.currentPosition.coerceAtLeast(0)
                    durationMs = c.duration.coerceAtLeast(0)
                    if (c.currentMediaItem?.mediaId != null) currentId = c.currentMediaItem!!.mediaId
                    playing = c.isPlaying
                }
                frame = sampleFrame()
                delay(16)
            }
        }
    }

    fun selectTab(t: Tab) {
        tab = t
    }

    fun dismissPermission() {
        showPermission = false
    }

    fun setPillar(key: String) {
        activePillar = key
        tab = Tab.Vasp
    }

    fun setSetting(next: SceneSettings) {
        settings = next
    }

    fun changeVolume(v: Float) {
        volume = v
        controller?.volume = v
    }

    fun playPause() {
        val c = controller ?: return
        if (c.isPlaying) c.pause() else c.play()
    }

    fun playTrack(id: String) {
        val c = controller ?: return
        val idx = (0 until c.mediaItemCount).firstOrNull { c.getMediaItemAt(it).mediaId == id }
        if (idx != null) {
            c.seekTo(idx, 0)
            c.play()
        }
        currentId = id
    }

    fun next() {
        controller?.seekToNextMediaItem()
        controller?.play()
    }

    fun prev() {
        val c = controller ?: return
        if (c.currentPosition > 3000) c.seekTo(0) else {
            c.seekToPreviousMediaItem()
            c.play()
        }
    }

    fun seek(ms: Long) {
        controller?.seekTo(ms)
    }

    fun addFiles(uris: List<Uri>) {
        if (uris.isEmpty()) return
        val app = getApplication<Application>()
        val added = mutableListOf<Track>()
        uris.forEach { uri ->
            runCatching {
                app.contentResolver.takePersistableUriPermission(uri, Intent.FLAG_GRANT_READ_URI_PERMISSION)
            }
            val name = queryName(uri) ?: uri.lastPathSegment ?: "Audio"
            val title = name.substringBeforeLast('.').replace('_', ' ').replace('-', ' ').trim()
            val mime = app.contentResolver.getType(uri) ?: guessMime(name)
            val id = "file-${uri.hashCode()}-${title.hashCode()}"
            if (library.any { it.id == id || it.uri == uri }) return@forEach
            val track = Track(id, title, "Local file", Track.Kind.FILE, uri, mime, DemoCatalog.fileProfile(title))
            added += track
            controller?.addMediaItem(
                MediaItem.Builder()
                    .setMediaId(id)
                    .setUri(uri)
                    .setMimeType(mime)
                    .setMediaMetadata(
                        MediaMetadata.Builder()
                            .setTitle(title)
                            .setArtist("Local file")
                            .setAlbumTitle("Vibe Audio Player")
                            .build(),
                    )
                    .build(),
            )
        }
        if (added.isEmpty()) return
        library = library + added
        store.saveFiles(library)
        tab = Tab.Library
        notice = "Added ${added.size} track${if (added.size == 1) "" else "s"}."
        if (!playing) playTrack(added.first().id)
    }

    fun remove(id: String) {
        val track = library.find { it.id == id } ?: return
        if (track.kind == Track.Kind.DEMO) return
        val c = controller
        val idx = (0 until (c?.mediaItemCount ?: 0)).firstOrNull { c?.getMediaItemAt(it)?.mediaId == id }
        if (idx != null) c?.removeMediaItem(idx)
        library = library.filterNot { it.id == id }
        store.saveFiles(library)
        if (currentId == id) {
            playTrack(library.first().id)
        }
    }

    fun clearNotice() {
        notice = null
    }

    private fun markUnsupported(id: String) {
        library = library.map { if (it.id == id) it.copy(unsupported = true) else it }
        store.saveFiles(library)
    }

    private fun syncFromPlayer() {
        val c = controller ?: return
        playing = c.isPlaying
        c.currentMediaItem?.mediaId?.let { currentId = it }
        positionMs = c.currentPosition
        durationMs = c.duration.coerceAtLeast(0)
    }

    private fun queryName(uri: Uri): String? {
        val cr = getApplication<Application>().contentResolver
        cr.query(uri, arrayOf(android.provider.OpenableColumns.DISPLAY_NAME), null, null, null)?.use { cursor ->
            if (cursor.moveToFirst()) return cursor.getString(0)
        }
        return null
    }

    private fun guessMime(name: String): String {
        val n = name.lowercase()
        return when {
            n.endsWith(".mp3") -> "audio/mpeg"
            n.endsWith(".wav") -> "audio/wav"
            n.endsWith(".flac") -> "audio/flac"
            n.endsWith(".ogg") -> "audio/ogg"
            n.endsWith(".m4a") -> "audio/mp4"
            n.endsWith(".aac") -> "audio/aac"
            else -> "audio/*"
        }
    }

    private fun sampleFrame(): OrbFrame {
        val p = profile
        val bpm = p.bpm.coerceAtLeast(60)
        val beatLen = 60_000f / bpm
        val t = (positionMs / 1000f)
        val phase = if (playing) ((positionMs % beatLen.toLong()) / beatLen) else 0.4f
        val kick = if (playing) exp(-(phase * 7.2f) * (phase * 7.2f)) else 0.12f
        val hat = if (playing) {
            val hp = (t * bpm / 60f * 4f)
            val frac = hp - hp.toInt()
            if (frac < 0.12f) (1f - frac / 0.12f) * 0.45f else 0.05f
        } else 0.05f
        val bass = if (playing) 0.35f + 0.35f * sin(t * Math.PI * (bpm / 60.0) * 0.5).toFloat() + kick * 0.5f else 0.12f
        val motion = if (settings.reducedMotion) 0.18f else settings.motionIntensity
        val color = settings.colorIntensity
        val chrom = floatArrayOf(
            (bass * 0.7f + kick * 0.5f) * color,
            (kick * 0.85f + bass * 0.25f) * color,
            (0.25f + 0.4f * sin(t * 2.1f + 1f)) * (if (playing) 1f else 0.3f) * color,
            hat * (0.6f + p.brightness) * color,
        )
        val energy = (chrom[0] * 0.4f + chrom[1] * 0.35f + chrom[2] * 0.15f).coerceIn(0f, 1f)
        val arousal = (p.arousal * 0.55f + energy * 0.45f).coerceIn(0f, 1f)
        val pulse = if (settings.beatPulse) kick else 0f
        val ceiling = (p.brightness * (0.7f + color * 0.5f)).coerceIn(0.2f, 1f)
        return OrbFrame(
            time = t * (0.35f + motion * 0.9f),
            chrom = chrom,
            centroid = 180f + chrom[3] * 2400f + chrom[2] * 800f,
            saturation = (chrom[3] / (chrom[0] + chrom[1] + 0.05f)).coerceIn(0f, 1f),
            syncopation = p.syncopation + pulse * 0.08f,
            bpmNorm = (bpm / 180f).coerceAtMost(1f) * (0.35f + motion * 0.65f),
            groove = p.groove,
            dissonance = p.dissonance,
            valence = p.valence,
            arousal = arousal,
            fog = p.fog,
            primary = p.primaryColor,
            secondary = p.secondaryColor,
            brightnessFloor = 0.04f,
            brightnessCeiling = ceiling,
            visualNoise = if (settings.reducedMotion) 0f else 0.12f,
            entrainment = p.entrainment,
            bloom = arousal * ceiling * 1.5f * color,
            beatPulse = pulse,
        )
    }

    override fun onCleared() {
        controller?.removeListener(listener)
        MediaController.releaseFuture(controllerFuture)
        super.onCleared()
    }
}
