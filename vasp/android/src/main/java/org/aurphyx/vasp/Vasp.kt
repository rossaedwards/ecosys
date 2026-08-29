package org.aurphyx.vasp

/**
 * VASP protocol constants. Active version is 3.69.
 * Nested JSON keys are the protocol; the player flattened profile is a readout.
 */
object Vasp {
    const val VERSION = "3.69"

    val REQUIRED_PILLARS = listOf(
        "STRUCTURAL",
        "TONAL",
        "TIMBRAL",
        "LINGUISTIC",
        "AFFECTIVE",
        "CONTEXTUAL",
        "PHOTOMETRIC",
        "KINETIC",
        "GENEALOGICAL",
    )
}
