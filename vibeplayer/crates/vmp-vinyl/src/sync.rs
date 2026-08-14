//! Beat sync — simplified Mixxx SyncControl / EngineSync behaviour.

use crate::engine_buffer::EngineBuffer;

/// Leader/follower sync: match follower rate so effective BPMs align.
pub fn sync_follower_to_leader(leader: &EngineBuffer, follower: &mut EngineBuffer) {
    let Some(lb) = leader.effective_bpm().or(leader.bpm) else {
        return;
    };
    let Some(fb) = follower.bpm else {
        return;
    };
    if fb <= 0.0 || lb <= 0.0 {
        return;
    }
    // rate so follower.bpm * rate ≈ leader effective bpm
    follower.rate = lb / fb;
}

/// Quantize position to nearest beat of deck BPM.
pub fn quantize_to_beat(deck: &mut EngineBuffer) {
    let Some(bpm) = deck.bpm else {
        return;
    };
    if bpm <= 0.0 {
        return;
    }
    let fpb = deck.sample_rate as f64 * 60.0 / bpm;
    if fpb <= 0.0 {
        return;
    }
    let beat = (deck.play_pos / fpb).round();
    deck.play_pos = beat * fpb;
}
