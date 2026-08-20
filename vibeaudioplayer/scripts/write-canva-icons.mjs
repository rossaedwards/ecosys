import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const dir = "/workspace/public/canva-kit/icons";
mkdirSync(dir, { recursive: true });

const svg = (inner, { size = 24, stroke = "#5F7CFF", fill = "none", sw = 1.75 } = {}) =>
  `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 ${size} ${size}" fill="${fill}" stroke="${stroke}" stroke-width="${sw}" stroke-linecap="round" stroke-linejoin="round">${inner}</svg>\n`;

const icons = {
  "mark-orb": svg(
    `<circle cx="12" cy="12" r="3.2" fill="#4B0082" stroke="#2ee6c8" stroke-width="1.2"/>
     <circle cx="12" cy="12" r="7.2" fill="none" stroke="#008080" stroke-width="1.1"/>
     ${Array.from({ length: 9 }, (_, i) => {
       const a = (Math.PI * 2 * i) / 9 - Math.PI / 2;
       const x = 12 + Math.cos(a) * 7.2;
       const y = 12 + Math.sin(a) * 7.2;
       return `<circle cx="${x.toFixed(2)}" cy="${y.toFixed(2)}" r="1.05" fill="#2ee6c8" stroke="none"/>`;
     }).join("")}`,
    { fill: "none", stroke: "none" },
  ),
  visualizer: svg(`<circle cx="12" cy="12" r="3"/><path d="M12 5v2M12 17v2M5 12h2M17 12h2"/><path d="M7.2 7.2l1.3 1.3M15.5 15.5l1.3 1.3M7.2 16.8l1.3-1.3M15.5 8.5l1.3-1.3"/>`),
  library: svg(`<path d="M4 5h10v14H4z"/><path d="M14 8h6v11H8"/><path d="M7 9v6M10 9v6"/>`),
  profile: svg(`<circle cx="12" cy="12" r="8"/><circle cx="12" cy="12" r="2.4"/>${Array.from({ length: 9 }, (_, i) => {
    const a = (Math.PI * 2 * i) / 9 - Math.PI / 2;
    return `<circle cx="${(12 + Math.cos(a) * 5.4).toFixed(2)}" cy="${(12 + Math.sin(a) * 5.4).toFixed(2)}" r="0.7" fill="#5F7CFF" stroke="none"/>`;
  }).join("")}`),
  scenes: svg(`<rect x="3" y="5" width="18" height="12" rx="2"/><path d="M3 14l4.5-3.5 3.5 2.5 4-4.5L21 13"/>`),
  equalizer: svg(`<path d="M6 20V10M12 20V4M18 20v-7"/>`),
  settings: svg(`<circle cx="12" cy="12" r="3"/><path d="M12 3.5v2.2M12 18.3v2.2M4.8 7.2l1.9 1.1M17.3 15.7l1.9 1.1M4.8 16.8l1.9-1.1M17.3 8.3l1.9-1.1"/>`),
  play: svg(`<path d="M8 6.5v11l10-5.5z" fill="#5F7CFF"/>`, { fill: "#5F7CFF" }),
  pause: svg(`<path d="M7 6h3.2v12H7zM13.8 6H17v12h-3.2z" fill="#5F7CFF" stroke="none"/>`),
  stop: svg(`<rect x="7" y="7" width="10" height="10" rx="1.2" fill="#5F7CFF" stroke="none"/>`),
  previous: svg(`<path d="M18 6v12l-8-6z"/><path d="M6 6v12"/>`),
  next: svg(`<path d="M6 6v12l8-6z"/><path d="M18 6v12"/>`),
  shuffle: svg(`<path d="M4 7h3.5l8 10H20"/><path d="M16 7h4v4"/><path d="M4 17h3.5l2-2.4"/><path d="M16 17h4v-4"/>`),
  "repeat-off": svg(`<path d="M17 3l3 3-3 3"/><path d="M6 7h12.5"/><path d="M7 21l-3-3 3-3"/><path d="M18 17H5.5"/><path d="M8 12h.01"/>`),
  "repeat-all": svg(`<path d="M17 3l3 3-3 3"/><path d="M6 7h12.5"/><path d="M7 21l-3-3 3-3"/><path d="M18 17H5.5"/>`),
  "repeat-one": svg(`<path d="M17 3l3 3-3 3"/><path d="M6 7h12.5"/><path d="M7 21l-3-3 3-3"/><path d="M18 17H5.5"/><path d="M12 10v5"/>`),
  queue: svg(`<path d="M4 7h16M4 12h16M4 17h10"/>`),
  "add-file": svg(`<path d="M9 3h6l5 5v13H9z"/><path d="M15 3v5h5"/><path d="M12 12v6M9 15h6"/>`),
  folder: svg(`<path d="M3 7h6l2 2h10v10H3z"/>`),
  search: svg(`<circle cx="11" cy="11" r="6"/><path d="M16 16l4 4"/>`),
  more: svg(`<circle cx="6" cy="12" r="1.2" fill="#5F7CFF" stroke="none"/><circle cx="12" cy="12" r="1.2" fill="#5F7CFF" stroke="none"/><circle cx="18" cy="12" r="1.2" fill="#5F7CFF" stroke="none"/>`),
  back: svg(`<path d="M14 5l-7 7 7 7"/>`),
  close: svg(`<path d="M6 6l12 12M18 6L6 18"/>`),
  expand: svg(`<path d="M9 5H5v4M15 5h4v4M5 15v4h4M19 15v4h-4"/>`),
  collapse: svg(`<path d="M8 4v4H4M16 4v4h4M4 16h4v4M20 16h-4v4"/>`),
  fullscreen: svg(`<path d="M4 9V4h5M20 9V4h-5M4 15v5h5M20 15v5h-5"/>`),
  cast: svg(`<path d="M3 7.5A2.5 2.5 0 0 1 5.5 5h13A2.5 2.5 0 0 1 21 7.5v9a2.5 2.5 0 0 1-2.5 2.5H14"/><path d="M3 19a2 2 0 0 1 2-2M3 15a6 6 0 0 1 6 6"/>`),
  home: svg(`<path d="M4 11.5 12 5l8 6.5V20H4z"/><path d="M10 20v-6h4v6"/>`),
  microphone: svg(`<rect x="9" y="3.5" width="6" height="10" rx="3"/><path d="M6 11a6 6 0 0 0 12 0M12 17v3"/>`),
  volume: svg(`<path d="M4 10v4h3l4 3V7L7 10H4z"/><path d="M16 9.5a3.5 3.5 0 0 1 0 5"/>`),
  "mini-player": svg(`<rect x="3" y="14" width="18" height="7" rx="1.5"/><circle cx="7.5" cy="17.5" r="1.6"/><path d="M11 16.5h7M11 18.8h4"/>`),
  bluetooth: svg(`<path d="M7 7l10 10-5 3V4l5 3L7 17"/>`),
  wired: svg(`<path d="M8 4v7a4 4 0 0 0 8 0V4"/><path d="M8 4H6M16 4h2M12 15v5"/>`),
  battery: svg(`<rect x="3" y="8" width="15" height="8" rx="1.5"/><path d="M20 11v2"/><path d="M5.5 10.5h8v3h-8z" fill="#2ee6c8" stroke="none"/>`),
  theme: svg(`<circle cx="12" cy="12" r="8"/><path d="M12 4a8 8 0 0 0 0 16z" fill="#5F7CFF"/>`),
  "pillar-structural": svg(`<path d="M5 19V9M9.5 19V5M14 19v-7M19 19V8"/>`),
  "pillar-tonal": svg(`<path d="M8 16a4 4 0 1 0 .4-1.8"/><path d="M12 16V6l6 1.2V8"/>`),
  "pillar-timbral": svg(`<path d="M4 16c2-6 4-6 6 0s4 6 6 0 4-6 6 0"/>`),
  "pillar-linguistic": svg(`<path d="M5 7h9a3 3 0 0 1 0 6H9l-4 4V7z"/>`),
  "pillar-affective": svg(`<path d="M12 18s-7-4.3-7-9a4 4 0 0 1 7-2 4 4 0 0 1 7 2c0 4.7-7 9-7 9z"/>`),
  "pillar-contextual": svg(`<path d="M12 21s7-6.2 7-11a7 7 0 1 0-14 0c0 4.8 7 11 7 11z"/><circle cx="12" cy="10" r="2.2"/>`),
  "pillar-photometric": svg(`<circle cx="12" cy="12" r="3"/><path d="M2.8 12h3M18.2 12h3M12 2.8v3M12 18.2v3M5.4 5.4l2.1 2.1M16.5 16.5l2.1 2.1M5.4 18.6l2.1-2.1M16.5 7.5l2.1-2.1"/>`),
  "pillar-kinetic": svg(`<path d="M5 16c3-8 5-8 7 0s4 8 7 0"/><path d="M4 8h3l2 4 2-8 2 10 2-6 3 4"/>`),
  "pillar-genealogical": svg(`<path d="M12 4v16M12 8h6v4M12 14H6v5"/>`),
  knob: svg(`<circle cx="12" cy="12" r="8"/><path d="M12 12l3.5-4.2"/><circle cx="12" cy="12" r="1.4" fill="#2ee6c8" stroke="none"/>`),
  toggle: svg(`<rect x="3" y="7" width="18" height="10" rx="5"/><circle cx="16" cy="12" r="3.2" fill="#2ee6c8" stroke="none"/>`),
  slider: svg(`<path d="M4 12h16"/><circle cx="14" cy="12" r="2.6" fill="#2ee6c8" stroke="#5F7CFF"/>`),
  "status-ready": svg(`<circle cx="12" cy="12" r="8"/><path d="M8 12.5l2.5 2.5L16 9"/>`),
  "status-analyzing": svg(`<circle cx="12" cy="12" r="8"/><path d="M12 7v5l3 2"/>`),
  "status-error": svg(`<circle cx="12" cy="12" r="8"/><path d="M12 8v5M12 16h.01"/>`),
  "choose-file": svg(`<rect x="4" y="4" width="16" height="16" rx="3"/><path d="M8 12h8M12 8v8"/>`),
  "choose-folder": svg(`<path d="M3 8h6l2 2h10v9H3z"/><path d="M12 12v5M9.5 14.5H14.5"/>`),
};

for (const [name, xml] of Object.entries(icons)) {
  writeFileSync(join(dir, `${name}.svg`), xml);
}
console.log(`wrote ${Object.keys(icons).length} icons to ${dir}`);
