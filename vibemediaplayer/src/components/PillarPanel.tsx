import type { PillarId } from '../vap/types';
import { PILLAR_TABS } from '../vap/types';

interface Props {
  pillarId: PillarId;
  data: unknown;
  editMode: boolean;
  onChangeJson?: (raw: string) => void;
}

function flattenEntries(obj: unknown, prefix = ''): { key: string; value: string }[] {
  if (obj === null || obj === undefined) return [];
  if (typeof obj !== 'object') {
    return [{ key: prefix || 'value', value: String(obj) }];
  }
  if (Array.isArray(obj)) {
    return [{ key: prefix || 'list', value: obj.join(' → ') }];
  }
  const rows: { key: string; value: string }[] = [];
  for (const [k, v] of Object.entries(obj as Record<string, unknown>)) {
    const path = prefix ? `${prefix}.${k}` : k;
    if (v !== null && typeof v === 'object' && !Array.isArray(v)) {
      rows.push(...flattenEntries(v, path));
    } else if (Array.isArray(v)) {
      rows.push({ key: path, value: v.map(String).join(' → ') });
    } else {
      rows.push({ key: path, value: String(v) });
    }
  }
  return rows;
}

/** Metadata viewer/editor body for one pillar tab. */
export function PillarPanel({ pillarId, data, editMode, onChangeJson }: Props) {
  const meta = PILLAR_TABS.find((t) => t.id === pillarId)!;
  const rows = flattenEntries(data);
  const pretty = JSON.stringify(data ?? {}, null, 2);

  return (
    <section className="pillar-panel" style={{ borderColor: meta.accent }}>
      <header className="pillar-panel-header">
        <div>
          <span className="pillar-badge" style={{ color: meta.accent }}>
            {meta.short}
          </span>
          <h2 className="pillar-title">{meta.label}</h2>
        </div>
        <span className="pillar-hint">V.A.P. · Metadata Editor</span>
      </header>

      {editMode ? (
        <textarea
          className="pillar-json-editor"
          value={pretty}
          onChange={(e) => onChangeJson?.(e.target.value)}
          spellCheck={false}
          aria-label={`${meta.label} JSON editor`}
        />
      ) : (
        <div className="pillar-fields">
          {rows.length === 0 ? (
            <p className="pillar-empty">No data for this pillar — run analyzer or edit manually.</p>
          ) : (
            rows.map((r) => (
              <div key={r.key} className="pillar-row">
                <span className="pillar-key">{r.key}</span>
                <span className="pillar-val">
                  {r.key.toLowerCase().includes('hex') && r.value.startsWith('#') ? (
                    <span className="hex-swatch">
                      <i style={{ background: r.value }} />
                      {r.value}
                    </span>
                  ) : (
                    r.value
                  )}
                </span>
              </div>
            ))
          )}
        </div>
      )}
    </section>
  );
}
