import type { CSSProperties } from 'react';
import type { PillarTab, PillarId } from '../vap/types';
import { PILLAR_TABS } from '../vap/types';

interface Props {
  active: PillarId;
  onSelect: (id: PillarId) => void;
  tabs?: PillarTab[];
}

/** Firefox / Opera–style vertical tab rail for the 9 V.A.P. pillars. */
export function VerticalPillarTabs({ active, onSelect, tabs = PILLAR_TABS }: Props) {
  return (
    <nav className="vtab-rail" aria-label="V.A.P. pillars">
      {tabs.map((tab) => {
        const isActive = tab.id === active;
        return (
          <button
            key={tab.id}
            type="button"
            className={`vtab ${isActive ? 'active' : ''}`}
            style={{ '--tab-accent': tab.accent } as CSSProperties}
            onClick={() => onSelect(tab.id)}
            title={`${tab.short} · ${tab.label}`}
            aria-current={isActive ? 'page' : undefined}
          >
            <span className="vtab-num">{tab.short}</span>
            <span className="vtab-label">{tab.label}</span>
          </button>
        );
      })}
    </nav>
  );
}
