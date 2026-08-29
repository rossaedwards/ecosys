import { useCallback, useRef, type ReactNode } from 'react';

export type ModulePos = {
  id: string;
  x: number;
  y: number;
  w: number;
  h: number;
  z: number;
  visible: boolean;
};

interface Props {
  mod: ModulePos;
  title: string;
  onMove: (id: string, x: number, y: number) => void;
  onResize?: (id: string, w: number, h: number) => void;
  onFocus: (id: string) => void;
  onClose: (id: string) => void;
  /** Extra buttons rendered in the header before the close button. */
  headerActions?: ReactNode;
  children: ReactNode;
}

/** Draggable / closable panel — WinAmp / VLC style modules. */
export function FloatingModule({
  mod,
  title,
  onMove,
  onResize,
  onFocus,
  onClose,
  headerActions,
  children,
}: Props) {
  const drag = useRef<{ dx: number; dy: number } | null>(null);

  const onPointerDown = useCallback(
    (e: React.PointerEvent) => {
      if ((e.target as HTMLElement).closest('.mod-actions')) return;
      onFocus(mod.id);
      drag.current = { dx: e.clientX - mod.x, dy: e.clientY - mod.y };
      (e.currentTarget as HTMLElement).setPointerCapture(e.pointerId);
    },
    [mod.id, mod.x, mod.y, onFocus],
  );

  const onPointerMove = useCallback(
    (e: React.PointerEvent) => {
      if (!drag.current) return;
      onMove(mod.id, Math.max(0, e.clientX - drag.current.dx), Math.max(0, e.clientY - drag.current.dy));
    },
    [mod.id, onMove],
  );

  const onPointerUp = useCallback(() => {
    drag.current = null;
  }, []);

  if (!mod.visible) return null;

  return (
    <div
      className="float-mod"
      style={{
        left: mod.x,
        top: mod.y,
        width: mod.w,
        height: mod.h,
        zIndex: mod.z,
      }}
      onMouseDown={() => onFocus(mod.id)}
    >
      <header
        className="float-mod-title"
        onPointerDown={onPointerDown}
        onPointerMove={onPointerMove}
        onPointerUp={onPointerUp}
      >
        <span>{title}</span>
        <div className="mod-actions">
          {headerActions}
          <button type="button" title="Close module" onClick={() => onClose(mod.id)}>
            ×
          </button>
        </div>
      </header>
      <div className="float-mod-body">{children}</div>
      {onResize && (
        <div
          className="float-mod-resize"
          onPointerDown={(e) => {
            e.stopPropagation();
            const startX = e.clientX;
            const startY = e.clientY;
            const startW = mod.w;
            const startH = mod.h;
            const el = e.currentTarget;
            const move = (ev: PointerEvent) => {
              onResize(
                mod.id,
                Math.max(200, startW + (ev.clientX - startX)),
                Math.max(120, startH + (ev.clientY - startY)),
              );
            };
            const up = () => {
              window.removeEventListener('pointermove', move);
              window.removeEventListener('pointerup', up);
            };
            window.addEventListener('pointermove', move);
            window.addEventListener('pointerup', up);
            el.setPointerCapture(e.pointerId);
          }}
        />
      )}
    </div>
  );
}
