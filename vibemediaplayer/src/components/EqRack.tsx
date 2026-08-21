import { EQ_10_BANDS, type EqMode } from '../vap/types';

interface Props {
  gains: number[];
  mode: EqMode;
  onGain: (index: number, value: number) => void;
  onMode: (mode: EqMode) => void;
}

const MODES: { id: EqMode; label: string }[] = [
  { id: 'graphic10', label: '10-Band' },
  { id: 'graphic31', label: '31-Band' },
  { id: 'parametric', label: 'Parametric' },
  { id: 'vap_guided', label: 'VAP-Guided' },
  { id: 'context_linked', label: 'Context' },
  { id: 'bypass', label: 'Bypass' },
];

export function EqRack({ gains, mode, onGain, onMode }: Props) {
  const labels =
    mode === 'graphic31'
      ? Array.from({ length: 31 }, (_, i) => (i % 4 === 0 ? `${i}` : ''))
      : [...EQ_10_BANDS];
  const displayGains =
    mode === 'graphic31'
      ? Array.from({ length: 31 }, (_, i) => gains[i] ?? 0)
      : gains.slice(0, 10);

  return (
    <div className="eq-rack">
      <div className="eq-header">
        <span className="panel-title accent-t">EQ · DSP Rack</span>
        <div className="eq-modes">
          {MODES.map((m) => (
            <button
              key={m.id}
              type="button"
              className={`eq-mode-btn ${mode === m.id ? 'active' : ''}`}
              onClick={() => onMode(m.id)}
            >
              {m.label}
            </button>
          ))}
        </div>
      </div>
      <div className={`eq-grid ${mode === 'graphic31' ? 'eq-31' : ''}`}>
        {displayGains.map((g, i) => (
          <div key={i} className="eq-band">
            <div className="eq-val">
              {g > 0 ? '+' : ''}
              {g.toFixed(1)}
            </div>
            <div className="eq-slider-wrap">
              <input
                type="range"
                className="eq-slider"
                min={-12}
                max={12}
                step={0.5}
                value={g}
                onChange={(e) => onGain(i, parseFloat(e.target.value))}
                disabled={mode === 'bypass'}
                aria-label={`EQ band ${labels[i] || i}`}
              />
            </div>
            <div className="eq-label">{labels[i]}</div>
          </div>
        ))}
      </div>
    </div>
  );
}
