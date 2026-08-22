import { useEffect, useState } from 'react';
import type { CSSProperties, DragEvent, KeyboardEvent, MouseEvent } from 'react';
import { HUBS, SUITE_STUBS, type HubId } from '../hubs';
import {
  broadcastPrompt,
  memoreeHealth,
  memoreeSaveClip,
  oracleStatus,
  routeClip,
  type MemoreeHealth,
} from '../native';

interface Popup {
  visible: boolean;
  x: number;
  y: number;
  text: string;
  sourceHub: HubId | null;
}

interface ActiveRitual {
  active: boolean;
  source: HubId | null;
  target: HubId | null;
}

const TOPOLOGY = [
  { label: 'LINK', locked: false },
  { label: 'CHAIN', locked: false },
  { label: 'RITUAL', locked: true },
  { label: 'FORKZ', locked: false },
];

export default function TuiCockpit() {
  const [terminals, setTerminals] = useState<Record<HubId, string>>(() => {
    const initial = {} as Record<HubId, string>;
    for (const h of HUBS) initial[h.id] = h.defaultText;
    return initial;
  });
  const [bound, setBound] = useState<Partial<Record<HubId, boolean>>>({});
  const [health, setHealth] = useState<MemoreeHealth | null>(null);
  const [globalInput, setGlobalInput] = useState('');
  const [popup, setPopup] = useState<Popup>({ visible: false, x: 0, y: 0, text: '', sourceHub: null });
  const [activeRitual, setActiveRitual] = useState<ActiveRitual>({ active: false, source: null, target: null });
  const [dancingHub, setDancingHub] = useState<HubId | null>(null);
  const [dropTarget, setDropTarget] = useState<HubId | null>(null);

  useEffect(() => {
    const refresh = () => {
      oracleStatus()
        .then((rows) => {
          const next: Partial<Record<HubId, boolean>> = {};
          for (const row of rows) next[row.hub as HubId] = row.bound;
          setBound(next);
        })
        .catch(() => {});
      memoreeHealth().then(setHealth).catch(() => setHealth(null));
    };
    refresh();
    const interval = setInterval(refresh, 20000);
    return () => clearInterval(interval);
  }, []);

  useEffect(() => {
    const closePopup = () => setPopup((prev) => ({ ...prev, visible: false }));
    window.addEventListener('click', closePopup);
    return () => window.removeEventListener('click', closePopup);
  }, []);

  const appendLine = (hub: HubId, line: string) => {
    setTerminals((prev) => ({ ...prev, [hub]: `${prev[hub]}\n\n${line}` }));
  };

  const handleSelection = (e: MouseEvent, hub: HubId) => {
    e.stopPropagation();
    const selection = window.getSelection();
    const text = selection ? selection.toString().trim() : '';
    if (!text) {
      setPopup((prev) => ({ ...prev, visible: false }));
      return;
    }
    const range = selection!.getRangeAt(0);
    const rect = range.getBoundingClientRect();
    setPopup({
      visible: true,
      x: rect.left + rect.width / 2,
      y: rect.top - 10,
      text,
      sourceHub: hub,
    });
  };

  const chainLink = (source: HubId, target: HubId, text: string) => {
    setActiveRitual({ active: true, source, target });
    setTimeout(async () => {
      const [reply] = await routeClip(text, source, [target]).catch(() => [
        { hub: target, ok: false, configured: false, text: 'error: route_clip failed' },
      ]);
      appendLine(target, `>> [CHAIN-LINK from ${source}]\n>> ${reply?.text ?? 'not configured'}`);
    }, 800);
    setTimeout(() => setActiveRitual({ active: false, source: null, target: null }), 1600);
  };

  const doCopy = () => {
    if (!popup.sourceHub) return;
    navigator.clipboard.writeText(popup.text).catch(() => {});
    setDancingHub(popup.sourceHub);
    setTimeout(() => setDancingHub(null), 1000);
    setPopup((prev) => ({ ...prev, visible: false }));
  };

  const doChainLinkFromPopup = (target: HubId) => {
    if (!popup.sourceHub) return;
    chainLink(popup.sourceHub, target, popup.text);
    setPopup((prev) => ({ ...prev, visible: false }));
  };

  const doSave = () => {
    if (!popup.sourceHub) return;
    const { sourceHub, text } = popup;
    setPopup((prev) => ({ ...prev, visible: false }));
    memoreeSaveClip(text, sourceHub)
      .then(() => appendLine(sourceHub, '>> [MEMORY-LINK] saved to Memoree'))
      .catch((err) => appendLine(sourceHub, `>> [MEMORY-LINK] failed: ${String(err)}`));
  };

  const handleDragStart = (e: DragEvent, hub: HubId) => {
    const selection = window.getSelection();
    const text = selection ? selection.toString().trim() : '';
    if (!text) {
      e.preventDefault();
      return;
    }
    e.dataTransfer.setData('application/json', JSON.stringify({ sourceHub: hub, text }));
  };

  const handleDrop = (e: DragEvent, target: HubId) => {
    e.preventDefault();
    setDropTarget(null);
    const raw = e.dataTransfer.getData('application/json');
    if (!raw) return;
    try {
      const { sourceHub, text } = JSON.parse(raw) as { sourceHub: HubId; text: string };
      if (sourceHub === target) return;
      chainLink(sourceHub, target, text);
    } catch {
      /* malformed payload — ignore */
    }
  };

  const handleGlobalSubmit = (e: KeyboardEvent<HTMLInputElement>) => {
    if (e.key !== 'Enter' || e.shiftKey) return;
    e.preventDefault();
    const prompt = globalInput.trim();
    if (!prompt) return;
    setGlobalInput('');
    const targets = HUBS.map((h) => h.id);
    for (const hub of targets) appendLine(hub, `>> [G0DM0D3_BROADCAST]: ${prompt}\n>> Executing...`);
    broadcastPrompt(prompt, targets)
      .then((replies) => {
        for (const reply of replies) {
          appendLine(reply.hub as HubId, reply.text);
        }
      })
      .catch((err) => {
        for (const hub of targets) appendLine(hub, `>> broadcast error: ${String(err)}`);
      });
  };

  return (
    <div className="cockpit">
      {popup.visible && (
        <div
          className="ritual-popup"
          style={{ top: popup.y, left: popup.x }}
          onClick={(e) => e.stopPropagation()}
        >
          <div className="ritual-popup-head">Initiate Ritual</div>
          <button onClick={doCopy}>[COPY] to clipboard</button>
          <div className="submenu" tabIndex={0}>
            <button>[CHAIN-LINK] to... ▶</button>
            <div className="submenu-items">
              {HUBS.filter((h) => h.id !== popup.sourceHub).map((h) => (
                <button key={h.id} style={{ color: h.color }} onClick={() => doChainLinkFromPopup(h.id)}>
                  {h.name}
                </button>
              ))}
            </div>
          </div>
          <button onClick={doSave}>[SAVE] to Memoree</button>
        </div>
      )}

      <div className="cockpit-header">
        <div>
          <h1 className="cockpit-title">g0dm0d3-ktrl</h1>
          <p className="cockpit-subtitle">Aura Control Deck :: Security Console</p>
        </div>
        <div className="prompt-bus">
          <span className="prompt-lead">g0d@m0d3:~#</span>
          <input
            type="text"
            placeholder="Broadcast a prompt to every hub..."
            value={globalInput}
            onChange={(e) => setGlobalInput(e.target.value)}
            onKeyDown={handleGlobalSubmit}
          />
        </div>
      </div>

      {health && (
        <div className={`memoree-banner ${health.ok ? 'ok' : 'down'}`}>
          {health.ok ? `Memoree paired — ${health.source}` : `Memoree unreachable — ${health.detail}`}
        </div>
      )}

      <div className="hub-grid scanlines">
        {HUBS.map((model) => {
          const isSourceTeleporting = activeRitual.active && activeRitual.source === model.id;
          const isTargetReceiving = activeRitual.active && activeRitual.target === model.id;
          const isBound = model.id === 'ollama' ? true : Boolean(bound[model.id]);
          const styleVars = {
            '--hub-color': model.color,
            '--hub-glow': model.glow,
          } as CSSProperties;

          return (
            <div
              key={model.id}
              className={`hub-pane${isTargetReceiving ? ' receiving' : ''}${dropTarget === model.id ? ' drop-target' : ''}`}
              style={styleVars}
              onDragOver={(e) => {
                e.preventDefault();
                setDropTarget(model.id);
              }}
              onDragLeave={() => setDropTarget((prev) => (prev === model.id ? null : prev))}
              onDrop={(e) => handleDrop(e, model.id)}
            >
              <div
                className={`shimoji${isSourceTeleporting ? ' zipping-out' : ''}${dancingHub === model.id ? ' dance' : ''}`}
                draggable
                onDragStart={(e) => handleDragStart(e, model.id)}
              >
                <div className="shimoji-orb">{model.icon}</div>
                <div className="shimoji-legs">
                  <span className="shimoji-leg" />
                  <span className="shimoji-leg right" />
                </div>
              </div>

              {isTargetReceiving && activeRitual.source && (
                <div className="shimoji-ghost" style={{ '--hub-color': model.color, '--hub-glow': model.glow } as CSSProperties}>
                  <div className="shimoji-orb">{HUBS.find((h) => h.id === activeRitual.source)?.icon}</div>
                  <div className="shimoji-ghost-label" style={{ color: model.color }}>
                    [DELIVERING]
                  </div>
                </div>
              )}

              <div className="hub-pane-titlebar">
                <span>
                  <span className={`hub-dot${isBound ? '' : ' unbound'}`} />
                  {model.id}@aura-node
                </span>
                <span className="hub-tty">tty{HUBS.indexOf(model) + 1}</span>
              </div>

              <div
                className={`hub-output${isTargetReceiving ? ' receiving' : ''}`}
                onMouseUp={(e) => handleSelection(e, model.id)}
              >
                <span className="timestamp">[{new Date().toLocaleTimeString()}]</span> {terminals[model.id]}
                <span className="hub-cursor" />
              </div>
            </div>
          );
        })}
      </div>

      <div className="locked-row" aria-label="Topology tiers">
        {TOPOLOGY.map((t) => (
          <span key={t.label} className="locked-chip" title={t.locked ? 'Phase 3+' : undefined}>
            {t.label} {t.locked ? '○' : '●'}
          </span>
        ))}
      </div>

      <div className="locked-row" aria-label="Alchemy Suite (Phase 5)">
        {SUITE_STUBS.map((s) => (
          <span key={s} className="locked-chip">
            {s}
          </span>
        ))}
      </div>

      <div className="cockpit-footer">
        <span>g0dm0d3-ktrl v0.1.0 :: Duality Kernel stub</span>
        <span className="waiting pulse">Waiting for ritual input...</span>
      </div>
    </div>
  );
}
