//! Multi-mode equalizer: graphic 10/31, parametric nodes, bypass.

use serde::{Deserialize, Serialize};

/// Equalizer operating mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum EqMode {
    #[default]
    Graphic10,
    Graphic31,
    Parametric,
    VapGuided,
    ContextLinked,
    Bypass,
}

/// Direct-form I biquad (RBJ cookbook).
#[derive(Debug, Clone, Copy)]
pub struct Biquad {
    pub b0: f32,
    pub b1: f32,
    pub b2: f32,
    pub a1: f32,
    pub a2: f32,
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
}

impl Default for Biquad {
    fn default() -> Self {
        Self::passthrough()
    }
}

impl Biquad {
    pub fn passthrough() -> Self {
        Self {
            b0: 1.0,
            b1: 0.0,
            b2: 0.0,
            a1: 0.0,
            a2: 0.0,
            x1: 0.0,
            x2: 0.0,
            y1: 0.0,
            y2: 0.0,
        }
    }

    /// Peaking EQ: `gain_db` at `freq_hz`, Q factor, sample rate.
    pub fn peaking(freq_hz: f32, sample_rate: f32, q: f32, gain_db: f32) -> Self {
        let a = 10_f32.powf(gain_db / 40.0);
        let w0 = 2.0 * std::f32::consts::PI * freq_hz / sample_rate;
        let cos_w0 = w0.cos();
        let sin_w0 = w0.sin();
        let alpha = sin_w0 / (2.0 * q.max(0.05));

        let b0 = 1.0 + alpha * a;
        let b1 = -2.0 * cos_w0;
        let b2 = 1.0 - alpha * a;
        let a0 = 1.0 + alpha / a;
        let a1 = -2.0 * cos_w0;
        let a2 = 1.0 - alpha / a;

        Self {
            b0: b0 / a0,
            b1: b1 / a0,
            b2: b2 / a0,
            a1: a1 / a0,
            a2: a2 / a0,
            x1: 0.0,
            x2: 0.0,
            y1: 0.0,
            y2: 0.0,
        }
    }

    pub fn process(&mut self, x: f32) -> f32 {
        let y = self.b0 * x + self.b1 * self.x1 + self.b2 * self.x2
            - self.a1 * self.y1
            - self.a2 * self.y2;
        self.x2 = self.x1;
        self.x1 = x;
        self.y2 = self.y1;
        self.y1 = y;
        y
    }

    pub fn reset(&mut self) {
        self.x1 = 0.0;
        self.x2 = 0.0;
        self.y1 = 0.0;
        self.y2 = 0.0;
    }
}

/// Parametric node types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParametricKind {
    Peaking,
    LowShelf,
    HighShelf,
    HighPass,
    LowPass,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParametricNode {
    pub kind: ParametricKind,
    pub freq_hz: f32,
    pub gain_db: f32,
    pub q: f32,
    pub enabled: bool,
}

impl Default for ParametricNode {
    fn default() -> Self {
        Self {
            kind: ParametricKind::Peaking,
            freq_hz: 1000.0,
            gain_db: 0.0,
            q: 1.0,
            enabled: true,
        }
    }
}

/// Multi-band graphic / parametric equalizer state.
#[derive(Debug, Clone)]
pub struct GraphicEq {
    pub mode: EqMode,
    pub sample_rate: f32,
    pub gains_db: Vec<f32>,
    pub centers_hz: Vec<f32>,
    pub parametric: Vec<ParametricNode>,
    pub linear_phase: bool,
    filters: Vec<Biquad>,
    bypass: bool,
}

impl GraphicEq {
    pub fn new_10(sample_rate: f32) -> Self {
        let centers = crate::GRAPHIC_10_BANDS_HZ.to_vec();
        let n = centers.len();
        let mut eq = Self {
            mode: EqMode::Graphic10,
            sample_rate,
            gains_db: vec![0.0; n],
            centers_hz: centers,
            parametric: Vec::new(),
            linear_phase: false,
            filters: vec![Biquad::passthrough(); n],
            bypass: false,
        };
        eq.rebuild();
        eq
    }

    pub fn new_31(sample_rate: f32) -> Self {
        let centers = crate::GRAPHIC_31_BANDS_HZ.to_vec();
        let n = centers.len();
        let mut eq = Self {
            mode: EqMode::Graphic31,
            sample_rate,
            gains_db: vec![0.0; n],
            centers_hz: centers,
            parametric: Vec::new(),
            linear_phase: false,
            filters: vec![Biquad::passthrough(); n],
            bypass: false,
        };
        eq.rebuild();
        eq
    }

    pub fn band_count(&self) -> usize {
        self.centers_hz.len()
    }

    pub fn set_mode(&mut self, mode: EqMode) {
        self.mode = mode;
        self.bypass = matches!(mode, EqMode::Bypass);
        match mode {
            EqMode::Graphic10 => {
                self.centers_hz = crate::GRAPHIC_10_BANDS_HZ.to_vec();
                self.gains_db.resize(10, 0.0);
            }
            EqMode::Graphic31 => {
                self.centers_hz = crate::GRAPHIC_31_BANDS_HZ.to_vec();
                self.gains_db.resize(31, 0.0);
            }
            EqMode::Bypass => {}
            _ => {}
        }
        self.rebuild();
    }

    pub fn set_band(&mut self, index: usize, gain_db: f32) {
        if let Some(g) = self.gains_db.get_mut(index) {
            *g = gain_db.clamp(-12.0, 12.0);
            self.rebuild();
        }
    }

    pub fn set_gains(&mut self, gains: &[f32]) {
        for (i, g) in gains.iter().enumerate() {
            if i < self.gains_db.len() {
                self.gains_db[i] = g.clamp(-12.0, 12.0);
            }
        }
        self.rebuild();
    }

    pub fn rebuild(&mut self) {
        self.filters.clear();
        if self.bypass || matches!(self.mode, EqMode::Bypass) {
            return;
        }
        if matches!(self.mode, EqMode::Parametric) {
            for node in &self.parametric {
                if !node.enabled {
                    continue;
                }
                // Peaking only for min-phase v1; shelves later.
                self.filters.push(Biquad::peaking(
                    node.freq_hz,
                    self.sample_rate,
                    node.q,
                    node.gain_db,
                ));
            }
            return;
        }
        for (i, &hz) in self.centers_hz.iter().enumerate() {
            let gain = self.gains_db.get(i).copied().unwrap_or(0.0);
            if gain.abs() < 0.01 {
                self.filters.push(Biquad::passthrough());
            } else {
                // Q ~ proportional to band density
                let q = 1.4;
                self.filters
                    .push(Biquad::peaking(hz, self.sample_rate, q, gain));
            }
        }
    }

    /// Immutable passthrough helper; RT code must use [`GraphicEq::process_sample_mut`].
    pub fn process_sample(&self, x: f32) -> f32 {
        if self.bypass || matches!(self.mode, EqMode::Bypass) {
            return x;
        }
        x
    }

    pub fn process_sample_mut(&mut self, mut x: f32) -> f32 {
        if self.bypass || matches!(self.mode, EqMode::Bypass) {
            return x;
        }
        for f in &mut self.filters {
            x = f.process(x);
        }
        x
    }

    pub fn process_buffer_mut(&mut self, buf: &mut [f32]) {
        for s in buf.iter_mut() {
            *s = self.process_sample_mut(*s);
        }
    }

    /// Suggest a gentle curve from timbral freq balance tags.
    pub fn apply_vap_guided(&mut self, freq_balance: &str) {
        self.mode = EqMode::VapGuided;
        let tag = freq_balance.to_lowercase();
        let mut gains = vec![0.0_f32; self.band_count()];
        if tag.contains("sub") {
            for g in gains.iter_mut().take(3) {
                *g = 3.0;
            }
        } else if tag.contains("mid") {
            for g in gains.iter_mut().skip(3).take(4) {
                *g = 2.5;
            }
        } else if tag.contains("air") || tag.contains("bright") {
            for g in gains.iter_mut().skip(7) {
                *g = 2.0;
            }
        }
        self.gains_db = gains;
        self.rebuild();
    }
}

/// Snapshot for UI / IPC.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EqStateSnapshot {
    pub mode: EqMode,
    pub gains_db: Vec<f32>,
    pub centers_hz: Vec<f32>,
    pub linear_phase: bool,
    pub bypass: bool,
}

impl GraphicEq {
    pub fn snapshot(&self) -> EqStateSnapshot {
        EqStateSnapshot {
            mode: self.mode,
            gains_db: self.gains_db.clone(),
            centers_hz: self.centers_hz.clone(),
            linear_phase: self.linear_phase,
            bypass: self.bypass,
        }
    }
}
