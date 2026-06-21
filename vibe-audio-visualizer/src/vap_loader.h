/*****************************************************************************
 * vap_loader.h — V.A.P. v3.1 Sidecar & ID3 Loader
 * Aurphyx SUXS / rAE — Vibe Audio Visualizer
 *
 * Responsibilities:
 *   1. Locate .vap.json sidecar adjacent to the playing audio file
 *   2. Parse JSON into vap_runtime_t (zero-dependency micro-parser)
 *   3. Fallback: scan ID3v2 TXXX frame for embedded VAP_OBJECT string
 *   4. Fallback: scan Vorbis COMMENT block for VAP_OBJECT= field
 *   5. If no VAP data exists anywhere → apply safe neutral defaults
 *      (backward compat per V.A.P. spec §3.2)
 *
 * Dependencies: vap_runtime.h only. No external JSON library required.
 *****************************************************************************/

#ifndef VAP_LOADER_H
#define VAP_LOADER_H

#include "vap_runtime.h"

/* ── Return codes ──────────────────────────────────────────────────────── */
#define VAP_LOAD_OK            0   /* Full VAP sidecar loaded successfully  */
#define VAP_LOAD_FROM_ID3      1   /* Loaded from embedded ID3v2 TXXX frame */
#define VAP_LOAD_FROM_VORBIS   2   /* Loaded from Vorbis COMMENT block      */
#define VAP_LOAD_DEFAULTS      3   /* No VAP data found; safe defaults used  */
#define VAP_LOAD_ERR_VERSION  -1   /* VAP_VERSION mismatch (not "3.1")      */
#define VAP_LOAD_ERR_CORRUPT  -2   /* JSON found but failed structural check */

/* ── Public API ────────────────────────────────────────────────────────── */

/**
 * vap_loader_load()
 *
 * Master entry point. Given the full path of the audio file currently
 * playing in VLC, this function:
 *   1. Constructs <audio_path>.vap.json and attempts to parse it
 *   2. On failure, inspects the audio file for ID3v2 TXXX "VAP_OBJECT"
 *   3. On failure, inspects for Vorbis COMMENT "VAP_OBJECT="
 *   4. On all failures, calls vap_loader_apply_defaults() and returns
 *      VAP_LOAD_DEFAULTS (never returns a hard error to the caller)
 *
 * @param  vap        Pointer to an already-initialised vap_runtime_t
 * @param  audio_path Absolute path to the audio file (UTF-8)
 * @return            One of the VAP_LOAD_* codes above
 */
int  vap_loader_load(vap_runtime_t *vap, const char *audio_path);

/**
 * vap_loader_parse_json()
 *
 * Parse a null-terminated JSON string (from any source) directly into
 * a vap_runtime_t.  Exposed publicly so the ID3/Vorbis paths can share it.
 *
 * @return  VAP_LOAD_OK | VAP_LOAD_ERR_VERSION | VAP_LOAD_ERR_CORRUPT
 */
int  vap_loader_parse_json(vap_runtime_t *vap, const char *json_src);

/**
 * vap_loader_apply_defaults()
 *
 * Fills vap with perceptually safe, visually active neutral values.
 * Called automatically when no VAP data is available.
 * Per V.A.P. spec §3.2 backward compatibility guarantee.
 */
void vap_loader_apply_defaults(vap_runtime_t *vap);

/**
 * vap_loader_result_str()
 * Human-readable description of a VAP_LOAD_* code (for VLC msg_Info).
 */
const char *vap_loader_result_str(int result_code);

#endif /* VAP_LOADER_H */
