export const CANVA_BRAND = {
  product: "VASP Visualizer",
  player: "Vibe Audio Player",
  version: "3.69",
  owner: "Aurphyx LLC",
  style: "Aurphyx nocturnal-tech — dark cosmic base, indigo/teal photometric energy, controlled motion.",
};

export const PILLARS = [
  { n: 1, key: "Structural", archetype: "Skeleton", role: "BPM, beat grid, waveform, tempo pulse", hex: "#5F7CFF", icon: "/canva-kit/icons/pillar-structural.svg" },
  { n: 2, key: "Tonal", archetype: "Flesh", role: "Key, harmonic geometry, tonal color shifts", hex: "#7C6CFF", icon: "/canva-kit/icons/pillar-tonal.svg" },
  { n: 3, key: "Timbral", archetype: "Skin", role: "Spectrum, grain, glow, blur, texture", hex: "#2EE6C8", icon: "/canva-kit/icons/pillar-timbral.svg" },
  { n: 4, key: "Linguistic", archetype: "Voice", role: "Lyrics, vocal state, explicit-content badge", hex: "#9AA5C5", icon: "/canva-kit/icons/pillar-linguistic.svg" },
  { n: 5, key: "Affective", archetype: "Heart", role: "Mood, energy, contrast, animation intensity", hex: "#FF4D6D", icon: "/canva-kit/icons/pillar-affective.svg" },
  { n: 6, key: "Contextual", archetype: "Scene", role: "Presets: Night Drive, Gym, Focus, Club", hex: "#008080", icon: "/canva-kit/icons/pillar-contextual.svg" },
  { n: 7, key: "Photometric", archetype: "Eye", role: "Palette, brightness, fades, particle color", hex: "#4B0082", icon: "/canva-kit/icons/pillar-photometric.svg" },
  { n: 8, key: "Kinetic", archetype: "Body", role: "Beat impact, motion scale, entrainment", hex: "#FFB000", icon: "/canva-kit/icons/pillar-kinetic.svg" },
  { n: 9, key: "Genealogical", archetype: "Roots", role: "Genre, era, lineage, theme accent", hex: "#B7B0C7", icon: "/canva-kit/icons/pillar-genealogical.svg" },
] as const;

export const COLORS = {
  dark: [
    { token: "void", hex: "#080A12", use: "App background" },
    { token: "surface", hex: "#111526", use: "Cards and panels" },
    { token: "surfaceRaised", hex: "#181D33", use: "Sheets, selected controls" },
    { token: "primary", hex: "#4B0082", use: "Indigo action / glow" },
    { token: "secondary", hex: "#008080", use: "Teal signal" },
    { token: "signal", hex: "#5F7CFF", use: "Spectrum / active" },
    { token: "warning", hex: "#FFB000", use: "Caution / processing" },
    { token: "danger", hex: "#FF4D6D", use: "Stop / delete / error" },
    { token: "textPrimary", hex: "#F4F7FF", use: "Main text" },
    { token: "textMuted", hex: "#9AA5C5", use: "Labels" },
  ],
  light: [
    { token: "void", hex: "#F5F7FC", use: "App background" },
    { token: "surface", hex: "#FFFFFF", use: "Cards and panels" },
    { token: "surfaceRaised", hex: "#E9EEFA", use: "Sheets, selected controls" },
    { token: "primary", hex: "#6420A8", use: "Indigo action" },
    { token: "secondary", hex: "#007A7A", use: "Teal signal" },
    { token: "signal", hex: "#3658E8", use: "Spectrum / active" },
    { token: "warning", hex: "#B87500", use: "Caution" },
    { token: "danger", hex: "#C92C4D", use: "Stop / delete" },
    { token: "textPrimary", hex: "#111827", use: "Main text" },
    { token: "textMuted", hex: "#5C667A", use: "Labels" },
  ],
};

export const CANVA_SIZES = [
  { name: "App icon master", w: 1024, h: 1024, canva: "Custom 1024 × 1024 px", file: "/canva-kit/logos/app-icon-1024.jpg" },
  { name: "Adaptive icon FG", w: 432, h: 432, canva: "Custom 432 × 432 px", file: "/canva-kit/logos/adaptive-foreground-432.jpg" },
  { name: "Instagram / logo", w: 1080, h: 1080, canva: "Instagram Post", file: "/canva-kit/logos/app-icon-1024.jpg" },
  { name: "Story splash", w: 1080, h: 1920, canva: "Instagram Story", file: "/canva-kit/illustrations/splash-1080x1920.jpg" },
  { name: "Phone splash", w: 1080, h: 2400, canva: "Custom 1080 × 2400 px", file: "/canva-kit/illustrations/splash-1080x2400.jpg" },
  { name: "Onboarding hero", w: 1080, h: 1350, canva: "Custom 1080 × 1350 px", file: "/canva-kit/illustrations/onboarding-1080x1350.jpg" },
  { name: "Scene thumbnail", w: 720, h: 405, canva: "Custom 720 × 405 px", file: "/canva-kit/scenes/night-drive-720x405.jpg" },
  { name: "Presentation / brand board", w: 1920, h: 1080, canva: "Presentation (16:9)", file: null },
  { name: "Empty library", w: 1440, h: 1440, canva: "Custom 1440 × 1440 px", file: "/canva-kit/illustrations/empty-library-1440.jpg" },
];

export const ART = [
  { title: "App mark", src: "/canva-kit/logos/app-icon-1024.jpg", hint: "Brand Kit → Logos (primary)" },
  { title: "Splash background", src: "/canva-kit/illustrations/splash-1080x1920.jpg", hint: "Add Syne wordmark in Canva" },
  { title: "Poster lockup", src: "/canva-kit/illustrations/poster-1080x1920.jpg", hint: "Replace AI type with Brand Kit fonts" },
  { title: "Onboarding", src: "/canva-kit/illustrations/onboarding-1080x1350.jpg", hint: "Text in lower fifth" },
  { title: "Night Drive", src: "/canva-kit/scenes/night-drive-720x405.jpg", hint: "Scene card" },
  { title: "Focus", src: "/canva-kit/scenes/focus-720x405.jpg", hint: "Scene card" },
  { title: "Gym Peak", src: "/canva-kit/scenes/gym-peak-720x405.jpg", hint: "Scene card" },
  { title: "Deep Space", src: "/canva-kit/scenes/deep-space-720x405.jpg", hint: "Scene card" },
  { title: "Empty library", src: "/canva-kit/illustrations/empty-library-720.jpg", hint: "Empty state" },
  { title: "Album placeholder", src: "/canva-kit/illustrations/album-placeholder-1024.jpg", hint: "Track row / now playing" },
  { title: "Nine-pillar diagram", src: "/canva-kit/illustrations/nine-pillar-diagram-1024.jpg", hint: "About / protocol" },
  { title: "Glow sprite", src: "/canva-kit/sprites/glow-512.jpg", hint: "Multiply / screen blend" },
];

export const NAV_ICONS = [
  "visualizer", "library", "profile", "scenes", "equalizer", "settings",
  "play", "pause", "stop", "previous", "next", "shuffle",
  "repeat-off", "repeat-all", "repeat-one", "queue", "add-file", "folder",
  "search", "more", "back", "close", "expand", "collapse",
  "fullscreen", "cast", "home", "microphone", "volume", "mini-player",
  "bluetooth", "wired", "battery", "theme", "knob", "toggle",
  "slider", "choose-file", "choose-folder", "status-ready", "status-analyzing", "status-error",
];

export const COMPONENTS = [
  { name: "Primary button", spec: "48 dp / 48 px height, 999 radius, fill primary, text on indigo. Glow only when enabled." },
  { name: "Secondary button", spec: "48 dp glass outline, 1 px hairline, no fill glow." },
  { name: "Destructive", spec: "Danger fill, used only for delete/stop. Never as a default CTA." },
  { name: "Play button", spec: "72 dp circle, 40 dp glyph, optical shift of the triangle 2 px right." },
  { name: "Icon button", spec: "40–48 dp hit target, 24 dp glyph." },
  { name: "Toggle", spec: "52 × 32 dp track, 24 dp thumb. Teal when on." },
  { name: "Slider", spec: "4–6 dp track, 20–24 dp thumb, teal fill." },
  { name: "Knob", spec: "96 dp standard, 128 dp hero. Tick marks at 0/25/50/75/100." },
  { name: "Mini-player", spec: "64–72 dp high, full width, 96 px thumb, title, play, visualizer jump." },
  { name: "Scene pill", spec: "Height 32–36 dp, 999 radius, selected = signal border + teal tick." },
];

export const SCREENS = [
  { name: "Visualizer home", assets: "Orb, waveform, spectrum, transport, now-playing card" },
  { name: "Local library", assets: "Folder, file rows, waveform thumb, search/sort, empty state" },
  { name: "Local file player", assets: "Album placeholder, seek, queue, transport" },
  { name: "VASP profile", assets: "Nine pillar cards, metadata chips, completeness" },
  { name: "Scene browser", assets: "Thumbnails 720×405, selected border, apply" },
  { name: "Audio configuration", assets: "Source cards, gain, FFT labels, latency, reset" },
  { name: "Settings", assets: "Theme cards, motion toggles, storage, about" },
  { name: "About / protocol", assets: "v3.69 badge, nine-pillar diagram" },
];

export const AUDIO_CONFIG = [
  "Audio source: Local file / Demo signal / Microphone (future)",
  "Output: Device / Bluetooth / Wired",
  "Master volume + visual gain / sensitivity",
  "Beat detection sensitivity",
  "Spectrum resolution: Low / Balanced / High",
  "FFT size (internal) shown as simplified labels",
  "Smoothing / decay",
  "Frame-rate: Battery Saver / Balanced / Performance",
  "Background processing toggle",
  "Latency indicator + reset audio configuration",
];

export const SETTINGS_SECTIONS = [
  { name: "Appearance", items: "Light, Dark, System — do not invert; reduce glow and raise borders in light." },
  { name: "Visualizer", items: "Reduced motion, particles, spectrum, waveform, glow, FPS." },
  { name: "Playback", items: "Auto-play next, repeat default, remember last track." },
  { name: "VASP", items: "Advanced metadata, confidence, reset demo profile." },
  { name: "Storage", items: "Library folder, cache size, clear cache." },
  { name: "About", items: "VASP v3.69, app version, licenses, feedback." },
];

export const MAGIC_PROMPTS = [
  {
    title: "App icon",
    size: "Custom 1024 × 1024",
    prompt:
      "Square luxury Android app icon, nine-node orbital mandala, indigo core #4B0082, teal ring #008080, deep void #080A12, generous padding, no letters, no music note, no people.",
  },
  {
    title: "Splash background",
    size: "Instagram Story 1080 × 1920",
    prompt:
      "Tall phone splash, cosmic void, indigo-teal aurora, glowing mandala orb in the upper half, empty dark lower half for text, no letters, no logos, no people.",
  },
  {
    title: "Scene — Night Drive",
    size: "Custom 720 × 405",
    prompt:
      "Cinematic wet night freeway from a car interior, rain, indigo and teal neon on asphalt, no faces, no logos, no text.",
  },
  {
    title: "Scene — Focus",
    size: "Custom 720 × 405",
    prompt:
      "Dark observatory interior, one indigo lamp, teal rim light, empty desk, faint circular glow, no people, no text.",
  },
  {
    title: "Scene — Gym Peak",
    size: "Custom 720 × 405",
    prompt:
      "Dark industrial gym at night, kinetic teal and indigo lighting, steel plates, no people, no brand machines, no text.",
  },
  {
    title: "Scene — Deep Space",
    size: "Custom 720 × 405",
    prompt:
      "Near-black cosmos, indigo nebula, teal stardust, faint nine-node orbital ring, no text, no logos.",
  },
  {
    title: "Empty library",
    size: "Custom 1440 × 1440",
    prompt:
      "Dark glass hexagonal tray, faint indigo orb, geometric teal folder of light, dashed rings, calm empty state, no text.",
  },
  {
    title: "Knob texture",
    size: "Custom 512 × 512",
    prompt:
      "Top-down circular audio knob, dark metal-glass, indigo tick ring, teal indicator, no text, centered, square.",
  },
  {
    title: "Particle — micro star",
    size: "Custom 256 × 256",
    prompt:
      "Single soft four-point micro-star glow, teal-indigo, on pure black, no background objects, no text.",
  },
  {
    title: "Permission explainer",
    size: "Custom 1080 × 1080",
    prompt:
      "Abstract nocturnal illustration of a phone unlocking a local audio folder as light, indigo-teal, no brands, no people faces, no text.",
  },
];

export const BRAND_KIT_STEPS = [
  "Canva → Projects → Brand Kit (or Brand Hub on Business).",
  "Logos: upload app-icon-1024 and the SVG mark-orb.svg as primary and icon.",
  "Colors: paste void, primary, secondary first, then signal, warning, danger, both text colors, and the light-mode set.",
  "Fonts: set Syne (or Outfit) as heading, Lexend as body. If Syne is missing, upload it or use Sora / Outfit.",
  "Graphics: upload the /icons SVG folder and scene JPEGs so Magic Design can reuse them.",
  "Create a Brand template folder named VASP Visualizer with the custom sizes in the size table.",
  "For every Magic Media image with letters, replace the type in Canva using Brand Kit fonts — never ship model lettering.",
];

export const FIRST_BATCH = [
  "App icon + splash background",
  "Nine pillar icons (SVG)",
  "Primary / secondary / icon-button states",
  "Playback control set",
  "Dark and light token sheet",
  "One orb, one waveform skin, one spectrum skin",
  "Glow + particle sprites",
  "Four scene thumbnails",
  "Empty library + album placeholder",
  "Audio config + settings iconography",
];
