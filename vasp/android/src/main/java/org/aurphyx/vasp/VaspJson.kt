package org.aurphyx.vasp

import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json
import org.aurphyx.vasp.model.VaspObject

object VaspJson {
    val format: Json = Json {
        encodeDefaults = true
        explicitNulls = true
        prettyPrint = true
        prettyPrintIndent = "  "
        ignoreUnknownKeys = true
        coerceInputValues = true
    }

    fun encode(obj: VaspObject): String = format.encodeToString(obj)

    fun decode(json: String): VaspObject = format.decodeFromString(json)
}

fun VaspObject.encodeToString(): String = VaspJson.encode(this)
