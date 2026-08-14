import type { DeviceInfo } from '../vap/types';

interface Props {
  inputs: DeviceInfo[];
  outputs: DeviceInfo[];
  selectedInput: string;
  selectedOutput: string;
  sampleRate: number;
  bufferSize: number;
  onInput: (id: string) => void;
  onOutput: (id: string) => void;
  onSampleRate: (n: number) => void;
  onBufferSize: (n: number) => void;
}

/** Audacity-style host device matrix (virtual cable comes later). */
export function DeviceMatrixPanel(props: Props) {
  const {
    inputs,
    outputs,
    selectedInput,
    selectedOutput,
    sampleRate,
    bufferSize,
    onInput,
    onOutput,
    onSampleRate,
    onBufferSize,
  } = props;

  return (
    <div className="device-matrix">
      <div className="eq-header">
        <span className="panel-title">Devices · I/O Matrix</span>
        <span className="panel-badge">Audacity-class</span>
      </div>
      <div className="device-grid">
        <label>
          Input
          <select value={selectedInput} onChange={(e) => onInput(e.target.value)}>
            {inputs.map((d) => (
              <option key={d.id} value={d.id}>
                {d.name}
              </option>
            ))}
          </select>
        </label>
        <label>
          Output
          <select value={selectedOutput} onChange={(e) => onOutput(e.target.value)}>
            {outputs.map((d) => (
              <option key={d.id} value={d.id}>
                {d.name}
              </option>
            ))}
          </select>
        </label>
        <label>
          Sample Rate
          <select
            value={sampleRate}
            onChange={(e) => onSampleRate(parseInt(e.target.value, 10))}
          >
            {[44100, 48000, 96000, 192000].map((r) => (
              <option key={r} value={r}>
                {r} Hz
              </option>
            ))}
          </select>
        </label>
        <label>
          Buffer
          <select
            value={bufferSize}
            onChange={(e) => onBufferSize(parseInt(e.target.value, 10))}
          >
            {[64, 128, 256, 512, 1024].map((b) => (
              <option key={b} value={b}>
                {b} samples
              </option>
            ))}
          </select>
        </label>
      </div>
      <p className="device-note">
        Phase 1: OS devices via cpal. Phase 5: PipeWire “Vibe Cable” virtual sound card.
      </p>
    </div>
  );
}
