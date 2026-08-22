import { useEffect, useRef, useState } from 'react';
import { BIND_HUBS, HUBS, type HubId } from '../hubs';
import { bindOracle, isTauri, oracleStatus, setKiosk } from '../native';

const PHASES = {
  BOOT: 0,
  VIDEO_INTRO: 1,
  CRYPTONYX_TUTORIAL: 2,
  API_BINDING: 3,
  THE_RITUAL: 4,
} as const;

type Phase = (typeof PHASES)[keyof typeof PHASES];

const BOOT_SEQUENCE = [
  'INITIATING AURA OS KERNEL...',
  'MOUNTING AuraFS PHOTONIC MESH...',
  'WAKING S.A.G.E.S. IMMUNE SYSTEM...',
  'LOADING THE BOOK OF FUX...',
  'INITIALIZING DUALITY KERNEL...',
  'REQUESTING EXCLUSIVE DISPLAY... [OPTIONAL]',
  'WELCOME TO g0dm0d3.',
];

const RITUAL_STAGES = [
  { id: 0, label: 'CTRL' },
  { id: 1, label: 'ALT' },
  { id: 2, label: 'RE-d3s1gN' },
] as const;

const HOLD_MS = 5000;
const HOLD_TICK_MS = 100;
const HOLD_STEP = (HOLD_TICK_MS / HOLD_MS) * 100;

export default function FirstBoot({ onSealed }: { onSealed: () => void }) {
  const [phase, setPhase] = useState<Phase>(PHASES.BOOT);
  const [bootLines, setBootLines] = useState<string[]>([]);

  const [bound, setBound] = useState<Partial<Record<HubId, boolean>>>({});
  const [pending, setPending] = useState<Partial<Record<HubId, string>>>({});
  const [nativeMissing, setNativeMissing] = useState(!isTauri());

  const [holdProgress, setHoldProgress] = useState(0);
  const [ritualStage, setRitualStage] = useState(0);
  const holdInterval = useRef<ReturnType<typeof setInterval> | null>(null);

  useEffect(() => {
    if (phase !== PHASES.BOOT) return;
    let i = 0;
    const interval = setInterval(() => {
      setBootLines((prev) => [...prev, BOOT_SEQUENCE[i]]);
      i += 1;
      if (i >= BOOT_SEQUENCE.length) {
        clearInterval(interval);
        setTimeout(() => setPhase(PHASES.VIDEO_INTRO), 1500);
      }
    }, 400);
    return () => clearInterval(interval);
  }, [phase]);

  useEffect(() => {
    if (phase !== PHASES.VIDEO_INTRO) return;
    const timer = setTimeout(() => setPhase(PHASES.CRYPTONYX_TUTORIAL), 4000);
    return () => clearTimeout(timer);
  }, [phase]);

  // Administrator override = optional exclusive/fullscreen window, not a hostile OS lock.
  useEffect(() => {
    if (phase !== PHASES.CRYPTONYX_TUTORIAL) return;
    setKiosk(true).catch(() => {
      /* Vite-only dev or platform refusal — proceed windowed. */
    });
  }, [phase]);

  useEffect(() => {
    if (phase !== PHASES.API_BINDING) return;
    if (!isTauri()) {
      setNativeMissing(true);
      return;
    }
    oracleStatus()
      .then((rows) => {
        const next: Partial<Record<HubId, boolean>> = {};
        for (const row of rows) next[row.hub as HubId] = row.bound;
        setBound(next);
      })
      .catch(() => setNativeMissing(true));
  }, [phase]);

  const refreshStatus = () => {
    oracleStatus()
      .then((rows) => {
        const next: Partial<Record<HubId, boolean>> = {};
        for (const row of rows) next[row.hub as HubId] = row.bound;
        setBound(next);
      })
      .catch(() => setNativeMissing(true));
  };

  const bind = async (hub: HubId) => {
    const value = pending[hub] ?? '';
    try {
      await bindOracle(hub, value);
      setPending((prev) => ({ ...prev, [hub]: '' }));
      refreshStatus();
    } catch {
      setNativeMissing(true);
    }
  };

  const unbind = async (hub: HubId) => {
    try {
      await bindOracle(hub, '');
      refreshStatus();
    } catch {
      setNativeMissing(true);
    }
  };

  const handleHoldStart = (targetStage: number) => {
    if (ritualStage !== targetStage) return;
    holdInterval.current = setInterval(() => {
      setHoldProgress((prev) => {
        if (prev + HOLD_STEP >= 100) {
          if (holdInterval.current) clearInterval(holdInterval.current);
          if (targetStage === RITUAL_STAGES.length - 1) {
            setKiosk(false).catch(() => {});
            localStorage.setItem('g0dm0d3-ktrl.sealed', '1');
            setTimeout(onSealed, 1000);
          } else {
            setRitualStage(targetStage + 1);
          }
          return 0;
        }
        return prev + HOLD_STEP;
      });
    }, HOLD_TICK_MS);
  };

  const handleHoldEnd = () => {
    if (holdInterval.current) clearInterval(holdInterval.current);
    setHoldProgress(0);
  };

  return (
    <div className="boot-screen scanlines">
      {phase === PHASES.BOOT && (
        <div className="boot-log">
          {bootLines.map((line, i) => (
            <div key={i} className="boot-log-line">
              &gt; {line}
            </div>
          ))}
          <span className="boot-cursor" />
        </div>
      )}

      {phase === PHASES.VIDEO_INTRO && (
        <div className="video-intro pulse">
          <div className="video-intro-title">[ PLAYING: g0dm0d3-welcome2tribe.mp4 ]</div>
          <div className="video-intro-body">
            *Epic cyberpunk / occult montage plays...*
            <br />
            *Fractal geometry folding into the AuraOS logo...*
          </div>
        </div>
      )}

      {(phase === PHASES.CRYPTONYX_TUTORIAL || phase === PHASES.API_BINDING) && (
        <div className="tutorial-panel">
          <div className="cryptonyx-portrait">
            <div className="cryptonyx-frame">
              <span>💀</span>
              <span className="tag">AETHORNYX</span>
            </div>
            <div className="cryptonyx-name glitch-text">CRYPTONYX</div>
            <div className="cryptonyx-title">Undead Deity of Orchestration</div>
          </div>

          {phase === PHASES.CRYPTONYX_TUTORIAL ? (
            <div className="tutorial-body">
              <p>&quot;Ah... another soul seeking the lattice. I am Cryptonyx. You stand at the threshold of the Duality Kernel.&quot;</p>
              <p style={{ color: '#999' }}>
                &quot;Before you weave rituals, you may bind your Oracles — the AI hubs g0dm0d3 will speak to on your
                behalf. Zero, some, or all. Unbound hubs simply answer &apos;not configured&apos;.&quot;
              </p>
              <button className="ghost-btn" onClick={() => setPhase(PHASES.API_BINDING)}>
                [ Initiate Binding ]
              </button>
            </div>
          ) : (
            <div className="tutorial-body">
              <h3 style={{ color: '#ff66dd', fontSize: '0.85rem', textTransform: 'uppercase', letterSpacing: '0.15em', margin: 0 }}>
                Bind Your Legion
              </h3>
              {nativeMissing && (
                <div className="native-warning">
                  Tauri IPC missing — run <code>npm run tauri:dev</code> to bind oracles. You can still proceed; panes
                  will read &quot;not configured&quot;.
                </div>
              )}
              <div className="oracle-grid">
                {BIND_HUBS.map((hubId) => {
                  const spec = HUBS.find((h) => h.id === hubId)!;
                  const isOllama = hubId === 'ollama';
                  const isBound = Boolean(bound[hubId]) || isOllama;
                  return (
                    <div key={hubId} className={`oracle-row${isBound ? ' bound' : ''}`}>
                      <div className="oracle-row-head">
                        <span style={{ color: spec.color }}>{spec.icon} {spec.name}</span>
                        <span>{isOllama ? '[ LOCAL ]' : isBound ? '[ BOUND ]' : '[ OFFLINE ]'}</span>
                      </div>
                      <input
                        type={isOllama ? 'text' : 'password'}
                        placeholder={isOllama ? 'model (default llama3.2)' : 'API key'}
                        value={pending[hubId] ?? ''}
                        onChange={(e) => setPending((prev) => ({ ...prev, [hubId]: e.target.value }))}
                        disabled={nativeMissing}
                      />
                      <div className="oracle-row-actions">
                        <button onClick={() => bind(hubId)} disabled={nativeMissing}>
                          Bind
                        </button>
                        <button onClick={() => unbind(hubId)} disabled={nativeMissing}>
                          Unbind
                        </button>
                      </div>
                    </div>
                  );
                })}
              </div>
              <button className="proceed-btn" onClick={() => setPhase(PHASES.THE_RITUAL)}>
                Proceed to Reality Rewrite
              </button>
            </div>
          )}
        </div>
      )}

      {phase === PHASES.THE_RITUAL && (
        <div className="ritual-screen">
          <h1 className="ritual-title">
            ARE YOU READY TO REWRITE REALITY
            <br />
            WITH <span className="accent glitch-text">AURPHYX</span>??
          </h1>
          <div className="ritual-buttons">
            {RITUAL_STAGES.map((stage, index) => (
              <div key={stage.id} style={{ display: 'flex', alignItems: 'center', gap: '2rem' }}>
                {index > 0 && <span className="ritual-plus">+</span>}
                <div className="ritual-btn-wrap">
                  {ritualStage === stage.id && (
                    <div className="ritual-btn-progress" style={{ width: `${holdProgress}%` }} />
                  )}
                  <button
                    className={`ritual-btn${
                      ritualStage > stage.id ? ' sealed' : ritualStage === stage.id ? ' active' : ''
                    }`}
                    disabled={ritualStage !== stage.id}
                    onMouseDown={() => handleHoldStart(stage.id)}
                    onMouseUp={handleHoldEnd}
                    onMouseLeave={handleHoldEnd}
                    onTouchStart={(e) => {
                      e.preventDefault();
                      handleHoldStart(stage.id);
                    }}
                    onTouchEnd={handleHoldEnd}
                  >
                    {ritualStage > stage.id ? '[ SEALED ]' : stage.label}
                  </button>
                  {ritualStage === stage.id && (
                    <span className="ritual-hint">{holdProgress > 0 ? `${(holdProgress / 20).toFixed(1)}s` : 'HOLD (5s)'}</span>
                  )}
                </div>
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}
