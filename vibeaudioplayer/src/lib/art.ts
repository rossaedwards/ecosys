/**
 * Player chrome sourced from C:\rossaedwards\ecosys\vibeaudioplayer\assets\
 * Copied to public/chrome for static serving. Do not invent new menu art.
 */
export const ART = {
  menu1: "/chrome/vap_menu_1.png",
  menu1Tb: "/chrome/vap_menu_1_tb.png",
  menu2: "/chrome/vap_menu_2.png",
  menu2Tb: "/chrome/vap_menu_2_tb.png",
  loading: "/chrome/vibeaudioplayer_loading.png",
  loading2: "/chrome/vibeaudioplayer_loading2.png",
  icon: "/chrome/app-icon-1024.jpg",
  qr: "/chrome/aurphyx-business-card-qr.png",
} as const;

/** Crop slots on vap_menu_*_tb.png (2×3 orb poster). */
export const TAB_SLOTS = {
  orb: { x: "32%", y: "56%" },
  library: { x: "32%", y: "74%" },
  vasp: { x: "32%", y: "38%" },
  scene: { x: "68%", y: "74%" },
  about: { x: "68%", y: "56%" },
} as const;
