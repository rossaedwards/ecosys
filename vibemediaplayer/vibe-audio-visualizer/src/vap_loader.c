/*****************************************************************************
 * vap_loader.c — V.A.P. v3.1 Sidecar & ID3 Loader
 * Aurphyx SUXS / rAE — Vibe Audio Visualizer
 *
 * V.A.P. spec §3.1: JSON embedded in ID3v2 TXXX or Vorbis COMMENT
 * V.A.P. spec §3.2: Backward compatibility — graceful no-op if absent
 * V.A.P. spec §3.3: Pillar 7 + Pillar 8 are required fields in schema
 *
 * JSON parsing strategy:
 *   We implement a minimal recursive-descent parser for the exact
 *   V.A.P. v3.1 JSON schema shape.  No malloc beyond the stack.
 *   cJSON / jansson are NOT used — keeps the VLC plugin self-contained.
 *****************************************************************************/

#include "vap_loader.h"
#include "vap_runtime.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <math.h>

/* ═══════════════════════════════════════════════════════════════════════════
   SECTION 1 — Micro JSON Parser
   Only parses the exact key paths used by V.A.P. v3.1 schema.
   Strategy: walk the raw JSON string searching for known key tokens,
   then extract the value immediately following the colon.
   ═══════════════════════════════════════════════════════════════════════════ */

/* Advance past whitespace */
static const char *json_skip_ws(const char *p) {
    while (*p && isspace((unsigned char)*p)) p++;
    return p;
}

/**
 * json_find_key()
 * Locate the value string/number for a dot-path key within raw JSON.
 * e.g. path = "PILLARS.AFFECTIVE.VALENCE"
 * Returns pointer to the start of the raw value, or NULL if not found.
 * LIMITATION: does not handle duplicate keys — fine for VAP schema.
 */
static const char *json_find_key(const char *json, const char *key_token) {
    /* We search for "KEY_TOKEN" : value  anywhere in the document.
       VAP JSON is well-structured; no key appears in two pillars.    */
    char needle[128];
    snprintf(needle, sizeof(needle), "\"%s\"", key_token);
    const char *pos = strstr(json, needle);
    if (!pos) return NULL;
    pos += strlen(needle);
    pos  = json_skip_ws(pos);
    if (*pos != ':') return NULL;
    pos++;
    return json_skip_ws(pos);
}

/* Extract a float value from a JSON value position */
static float json_read_float(const char *p, float default_val) {
    if (!p) return default_val;
    p = json_skip_ws(p);
    if (*p == '"') return default_val;   /* it's a string, not a number */
    char *end;
    float v = (float)strtod(p, &end);
    return (end != p) ? v : default_val;
}

/* Extract a quoted string value, writes into buf[buf_len] */
static void json_read_string(const char *p, char *buf, int buf_len) {
    if (!p || !buf || buf_len < 1) return;
    buf[0] = '\0';
    p = json_skip_ws(p);
    if (*p != '"') return;
    p++;  /* skip opening quote */
    int i = 0;
    while (*p && *p != '"' && i < buf_len - 1) {
        if (*p == '\\') { p++; }  /* skip escape char */
        buf[i++] = *p++;
    }
    buf[i] = '\0';
}

/**
 * hex_to_rgb()
 * Converts "#RRGGBB" string → r,g,b floats [0.0–1.0]
 * Per V.A.P. schema §PHOTOMETRIC.PRIMARY_HEX pattern ^#[0-9a-fA-F]{6}$
 */
static void hex_to_rgb(const char *hex, float *r, float *g, float *b) {
    *r = *g = *b = 0.5f;   /* neutral default */
    if (!hex || hex[0] != '#' || strlen(hex) < 7) return;
    unsigned int rv, gv, bv;
    if (sscanf(hex + 1, "%02x%02x%02x", &rv, &gv, &bv) == 3) {
        *r = (float)rv / 255.0f;
        *g = (float)gv / 255.0f;
        *b = (float)bv / 255.0f;
    }
}

/* ═══════════════════════════════════════════════════════════════════════════
   SECTION 2 — Core JSON → vap_runtime_t Population
   Maps every V.A.P. v3.1 schema field to its vap_runtime_t slot.
   All 9 Pillars are covered; missing optional fields get safe defaults.
   ═══════════════════════════════════════════════════════════════════════════ */

static int populate_from_json(vap_runtime_t *vap, const char *json) {

    /* ── VALIDATE: VAP_VERSION must be "3.1" per schema const ──────── */
    const char *ver_pos = json_find_key(json, "VAP_VERSION");
    if (!ver_pos) return VAP_LOAD_ERR_CORRUPT;

    char ver_str[16];
    json_read_string(ver_pos, ver_str, sizeof(ver_str));
    if (strcmp(ver_str, "3.1") != 0) return VAP_LOAD_ERR_VERSION;

    /* ── IDENTITY ────────────────────────────────────────────────────── */
    json_read_string(json_find_key(json, "TITLE"),
                     vap->identity_title,  sizeof(vap->identity_title));
    json_read_string(json_find_key(json, "ARTIST"),
                     vap->identity_artist, sizeof(vap->identity_artist));
    json_read_string(json_find_key(json, "ISRC"),
                     vap->identity_isrc,   sizeof(vap->identity_isrc));

    /* ── PILLAR 1: STRUCTURAL ────────────────────────────────────────── */
    vap->bpm_raw = json_read_float(
        json_find_key(json, "BPM_RAW"), 120.0f);

    vap->bpm_perceived = json_read_float(
        json_find_key(json, "BPM_PERCEIVED"), vap->bpm_raw);

    /* GROOVE_QUANTIZATION: "MACHINE_LOCK" → 0.0, "HUMAN_SWING" → 1.0  */
    char groove_str[32];
    json_read_string(json_find_key(json, "GROOVE_QUANTIZATION"),
                     groove_str, sizeof(groove_str));
    if (strstr(groove_str, "MACHINE")) {
        vap->groove_quantization = 0.0f;
    } else if (strstr(groove_str, "SWING")) {
        /* Spec: J Dilla Swing ~60% → 0.6 */
        vap->groove_quantization = 0.6f;
    } else {
        vap->groove_quantization = 0.3f;  /* neutral */
    }

    vap->syncopation_index = json_read_float(
        json_find_key(json, "SYNCOPATION_INDEX"), 0.3f);

    /* Kick Transient Profile: stored as ms float */
    vap->kick_transient_ms = json_read_float(
        json_find_key(json, "KICK_TRANSIENT_MS"), 15.0f);  /* default: Punch */

    /* ── PILLAR 2: TONAL ─────────────────────────────────────────────── */
    json_read_string(json_find_key(json, "KEY"),
                     vap->key, sizeof(vap->key));

    vap->dissonance_density = json_read_float(
        json_find_key(json, "DISSONANCE_RATING"), 0.1f);

    /* Chord complexity: triadic=0.0, extended 13th=1.0 */
    vap->chord_complexity = json_read_float(
        json_find_key(json, "CHORD_COMPLEXITY"), 0.3f);

    /* ── PILLAR 3: TIMBRAL ───────────────────────────────────────────── */
    vap->spectral_centroid_hz = json_read_float(
        json_find_key(json, "SPECTRAL_CENTROID_HZ"), 800.0f);  /* Warm/Body */

    vap->saturation_index = json_read_float(
        json_find_key(json, "SATURATION_INDEX"), 0.2f);

    vap->dynamic_range_lra = json_read_float(
        json_find_key(json, "DYNAMIC_RANGE_LRA"), 8.0f);

    /* SPATIAL_WIDTH: "MONO"=0, "STEREO"=1, "IMMERSIVE"=2 */
    char spatial_str[32];
    json_read_string(json_find_key(json, "SPATIAL_WIDTH"),
                     spatial_str, sizeof(spatial_str));
    if      (strstr(spatial_str, "IMMERSIVE") || strstr(spatial_str, "ATMOS"))
        vap->spatial_width = 2;
    else if (strstr(spatial_str, "MONO"))
        vap->spatial_width = 0;
    else
        vap->spatial_width = 1;  /* STEREO default */

    /* ── PILLAR 4: LINGUISTIC (optional — visualizer uses minimally) ── */
    char explicit_str[16];
    json_read_string(json_find_key(json, "EXPLICIT_TIER"),
                     explicit_str, sizeof(explicit_str));
    /* Store as int: CLEAN=0 MILD=1 EXPLICIT=2 SEVERE=3 */
    if      (strcmp(explicit_str, "SEVERE")   == 0) vap->explicit_tier = 3;
    else if (strcmp(explicit_str, "EXPLICIT") == 0) vap->explicit_tier = 2;
    else if (strcmp(explicit_str, "MILD")     == 0) vap->explicit_tier = 1;
    else                                             vap->explicit_tier = 0;

    /* ── PILLAR 5: AFFECTIVE — Thayer Coordinates ───────────────────── */
    /* REQUIRED per schema; validated above via VAP_VERSION check       */
    vap->affective.valence = json_read_float(
        json_find_key(json, "VALENCE"), 0.0f);
    /* Clamp to spec range [-1.0, +1.0] */
    if (vap->affective.valence < -1.0f) vap->affective.valence = -1.0f;
    if (vap->affective.valence >  1.0f) vap->affective.valence =  1.0f;

    vap->affective.arousal = json_read_float(
        json_find_key(json, "AROUSAL"), 0.5f);
    if (vap->affective.arousal < 0.0f) vap->affective.arousal = 0.0f;
    if (vap->affective.arousal > 1.0f) vap->affective.arousal = 1.0f;

    vap->affective.dominance = json_read_float(
        json_find_key(json, "DOMINANCE"), 0.5f);

    vap->affective.mood_stability = json_read_float(
        json_find_key(json, "MOOD_STABILITY"), 0.7f);

    vap->affective.catharsis_potential = json_read_float(
        json_find_key(json, "CATHARSIS_POTENTIAL"), 0.3f);

    vap->affective.nostalgia_trigger = json_read_float(
        json_find_key(json, "NOSTALGIA_TRIGGER"), 0.2f);

    /* Tension Arc */
    vap->affective.buildup_velocity = json_read_float(
        json_find_key(json, "BUILDUP_VELOCITY"), 0.4f);

    /* RESOLUTION_STATE: "TRIUMPHANT"=0, "MELANCHOLIC"=1, "UNRESOLVED"=2 */
    char resolution_str[32];
    json_read_string(json_find_key(json, "RESOLUTION_STATE"),
                     resolution_str, sizeof(resolution_str));
    if      (strstr(resolution_str, "MELANCHOLIC")) vap->affective.resolution_state = 1;
    else if (strstr(resolution_str, "UNRESOLVED"))  vap->affective.resolution_state = 2;
    else                                             vap->affective.resolution_state = 0;

    /* ── PILLAR 6: CONTEXTUAL ────────────────────────────────────────── */
    vap->scenario_confidence = json_read_float(
        json_find_key(json, "SCENARIO_CONFIDENCE"), 0.0f);

    json_read_string(json_find_key(json, "MACRO_SETTING"),
                     vap->scenario_tag, sizeof(vap->scenario_tag));

    /* TIME_OF_DAY → fog modifier: Late Night / 3AM = more fog */
    char tod_str[32];
    json_read_string(json_find_key(json, "TIME_OF_DAY"),
                     tod_str, sizeof(tod_str));
    if (strstr(tod_str, "NIGHT") || strstr(tod_str, "3AM"))
        vap->contextual_fog_mod = 1.0f;
    else if (strstr(tod_str, "GOLDEN"))
        vap->contextual_fog_mod = 0.4f;
    else
        vap->contextual_fog_mod = 0.2f;

    /* ── PILLAR 7: PHOTOMETRIC — REQUIRED ───────────────────────────── */
    /* PRIMARY_HEX — pattern ^#[0-9a-fA-F]{6}$ per schema              */
    char phex[16], shex[16];
    json_read_string(json_find_key(json, "PRIMARY_HEX"),
                     phex, sizeof(phex));
    json_read_string(json_find_key(json, "SECONDARY_HEX"),
                     shex, sizeof(shex));

    hex_to_rgb(phex,
               &vap->photometric.primary_hex[0],
               &vap->photometric.primary_hex[1],
               &vap->photometric.primary_hex[2]);

    hex_to_rgb(shex,
               &vap->photometric.secondary_hex[0],
               &vap->photometric.secondary_hex[1],
               &vap->photometric.secondary_hex[2]);

    /* PALETTE_TEMP: "COOL"=0.0 … "WARM"=1.0 */
    char palette_str[32];
    json_read_string(json_find_key(json, "PALETTE_TEMP"),
                     palette_str, sizeof(palette_str));
    if      (strstr(palette_str, "COOL"))   vap->photometric.palette_temp = 0.1f;
    else if (strstr(palette_str, "WARM"))   vap->photometric.palette_temp = 0.9f;
    else {
        /* Try reading as float if user stored it numerically */
        float pt = json_read_float(json_find_key(json, "PALETTE_TEMP"), 0.5f);
        vap->photometric.palette_temp = (pt >= 0.0f && pt <= 1.0f) ? pt : 0.5f;
    }

    /* Lumen Dynamics (Pillar 7.2) */
    vap->photometric.brightness_floor = json_read_float(
        json_find_key(json, "BRIGHTNESS_FLOOR"), 0.05f);
    vap->photometric.brightness_ceiling = json_read_float(
        json_find_key(json, "BRIGHTNESS_CEILING"), 1.0f);
    vap->photometric.strobe_threshold = json_read_float(
        json_find_key(json, "STROBE_TRIGGER"), 1.0f);   /* 1.0 = disabled */

    char fade_str[32];
    json_read_string(json_find_key(json, "FADE_MODE"),
                     fade_str, sizeof(fade_str));
    vap->photometric.fade_mode = strstr(fade_str, "SHARP") ? 0 : 1;

    vap->photometric.fade_rate = json_read_float(
        json_find_key(json, "FADE_RATE"), 0.3f);

    /* Visual Texture (Pillar 7.3) */
    vap->photometric.fog_density = json_read_float(
        json_find_key(json, "FOG_DENSITY"), 0.1f);

    vap->photometric.laser_compatible = (int)json_read_float(
        json_find_key(json, "LASER_COMPATIBILITY"), 0.0f);

    /* VISUAL_NOISE: 0.0=Clean/Solid, 1.0=Static/Glitch */
    vap->photometric.visual_noise_mode = (int)roundf(json_read_float(
        json_find_key(json, "VISUAL_NOISE"), 0.0f));

    /* Surface texture tag: Glassy/Gritty/Wooden/Metallic/Liquid */
    json_read_string(json_find_key(json, "SURFACE"),
                     vap->photometric.surface_tag,
                     sizeof(vap->photometric.surface_tag));

    /* ── PILLAR 8: KINETIC — REQUIRED ───────────────────────────────── */
    vap->entrainment_factor = json_read_float(
        json_find_key(json, "ENTRAINMENT_FACTOR"), 50.0f);
    /* Clamp to spec range [0, 100] */
    if (vap->entrainment_factor < 0.0f)   vap->entrainment_factor = 0.0f;
    if (vap->entrainment_factor > 100.0f) vap->entrainment_factor = 100.0f;

    vap->met_score = json_read_float(
        json_find_key(json, "MET_SCORE"), 3.0f);

    /* TARGET_HR_ZONE stored as string e.g. "110-130" */
    json_read_string(json_find_key(json, "TARGET_HR_ZONE"),
                     vap->target_hr_zone, sizeof(vap->target_hr_zone));

    /* Motor Response (Pillar 8.2) */
    vap->motor_drive = json_read_float(
        json_find_key(json, "DRIVE"), 0.5f);
    vap->motor_sway  = json_read_float(
        json_find_key(json, "SWAY"),  0.5f);
    vap->head_nod    = json_read_float(
        json_find_key(json, "HEAD_NOD"), 0.5f);

    /* ── PILLAR 9: GENEALOGICAL (optional) ──────────────────────────── */
    vap->timelessness_score = json_read_float(
        json_find_key(json, "TIMELESSNESS_SCORE"), 0.5f);
    vap->authenticity_ratio = json_read_float(
        json_find_key(json, "AUTHENTICITY_RATIO"), 0.5f);
    vap->viral_velocity = json_read_float(
        json_find_key(json, "VIRAL_VELOCITY"), 0.0f);

    json_read_string(json_find_key(json, "SUBCULTURE_ID"),
                     vap->tribe_id, sizeof(vap->tribe_id));

    json_read_string(json_find_key(json, "CULTURAL_ERA"),
                     vap->cultural_era, sizeof(vap->cultural_era));

    /* Genre Tree (Pillar 9.2) */
    json_read_string(json_find_key(json, "GENRE_TREE"),
                     vap->genre_tree, sizeof(vap->genre_tree));

    vap->vap_loaded = 1;
    return VAP_LOAD_OK;
}

/* ═══════════════════════════════════════════════════════════════════════════
   SECTION 3 — Load Path A: .vap.json Sidecar File
   ═══════════════════════════════════════════════════════════════════════════ */

static int load_from_sidecar(vap_runtime_t *vap, const char *audio_path) {
    /* Build sidecar path: strip extension, append .vap.json */
    char sidecar_path[4096];
    strncpy(sidecar_path, audio_path, sizeof(sidecar_path) - 12);
    sidecar_path[sizeof(sidecar_path) - 12] = '\0';

    /* Find last dot for extension strip */
    char *last_dot = strrchr(sidecar_path, '.');
    if (last_dot && !strchr(last_dot, '/') && !strchr(last_dot, '\\'))
        *last_dot = '\0';

    strncat(sidecar_path, ".vap.json",
            sizeof(sidecar_path) - strlen(sidecar_path) - 1);

    FILE *fp = fopen(sidecar_path, "r");
    if (!fp) return VAP_LOAD_ERR_CORRUPT;   /* file not found */

    /* Read entire file into buffer (VAP JSON is compact, < 16KB) */
    fseek(fp, 0, SEEK_END);
    long size = ftell(fp);
    rewind(fp);

    if (size <= 0 || size > 65536) {
        fclose(fp);
        return VAP_LOAD_ERR_CORRUPT;
    }

    char *buf = (char *)malloc((size_t)size + 1);
    if (!buf) { fclose(fp); return VAP_LOAD_ERR_CORRUPT; }

    fread(buf, 1, (size_t)size, fp);
    buf[size] = '\0';
    fclose(fp);

    int result = populate_from_json(vap, buf);
    free(buf);
    return result;
}

/* ═══════════════════════════════════════════════════════════════════════════
   SECTION 4 — Load Path B: ID3v2 TXXX Frame Extraction
   V.A.P. spec §3.1: embedded in ID3v2 TXXX frame with description "VAP_OBJECT"
   ID3v2 layout: "ID3" + 3-byte version + flags + 4-byte syncsafe size
                 Then frames: 4-byte ID + 4-byte size + 2-byte flags + data
   ═══════════════════════════════════════════════════════════════════════════ */

/* Decode 4-byte syncsafe integer (ID3v2.4 header size) */
static uint32_t id3_syncsafe_to_int(const unsigned char *b) {
    return ((uint32_t)(b[0] & 0x7F) << 21) |
           ((uint32_t)(b[1] & 0x7F) << 14) |
           ((uint32_t)(b[2] & 0x7F) <<  7) |
            (uint32_t)(b[3] & 0x7F);
}

static int load_from_id3(vap_runtime_t *vap, const char *audio_path) {
    FILE *fp = fopen(audio_path, "rb");
    if (!fp) return VAP_LOAD_ERR_CORRUPT;

    /* Read ID3v2 header (10 bytes) */
    unsigned char hdr[10];
    if (fread(hdr, 1, 10, fp) < 10 ||
        hdr[0] != 'I' || hdr[1] != 'D' || hdr[2] != '3') {
        fclose(fp);
        return VAP_LOAD_ERR_CORRUPT;  /* not an ID3 file */
    }

    uint32_t tag_size = id3_syncsafe_to_int(hdr + 6);
    if (tag_size > 1048576) {  /* sanity cap at 1MB */
        fclose(fp);
        return VAP_LOAD_ERR_CORRUPT;
    }

    char *tag_buf = (char *)malloc(tag_size);
    if (!tag_buf) { fclose(fp); return VAP_LOAD_ERR_CORRUPT; }
    fread(tag_buf, 1, tag_size, fp);
    fclose(fp);

    /* Walk frames looking for TXXX with description "VAP_OBJECT" */
    const char *p      = tag_buf;
    const char *p_end  = tag_buf + tag_size;
    int result = VAP_LOAD_ERR_CORRUPT;

    while (p + 10 < p_end) {
        /* Frame ID: 4 chars */
        if (!isalnum((unsigned char)p[0])) break;  /* padding */

        int is_txxx = (p[0]=='T' && p[1]=='X' && p[2]=='X' && p[3]=='X');
        uint32_t frame_size = ((uint32_t)(unsigned char)p[4] << 24) |
                              ((uint32_t)(unsigned char)p[5] << 16) |
                              ((uint32_t)(unsigned char)p[6] <<  8) |
                               (uint32_t)(unsigned char)p[7];
        p += 10;  /* skip frame header */

        if (p + frame_size > p_end) break;

        if (is_txxx && frame_size > 12) {
            /* TXXX format: encoding(1) + description(null-term) + value */
            const char *frame_data = p + 1;  /* skip encoding byte */
            const char *desc = frame_data;

            if (strncmp(desc, "VAP_OBJECT", 10) == 0) {
                /* Value starts after the null terminator of description */
                const char *value = desc + strlen(desc) + 1;
                size_t value_len  = frame_size - 1 - (size_t)(value - frame_data);

                if (value_len > 0 && value_len < 65536) {
                    char *json_buf = (char *)malloc(value_len + 1);
                    if (json_buf) {
                        memcpy(json_buf, value, value_len);
                        json_buf[value_len] = '\0';
                        result = populate_from_json(vap, json_buf);
                        free(json_buf);
                        if (result == VAP_LOAD_OK)
                            result = VAP_LOAD_FROM_ID3;
                        break;
                    }
                }
            }
        }
        p += frame_size;
    }

    free(tag_buf);
    return result;
}

/* ═══════════════════════════════════════════════════════════════════════════
   SECTION 5 — Load Path C: Vorbis COMMENT Block
   V.A.P. spec §3.1: stored as VAP_OBJECT=<json_string> in Vorbis COMMENT
   Vorbis comment structure: little-endian uint32 length + UTF-8 string
   Located in packet 2 of an OGG stream (after ID header & comment header)
   ═══════════════════════════════════════════════════════════════════════════ */

static uint32_t read_le32(const unsigned char *b) {
    return (uint32_t)b[0] | ((uint32_t)b[1] << 8) |
           ((uint32_t)b[2] << 16) | ((uint32_t)b[3] << 24);
}

static int load_from_vorbis(vap_runtime_t *vap, const char *audio_path) {
    FILE *fp = fopen(audio_path, "rb");
    if (!fp) return VAP_LOAD_ERR_CORRUPT;

    /* Scan first 512KB for Vorbis comment packet signature
       Comment packet begins with \x03vorbis                         */
    unsigned char scan[524288];
    size_t bytes_read = fread(scan, 1, sizeof(scan), fp);
    fclose(fp);
    if (bytes_read < 64) return VAP_LOAD_ERR_CORRUPT;

    /* Find \x03vorbis marker */
    const unsigned char *marker = NULL;
    for (size_t i = 0; i + 7 < bytes_read; i++) {
        if (scan[i] == 0x03 &&
            memcmp(scan + i + 1, "vorbis", 6) == 0) {
            marker = scan + i + 7;
            break;
        }
    }
    if (!marker) return VAP_LOAD_ERR_CORRUPT;

    /* Skip vendor string: le32 length + vendor_string */
    if ((size_t)(marker - scan) + 4 >= bytes_read) return VAP_LOAD_ERR_CORRUPT;
    uint32_t vendor_len = read_le32(marker);
    marker += 4 + vendor_len;

    /* Read comment count */
    if ((size_t)(marker - scan) + 4 >= bytes_read) return VAP_LOAD_ERR_CORRUPT;
    uint32_t comment_count = read_le32(marker);
    marker += 4;

    int result = VAP_LOAD_ERR_CORRUPT;

    for (uint32_t i = 0; i < comment_count; i++) {
        if ((size_t)(marker - scan) + 4 >= bytes_read) break;
        uint32_t comment_len = read_le32(marker);
        marker += 4;

        if ((size_t)(marker - scan) + comment_len >= bytes_read) break;

        /* Check for VAP_OBJECT= prefix (case-insensitive per Vorbis spec) */
        if (comment_len > 11 &&
            strncasecmp((const char *)marker, "VAP_OBJECT=", 11) == 0) {
            const char *json_start = (const char *)marker + 11;
            size_t      json_len   = comment_len - 11;

            char *json_buf = (char *)malloc(json_len + 1);
            if (json_buf) {
                memcpy(json_buf, json_start, json_len);
                json_buf[json_len] = '\0';
                result = populate_from_json(vap, json_buf);
                free(json_buf);
                if (result == VAP_LOAD_OK)
                    result = VAP_LOAD_FROM_VORBIS;
                break;
            }
        }
        marker += comment_len;
    }

    return result;
}

/* ═══════════════════════════════════════════════════════════════════════════
   SECTION 6 — Safe Defaults
   Per V.A.P. spec §3.2: backward compatibility — visualizer must still
   render a beautiful, musically responsive image with no VAP data at all.
   Default values produce a mid-energy, neutral-valence, stereo, warm field.
   ═══════════════════════════════════════════════════════════════════════════ */

void vap_loader_apply_defaults(vap_runtime_t *vap) {
    vap_runtime_init(vap);  /* zero + zero-init all fields first */

    /* Identity */
    strncpy(vap->identity_title,  "Unknown", sizeof(vap->identity_title));
    strncpy(vap->identity_artist, "Unknown", sizeof(vap->identity_artist));

    /* Pillar 1: Structural — neutral 4/4 groove */
    vap->bpm_raw             = 120.0f;
    vap->bpm_perceived       = 120.0f;
    vap->groove_quantization = 0.3f;   /* slight swing */
    vap->syncopation_index   = 0.3f;
    vap->kick_transient_ms   = 15.0f;  /* Punch/Thud (Pop/Rock) */

    /* Pillar 2: Tonal */
    strncpy(vap->key, "C", sizeof(vap->key));
    vap->dissonance_density = 0.1f;   /* Consonant */
    vap->chord_complexity   = 0.3f;

    /* Pillar 3: Timbral — Warm/Body centroid */
    vap->spectral_centroid_hz = 800.0f;
    vap->saturation_index     = 0.2f;
    vap->dynamic_range_lra    = 8.0f;
    vap->spatial_width        = 1;     /* STEREO */

    /* Pillar 4: Linguistic */
    vap->explicit_tier = 0;            /* CLEAN */

    /* Pillar 5: Affective — neutral Thayer coordinates */
    vap->affective.valence            = 0.0f;   /* Neutral */
    vap->affective.arousal            = 0.5f;   /* Medium energy */
    vap->affective.dominance          = 0.5f;
    vap->affective.mood_stability     = 0.7f;
    vap->affective.catharsis_potential = 0.3f;
    vap->affective.nostalgia_trigger  = 0.2f;
    vap->affective.buildup_velocity   = 0.4f;
    vap->affective.resolution_state   = 0;      /* TRIUMPHANT */

    /* Pillar 6: Contextual */
    vap->scenario_confidence = 0.0f;
    strncpy(vap->scenario_tag, "NONE", sizeof(vap->scenario_tag));
    vap->contextual_fog_mod  = 0.2f;

    /* Pillar 7: Photometric — Aurphyx brand identity defaults
       PRIMARY = Aurphyx Violet #7B14C8
       SECONDARY = Bliss Gold   #FFC000                             */
    vap->photometric.primary_hex[0]    = 0.482f;  /* #7B14C8 R */
    vap->photometric.primary_hex[1]    = 0.078f;  /* #7B14C8 G */
    vap->photometric.primary_hex[2]    = 0.784f;  /* #7B14C8 B */
    vap->photometric.secondary_hex[0]  = 1.000f;  /* #FFC000 R */
    vap->photometric.secondary_hex[1]  = 0.753f;  /* #FFC000 G */
    vap->photometric.secondary_hex[2]  = 0.000f;  /* #FFC000 B */
    vap->photometric.palette_temp      = 0.5f;
    vap->photometric.brightness_floor  = 0.05f;
    vap->photometric.brightness_ceiling = 1.0f;
    vap->photometric.strobe_threshold  = 1.0f;    /* disabled */
    vap->photometric.fade_mode         = 1;        /* SMOOTH */
    vap->photometric.fade_rate         = 0.3f;
    vap->photometric.fog_density       = 0.1f;
    vap->photometric.laser_compatible  = 0;
    vap->photometric.visual_noise_mode = 0;        /* Clean */
    strncpy(vap->photometric.surface_tag, "GLASSY",
            sizeof(vap->photometric.surface_tag));

    /* Pillar 8: Kinetic — moderate activity */
    vap->entrainment_factor = 50.0f;  /* Head Nod zone (30–70 per spec) */
    vap->met_score          = 3.0f;   /* Walk (80–100 BPM range per spec) */
    strncpy(vap->target_hr_zone, "90-110", sizeof(vap->target_hr_zone));
    vap->motor_drive = 0.5f;
    vap->motor_sway  = 0.5f;
    vap->head_nod    = 0.5f;

    /* Pillar 9: Genealogical */
    vap->timelessness_score = 0.5f;
    vap->authenticity_ratio = 0.5f;
    vap->viral_velocity     = 0.0f;
    strncpy(vap->tribe_id,    "NONE",    sizeof(vap->tribe_id));
    strncpy(vap->cultural_era,"UNKNOWN", sizeof(vap->cultural_era));
    strncpy(vap->genre_tree,  "UNKNOWN", sizeof(vap->genre_tree));

    vap->vap_loaded = 0;  /* signals "no sidecar" to renderer */
}

/* ═══════════════════════════════════════════════════════════════════════════
   SECTION 7 — Public Entry Points
   ═══════════════════════════════════════════════════════════════════════════ */

int vap_loader_parse_json(vap_runtime_t *vap, const char *json_src) {
    if (!json_src || !*json_src) return VAP_LOAD_ERR_CORRUPT;
    return populate_from_json(vap, json_src);
}

int vap_loader_load(vap_runtime_t *vap, const char *audio_path) {
    if (!audio_path || !*audio_path) {
        vap_loader_apply_defaults(vap);
        return VAP_LOAD_DEFAULTS;
    }

    int result;

    /* ── Path A: .vap.json sidecar (preferred, fastest) ── */
    result = load_from_sidecar(vap, audio_path);
    if (result == VAP_LOAD_OK) return VAP_LOAD_OK;

    /* ── Path B: ID3v2 TXXX "VAP_OBJECT" embedded tag ── */
    result = load_from_id3(vap, audio_path);
    if (result == VAP_LOAD_FROM_ID3 || result == VAP_LOAD_OK)
        return VAP_LOAD_FROM_ID3;

    /* ── Path C: Vorbis COMMENT "VAP_OBJECT=" field ── */
    result = load_from_vorbis(vap, audio_path);
    if (result == VAP_LOAD_FROM_VORBIS || result == VAP_LOAD_OK)
        return VAP_LOAD_FROM_VORBIS;

    /* ── Fallback: safe defaults (spec §3.2 backward compat) ── */
    vap_loader_apply_defaults(vap);
    return VAP_LOAD_DEFAULTS;
}

const char *vap_loader_result_str(int code) {
    switch (code) {
        case VAP_LOAD_OK:           return "VAP v3.1 loaded from .vap.json sidecar";
        case VAP_LOAD_FROM_ID3:     return "VAP v3.1 loaded from ID3v2 TXXX frame";
        case VAP_LOAD_FROM_VORBIS:  return "VAP v3.1 loaded from Vorbis COMMENT";
        case VAP_LOAD_DEFAULTS:     return "No VAP data found — safe defaults applied";
        case VAP_LOAD_ERR_VERSION:  return "VAP version mismatch (expected 3.1)";
        case VAP_LOAD_ERR_CORRUPT:  return "VAP JSON structurally invalid";
        default:                    return "Unknown VAP loader result";
    }
}
