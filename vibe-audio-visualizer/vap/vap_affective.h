/* VAP pillar: vap_affective.h */
#ifndef VAP_AFFECTIVE_H
#define VAP_AFFECTIVE_H

/* V.A.P. v3.1 — Pillar 5: AFFECTIVE (The Heart)
   Thayer Coordinate System
   Valence:  -1.0 (Despair) → 0.0 (Neutral) → +1.0 (Euphoria)
   Arousal:   0.0 (Sleep)   →                   1.0 (Rage/Panic)
   Dominance: 0.0 (Vulnerable) →                1.0 (Aggressive)  */

typedef struct {
    float valence;           /* -1.0 to +1.0 */
    float arousal;           /*  0.0 to  1.0 */
    float dominance;         /*  0.0 to  1.0 */
    float mood_stability;    /*  0.0=volatile  1.0=constant */
    float catharsis_potential;
    float nostalgia_trigger;
    /* Tension Arc */
    float buildup_velocity;
    int   resolution_state;  /* 0=Triumphant 1=Melancholic 2=Unresolved */
} vap_affective_t;

/* Maps Valence+Arousal to background atmosphere color blend factor
   Used in vibe.frag as u_vap_valence, u_vap_arousal uniforms         */
static inline float vap_affective_warmth(const vap_affective_t *a) {
    /* Positive valence + high arousal = warm/gold
       Negative valence + high arousal = red/aggressive
       Low arousal = cool/ambient regardless of valence               */
    return (a->valence * 0.5f + 0.5f) * a->arousal;
}

#endif /* VAP_AFFECTIVE_H */
