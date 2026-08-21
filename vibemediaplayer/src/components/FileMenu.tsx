import { useEffect, useRef, useState } from 'react';
import { FILE_MENU, type FileMenuId } from '../media/formats';

interface Props {
  recent: string[];
  onAction: (id: FileMenuId) => void;
  onRecent: (name: string) => void;
}

export function FileMenuBar({ recent, onAction, onRecent }: Props) {
  const [open, setOpen] = useState(false);
  const [recentOpen, setRecentOpen] = useState(false);
  const ref = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const onDoc = (e: MouseEvent) => {
      if (!ref.current?.contains(e.target as Node)) {
        setOpen(false);
        setRecentOpen(false);
      }
    };
    document.addEventListener('mousedown', onDoc);
    return () => document.removeEventListener('mousedown', onDoc);
  }, []);

  return (
    <div className="menubar" ref={ref}>
      <button type="button" className="menu-top" onClick={() => setOpen((o) => !o)}>
        File
      </button>
      {open && (
        <div className="menu-dropdown" role="menu">
          {FILE_MENU.map((item) => {
            if (item.id === 'open_recent') {
              return (
                <div key={item.id} className="menu-item-wrap">
                  <button
                    type="button"
                    className="menu-item"
                    onMouseEnter={() => setRecentOpen(true)}
                    onClick={() => setRecentOpen((r) => !r)}
                  >
                    <span>{item.label}</span>
                    <span className="menu-caret">▸</span>
                  </button>
                  {recentOpen && (
                    <div className="menu-submenu">
                      {recent.length === 0 ? (
                        <div className="menu-item muted">No recent media</div>
                      ) : (
                        recent.map((r) => (
                          <button
                            key={r}
                            type="button"
                            className="menu-item"
                            onClick={() => {
                              onRecent(r);
                              setOpen(false);
                            }}
                          >
                            {r}
                          </button>
                        ))
                      )}
                    </div>
                  )}
                  {item.sepAfter && <div className="menu-sep" />}
                </div>
              );
            }
            return (
              <div key={item.id}>
                <button
                  type="button"
                  className="menu-item"
                  onClick={() => {
                    onAction(item.id);
                    setOpen(false);
                  }}
                >
                  <span>{item.label}</span>
                  {item.shortcut && <span className="menu-sc">{item.shortcut}</span>}
                </button>
                {item.sepAfter && <div className="menu-sep" />}
              </div>
            );
          })}
        </div>
      )}
    </div>
  );
}
