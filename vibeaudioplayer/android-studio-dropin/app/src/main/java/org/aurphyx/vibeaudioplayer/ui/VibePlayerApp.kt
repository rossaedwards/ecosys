package org.aurphyx.vibeaudioplayer.ui

import android.Manifest
import android.os.Build
import androidx.activity.compose.rememberLauncherForActivityResult
import androidx.activity.result.contract.ActivityResultContracts
import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.navigationBarsPadding
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Pause
import androidx.compose.material.icons.filled.PlayArrow
import androidx.compose.material.icons.filled.SkipNext
import androidx.compose.material.icons.filled.SkipPrevious
import androidx.compose.material3.AlertDialog
import androidx.compose.material3.Button
import androidx.compose.material3.ButtonDefaults
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.Slider
import androidx.compose.material3.SliderDefaults
import androidx.compose.material3.Switch
import androidx.compose.material3.SwitchDefaults
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableIntStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.BiasAlignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import org.aurphyx.vibeaudioplayer.R
import org.aurphyx.vibeaudioplayer.audio.PlayerViewModel
import org.aurphyx.vibeaudioplayer.data.Tab
import org.aurphyx.vibeaudioplayer.data.Track
import org.aurphyx.vibeaudioplayer.ui.orb.OrbCanvas
import org.aurphyx.vibeaudioplayer.ui.theme.VibeAccent
import org.aurphyx.vibeaudioplayer.ui.theme.VibeFg
import org.aurphyx.vibeaudioplayer.ui.theme.VibeMuted
import org.aurphyx.vibeaudioplayer.ui.theme.VibeSurface
import org.aurphyx.vibeaudioplayer.ui.theme.VibeSurface2
import org.aurphyx.vibeaudioplayer.ui.theme.VibeVoid

@Composable
fun VibePlayerApp(vm: PlayerViewModel) {
    val openDocs = rememberLauncherForActivityResult(
        contract = ActivityResultContracts.OpenMultipleDocuments(),
    ) { uris -> vm.addFiles(uris) }

    val permissionLauncher = rememberLauncherForActivityResult(
        contract = ActivityResultContracts.RequestMultiplePermissions(),
    ) { vm.dismissPermission() }

    Box(
        Modifier
            .fillMaxSize()
            .background(VibeVoid),
    ) {
        when (vm.tab) {
            Tab.Orb, Tab.Library, Tab.Vasp, Tab.Scene, Tab.About -> {
                if (vm.tab == Tab.Orb) {
                    OrbHome(vm)
                } else {
                    Column(
                        Modifier
                            .fillMaxSize()
                            .statusBarsPadding()
                            .padding(bottom = 84.dp),
                    ) {
                        when (vm.tab) {
                            Tab.Library -> LibraryPane(vm) {
                                openDocs.launch(arrayOf("audio/*"))
                            }
                            Tab.Vasp -> VaspPane(vm)
                            Tab.Scene -> ScenePane(vm)
                            Tab.About -> AboutPane()
                            else -> {}
                        }
                    }
                }
            }
        }

        PlayerTabBar(
            tab = vm.tab,
            onTab = vm::selectTab,
            modifier = Modifier.align(Alignment.BottomCenter),
        )

        vm.notice?.let { msg ->
            Text(
                msg,
                color = VibeFg,
                modifier = Modifier
                    .align(Alignment.TopCenter)
                    .statusBarsPadding()
                    .padding(top = 72.dp)
                    .clip(RoundedCornerShape(16.dp))
                    .background(VibeSurface)
                    .padding(horizontal = 16.dp, vertical = 10.dp)
                    .clickable { vm.clearNotice() },
            )
        }

        if (vm.showPermission) {
            AlertDialog(
                onDismissRequest = { vm.dismissPermission() },
                containerColor = VibeSurface,
                title = { Text("Local audio access", color = VibeFg) },
                text = {
                    Text(
                        "Vibe Audio Player reads music already on this phone so you can open MP3, WAV, FLAC, OGG, and M4A. Nothing is uploaded. Demos work without this permission.",
                        color = VibeMuted,
                    )
                },
                confirmButton = {
                    Button(
                        onClick = {
                            val perms = buildList {
                                if (Build.VERSION.SDK_INT >= 33) {
                                    add(Manifest.permission.READ_MEDIA_AUDIO)
                                    add(Manifest.permission.POST_NOTIFICATIONS)
                                } else {
                                    add(Manifest.permission.READ_EXTERNAL_STORAGE)
                                }
                            }
                            permissionLauncher.launch(perms.toTypedArray())
                        },
                        colors = ButtonDefaults.buttonColors(containerColor = VibeAccent, contentColor = VibeVoid),
                    ) { Text("Continue") }
                },
                dismissButton = {
                    TextButton(onClick = { vm.dismissPermission() }) {
                        Text("Play demos only", color = VibeMuted)
                    }
                },
            )
        }

        if (vm.showSplash) {
            Image(
                painter = painterResource(R.drawable.splash_loading),
                contentDescription = "Vibe Audio Player",
                contentScale = ContentScale.Crop,
                modifier = Modifier.fillMaxSize(),
            )
        }
    }
}

@Composable
private fun OrbHome(vm: PlayerViewModel) {
    val track = vm.current
    val readable = if (vm.settings.readableType) 1.12f else 1f
    Box(Modifier.fillMaxSize()) {
        OrbCanvas(vm.frame, vm.settings, Modifier.fillMaxSize())
        Column(
            Modifier
                .align(Alignment.TopStart)
                .statusBarsPadding()
                .padding(16.dp),
        ) {
            Text("AURPHYX", color = VibeAccent, fontSize = (11 * readable).sp, letterSpacing = 3.sp, fontWeight = FontWeight.SemiBold)
            Text("Vibe Audio Player", color = VibeFg, fontSize = (20 * readable).sp, fontWeight = FontWeight.SemiBold)
            Text("VASP 3.69", color = VibeMuted, fontSize = (11 * readable).sp, letterSpacing = 2.sp)
        }
        Column(
            Modifier
                .align(Alignment.BottomCenter)
                .padding(bottom = 88.dp)
                .padding(horizontal = 12.dp)
                .clip(RoundedCornerShape(28.dp))
                .background(VibeSurface.copy(alpha = 0.82f))
                .padding(12.dp),
        ) {
            Row(Modifier.fillMaxWidth(), horizontalArrangement = Arrangement.SpaceBetween) {
                Column(Modifier.weight(1f)) {
                    Text(track.title, color = VibeFg, fontSize = 18.sp, fontWeight = FontWeight.SemiBold, maxLines = 1, overflow = TextOverflow.Ellipsis)
                    Text(track.artist, color = VibeMuted, fontSize = 13.sp, maxLines = 1, overflow = TextOverflow.Ellipsis)
                }
                Text(
                    track.vasp.scene,
                    color = VibeAccent,
                    fontSize = 10.sp,
                    letterSpacing = 1.sp,
                    modifier = Modifier
                        .clip(CircleShape)
                        .background(VibeAccent.copy(alpha = 0.15f))
                        .padding(horizontal = 10.dp, vertical = 6.dp),
                )
            }
            Spacer(Modifier.height(8.dp))
            Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
                Chip("${track.vasp.bpm} BPM")
                Chip(track.vasp.key)
                Chip(track.vasp.mood.take(22))
            }
            val dur = vm.durationMs.coerceAtLeast(1L)
            val canSeek = track.kind == Track.Kind.FILE && vm.durationMs > 0
            Row(verticalAlignment = Alignment.CenterVertically, modifier = Modifier.padding(top = 8.dp)) {
                Text(if (track.kind == Track.Kind.DEMO) "LIVE" else formatMs(vm.positionMs), color = VibeMuted, fontSize = 11.sp, modifier = Modifier.width(36.dp))
                if (canSeek) {
                    Slider(
                        value = vm.positionMs.toFloat(),
                        onValueChange = { vm.seek(it.toLong()) },
                        valueRange = 0f..dur.toFloat(),
                        modifier = Modifier.weight(1f),
                        colors = SliderDefaults.colors(thumbColor = VibeAccent, activeTrackColor = VibeAccent),
                    )
                } else {
                    Box(
                        Modifier
                            .weight(1f)
                            .height(6.dp)
                            .clip(CircleShape)
                            .background(Color.White.copy(alpha = 0.12f)),
                    ) {
                        Box(
                            Modifier
                                .fillMaxWidth(if (vm.playing) 0.66f else 0.25f)
                                .height(6.dp)
                                .background(VibeAccent),
                        )
                    }
                }
                Text(if (track.kind == Track.Kind.DEMO) "∞" else formatMs(vm.durationMs), color = VibeMuted, fontSize = 11.sp, modifier = Modifier.width(36.dp), textAlign = TextAlign.End)
            }
            Row(
                Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceEvenly,
                verticalAlignment = Alignment.CenterVertically,
            ) {
                IconButton(onClick = { vm.prev() }) {
                    Icon(Icons.Filled.SkipPrevious, contentDescription = "Previous", tint = VibeFg, modifier = Modifier.size(28.dp))
                }
                IconButton(
                    onClick = { vm.playPause() },
                    modifier = Modifier
                        .size(64.dp)
                        .clip(CircleShape)
                        .background(VibeAccent),
                ) {
                    Icon(
                        if (vm.playing) Icons.Filled.Pause else Icons.Filled.PlayArrow,
                        contentDescription = if (vm.playing) "Pause" else "Play",
                        tint = VibeVoid,
                        modifier = Modifier.size(36.dp),
                    )
                }
                IconButton(onClick = { vm.next() }) {
                    Icon(Icons.Filled.SkipNext, contentDescription = "Next", tint = VibeFg, modifier = Modifier.size(28.dp))
                }
            }
        }
    }
}

@Composable
private fun Chip(text: String) {
    Text(
        text,
        color = VibeFg.copy(alpha = 0.9f),
        fontSize = 11.sp,
        modifier = Modifier
            .clip(CircleShape)
            .background(Color.White.copy(alpha = 0.06f))
            .padding(horizontal = 8.dp, vertical = 3.dp),
    )
}

@Composable
private fun LibraryPane(vm: PlayerViewModel, onOpen: () -> Unit) {
    LazyColumn(
        contentPadding = PaddingValues(16.dp),
        verticalArrangement = Arrangement.spacedBy(8.dp),
    ) {
        item {
            Text("Local library", color = VibeAccent, fontSize = 11.sp, letterSpacing = 2.sp)
            Text("Vibe Audio Player", color = VibeFg, fontSize = 22.sp, fontWeight = FontWeight.SemiBold)
            Text(
                "Play built-in VASP demos or open audio from this device. Files stay on your phone — nothing is uploaded.",
                color = VibeMuted,
                fontSize = 14.sp,
                modifier = Modifier.padding(top = 8.dp, bottom = 12.dp),
            )
            Button(
                onClick = onOpen,
                colors = ButtonDefaults.buttonColors(containerColor = VibeAccent, contentColor = VibeVoid),
                modifier = Modifier.fillMaxWidth(),
            ) { Text("Open local audio") }
        }
        items(vm.library, key = { it.id }) { track ->
            val active = track.id == vm.currentId
            Row(
                Modifier
                    .fillMaxWidth()
                    .clip(RoundedCornerShape(16.dp))
                    .background(if (active) Color.White.copy(alpha = 0.08f) else Color.White.copy(alpha = 0.03f))
                    .clickable { vm.playTrack(track.id) }
                    .padding(10.dp),
                verticalAlignment = Alignment.CenterVertically,
            ) {
                Column(Modifier.weight(1f)) {
                    Text(track.title, color = VibeFg, fontWeight = FontWeight.Medium, maxLines = 1, overflow = TextOverflow.Ellipsis)
                    Text(
                        buildString {
                            append(track.artist)
                            if (track.kind == Track.Kind.DEMO) append(" · Demo")
                        },
                        color = VibeMuted,
                        fontSize = 12.sp,
                    )
                    if (track.unsupported) {
                        Text("Format not supported", color = Color(0xFFFF8A80), fontSize = 11.sp)
                    }
                }
                if (track.kind == Track.Kind.FILE) {
                    TextButton(onClick = { vm.remove(track.id) }) { Text("Remove", color = VibeMuted) }
                }
            }
        }
    }
}

@Composable
private fun VaspPane(vm: PlayerViewModel) {
    val pillar = vm.profile.pillars.find { it.key == vm.activePillar } ?: vm.profile.pillars.first()
    LazyColumn(contentPadding = PaddingValues(16.dp), verticalArrangement = Arrangement.spacedBy(10.dp)) {
        item {
            Text("VASP ${vm.profile.title}", color = VibeAccent, fontSize = 11.sp, letterSpacing = 2.sp)
            Text("Nine-pillar profile", color = VibeFg, fontSize = 22.sp, fontWeight = FontWeight.SemiBold)
            Text(
                "Creative metadata for how this audio is structured, feels, appears, and moves. Visual mappings only.",
                color = VibeMuted,
                fontSize = 14.sp,
                modifier = Modifier.padding(top = 8.dp, bottom = 8.dp),
            )
        }
        item {
            val rows = vm.profile.pillars.chunked(3)
            Column(verticalArrangement = Arrangement.spacedBy(8.dp)) {
                rows.forEach { row ->
                    Row(horizontalArrangement = Arrangement.spacedBy(8.dp), modifier = Modifier.fillMaxWidth()) {
                        row.forEach { card ->
                            val on = card.key == vm.activePillar
                            Column(
                                Modifier
                                    .weight(1f)
                                    .clip(RoundedCornerShape(16.dp))
                                    .background(if (on) Color.White.copy(alpha = 0.1f) else Color.White.copy(alpha = 0.03f))
                                    .clickable { vm.setPillar(card.key) }
                                    .padding(10.dp),
                            ) {
                                Text(card.label, color = if (on) VibeAccent else VibeFg, fontSize = 12.sp, fontWeight = FontWeight.Medium)
                            }
                        }
                        repeat(3 - row.size) { Spacer(Modifier.weight(1f)) }
                    }
                }
            }
        }
        item {
            Column(
                Modifier
                    .fillMaxWidth()
                    .clip(RoundedCornerShape(24.dp))
                    .background(Color.White.copy(alpha = 0.04f))
                    .padding(16.dp),
            ) {
                Text(pillar.label, color = VibeFg, fontSize = 18.sp, fontWeight = FontWeight.SemiBold)
                Text(pillar.archetype.uppercase(), color = VibeMuted, fontSize = 11.sp, letterSpacing = 2.sp)
                Text(pillar.purpose, color = VibeMuted, fontSize = 14.sp, modifier = Modifier.padding(vertical = 8.dp))
                pillar.fields.forEach { (label, field) ->
                    Row(Modifier.fillMaxWidth().padding(vertical = 4.dp), horizontalArrangement = Arrangement.SpaceBetween) {
                        Text(label.uppercase(), color = VibeMuted, fontSize = 11.sp, letterSpacing = 1.sp)
                        Text(
                            field.value + if (field.status != "known") "  ${field.status}" else "",
                            color = VibeFg,
                            fontSize = 14.sp,
                            fontWeight = FontWeight.Medium,
                            modifier = Modifier.padding(start = 12.dp),
                            textAlign = TextAlign.End,
                        )
                    }
                }
            }
        }
    }
}

@Composable
private fun ScenePane(vm: PlayerViewModel) {
    val s = vm.settings
    LazyColumn(contentPadding = PaddingValues(16.dp)) {
        item {
            Text("Scene", color = VibeAccent, fontSize = 11.sp, letterSpacing = 2.sp)
            Text("Visualizer settings", color = VibeFg, fontSize = 22.sp, fontWeight = FontWeight.SemiBold)
            Spacer(Modifier.height(12.dp))
            Text("Output level", color = VibeFg)
            Slider(value = vm.volume, onValueChange = vm::changeVolume, colors = SliderDefaults.colors(thumbColor = VibeAccent, activeTrackColor = VibeAccent))
            Text("Color intensity", color = VibeFg)
            Slider(value = s.colorIntensity, onValueChange = { vm.setSetting(s.copy(colorIntensity = it)) }, valueRange = 0.2f..1f, colors = SliderDefaults.colors(thumbColor = VibeAccent, activeTrackColor = VibeAccent))
            Text("Motion intensity", color = VibeFg)
            Slider(value = s.motionIntensity, onValueChange = { vm.setSetting(s.copy(motionIntensity = it)) }, colors = SliderDefaults.colors(thumbColor = VibeAccent, activeTrackColor = VibeAccent))
            SettingRow("Particle field", s.particles) { vm.setSetting(s.copy(particles = it)) }
            SettingRow("Spectrum rings", s.spectrum) { vm.setSetting(s.copy(spectrum = it)) }
            SettingRow("Beat pulse", s.beatPulse) { vm.setSetting(s.copy(beatPulse = it)) }
            SettingRow("Reduced motion", s.reducedMotion) { vm.setSetting(s.copy(reducedMotion = it)) }
            SettingRow("Readable type", s.readableType) { vm.setSetting(s.copy(readableType = it)) }
        }
    }
}

@Composable
private fun SettingRow(label: String, checked: Boolean, on: (Boolean) -> Unit) {
    Row(
        Modifier.fillMaxWidth().padding(vertical = 10.dp),
        horizontalArrangement = Arrangement.SpaceBetween,
        verticalAlignment = Alignment.CenterVertically,
    ) {
        Text(label, color = VibeFg)
        Switch(checked = checked, onCheckedChange = on, colors = SwitchDefaults.colors(checkedThumbColor = VibeVoid, checkedTrackColor = VibeAccent))
    }
}

@Composable
private fun AboutPane() {
    var taps by remember { mutableIntStateOf(0) }
    val unlocked = taps >= 13
    LazyColumn(
        contentPadding = PaddingValues(16.dp),
        horizontalAlignment = Alignment.CenterHorizontally,
        modifier = Modifier.fillMaxWidth(),
    ) {
        item {
            Text("AURPHYX", color = VibeAccent, fontSize = 11.sp, letterSpacing = 3.sp, modifier = Modifier.clickable { taps += 1 })
            Text(
                "About Aurphyx",
                color = VibeFg,
                fontSize = 22.sp,
                fontWeight = FontWeight.SemiBold,
                modifier = Modifier.clickable { taps += 1 },
            )
            Image(
                painter = painterResource(R.drawable.ic_launcher_photo),
                contentDescription = "Aurphyx",
                modifier = Modifier
                    .padding(vertical = 16.dp)
                    .size(96.dp)
                    .clip(CircleShape)
                    .clickable { taps += 1 },
            )
            Text(
                "Aurphyx LLC — Ross A. Edwards. Vibe Audio Player is a local music surface for the V.A.P. nine-pillar TSLCA orb. Nothing leaves the device.",
                color = VibeMuted,
                fontSize = 15.sp,
                textAlign = TextAlign.Center,
                modifier = Modifier.clickable { taps += 1 }.padding(horizontal = 8.dp),
            )
            Spacer(Modifier.height(16.dp))
            if (unlocked) {
                Text("BUSINESS CARD", color = VibeAccent, fontSize = 11.sp, letterSpacing = 2.sp)
                Text("Ross A. Edwards", color = VibeFg, fontSize = 18.sp, fontWeight = FontWeight.SemiBold)
                Text("Founder, Aurphyx LLC", color = VibeMuted, fontSize = 14.sp)
                Image(
                    painter = painterResource(R.drawable.aurphyx_business_card_qr),
                    contentDescription = "Aurphyx business card QR",
                    modifier = Modifier
                        .padding(top = 16.dp)
                        .size(220.dp)
                        .clip(RoundedCornerShape(16.dp))
                        .background(VibeFg)
                        .padding(8.dp),
                )
                Text("Scan for vCard · GitHub + LinkedIn", color = VibeMuted, fontSize = 12.sp, modifier = Modifier.padding(top = 8.dp))
            } else if (taps in 8..12) {
                Text("…", color = VibeMuted, fontSize = 12.sp)
            }
        }
    }
}

@Composable
private fun PlayerTabBar(tab: Tab, onTab: (Tab) -> Unit, modifier: Modifier = Modifier) {
    data class Item(val id: Tab, val label: String, val align: Offset)
    val items = listOf(
        Item(Tab.Orb, "Orb", Offset(-0.36f, 0.12f)),
        Item(Tab.Library, "Library", Offset(-0.36f, 0.48f)),
        Item(Tab.Vasp, "VASP", Offset(-0.36f, -0.24f)),
        Item(Tab.Scene, "Scene", Offset(0.36f, 0.48f)),
        Item(Tab.About, "About", Offset(0.36f, 0.12f)),
    )
    Row(
        modifier
            .fillMaxWidth()
            .background(VibeSurface.copy(alpha = 0.92f))
            .navigationBarsPadding()
            .padding(horizontal = 4.dp, vertical = 4.dp),
        horizontalArrangement = Arrangement.SpaceEvenly,
    ) {
        items.forEach { item ->
            val on = tab == item.id
            Column(
                horizontalAlignment = Alignment.CenterHorizontally,
                modifier = Modifier
                    .weight(1f)
                    .clip(RoundedCornerShape(16.dp))
                    .clickable { onTab(item.id) }
                    .padding(vertical = 4.dp),
            ) {
                Image(
                    painter = painterResource(R.drawable.vap_menu_2_tb),
                    contentDescription = item.label,
                    contentScale = ContentScale.Crop,
                    alignment = BiasAlignment(item.align.x, item.align.y),
                    modifier = Modifier
                        .size(36.dp)
                        .clip(CircleShape)
                        .background(if (on) VibeAccent.copy(alpha = 0.25f) else VibeSurface2),
                )
                Text(item.label, color = if (on) VibeAccent else VibeMuted, fontSize = 10.sp)
            }
        }
    }
}

private fun formatMs(ms: Long): String {
    val s = (ms / 1000).coerceAtLeast(0)
    return "${s / 60}:${(s % 60).toString().padStart(2, '0')}"
}
