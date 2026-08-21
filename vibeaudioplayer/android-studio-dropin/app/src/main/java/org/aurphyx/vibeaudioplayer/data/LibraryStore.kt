package org.aurphyx.vibeaudioplayer.data

import android.content.Context
import android.net.Uri
import org.json.JSONArray
import org.json.JSONObject

class LibraryStore(context: Context) {
    private val prefs = context.getSharedPreferences("vibe_library", Context.MODE_PRIVATE)

    fun loadFiles(): List<Track> {
        val raw = prefs.getString("files", "[]") ?: "[]"
        val arr = JSONArray(raw)
        val out = mutableListOf<Track>()
        for (i in 0 until arr.length()) {
            val o = arr.getJSONObject(i)
            val id = o.getString("id")
            val title = o.getString("title")
            val uri = Uri.parse(o.getString("uri"))
            out += Track(
                id = id,
                title = title,
                artist = o.optString("artist", "Local file"),
                kind = Track.Kind.FILE,
                uri = uri,
                mime = o.optString("mime", "audio/*"),
                vasp = DemoCatalog.fileProfile(title),
                unsupported = o.optBoolean("unsupported", false),
            )
        }
        return out
    }

    fun saveFiles(tracks: List<Track>) {
        val arr = JSONArray()
        tracks.filter { it.kind == Track.Kind.FILE }.forEach { t ->
            arr.put(
                JSONObject()
                    .put("id", t.id)
                    .put("title", t.title)
                    .put("artist", t.artist)
                    .put("uri", t.uri?.toString() ?: "")
                    .put("mime", t.mime ?: "audio/*")
                    .put("unsupported", t.unsupported),
            )
        }
        prefs.edit().putString("files", arr.toString()).apply()
    }
}
