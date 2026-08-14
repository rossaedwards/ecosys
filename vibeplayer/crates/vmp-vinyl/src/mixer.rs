//! Mixer — Mixxx `EngineMixer` / channel gain + crossfader analogue.

use serde::{Deserialize, Serialize};

/// Crossfader curve.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum CrossfaderCurve {
    /// Equal-power (constant energy) — default DJ curve
    #[default]
    EqualPower,
    /// Linear amplitude
    Linear,
    /// Constant gain (full both until edges)
    ConstantGain,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineMixer {
    /// 0.0 = full A, 1.0 = full B
    pub crossfader: f32,
    pub gain_a: f32,
    pub gain_b: f32,
    pub filter_a: f32,
    pub filter_b: f32,
    pub headroom: f32,
    pub curve: CrossfaderCurve,
}

impl Default for EngineMixer {
    fn default() -> Self {
        Self {
            crossfader: 0.5,
            gain_a: 1.0,
            gain_b: 1.0,
            filter_a: 0.5,
            filter_b: 0.5,
            headroom: 1.0,
            curve: CrossfaderCurve::EqualPower,
        }
    }
}

impl EngineMixer {
    pub fn set_crossfader(&mut self, x: f32) {
        self.crossfader = x.clamp(0.0, 1.0);
    }

    /// Returns (gain_a, gain_b) after crossfader + channel faders.
    pub fn channel_gains(&self) -> (f32, f32) {
        let x = self.crossfader;
        let (xa, xb) = match self.curve {
            CrossfaderCurve::Linear => (1.0 - x, x),
            CrossfaderCurve::EqualPower => ((1.0 - x).sqrt(), x.sqrt()),
            CrossfaderCurve::ConstantGain => {
                // Full until last 10%
                let a = if x < 0.9 { 1.0 } else { (1.0 - x) / 0.1 };
                let b = if x > 0.1 { 1.0 } else { x / 0.1 };
                (a, b)
            }
        };
        (
            self.gain_a * xa * self.headroom,
            self.gain_b * xb * self.headroom,
        )
    }
}
