import type { PlaylistEntry } from '../media/formats';

interface DeckProps {
  name: string;
  track?: PlaylistEntry | null;
  playing: boolean;
  pitch: number;
  onPlay: () => void;
  onPitch: (v: number) => void;
  onLoad: () => void;
  onCue: () => void;
}

function Deck({ name, track, playing, pitch, onPlay, onPitch, onLoad, onCue }: DeckProps) {
  return (
    <div className="vinyl-deck">
      <div className="vinyl-deck-head">
        <span className="deck-name">{name}</span>
        <button type="button" className="vmp-tb-btn" onClick={onLoad}>
          Load
        </button>
      </div>
      <div className={`vinyl-platter ${playing ? 'spin' : ''}`}>
        <div className="vinyl-label">
          <div className="vinyl-title">{track?.title ?? '— empty —'}</div>
          <div className="vinyl-artist">{track?.artist ?? ''}</div>
        </div>
      </div>
      <div className="vinyl-deck-ctrls">
        <button type="button" className="ctrl-btn play-btn" onClick={onPlay}>
          {playing ? '⏸' : '▶'}
        </button>
        <button type="button" className="ctrl-btn" onClick={onCue}>
          CUE
        </button>
        <label className="pitch-label">
          Pitch {pitch > 0 ? '+' : ''}
          {pitch.toFixed(1)}%
          <input
            type="range"
            min={-8}
            max={8}
            step={0.1}
            value={pitch}
            onChange={(e) => onPitch(parseFloat(e.target.value))}
          />
        </label>
      </div>
    </div>
  );
}

interface Props {
  deckA?: PlaylistEntry | null;
  deckB?: PlaylistEntry | null;
  playA: boolean;
  playB: boolean;
  pitchA: number;
  pitchB: number;
  crossfader: number;
  onPlayA: () => void;
  onPlayB: () => void;
  onPitchA: (v: number) => void;
  onPitchB: (v: number) => void;
  onCross: (v: number) => void;
  onLoadA: () => void;
  onLoadB: () => void;
  library: PlaylistEntry[];
  onLibraryClick: (id: string) => void;
}

/** Vinyl Vibez surface — evolves into Mixxx-class dual-deck DJ. */
export function VinylVibez(props: Props) {
  return (
    <div className="vinyl-root">
      <div className="vinyl-banner">
        <span className="vinyl-brand">Vinyl Vibez</span>
        <span className="vinyl-sub">→ Mixxx-class dual deck · BPM sync · hotcues · effects rack</span>
      </div>
      <div className="vinyl-decks">
        <Deck
          name="Deck A"
          track={props.deckA}
          playing={props.playA}
          pitch={props.pitchA}
          onPlay={props.onPlayA}
          onPitch={props.onPitchA}
          onLoad={props.onLoadA}
          onCue={() => {}}
        />
        <div className="vinyl-mixer">
          <div className="mixer-title">Mixer</div>
          <label>
            Crossfader
            <input
              type="range"
              min={0}
              max={1}
              step={0.01}
              value={props.crossfader}
              onChange={(e) => props.onCross(parseFloat(e.target.value))}
            />
          </label>
          <div className="xfade-labels">
            <span>A</span>
            <span>B</span>
          </div>
          <div className="mixer-note">Sync · Quantize · FX — Mixxx roadmap</div>
        </div>
        <Deck
          name="Deck B"
          track={props.deckB}
          playing={props.playB}
          pitch={props.pitchB}
          onPlay={props.onPlayB}
          onPitch={props.onPitchB}
          onLoad={props.onLoadB}
          onCue={() => {}}
        />
      </div>
      <div className="vinyl-library">
        <div className="mixer-title">DJ Library</div>
        <div className="vinyl-lib-list">
          {props.library.length === 0 ? (
            <p className="pillar-empty">Open files or a folder to build the crate.</p>
          ) : (
            props.library.map((t) => (
              <button
                key={t.id}
                type="button"
                className="lib-row"
                onClick={() => props.onLibraryClick(t.id)}
              >
                <span>{t.title}</span>
                <span className="lib-fmt">{t.format}</span>
              </button>
            ))
          )}
        </div>
      </div>
    </div>
  );
}
