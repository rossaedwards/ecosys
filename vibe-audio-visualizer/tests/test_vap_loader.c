/*****************************************************************************
 * test_vap_loader.c — VAP Loader Unit Tests
 * Tests all 3 ingest paths + default fallback + version guard
 *****************************************************************************/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include "../src/vap_loader.h"
#include "../src/vap_runtime.h"

static int passed = 0;
static int failed = 0;

#define TEST(name, cond) do {                                       \
    if (cond) { printf("  ✓ %s\n", name); passed++; }              \
    else      { printf("  ✗ FAIL: %s\n", name); failed++; }        \
} while(0)

/* ── Test 1: Version guard rejects wrong version ────────────────────── */
static void test_version_guard(void) {
    printf("\n[Version Guard]\n");
    vap_runtime_t vap;
    vap_runtime_init(&vap);

    const char *bad_version =
        "{\"VAP_VERSION\":\"2.0\","
        "\"IDENTITY\":{\"TITLE\":\"Test\",\"ARTIST\":\"X\"},"
        "\"PILLARS\":{\"STRUCTURAL\":{\"BPM_RAW\":120},"
        "\"AFFECTIVE\":{\"VALENCE\":0.5,\"AROUSAL\":0.5},"
        "\"PHOTOMETRIC\":{\"PRIMARY_HEX\":\"#7B14C8\"},"
        "\"KINETIC\":{\"MET_SCORE\":3.0}}}";

    int r = vap_loader_parse_json(&vap, bad_version);
    TEST("Rejects VAP_VERSION 2.0", r == VAP_LOAD_ERR_VERSION);
}

/* ── Test 2: Valid v3.1 JSON parses all required fields ─────────────── */
static void test_valid_json(void) {
    printf("\n[Valid JSON Parse — All 9 Pillars]\n");
    vap_runtime_t vap;
    vap_runtime_init(&vap);

    const char *json =
        "{"
        "  \"VAP_VERSION\": \"3.1\","
        "  \"IDENTITY\": {"
        "    \"TITLE\": \"Underneath It All\","
        "    \"ARTIST\": \"No Doubt\","
        "    \"ISRC\": \"USAM00100001\""
        "  },"
        "  \"PILLARS\": {"
        "    \"STRUCTURAL\": {"
        "      \"BPM_RAW\": 78.5,"
        "      \"GROOVE_QUANTIZATION\": \"HUMAN_SWING\","
        "      \"SYNCOPATION_INDEX\": 0.42"
        "    },"
        "    \"TONAL\": {"
        "      \"KEY\": \"Bb\","
        "      \"DISSONANCE_RATING\": 0.08"
        "    },"
        "    \"TIMBRAL\": {"
        "      \"SPECTRAL_CENTROID_HZ\": 1200.0,"
        "      \"SATURATION_INDEX\": 0.15,"
        "      \"FIDELITY\": \"HI-FI\""
        "    },"
        "    \"LINGUISTIC\": {"
        "      \"EXPLICIT_TIER\": \"CLEAN\""
        "    },"
        "    \"AFFECTIVE\": {"
        "      \"VALENCE\": 0.72,"
        "      \"AROUSAL\": 0.45,"
        "      \"DOMINANCE\": 0.6,"
        "      \"RESOLUTION_STATE\": \"TRIUMPHANT\""
        "    },"
        "    \"CONTEXTUAL\": {"
        "      \"MACRO_SETTING\": \"BEDROOM\","
        "      \"TIME_OF_DAY\": \"GOLDEN\","
        "      \"SCENARIO_CONFIDENCE\": 0.88"
        "    },"
        "    \"PHOTOMETRIC\": {"
        "      \"PRIMARY_HEX\": \"#7B14C8\","
        "      \"SECONDARY_HEX\": \"#FFC000\","
        "      \"PALETTE_TEMP\": \"COOL\","
        "      \"BRIGHTNESS_FLOOR\": 0.05,"
        "      \"BRIGHTNESS_CEILING\": 0.95,"
        "      \"STROBE_TRIGGER\": 1.0,"
        "      \"FADE_MODE\": \"SMOOTH\","
        "      \"FOG_DENSITY\": 0.12,"
        "      \"VISUAL_NOISE\": 0"
        "    },"
        "    \"KINETIC\": {"
        "      \"ENTRAINMENT_FACTOR\": 55.0,"
        "      \"MET_SCORE\": 3.0,"
        "      \"TARGET_HR_ZONE\": \"80-100\","
        "      \"HEAD_NOD\": 0.7"
        "    },"
        "    \"GENEALOGICAL\": {"
        "      \"TIMELESSNESS_SCORE\": 0.82,"
        "      \"AUTHENTICITY_RATIO\": 0.91,"
        "      \"SUBCULTURE_ID\": \"RAVER\","
        "      \"CULTURAL_ERA\": \"Y2K\""
        "    }"
        "  }"
        "}";

    int r = vap_loader_parse_json(&vap, json);
    TEST("Returns VAP_LOAD_OK",               r == VAP_LOAD_OK);
    TEST("vap_loaded flag set",               vap.vap_loaded == 1);

    /* Pillar 1 */
    TEST("BPM_RAW parsed",                    fabsf(vap.bpm_raw - 78.5f) < 0.01f);
    TEST("Groove = HUMAN_SWING (0.6)",        fabsf(vap.groove_quantization - 0.6f) < 0.01f);
    TEST("Syncopation parsed",                fabsf(vap.syncopation_index - 0.42f) < 0.01f);

    /* Pillar 2 */
    TEST("Key parsed as Bb",                  strcmp(vap.key, "Bb") == 0);
    TEST("Dissonance parsed",                 fabsf(vap.dissonance_density - 0.08f) < 0.01f);

    /* Pillar 3 */
    TEST("Spectral centroid parsed",
         fabsf(vap.spectral_centroid_hz - 1200.0f) < 1.0f);

    /* Pillar 4 */
    TEST("Explicit tier = CLEAN (0)",         vap.explicit_tier == 0);

    /* Pillar 5 — Thayer coordinates */
    TEST("Valence 0.72 parsed",               fabsf(vap.affective.valence - 0.72f) < 0.01f);
    TEST("Arousal 0.45 parsed",               fabsf(vap.affective.arousal - 0.45f) < 0.01f);
    TEST("Dominance 0.6 parsed",              fabsf(vap.affective.dominance - 0.6f) < 0.01f);
    TEST("Resolution = TRIUMPHANT (0)",       vap.affective.resolution_state == 0);

    /* Pillar 6 */
    TEST("Scenario confidence parsed",
         fabsf(vap.scenario_confidence - 0.88f) < 0.01f);
    TEST("Time of day GOLDEN → fog=0.4",
         fabsf(vap.contextual_fog_mod - 0.4f) < 0.01f);

    /* Pillar 7 — Photometric */
    TEST("PRIMARY_HEX #7B14C8 → R~0.482",
         fabsf(vap.photometric.primary_hex[0] - 0.482f) < 0.01f);
    TEST("PRIMARY_HEX #7B14C8 → G~0.078",
         fabsf(vap.photometric.primary_hex[1] - 0.078f) < 0.01f);
    TEST("PRIMARY_HEX #7B14C8 → B~0.784",
         fabsf(vap.photometric.primary_hex[2] - 0.784f) < 0.01f);
    TEST("SECONDARY_HEX #FFC000 → R=1.0",
         fabsf(vap.photometric.secondary_hex[0] - 1.000f) < 0.01f);
    TEST("Palette temp COOL → 0.1",
         fabsf(vap.photometric.palette_temp - 0.1f) < 0.01f);
    TEST("Brightness floor 0.05",
         fabsf(vap.photometric.brightness_floor - 0.05f) < 0.01f);
    TEST("Brightness ceiling 0.95",
         fabsf(vap.photometric.brightness_ceiling - 0.95f) < 0.01f);
    TEST("Fade mode SMOOTH → 1",             vap.photometric.fade_mode == 1);
    TEST("Fog density 0.12",
         fabsf(vap.photometric.fog_density - 0.12f) < 0.01f);
    TEST("Visual noise = Clean (0)",         vap.photometric.visual_noise_mode == 0);

    /* Pillar 8 — Kinetic */
    TEST("Entrainment 55.0 (Head Nod zone)", fabsf(vap.entrainment_factor - 55.0f) < 0.1f);
    TEST("MET 3.0 (Walk range)",             fabsf(vap.met_score - 3.0f) < 0.01f);
    TEST("HR zone string",                   strcmp(vap.target_hr_zone, "80-100") == 0);

    /* Pillar 9 — Genealogical */
    TEST("Timelessness 0.82",                fabsf(vap.timelessness_score - 0.82f) < 0.01f);
    TEST("Tribe = RAVER",                    strcmp(vap.tribe_id, "RAVER") == 0);
    TEST("Era = Y2K",                        strcmp(vap.cultural_era, "Y2K") == 0);

    /* Identity */
    TEST("Title parsed",                     strcmp(vap.identity_title, "Underneath It All") == 0);
    TEST("Artist parsed",                    strcmp(vap.identity_artist, "No Doubt") == 0);
}

/* ── Test 3: Valence clamping per schema range [-1.0, +1.0] ─────────── */
static void test_clamp_valence(void) {
    printf("\n[Thayer Range Clamping]\n");
    vap_runtime_t vap;
    vap_runtime_init(&vap);

    const char *json =
        "{\"VAP_VERSION\":\"3.1\","
        "\"IDENTITY\":{\"TITLE\":\"T\",\"ARTIST\":\"A\"},"
        "\"PILLARS\":{"
        "\"STRUCTURAL\":{\"BPM_RAW\":120},"
        "\"AFFECTIVE\":{\"VALENCE\":5.0,\"AROUSAL\":-3.0},"
        "\"PHOTOMETRIC\":{\"PRIMARY_HEX\":\"#7B14C8\"},"
        "\"KINETIC\":{\"MET_SCORE\":3.0}}}";

    vap_loader_parse_json(&vap, json);
    TEST("Valence clamped to +1.0",  vap.affective.valence  == 1.0f);
    TEST("Arousal clamped to  0.0",  vap.affective.arousal  == 0.0f);
}

/* ── Test 4: Safe defaults applied when no JSON found ───────────────── */
static void test_defaults(void) {
    printf("\n[Safe Defaults — Backward Compatibility §3.2]\n");
    vap_runtime_t vap;
    vap_runtime_init(&vap);
    vap_loader_apply_defaults(&vap);

    TEST("vap_loaded == 0 (no sidecar)",      vap.vap_loaded == 0);
    TEST("Default BPM = 120",                  fabsf(vap.bpm_raw - 120.0f) < 0.01f);
    TEST("Default Valence = 0.0 (neutral)",    fabsf(vap.affective.valence)  < 0.01f);
    TEST("Default Arousal = 0.5 (medium)",     fabsf(vap.affective.arousal - 0.5f) < 0.01f);
    TEST("Default Primary = Aurphyx Violet",
         fabsf(vap.photometric.primary_hex[0] - 0.482f) < 0.01f);
    TEST("Default Secondary = Bliss Gold",
         fabsf(vap.photometric.secondary_hex[0] - 1.000f) < 0.01f);
    TEST("Default entrainment = 50 (Head Nod)",
         fabsf(vap.entrainment_factor - 50.0f) < 0.1f);
    TEST("Default strobe disabled (1.0)",
         fabsf(vap.photometric.strobe_threshold - 1.0f) < 0.01f);
    TEST("Default fade = SMOOTH (1)",          vap.photometric.fade_mode == 1);
    TEST("Default explicit = CLEAN (0)",       vap.explicit_tier == 0);
}

/* ── Test 5: Missing VAP file path returns defaults ─────────────────── */
static void test_missing_file(void) {
    printf("\n[Missing File → Defaults]\n");
    vap_runtime_t vap;
    vap_runtime_init(&vap);
    int r = vap_loader_load(&vap, "/nonexistent/path/track.flac");
    TEST("Returns VAP_LOAD_DEFAULTS", r == VAP_LOAD_DEFAULTS);
    TEST("BPM set to 120 via defaults", fabsf(vap.bpm_raw - 120.0f) < 0.01f);
}

/* ── Test 6: NULL path returns defaults, no crash ───────────────────── */
static void test_null_path(void) {
    printf("\n[NULL Path Safety]\n");
    vap_runtime_t vap;
    vap_runtime_init(&vap);
    int r = vap_loader_load(&vap, NULL);
    TEST("Returns VAP_LOAD_DEFAULTS for NULL", r == VAP_LOAD_DEFAULTS);
}

/* ── Test 7: vap_loader_result_str covers all codes ─────────────────── */
static void test_result_strings(void) {
    printf("\n[Result Code Strings]\n");
    TEST("VAP_LOAD_OK string",
         strlen(vap_loader_result_str(VAP_LOAD_OK)) > 0);
    TEST("VAP_LOAD_FROM_ID3 string",
         strlen(vap_loader_result_str(VAP_LOAD_FROM_ID3)) > 0);
    TEST("VAP_LOAD_FROM_VORBIS string",
         strlen(vap_loader_result_str(VAP_LOAD_FROM_VORBIS)) > 0);
    TEST("VAP_LOAD_DEFAULTS string",
         strlen(vap_loader_result_str(VAP_LOAD_DEFAULTS)) > 0);
    TEST("VAP_LOAD_ERR_VERSION string",
         strlen(vap_loader_result_str(VAP_LOAD_ERR_VERSION)) > 0);
}

/* ── Runner ──────────────────────────────────────────────────────────── */
int main(void) {
    printf("╔══════════════════════════════════════════════════╗\n");
    printf("║  Vibe Audio Visualizer — VAP Loader Unit Tests   ║\n");
    printf("║  V.A.P. v3.1 / Aurphyx SUXS / rAE               ║\n");
    printf("╚══════════════════════════════════════════════════╝\n");

    test_version_guard();
    test_valid_json();
    test_clamp_valence();
    test_defaults();
    test_missing_file();
    test_null_path();
    test_result_strings();

    printf("\n══════════════════════════════════════════════════\n");
    printf("  Results: %d passed  |  %d failed  |  %d total\n",
           passed, failed, passed + failed);
    printf("══════════════════════════════════════════════════\n");

    return (failed == 0) ? 0 : 1;
}
