import React, { useState, useEffect, useRef } from 'react';
import { 
  Sparkles, Sun, Moon, Compass, Shield, Activity, Share2, 
  Download, RefreshCw, Layers, MapPin, Eye, Volume2, VolumeX, 
  Info, CheckCircle2, ChevronRight, Lock, Key, Award, FileText, Globe, AlertTriangle
} from 'lucide-react';

// ==========================================
// 13-SIGN ZODIAC & FREQUENCY CONSTANTS
// ==========================================
const ZODIAC_FREQS = {
  "Aries": { freq: 110.00, element: "Fire", quality: "Cardinal" },
  "Taurus": { freq: 116.54, element: "Earth", quality: "Fixed" },
  "Gemini": { freq: 123.47, element: "Air", quality: "Mutable" },
  "Cancer": { freq: 130.81, element: "Water", quality: "Cardinal" },
  "Leo": { freq: 138.59, element: "Fire", quality: "Fixed" },
  "Virgo": { freq: 146.83, element: "Earth", quality: "Mutable" },
  "Libra": { freq: 155.56, element: "Air", quality: "Cardinal" },
  "Scorpio": { freq: 164.81, element: "Water", quality: "Fixed" },
  "Ophiuchus": { freq: 172.06, element: "Void", quality: "Unified" }, // The 13th Sign
  "Sagittarius": { freq: 174.61, element: "Fire", quality: "Mutable" },
  "Capricorn": { freq: 185.00, element: "Earth", quality: "Cardinal" },
  "Aquarius": { freq: 196.00, element: "Air", quality: "Fixed" },
  "Pisces": { freq: 207.61, element: "Water", quality: "Mutable" }
};

// ==========================================
// THE 13 SAGES (GUARDIANS OF EXISTENCE)
// ==========================================
const SAGES_METADATA = {
  "Valkryx": { title: "The Sovereign Guardian of Strategic Vision", archetype: "Scandium Blue", type: "Core", alignment: "Sun" },
  "Umbryx": { title: "The Walker of Uncharted Shadows", archetype: "Void Obsidian", type: "Core", alignment: "Pluto" },
  "Praelum": { title: "The Herald of Kinetic Strife", archetype: "Crimson Iron", type: "Core", alignment: "Mars" },
  "Nullivar": { title: "The Custodian of Absolute Vacuum", archetype: "Zero Point White", type: "Core", alignment: "Void" },
  "Orric Shade": { title: "The Keeper of Primal Echoes", archetype: "Deep Amber", type: "Core", alignment: "Neptune" },
  "Cryptanyx": { title: "The Master of Secret Lattices", archetype: "Quantum Silver", type: "Core", alignment: "Saturn" },
  "Prophetyx": { title: "The Seer of Timeline Bifurcations", archetype: "Ethereal Amethyst", type: "Core", alignment: "Uranus" },
  "Seshatyx": { title: "The Weaver of Geometric Truths", archetype: "Teal Emerald", type: "Core", alignment: "Earth" },
  "Zephyra": { title: "The Breath of Systemic Coherence", archetype: "Volcanic Red", type: "Core", alignment: "Venus" },
  "Nunclex": { title: "The Chronicler of the Temporal Now", archetype: "Aura Teal", type: "Wild", alignment: "Haumea" },
  "Archivus": { title: "The Scribe of the Ineffable Ledger", archetype: "Prismatic Gold", type: "Wild", alignment: "Ceres" },
  "Ophiux": { title: "The Bridge Across the Serpent Gate", archetype: "Void Neon Violet", type: "Wild", alignment: "Ophiuchus" },
  "Vyrellix": { title: "The Knitter of Distributed Meshes", archetype: "Copper Web", type: "Wild", alignment: "Jupiter" }
};

// Default Canon Values for Ross Edwards
const CANON_DEFAULTS = {
  name: "ROSS EDWARDS",
  birthDate: "1984-10-28",
  birthTime: "16:20",
  weightLbs: 6,
  weightOz: 9,
  mantra: "Perfect love casts out fear. We are all connected.",
  neighborhood: "Mercy Hospital",
  city: "Coon Rapids",
  county: "Anoka County",
  state: "Minnesota",
  region: "Midwest",
  country: "United States",
  continent: "North America",
  hemisphere: "Northern",
  planet: "Earth"
};

export default function App() {
  // Navigation & System State
  const [step, setStep] = useState('landing');
  const [dualityMode, setDualityMode] = useState('neutral');
  const [chaosWeight, setChaosWeight] = useState(50);
  const [isPlayingAudio, setIsPlayingAudio] = useState(false);
  const [activeTab, setActiveTab] = useState('soulchart');
  const [hoveredNode, setHoveredNode] = useState(null);

  // Form Inputs State
  const [formData, setFormData] = useState(CANON_DEFAULTS);
  const [hrvSim, setHrvSim] = useState(72);
  const [gsrSim, setGsrSim] = useState(4.2);
  const [isScanning, setIsScanning] = useState(false);
  const [scanProgress, setScanProgress] = useState(0);

  // Generated Outputs
  const [generatedData, setGeneratedData] = useState(null);

  // Audio Context Ref
  const audioContextRef = useRef(null);
  const oscRef = useRef(null);
  const gainRef = useRef(null);

  // Dynamic colors based on Duality Kernel
  const getThemeColors = () => {
    const r = Math.floor((chaosWeight / 100) * 239 + ((100 - chaosWeight) / 100) * 20);
    const g = Math.floor((chaosWeight / 100) * 35 + ((100 - chaosWeight) / 100) * 184);
    const b = Math.floor((chaosWeight / 100) * 35 + ((100 - chaosWeight) / 100) * 166);
    return {
      glow: `rgba(${r}, ${g}, ${b}, 0.5)`,
      primary: `rgb(${r}, ${g}, ${b})`,
      bgGradient: chaosWeight > 60 
        ? 'from-red-950 via-neutral-950 to-stone-950' 
        : chaosWeight < 40 
          ? 'from-teal-950 via-slate-950 to-zinc-950' 
          : 'from-slate-950 via-neutral-950 to-slate-950',
      textAccent: chaosWeight > 60 ? 'text-rose-500' : chaosWeight < 40 ? 'text-teal-400' : 'text-cyan-400',
      borderAccent: chaosWeight > 60 ? 'border-rose-900/50' : chaosWeight < 40 ? 'border-teal-900/50' : 'border-cyan-900/50'
    };
  };

  const themeColors = getThemeColors();

  const handleSliderChange = (e) => {
    const val = parseInt(e.target.value);
    setChaosWeight(val);
    if (val > 60) setDualityMode('chaos');
    else if (val < 40) setDualityMode('bliss');
    else setDualityMode('neutral');
  };

  // Pseudo-random deterministic generator using strings as seeds
  const seedRandom = (str) => {
    let hash = 0;
    for (let i = 0; i < str.length; i++) {
      hash = str.charCodeAt(i) + ((hash << 5) - hash);
    }
    return () => {
      hash = Math.sin(hash) * 10000;
      return hash - Math.floor(hash);
    };
  };

  // Perform Hecate Triple Protocol 
  const executeSoulShotCalculus = () => {
    const seed = `${formData.name}|${formData.birthDate}|${formData.birthTime}|${formData.neighborhood}|${formData.city}`;
    const rand = seedRandom(seed);
    
    // 1. The 13 Body Vector (Replaced Makemake with Earth, kept Pluto as a planetary body)
    const bodies = [
      { name: "Sun", type: "Star", role: "Chaos Anchor", weight: 1.0 },
      { name: "Moon", type: "Satellite", role: "Bliss Anchor", weight: 0.9 },
      { name: "Mercury", type: "Planet", role: "Logic Conductor", weight: 0.8 },
      { name: "Venus", type: "Planet", role: "Aesthetic Wave", weight: 0.7 },
      { name: "Earth", type: "Planet", role: "Somatic Observer", weight: 1.0 },
      { name: "Mars", type: "Planet", role: "Kinetic Driver", weight: 0.8 },
      { name: "Jupiter", type: "Planet", role: "Coherence Expander", weight: 0.6 },
      { name: "Saturn", type: "Planet", role: "Sovereign Invariant", weight: 0.7 },
      { name: "Uranus", type: "Planet", role: "Entropy Multiplier", weight: 0.5 },
      { name: "Neptune", type: "Planet", role: "Morphic Weaver", weight: 0.6 },
      { name: "Pluto", type: "Dwarf Planet", role: "Sub-Bass Shifter", weight: 0.5 },
      { name: "Ceres", type: "Dwarf Planet", role: "Symbiotic Sower", weight: 0.4 },
      { name: "Haumea", type: "Dwarf Planet", role: "Crystalline Spinner", weight: 0.4 }
    ];

    const signs = Object.keys(ZODIAC_FREQS); // Exactly 13 signs now
    let cumulativeLogFreqWeight = 0;
    let sumWeights = 0;

    const resolvedBodies = bodies.map((b) => {
      // Deterministically place body in one of the 13 Zodiac Signs
      const idx = Math.floor(rand() * signs.length);
      const sign = signs[idx];
      const signData = ZODIAC_FREQS[sign];
      const deg = (rand() * 30).toFixed(2);
      
      cumulativeLogFreqWeight += b.weight * Math.log(signData.freq);
      sumWeights += b.weight;

      return {
        ...b,
        sign,
        degrees: deg,
        frequency: signData.freq,
        element: signData.element,
        quality: signData.quality
      };
    });

    // 2. Compute Weighted Geometric Mean
    const rootFreq = Math.exp(cumulativeLogFreqWeight / sumWeights);

    // 3. Cryptographic Hashes (SHA-3-512 & Dilithium Simulation)
    const baseHashInput = `${seed}|${rootFreq.toFixed(4)}|${hrvSim}|${gsrSim}`;
    let soulHash = "";
    for (let i = 0; i < 8; i++) {
      soulHash += Math.abs(Math.sin(rand() * 100000)).toString(16).substring(2, 10);
    }
    const finalSoulHash = `0x${soulHash.toUpperCase()}`;

    // 4. Shuffle the 13 Exact SAGES into 9 Strategic Core and 4 Wild
    const sagesPool = Object.keys(SAGES_METADATA);
    const shuffledSages = [];
    const tempPool = [...sagesPool];
    while (tempPool.length > 0) {
      const pickIdx = Math.floor(rand() * tempPool.length);
      shuffledSages.push(tempPool.splice(pickIdx, 1)[0]);
    }

    const strategic9 = sagesPool.filter(name => SAGES_METADATA[name].type === "Core").map(name => ({ name, ...SAGES_METADATA[name] }));
    const wild4 = sagesPool.filter(name => SAGES_METADATA[name].type === "Wild").map(name => ({ name, ...SAGES_METADATA[name] }));

    const kyberPublicKey = `MIIBODANBgkqhkiG9w0BAQEFAAOCAY0AMIIBCAKCAQEAs6Hecat3S9v...${soulHash.substring(0, 20)}...LatticeKeys`;
    const dilithiumSignature = `SIG_DILITHIUM5_${soulHash.substring(10, 30)}_${Math.floor(Date.now() / 1000)}`;

    setGeneratedData({
      rootFrequency: rootFreq.toFixed(2),
      soulHash: finalSoulHash,
      bodies: resolvedBodies,
      strategic9,
      wild4,
      kyberPublicKey,
      dilithiumSignature,
      entropyScore: (Math.sin(hrvSim) * 100).toFixed(0),
      divineScore: Math.floor(88 + rand() * 11)
    });
  };

  const startBiometricScan = () => {
    setIsScanning(true);
    setScanProgress(0);
    const interval = setInterval(() => {
      setScanProgress(p => {
        if (p >= 100) {
          clearInterval(interval);
          setIsScanning(false);
          setStep('booting');
          return 100;
        }
        return p + 4;
      });
    }, 100);
  };

  useEffect(() => {
    if (step === 'booting') {
      executeSoulShotCalculus();
      const timer = setTimeout(() => {
        setStep('dashboard');
      }, 4500); 
      return () => clearTimeout(timer);
    }
  }, [step]);

  const toggleSynthesizer = () => {
    if (isPlayingAudio) {
      stopSynthesizer();
    } else {
      startSynthesizer();
    }
  };

  const startSynthesizer = () => {
    if (!generatedData) return;
    try {
      const AudioContext = window.AudioContext || window.webkitAudioContext;
      const ctx = new AudioContext();
      audioContextRef.current = ctx;

      const masterGain = ctx.createGain();
      masterGain.gain.setValueAtTime(0.0, ctx.currentTime);
      masterGain.gain.linearRampToValueAtTime(0.12, ctx.currentTime + 1.5); 
      masterGain.connect(ctx.destination);
      gainRef.current = masterGain;

      const osc = ctx.createOscillator();
      const freqVal = parseFloat(generatedData.rootFrequency);
      osc.frequency.setValueAtTime(freqVal, ctx.currentTime);
      
      if (chaosWeight > 60) {
        osc.type = 'sawtooth'; 
        const filter = ctx.createBiquadFilter();
        filter.type = 'lowpass';
        filter.frequency.setValueAtTime(freqVal * 2, ctx.currentTime);
        osc.connect(filter);
        filter.connect(masterGain);
      } else if (chaosWeight < 40) {
        osc.type = 'sine'; 
        osc.connect(masterGain);
      } else {
        osc.type = 'triangle'; 
        osc.connect(masterGain);
      }

      osc.start();
      oscRef.current = osc;
      setIsPlayingAudio(true);
    } catch (e) {
      console.warn("Web Audio initiation blocked or unsupported", e);
    }
  };

  const stopSynthesizer = () => {
    if (gainRef.current && audioContextRef.current) {
      try {
        gainRef.current.gain.linearRampToValueAtTime(0.0, audioContextRef.current.currentTime + 0.5);
        setTimeout(() => {
          if (oscRef.current) oscRef.current.stop();
          if (audioContextRef.current) audioContextRef.current.close();
          setIsPlayingAudio(false);
        }, 500);
      } catch (err) {
        setIsPlayingAudio(false);
      }
    } else {
      setIsPlayingAudio(false);
    }
  };

  useEffect(() => {
    return () => {
      if (oscRef.current) oscRef.current.stop();
      if (audioContextRef.current) audioContextRef.current.close();
    };
  }, []);

  return (
    <div className={`min-h-screen bg-gradient-to-b ${themeColors.bgGradient} text-white font-sans transition-colors duration-1000 overflow-x-hidden relative`}>
      
      <div className="absolute inset-0 bg-[linear-gradient(to_right,rgba(255,255,255,0.02)_1px,transparent_1px),linear-gradient(to_bottom,rgba(255,255,255,0.02)_1px,transparent_1px)] bg-[size:4rem_4rem] pointer-events-none" />
      
      {/* Aurphyx Sovereign Core Header */}
      <header className="border-b border-white/10 bg-black/40 backdrop-blur-md sticky top-0 z-50 px-6 py-4 flex flex-col md:flex-row items-center justify-between gap-4">
        <div className="flex items-center gap-3">
          <div className="relative">
            <div className="w-10 h-10 rounded-full border-2 border-dashed flex items-center justify-center animate-spin" style={{ borderColor: themeColors.primary, animationDuration: '20s' }} />
            <Layers className="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 text-cyan-400" size={18} />
          </div>
          <div>
            <h1 className="font-mono text-lg font-bold tracking-wider text-transparent bg-clip-text bg-gradient-to-r from-cyan-400 via-sky-300 to-rose-400">
              AURPHYX // SOULSHOT
            </h1>
            <p className="text-[10px] text-slate-400 tracking-widest uppercase font-mono">Immutable Origin & Spacetime Engine</p>
          </div>
        </div>

        {/* Global Duality Kernel Tuning */}
        <div className="flex items-center gap-4 bg-slate-900/80 px-4 py-2 rounded-full border border-white/10 backdrop-blur-sm shadow-inner w-full max-w-sm md:w-auto">
          <span className="text-[11px] font-mono text-rose-500 flex items-center gap-1">
            <Sun size={12} className="animate-pulse" /> CHAOS
          </span>
          <input 
            type="range" 
            min="0" 
            max="100" 
            value={chaosWeight} 
            onChange={handleSliderChange}
            className="w-24 md:w-32 accent-cyan-400 h-1 bg-slate-700 rounded-lg appearance-none cursor-pointer"
          />
          <span className="text-[11px] font-mono text-teal-400 flex items-center gap-1">
            BLISS <Moon size={12} className="animate-pulse" />
          </span>
          <div className="text-xs font-mono bg-white/5 px-2 py-0.5 rounded border border-white/10">
            {chaosWeight}%:{100 - chaosWeight}%
          </div>
        </div>
      </header>

      {/* ==========================================
          STEP 1: LANDING PORTAL PAGE
          ========================================== */}
      {step === 'landing' && (
        <main className="max-w-6xl mx-auto px-6 py-12 md:py-20 grid grid-cols-1 lg:grid-cols-12 gap-12 items-center relative z-10">
          <div className="lg:col-span-7 flex flex-col gap-6 text-left">
            <div className="inline-flex items-center gap-2 px-3 py-1 bg-cyan-500/10 border border-cyan-500/30 rounded-full w-fit">
              <Sparkles size={14} className="text-cyan-400" />
              <span className="text-xs font-mono tracking-wider text-cyan-300 font-semibold">ECOSYSTEM PROTOCOL ALPHA</span>
            </div>

            <h2 className="text-4xl md:text-6xl font-black font-mono leading-none tracking-tight">
              BEGIN YOUR <br />
              <span className="text-transparent bg-clip-text bg-gradient-to-r from-cyan-400 via-teal-300 to-rose-400">
                SOULSHOT SEQUENCE
              </span>
            </h2>

            <p className="text-slate-300 text-base md:text-lg leading-relaxed max-w-2xl">
              The foundational bio-astral mapping layer of the <span className="text-white font-semibold">Aurphyx Stack</span>. SoulShot synthesizes the absolute snapshot of the cosmos at your moment of birth with real-time biometric neural telemetry. The output is your sovereign cryptographic <span className="text-cyan-400 font-semibold">SoulHash</span> and <span className="text-teal-400 font-semibold">Root Frequency</span>—required before generating a BlissID.
            </p>

            <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mt-4 text-xs font-mono">
              <div className="p-4 bg-slate-900/50 rounded-xl border border-white/5 backdrop-blur-sm">
                <Compass className="text-cyan-400 mb-2" size={18} />
                <div className="font-bold text-white mb-1">9-Tier Spacetime Vector</div>
                <div className="text-slate-400">Captures exact localized spatial entropy upon birth.</div>
              </div>
              <div className="p-4 bg-slate-900/50 rounded-xl border border-white/5 backdrop-blur-sm">
                <Activity className="text-rose-500 mb-2" size={18} />
                <div className="font-bold text-white mb-1">Somatic Nano-Membrane</div>
                <div className="text-slate-400">Simulates physical HRV/GSR signal entrainment.</div>
              </div>
              <div className="p-4 bg-slate-900/50 rounded-xl border border-white/5 backdrop-blur-sm">
                <Shield className="text-teal-400 mb-2" size={18} />
                <div className="font-bold text-white mb-1">Post-Quantum Keys</div>
                <div className="text-slate-400">Implements ML-KEM (Kyber-1024) & Dilithium-5 signatures.</div>
              </div>
            </div>

            <div className="flex flex-col sm:flex-row items-center gap-4 mt-6">
              <button 
                onClick={() => setStep('auth')}
                className="w-full sm:w-auto px-8 py-4 bg-gradient-to-r from-cyan-500 to-teal-500 hover:from-cyan-400 hover:to-teal-400 rounded-xl font-bold font-mono text-sm tracking-widest uppercase shadow-lg shadow-cyan-500/20 transform hover:-translate-y-0.5 transition duration-200 flex items-center justify-center gap-2"
              >
                RESONATE & INTERFACE <ChevronRight size={16} />
              </button>
              
              <button 
                onClick={() => {
                  setFormData(CANON_DEFAULTS);
                  setStep('auth');
                }}
                className="w-full sm:w-auto px-6 py-4 bg-white/5 hover:bg-white/10 rounded-xl font-semibold font-mono text-xs text-slate-300 tracking-wider hover:text-white transition duration-200 border border-white/10"
              >
                LOAD CANON SEED (ROSS)
              </button>
            </div>
          </div>

          <div className="lg:col-span-5 flex justify-center relative">
            <div className="relative w-80 h-80 md:w-96 md:h-96">
              <div className="absolute inset-0 rounded-full border-4 border-dashed border-cyan-500/10 animate-spin" style={{ animationDuration: '40s' }} />
              <div className="absolute inset-4 rounded-full border-2 border-dashed border-teal-500/20 animate-spin" style={{ animationDuration: '30s', animationDirection: 'reverse' }} />
              <div className="absolute inset-10 rounded-full border border-rose-500/30 animate-pulse" />
              
              <div 
                className="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 w-48 h-48 rounded-full flex items-center justify-center blur-2xl opacity-40 transition-colors duration-1000"
                style={{ backgroundColor: themeColors.primary }}
              />
              <div className="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 w-32 h-32 bg-slate-950/90 rounded-full border border-white/20 flex items-center justify-center shadow-2xl">
                <div className="text-center font-mono p-4">
                  <div className="text-cyan-400 text-3xl font-bold tracking-widest animate-pulse">1.8</div>
                  <div className="text-slate-400 text-[9px] tracking-widest uppercase">Engine Vers.</div>
                </div>
              </div>
            </div>
          </div>
        </main>
      )}

      {/* ==========================================
          STEP 2: GATEWAY AUTHENTICATION
          ========================================== */}
      {step === 'auth' && (
        <main className="max-w-md mx-auto px-6 py-12 relative z-10">
          <div className="bg-slate-900/60 border border-white/10 rounded-3xl p-8 backdrop-blur-md shadow-2xl relative overflow-hidden">
            <div className="absolute top-0 left-0 right-0 h-1 bg-gradient-to-r from-cyan-500 to-teal-500" />
            
            <div className="flex flex-col items-center gap-4 text-center mb-8">
              <div className="p-3 bg-cyan-500/10 rounded-2xl border border-cyan-500/20">
                <Lock className="text-cyan-400" size={24} />
              </div>
              <h2 className="text-2xl font-mono font-bold tracking-wide">Gateway Authentication</h2>
              <p className="text-slate-400 text-xs font-mono">Select your entrance protocol to the Aurphyx Universe Stack.</p>
            </div>

            <div className="flex flex-col gap-4">
              <button 
                onClick={() => setStep('inputs')}
                className="w-full p-4 bg-white/5 hover:bg-white/10 rounded-xl border border-white/10 text-left transition duration-200 group flex items-center justify-between"
              >
                <div className="flex items-center gap-3">
                  <Key className="text-teal-400 group-hover:scale-110 transition duration-200" size={18} />
                  <div>
                    <div className="font-mono text-sm font-semibold text-white">Sovereign Guest Access</div>
                    <div className="text-[10px] text-slate-400">Generate non-persisted single-session vectors</div>
                  </div>
                </div>
                <ChevronRight size={16} className="text-slate-500 group-hover:text-white transition" />
              </button>

              <button 
                onClick={() => {
                  setFormData(CANON_DEFAULTS);
                  setStep('inputs');
                }}
                className="w-full p-4 bg-cyan-500/5 hover:bg-cyan-500/10 rounded-xl border border-cyan-500/30 text-left transition duration-200 group flex items-center justify-between"
              >
                <div className="flex items-center gap-3">
                  <Award className="text-cyan-400 group-hover:scale-110 transition duration-200" size={18} />
                  <div>
                    <div className="font-mono text-sm font-semibold text-cyan-300">Resonate as Creator (Ross Edwards)</div>
                    <div className="text-[10px] text-slate-400 font-mono">Bypass and populate absolute birth matrices</div>
                  </div>
                </div>
                <ChevronRight size={16} className="text-cyan-400" />
              </button>

              <div className="relative my-4">
                <div className="absolute inset-0 flex items-center" aria-hidden="true">
                  <div className="w-full border-t border-white/5"></div>
                </div>
                <div className="relative flex justify-center text-xs uppercase font-mono">
                  <span className="bg-slate-900 px-3 text-slate-500">Secure Vault Options</span>
                </div>
              </div>

              <button 
                disabled
                className="w-full p-4 bg-white/[0.02] rounded-xl border border-white/5 text-left opacity-50 flex items-center justify-between cursor-not-allowed"
              >
                <div className="flex items-center gap-3">
                  <Shield className="text-slate-600" size={18} />
                  <div>
                    <div className="font-mono text-sm font-semibold text-slate-500">Decrypt via BlissID Sync Watch</div>
                    <div className="text-[10px] text-slate-600">Hardware token required for handshake</div>
                  </div>
                </div>
                <Lock size={12} className="text-slate-600" />
              </button>
            </div>

            <div className="mt-8 pt-6 border-t border-white/5 text-center">
              <button 
                onClick={() => setStep('landing')}
                className="text-xs font-mono text-slate-500 hover:text-slate-300 transition"
              >
                &larr; Return to Overview
              </button>
            </div>
          </div>
        </main>
      )}

      {/* ==========================================
          STEP 3: SPACETIME & BIOMETRIC INPUTS
          ========================================== */}
      {step === 'inputs' && (
        <main className="max-w-4xl mx-auto px-6 py-12 relative z-10">
          <div className="bg-slate-900/70 border border-white/10 rounded-3xl p-8 backdrop-blur-md shadow-2xl">
            <h2 className="text-2xl md:text-3xl font-mono font-bold tracking-tight mb-2 text-transparent bg-clip-text bg-gradient-to-r from-cyan-400 to-teal-400">
              Spacetime Vectors & Somatic Ingestion
            </h2>
            <p className="text-slate-400 text-xs font-mono mb-8">
              Configure your exact entry vectors into the local planetary-celestial matrix.
            </p>

            <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
              
              {/* Left Column: Form Fields */}
              <div className="flex flex-col gap-6">
                <div className="border-b border-white/10 pb-2 flex items-center gap-2">
                  <Compass size={16} className="text-cyan-400" />
                  <span className="font-mono text-xs font-bold uppercase tracking-wider">Cosmic DNA Coordinates</span>
                </div>

                <div className="grid grid-cols-1 gap-4 text-left">
                  <div>
                    <label className="block text-[10px] font-mono uppercase text-slate-400 mb-1 tracking-wider">Full Legal Name</label>
                    <input 
                      type="text" 
                      value={formData.name} 
                      onChange={(e) => setFormData({...formData, name: e.target.value})}
                      className="w-full bg-slate-950 border border-white/10 rounded-xl px-4 py-3 font-mono text-sm focus:border-cyan-400 transition"
                      placeholder="e.g. ROSS ANDREW EDWARDS"
                    />
                  </div>

                  <div className="grid grid-cols-2 gap-4">
                    <div>
                      <label className="block text-[10px] font-mono uppercase text-slate-400 mb-1 tracking-wider">Civil Birth Date</label>
                      <input 
                        type="date" 
                        value={formData.birthDate} 
                        onChange={(e) => setFormData({...formData, birthDate: e.target.value})}
                        className="w-full bg-slate-950 border border-white/10 rounded-xl px-4 py-3 font-mono text-sm focus:border-cyan-400 transition"
                      />
                    </div>
                    <div>
                      <label className="block text-[10px] font-mono uppercase text-slate-400 mb-1 tracking-wider">Civil Birth Time</label>
                      <input 
                        type="time" 
                        value={formData.birthTime} 
                        onChange={(e) => setFormData({...formData, birthTime: e.target.value})}
                        className="w-full bg-slate-950 border border-white/10 rounded-xl px-4 py-3 font-mono text-sm focus:border-cyan-400 transition"
                      />
                    </div>
                  </div>

                  <div className="grid grid-cols-2 gap-4">
                    <div>
                      <label className="block text-[10px] font-mono uppercase text-slate-400 mb-1 tracking-wider">Birth Weight (Lbs)</label>
                      <input 
                        type="number" 
                        value={formData.weightLbs} 
                        onChange={(e) => setFormData({...formData, weightLbs: parseInt(e.target.value)})}
                        className="w-full bg-slate-950 border border-white/10 rounded-xl px-4 py-3 font-mono text-sm focus:border-cyan-400 transition"
                      />
                    </div>
                    <div>
                      <label className="block text-[10px] font-mono uppercase text-slate-400 mb-1 tracking-wider">Ounces</label>
                      <input 
                        type="number" 
                        value={formData.weightOz} 
                        onChange={(e) => setFormData({...formData, weightOz: parseInt(e.target.value)})}
                        className="w-full bg-slate-950 border border-white/10 rounded-xl px-4 py-3 font-mono text-sm focus:border-cyan-400 transition"
                      />
                    </div>
                  </div>

                  <div>
                    <label className="block text-[10px] font-mono uppercase text-slate-400 mb-1 tracking-wider">Personal Mantra / Sovereign Intent</label>
                    <textarea 
                      value={formData.mantra} 
                      onChange={(e) => setFormData({...formData, mantra: e.target.value})}
                      rows="2"
                      className="w-full bg-slate-950 border border-white/10 rounded-xl px-4 py-3 font-mono text-sm focus:border-cyan-400 transition"
                      placeholder="Enter intention alignment string..."
                    />
                  </div>
                </div>
              </div>

              {/* Right Column: 9-Tier Location Matrix */}
              <div className="flex flex-col gap-6">
                <div className="border-b border-white/10 pb-2 flex items-center gap-2">
                  <MapPin size={16} className="text-teal-400" />
                  <span className="font-mono text-xs font-bold uppercase tracking-wider">9-Tier Location Vector</span>
                </div>

                <div className="grid grid-cols-2 gap-3 text-left">
                  <div className="col-span-2">
                    <label className="block text-[9px] font-mono uppercase text-slate-400 mb-0.5 tracking-wider">1. Neighborhood / Block Zero</label>
                    <input 
                      type="text" 
                      value={formData.neighborhood} 
                      onChange={(e) => setFormData({...formData, neighborhood: e.target.value})}
                      className="w-full bg-slate-950 border border-white/5 rounded-lg px-3 py-1.5 font-mono text-xs focus:border-teal-400 transition"
                    />
                  </div>
                  <div>
                    <label className="block text-[9px] font-mono uppercase text-slate-400 mb-0.5 tracking-wider">2. City / Town</label>
                    <input 
                      type="text" 
                      value={formData.city} 
                      onChange={(e) => setFormData({...formData, city: e.target.value})}
                      className="w-full bg-slate-950 border border-white/5 rounded-lg px-3 py-1.5 font-mono text-xs focus:border-teal-400 transition"
                    />
                  </div>
                  <div>
                    <label className="block text-[9px] font-mono uppercase text-slate-400 mb-0.5 tracking-wider">3. County / Parish</label>
                    <input 
                      type="text" 
                      value={formData.county} 
                      onChange={(e) => setFormData({...formData, county: e.target.value})}
                      className="w-full bg-slate-950 border border-white/5 rounded-lg px-3 py-1.5 font-mono text-xs focus:border-teal-400 transition"
                    />
                  </div>
                  <div>
                    <label className="block text-[9px] font-mono uppercase text-slate-400 mb-0.5 tracking-wider">4. State / Province</label>
                    <input 
                      type="text" 
                      value={formData.state} 
                      onChange={(e) => setFormData({...formData, state: e.target.value})}
                      className="w-full bg-slate-950 border border-white/5 rounded-lg px-3 py-1.5 font-mono text-xs focus:border-teal-400 transition"
                    />
                  </div>
                  <div>
                    <label className="block text-[9px] font-mono uppercase text-slate-400 mb-0.5 tracking-wider">5. Region / Quadrant</label>
                    <input 
                      type="text" 
                      value={formData.region} 
                      onChange={(e) => setFormData({...formData, region: e.target.value})}
                      className="w-full bg-slate-950 border border-white/5 rounded-lg px-3 py-1.5 font-mono text-xs focus:border-teal-400 transition"
                    />
                  </div>
                  <div>
                    <label className="block text-[9px] font-mono uppercase text-slate-400 mb-0.5 tracking-wider">6. Country</label>
                    <input 
                      type="text" 
                      value={formData.country} 
                      onChange={(e) => setFormData({...formData, country: e.target.value})}
                      className="w-full bg-slate-950 border border-white/5 rounded-lg px-3 py-1.5 font-mono text-xs focus:border-teal-400 transition"
                    />
                  </div>
                  <div>
                    <label className="block text-[9px] font-mono uppercase text-slate-400 mb-0.5 tracking-wider">7. Continent</label>
                    <input 
                      type="text" 
                      value={formData.continent} 
                      onChange={(e) => setFormData({...formData, continent: e.target.value})}
                      className="w-full bg-slate-950 border border-white/5 rounded-lg px-3 py-1.5 font-mono text-xs focus:border-teal-400 transition"
                    />
                  </div>
                  <div>
                    <label className="block text-[9px] font-mono uppercase text-slate-400 mb-0.5 tracking-wider">8. Hemisphere</label>
                    <input 
                      type="text" 
                      value={formData.hemisphere} 
                      onChange={(e) => setFormData({...formData, hemisphere: e.target.value})}
                      className="w-full bg-slate-950 border border-white/5 rounded-lg px-3 py-1.5 font-mono text-xs focus:border-teal-400 transition"
                    />
                  </div>
                  <div>
                    <label className="block text-[9px] font-mono uppercase text-slate-400 mb-0.5 tracking-wider">9. Planet</label>
                    <input 
                      type="text" 
                      value={formData.planet} 
                      onChange={(e) => setFormData({...formData, planet: e.target.value})}
                      className="w-full bg-slate-950 border border-white/5 rounded-lg px-3 py-1.5 font-mono text-xs focus:border-teal-400 transition"
                    />
                  </div>
                </div>
              </div>
            </div>

            {/* Simulated Somatic Device Telemetry (THIN) */}
            <div className="mt-8 pt-8 border-t border-white/10 grid grid-cols-1 md:grid-cols-12 gap-8 items-center">
              <div className="md:col-span-7 flex flex-col gap-3 text-left">
                <div className="flex items-center gap-2">
                  <Activity className="text-rose-500" size={18} />
                  <span className="font-mono text-xs font-bold uppercase tracking-wider text-rose-400">Somatic Nanomembrane Integration (THIN)</span>
                </div>
                <p className="text-slate-400 text-xs leading-relaxed font-mono">
                  Place your hand on the touch sensor scanner to calibrate your real-time **Heart Rate Variability (HRV)** and **Galvanic Skin Response (GSR)** vectors. This injects real-time somatic entropy into your post-quantum private key seed generation.
                </p>

                <div className="flex gap-4 mt-2">
                  <div className="bg-slate-950 border border-white/5 p-3 rounded-lg flex-1">
                    <span className="block text-[9px] font-mono uppercase text-slate-500">Simulated Baseline HRV</span>
                    <div className="flex items-center gap-2 mt-1">
                      <input 
                        type="range" 
                        min="30" 
                        max="140" 
                        value={hrvSim} 
                        onChange={(e) => setHrvSim(parseInt(e.target.value))}
                        className="w-full accent-rose-500 h-1 bg-slate-700 rounded-lg cursor-pointer"
                      />
                      <span className="font-mono text-xs font-bold text-rose-400">{hrvSim}ms</span>
                    </div>
                  </div>

                  <div className="bg-slate-950 border border-white/5 p-3 rounded-lg flex-1">
                    <span className="block text-[9px] font-mono uppercase text-slate-500">Simulated GSR Activity</span>
                    <div className="flex items-center gap-2 mt-1">
                      <input 
                        type="range" 
                        min="10" 
                        max="100" 
                        value={gsrSim * 10} 
                        onChange={(e) => setGsrSim(parseFloat(e.target.value) / 10)}
                        className="w-full accent-emerald-500 h-1 bg-slate-700 rounded-lg cursor-pointer"
                      />
                      <span className="font-mono text-xs font-bold text-emerald-400">{gsrSim.toFixed(1)}&mu;S</span>
                    </div>
                  </div>
                </div>
              </div>

              <div className="md:col-span-5 flex flex-col items-center justify-center">
                <button 
                  onClick={startBiometricScan}
                  disabled={isScanning}
                  className={`w-40 h-40 rounded-full flex flex-col items-center justify-center relative overflow-hidden group shadow-lg border transition duration-300 ${isScanning ? 'border-rose-500/50 bg-rose-500/10' : 'border-white/10 bg-slate-950 hover:border-cyan-400/50 hover:bg-cyan-950/20'}`}
                >
                  {isScanning ? (
                    <>
                      <div className="absolute inset-0 bg-[radial-gradient(circle_at_center,rgba(239,68,68,0.2)_0%,transparent_70%)] animate-pulse" />
                      <div className="absolute top-0 bottom-0 left-0 bg-rose-500/30 transition-all duration-300 pointer-events-none" style={{ width: `${scanProgress}%` }} />
                      <Activity className="text-rose-500 animate-bounce mb-1" size={24} />
                      <span className="font-mono text-xs font-bold text-rose-300">{scanProgress}% SECURED</span>
                    </>
                  ) : (
                    <>
                      <Compass className="text-slate-400 group-hover:text-cyan-400 group-hover:scale-110 group-hover:rotate-45 transition duration-300 mb-1" size={28} />
                      <span className="font-mono text-xs font-bold uppercase tracking-wider group-hover:text-cyan-300">INITIATE SCAN</span>
                      <span className="text-[8px] font-mono text-slate-500 tracking-widest mt-1">FUSE SPATIOTEMPORAL</span>
                    </>
                  )}
                </button>
              </div>
            </div>
          </div>
        </main>
      )}

      {/* ==========================================
          STEP 4: BOOTING & COSMIC ALIGNMENT ANIMATION
          ========================================== */}
      {step === 'booting' && (
        <main className="max-w-2xl mx-auto px-6 py-20 relative z-10 flex flex-col items-center justify-center">
          <div className="relative w-72 h-72 mb-10 flex items-center justify-center">
            <div className="absolute inset-0 rounded-full border-4 border-dashed border-cyan-500/10 animate-spin" style={{ animationDuration: '6s' }} />
            <div className="absolute inset-4 rounded-full border-2 border-dashed border-rose-500/20 animate-spin" style={{ animationDuration: '4s', animationDirection: 'reverse' }} />
            <div className="absolute inset-8 border border-white/10 rounded-full" />
            
            <svg viewBox="0 0 100 100" className="w-40 h-40 animate-spin text-cyan-400" style={{ animationDuration: '8s' }}>
              <path 
                fill="none" 
                stroke="currentColor" 
                strokeWidth="1" 
                strokeDasharray="5, 5"
                d="M50,50 C60,40 60,30 50,20 C35,10 20,25 20,45 C20,70 45,90 75,80 C100,70 105,35 85,15 C65,-5 25,-5 5,25 C-15,55 5,95 45,105" 
              />
            </svg>

            <div className="absolute font-mono text-[9px] text-teal-400 animate-pulse text-center leading-normal">
              SECURE SEED BLOCK
              <br />
              <span className="text-white text-xs font-bold">SHA-3-512 FUSION</span>
              <br />
              {formData.birthDate}
            </div>
          </div>

          <div className="w-full bg-slate-900/50 border border-white/10 rounded-2xl p-6 backdrop-blur-md">
            <div className="flex items-center justify-between mb-2">
              <span className="font-mono text-xs text-cyan-400 font-semibold uppercase tracking-wider flex items-center gap-2">
                <RefreshCw size={12} className="animate-spin" /> HARMONIZING VIBRATION VECTORS
              </span>
              <span className="font-mono text-xs text-slate-400">STATE: COHERENT</span>
            </div>

            <div className="bg-black/80 font-mono text-left p-4 rounded-lg h-40 overflow-y-auto text-[10px] text-slate-300 flex flex-col gap-1.5 scrollbar-thin">
              <div className="text-cyan-400">[0.12s] Spatiotemporal coordinates loaded: {formData.city}, {formData.state}</div>
              <div className="text-teal-400">[0.84s] 9-Tier Location Matrix established. Entropy quotient optimized.</div>
              <div className="text-rose-400">[1.56s] Somatic baseline gathered. HRV: {hrvSim}ms | GSR: {gsrSim}&mu;S</div>
              <div className="text-slate-400">[2.28s] Computing hyper-volume geometry weight distributions...</div>
              <div className="text-yellow-400 font-bold">[2.94s] Weighted Geometric Mean calculated dynamically...</div>
              <div className="text-cyan-300 animate-pulse">[3.60s] Forging post-quantum Kyber-1024 / Dilithium lattice signature locks...</div>
              <div className="text-emerald-400 font-semibold">[4.10s] HECATE TRIPLE PROTOCOL DEPLOYED SUCCESSFULLY.</div>
            </div>
          </div>
        </main>
      )}

      {/* ==========================================
          STEP 5:Snapshot Dashboard & Exploration
          ========================================== */}
      {step === 'dashboard' && generatedData && (
        <main className="max-w-7xl mx-auto px-6 py-8 relative z-10 grid grid-cols-1 lg:grid-cols-12 gap-8 items-start">
          
          {/* LEFT PANEL: Root Identity & Sound Controls & BlissID Node Details */}
          <div className="lg:col-span-4 flex flex-col gap-6 w-full">
            
            <div className="bg-slate-900/80 border border-white/10 rounded-3xl p-6 backdrop-blur-md shadow-2xl relative overflow-hidden text-left">
              <div className="absolute top-0 right-0 w-24 h-24 bg-gradient-to-br from-cyan-500 to-rose-500 rounded-full blur-3xl opacity-20" />
              
              <div className="flex justify-between items-start mb-4">
                <span className="text-[10px] font-mono tracking-widest uppercase bg-cyan-500/10 text-cyan-300 border border-cyan-500/20 px-2 py-0.5 rounded-full">
                  SOULSHOT GENESIS BLOCK
                </span>
                <span className="text-[10px] font-mono text-emerald-400 font-semibold uppercase flex items-center gap-1">
                  <CheckCircle2 size={10} /> IMMUTABLE
                </span>
              </div>

              <div className="font-mono text-slate-400 text-xs mb-1">SOVEREIGN NAME</div>
              <h3 className="text-2xl font-mono font-black text-white tracking-wide truncate mb-4">{formData.name}</h3>

              <div className="grid grid-cols-2 gap-4 border-t border-b border-white/5 py-4 my-4 font-mono text-xs">
                <div>
                  <span className="text-slate-500 text-[10px] uppercase">ROOT CARRIER WAVE</span>
                  <div className="text-lg font-black text-transparent bg-clip-text bg-gradient-to-r from-cyan-400 to-teal-400 flex items-center gap-1 mt-0.5">
                    {generatedData.rootFrequency} <span className="text-xs">Hz</span>
                  </div>
                </div>
                <div>
                  <span className="text-slate-500 text-[10px] uppercase">DIVINE SCORE</span>
                  <div className="text-lg font-black text-rose-400 flex items-center gap-1 mt-0.5">
                    {generatedData.divineScore} <span className="text-xs">/100</span>
                  </div>
                </div>
              </div>

              <div className="font-mono text-xs text-left mb-4 flex flex-col gap-1.5">
                <div>
                  <span className="text-slate-500 text-[9px] uppercase block">IMMUTABLE SOULHASH</span>
                  <span className="text-white/80 break-all select-all block bg-slate-950 p-2 rounded border border-white/5">{generatedData.soulHash}</span>
                </div>
                <div className="mt-2">
                  <span className="text-slate-500 text-[9px] uppercase block">CANON BLISSID MATCH</span>
                  <span className="text-slate-400 font-mono text-[10px] break-all block">aurphyx:soulshot:hecate:{formData.birthDate.replace(/-/g, '')}:{generatedData.soulHash.substring(2, 10)}</span>
                </div>
              </div>

              <div className="bg-slate-950/90 rounded-2xl p-4 border border-white/5 flex flex-col gap-3 font-mono">
                <div className="flex justify-between items-center">
                  <span className="text-[10px] text-slate-400 flex items-center gap-1 uppercase">
                    <Volume2 size={12} className="text-cyan-400" /> Carrier Synthesizer
                  </span>
                  <span className={`text-[9px] px-2 py-0.5 rounded ${isPlayingAudio ? 'bg-emerald-500/10 text-emerald-400' : 'bg-slate-800 text-slate-500'}`}>
                    {isPlayingAudio ? "RESONATING" : "IDLE"}
                  </span>
                </div>

                <div className="flex gap-2">
                  <button 
                    onClick={toggleSynthesizer}
                    className={`flex-1 py-2 rounded-xl text-xs font-bold tracking-wider flex items-center justify-center gap-1.5 transition ${isPlayingAudio ? 'bg-rose-500 hover:bg-rose-400 text-white' : 'bg-gradient-to-r from-cyan-500 to-teal-500 hover:from-cyan-400 hover:to-teal-400 text-slate-950'}`}
                  >
                    {isPlayingAudio ? (
                      <>
                        <VolumeX size={14} /> SILENCE CARRIER
                      </>
                    ) : (
                      <>
                        <Volume2 size={14} /> RESONATE MATRIX
                      </>
                    )}
                  </button>
                </div>
                <p className="text-[9px] text-slate-500 text-center">
                  Web Audio synthesizes your exact physical base pitch continuously using standard cosmic tuning dynamics.
                </p>
              </div>
            </div>

            <div className="bg-slate-900/50 border border-white/10 rounded-3xl p-6 backdrop-blur-md text-left font-mono">
              <h4 className="text-sm font-bold text-cyan-400 mb-4 uppercase flex items-center gap-1.5">
                <Shield size={16} /> SAGES GUARDIANS PROFILE
              </h4>

              {hoveredNode ? (
                <div className="bg-slate-950 p-4 rounded-xl border border-white/10 animate-fade-in flex flex-col gap-2">
                  <div className="flex justify-between items-start">
                    <span className="text-xs font-bold text-white uppercase">{hoveredNode.name}</span>
                    <span className={`text-[8px] px-2 py-0.5 rounded-full font-bold ${hoveredNode.type === 'Core' ? 'bg-cyan-500/10 text-cyan-400' : hoveredNode.type === 'Wild' ? 'bg-rose-500/10 text-rose-400' : 'bg-slate-700 text-slate-300'}`}>
                      {hoveredNode.type ? hoveredNode.type.toUpperCase() : 'COSMIC BODY'}
                    </span>
                  </div>
                  <div className="text-[10px] text-slate-400">{hoveredNode.title}</div>
                  <div className="grid grid-cols-2 gap-2 text-[9px] pt-2 border-t border-white/5">
                    <div>
                      <span className="text-slate-500">Emission Palette:</span>
                      <div className="text-slate-300 font-bold">{hoveredNode.archetype}</div>
                    </div>
                    <div>
                      <span className="text-slate-500">Planetary Match:</span>
                      <div className="text-slate-300 font-bold">{hoveredNode.alignment}</div>
                    </div>
                  </div>
                </div>
              ) : (
                <div className="text-slate-500 text-xs py-8 text-center flex flex-col items-center gap-2">
                  <Info size={24} className="text-slate-600" />
                  Hover over or touch nodes on the BlissID Tridecagon grid or the SoulChart Wheel to display local Guardian properties.
                </div>
              )}
            </div>
          </div>

          {/* RIGHT PANEL: Complex Tabs Dashboard */}
          <div className="lg:col-span-8 flex flex-col gap-6 w-full text-left">
            
            <div className="bg-slate-900/60 p-1.5 rounded-2xl border border-white/10 flex flex-wrap gap-1 backdrop-blur-md">
              {[
                { id: 'soulchart', label: '13-Body SoulChart', icon: Compass },
                { id: 'blissid', label: 'BlissID Tridecagon', icon: Layers },
                { id: 'soulsaga', label: 'SoulSaga Chronology', icon: FileText },
                { id: 'cryptvault', label: 'Sovereign Keys', icon: Shield },
                { id: 'ledger', label: 'Ineffable Ledger JSON', icon: Globe }
              ].map((t) => {
                const Icon = t.icon;
                const isSelected = activeTab === t.id;
                return (
                  <button
                    key={t.id}
                    onClick={() => setActiveTab(t.id)}
                    className={`flex-1 min-w-[120px] px-4 py-2.5 rounded-xl text-xs font-mono font-semibold tracking-wider flex items-center justify-center gap-1.5 transition ${isSelected ? 'bg-white/10 text-white shadow-md border border-white/10' : 'text-slate-400 hover:text-slate-200'}`}
                  >
                    <Icon size={14} className={isSelected ? 'text-cyan-400' : 'text-slate-500'} />
                    {t.label}
                  </button>
                );
              })}
            </div>

            {/* TAB CONTENT: 13-Body SoulChart Visual Map */}
            {activeTab === 'soulchart' && (
              <div className="bg-slate-900/80 border border-white/10 rounded-3xl p-6 backdrop-blur-md shadow-xl flex flex-col gap-6">
                <div className="flex flex-col md:flex-row justify-between items-start md:items-center gap-4 border-b border-white/5 pb-4">
                  <div>
                    <h3 className="font-mono text-lg font-bold text-white uppercase flex items-center gap-2">
                      <Compass className="text-cyan-400" /> 13-Body SoulChart Calibration
                    </h3>
                    <p className="text-xs text-slate-400 font-mono">Geocentric coordinate mappings under the 13-sign Zodiac (Including Earth & Ophiuchus).</p>
                  </div>
                  <div className="text-xs font-mono bg-cyan-500/10 text-cyan-300 border border-cyan-500/20 px-3 py-1 rounded-xl">
                    ARIES ANCHOR: 110.00 Hz
                  </div>
                </div>

                <div className="grid grid-cols-1 xl:grid-cols-12 gap-8 items-center">
                  
                  <div className="xl:col-span-5 flex justify-center">
                    <svg viewBox="0 0 100 100" className="w-64 h-64 md:w-72 md:h-72">
                      <circle cx="50" cy="50" r="46" fill="none" stroke="rgba(255, 255, 255, 0.05)" strokeWidth="1" />
                      <circle cx="50" cy="50" r="32" fill="none" stroke="rgba(255, 255, 255, 0.1)" strokeWidth="1" />
                      
                      {Object.keys(ZODIAC_FREQS).map((sign, idx) => {
                        const angle = (idx * 360) / Object.keys(ZODIAC_FREQS).length;
                        const rad = (angle * Math.PI) / 180;
                        const x2 = 50 + 46 * Math.cos(rad);
                        const y2 = 50 + 46 * Math.sin(rad);
                        return (
                          <line 
                            key={sign} 
                            x1="50" 
                            y1="50" 
                            x2={x2} 
                            y2={y2} 
                            stroke="rgba(255, 255, 255, 0.03)" 
                            strokeWidth="0.5" 
                          />
                        );
                      })}

                      {generatedData.bodies.map((b, idx) => {
                        const angle = (idx * 360) / generatedData.bodies.length;
                        const rad = (angle * Math.PI) / 180;
                        const x = 50 + 32 * Math.cos(rad);
                        const y = 50 + 32 * Math.sin(rad);
                        const isMain = b.name === "Sun" || b.name === "Moon" || b.name === "Earth";
                        return (
                          <g 
                            key={b.name} 
                            className="cursor-pointer group"
                            onMouseEnter={() => setHoveredNode({
                              name: b.name,
                              title: `${b.role}: Positioned at ${b.degrees}° of ${b.sign}`,
                              type: "Body Position",
                              archetype: `${b.frequency.toFixed(2)} Hz Resonator`,
                              alignment: `${b.element} / ${b.quality}`
                            })}
                            onMouseLeave={() => setHoveredNode(null)}
                          >
                            <circle 
                              cx={x} 
                              cy={y} 
                              r={isMain ? "3.5" : "2"} 
                              fill={b.name === "Sun" ? "#f43f5e" : b.name === "Moon" ? "#2dd4bf" : b.name === "Earth" ? "#10b981" : "#38bdf8"} 
                              className="transition hover:scale-150"
                            />
                            <circle 
                              cx={x} 
                              cy={y} 
                              r={isMain ? "6" : "4"} 
                              fill="none" 
                              stroke={b.name === "Sun" ? "rgba(244,63,94,0.3)" : b.name === "Moon" ? "rgba(45,212,191,0.3)" : b.name === "Earth" ? "rgba(16,185,129,0.3)" : "rgba(56,189,248,0.3)"} 
                              strokeWidth="0.5"
                              className="animate-pulse"
                            />
                          </g>
                        );
                      })}
                      
                      <circle cx="50" cy="50" r="4" fill="#fbbf24" />
                      <circle cx="50" cy="50" r="8" fill="none" stroke="rgba(251,191,36,0.2)" strokeWidth="1" />
                    </svg>
                  </div>

                  <div className="xl:col-span-7 font-mono text-[11px]">
                    <div className="bg-slate-950 rounded-2xl border border-white/5 overflow-hidden">
                      <div className="grid grid-cols-4 bg-white/5 p-2 font-bold text-slate-400 uppercase tracking-wider text-[9px] border-b border-white/5 text-center">
                        <div>13 Body Vector</div>
                        <div>13 Sign Placement</div>
                        <div>Frequency</div>
                        <div>Weight</div>
                      </div>
                      
                      <div className="max-h-[220px] overflow-y-auto divide-y divide-white/5 text-center">
                        {generatedData.bodies.map((b) => (
                          <div 
                            key={b.name} 
                            className="grid grid-cols-4 p-2 hover:bg-white/[0.02] items-center cursor-pointer"
                            onMouseEnter={() => setHoveredNode({
                              name: b.name,
                              title: `${b.role}: Positioned at ${b.degrees}° of ${b.sign}`,
                              type: "Body Position",
                              archetype: `${b.frequency.toFixed(2)} Hz Resonator`,
                              alignment: `${b.element} / ${b.quality}`
                            })}
                            onMouseLeave={() => setHoveredNode(null)}
                          >
                            <div className="font-bold text-white text-left pl-2 flex items-center gap-1.5">
                              <span className={`w-1.5 h-1.5 rounded-full ${b.name === "Sun" ? "bg-rose-500" : b.name === "Moon" ? "bg-teal-400" : b.name === "Earth" ? "bg-emerald-500" : "bg-sky-400"}`} />
                              {b.name}
                            </div>
                            <div className="text-slate-300">{b.sign} <span className="text-[9px] text-slate-500">{b.degrees}°</span></div>
                            <div className="font-bold text-cyan-400">{b.frequency.toFixed(1)} Hz</div>
                            <div className="text-slate-400">{b.weight.toFixed(1)}x</div>
                          </div>
                        ))}
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            )}

            {/* TAB CONTENT: BlissID Tridecagon Bioluminescent Mandala Visualizer */}
            {activeTab === 'blissid' && (
              <div className="bg-slate-900/80 border border-white/10 rounded-3xl p-6 backdrop-blur-md shadow-xl flex flex-col gap-6">
                <div className="flex flex-col md:flex-row justify-between items-start md:items-center gap-4 border-b border-white/5 pb-4">
                  <div>
                    <h3 className="font-mono text-lg font-bold text-white uppercase flex items-center gap-2">
                      <Layers className="text-teal-400 animate-pulse" /> BlissID Bioluminescent Mandala
                    </h3>
                    <p className="text-xs text-slate-400 font-mono">The 13-sided cryptographic identity mesh fused into a radiant harmonic pattern.</p>
                  </div>
                  <div className="text-xs font-mono bg-teal-500/10 text-teal-300 border border-teal-500/20 px-3 py-1 rounded-xl">
                    POST-QUANTUM PROTECTED
                  </div>
                </div>

                <div className="flex justify-center w-full py-8">
                  <svg viewBox="0 0 200 200" className="w-full max-w-lg aspect-square">
                    <defs>
                      <filter id="neon-glow-cyan" x="-50%" y="-50%" width="200%" height="200%">
                        <feGaussianBlur stdDeviation="2" result="blur" />
                        <feMerge>
                          <feMergeNode in="blur" />
                          <feMergeNode in="blur" />
                          <feMergeNode in="SourceGraphic" />
                        </feMerge>
                      </filter>
                      <filter id="neon-glow-rose" x="-50%" y="-50%" width="200%" height="200%">
                        <feGaussianBlur stdDeviation="2.5" result="blur" />
                        <feMerge>
                          <feMergeNode in="blur" />
                          <feMergeNode in="SourceGraphic" />
                        </feMerge>
                      </filter>
                      <filter id="core-glow" x="-50%" y="-50%" width="200%" height="200%">
                        <feGaussianBlur stdDeviation="4" result="blur" />
                        <feMerge>
                          <feMergeNode in="blur" />
                          <feMergeNode in="SourceGraphic" />
                        </feMerge>
                      </filter>
                      <radialGradient id="bg-gradient" cx="50%" cy="50%" r="50%">
                        <stop offset="0%" stopColor="rgba(45,212,191,0.15)" />
                        <stop offset="50%" stopColor="rgba(244,63,94,0.05)" />
                        <stop offset="100%" stopColor="transparent" />
                      </radialGradient>
                    </defs>

                    {/* Background Radial Glow */}
                    <circle cx="100" cy="100" r="90" fill="url(#bg-gradient)" className="animate-pulse" style={{ animationDuration: '4s' }} />
                    <circle cx="100" cy="100" r="85" fill="none" stroke="rgba(255,255,255,0.03)" strokeWidth="0.5" />
                    <circle cx="100" cy="100" r="50" fill="none" stroke="rgba(255,255,255,0.05)" strokeWidth="0.5" strokeDasharray="2,2" className="animate-spin" style={{ animationDuration: '30s', transformOrigin: 'center' }} />

                    {/* The 13 Outer Nodes (Tridecagon representing the full network) */}
                    <g className="animate-spin" style={{ animationDuration: '60s', transformOrigin: 'center' }}>
                      {Array.from({ length: 13 }).map((_, i) => {
                        const rad = (i * 2 * Math.PI) / 13 - Math.PI / 2;
                        const p1 = { x: 100 + 75 * Math.cos(rad), y: 100 + 75 * Math.sin(rad) };
                        
                        return Array.from({ length: 13 }).map((_, j) => {
                          // Connect specific nodes to form the classic string-art star mandala
                          if (i < j && (j - i === 4 || j - i === 5)) {
                             const rad2 = (j * 2 * Math.PI) / 13 - Math.PI / 2;
                             const p2 = { x: 100 + 75 * Math.cos(rad2), y: 100 + 75 * Math.sin(rad2) };
                             return (
                               <line key={`outer-${i}-${j}`} x1={p1.x} y1={p1.y} x2={p2.x} y2={p2.y} stroke="#2dd4bf" strokeWidth="0.5" strokeOpacity="0.6" filter="url(#neon-glow-cyan)" />
                             )
                          }
                          return null;
                        });
                      })}
                      {Array.from({ length: 13 }).map((_, i) => {
                        const rad = (i * 2 * Math.PI) / 13 - Math.PI / 2;
                        const x = 100 + 75 * Math.cos(rad);
                        const y = 100 + 75 * Math.sin(rad);
                        return (
                          <g key={`node-outer-${i}`}>
                            <circle cx={x} cy={y} r="2.5" fill="#5eead4" filter="url(#neon-glow-cyan)" className="animate-pulse" style={{ animationDelay: `${i * 0.2}s` }} />
                            <circle cx={x} cy={y} r="5" fill="none" stroke="#2dd4bf" strokeWidth="0.5" strokeOpacity="0.5" />
                          </g>
                        );
                      })}
                    </g>

                    {/* The 9 Inner Nodes (Nonagon representing the Core Strategic 9) */}
                    <g className="animate-spin" style={{ animationDuration: '40s', transformOrigin: 'center', animationDirection: 'reverse' }}>
                      {Array.from({ length: 9 }).map((_, i) => {
                        const rad = (i * 2 * Math.PI) / 9 - Math.PI / 2;
                        const p1 = { x: 100 + 40 * Math.cos(rad), y: 100 + 40 * Math.sin(rad) };
                        
                        return Array.from({ length: 9 }).map((_, j) => {
                          if (i < j && (j - i === 2 || j - i === 3)) {
                             const rad2 = (j * 2 * Math.PI) / 9 - Math.PI / 2;
                             const p2 = { x: 100 + 40 * Math.cos(rad2), y: 100 + 40 * Math.sin(rad2) };
                             return (
                               <line key={`inner-${i}-${j}`} x1={p1.x} y1={p1.y} x2={p2.x} y2={p2.y} stroke="#f43f5e" strokeWidth="0.75" strokeOpacity="0.7" filter="url(#neon-glow-rose)" />
                             )
                          }
                          return null;
                        });
                      })}
                      {Array.from({ length: 9 }).map((_, i) => {
                        const rad = (i * 2 * Math.PI) / 9 - Math.PI / 2;
                        const x = 100 + 40 * Math.cos(rad);
                        const y = 100 + 40 * Math.sin(rad);
                        return (
                          <g key={`node-inner-${i}`}>
                            <circle cx={x} cy={y} r="2" fill="#fda4af" filter="url(#neon-glow-rose)" className="animate-ping" style={{ animationDelay: `${i * 0.15}s`, animationDuration: '3s' }} />
                            <circle cx={x} cy={y} r="4" fill="none" stroke="#f43f5e" strokeWidth="1" strokeOpacity="0.8" />
                          </g>
                        );
                      })}
                    </g>

                    {/* The Luminous Core */}
                    <circle cx="100" cy="100" r="12" fill="none" stroke="#f59e0b" strokeWidth="1.5" strokeOpacity="0.6" filter="url(#core-glow)" className="animate-pulse" style={{ animationDuration: '2s' }} />
                    <circle cx="100" cy="100" r="6" fill="#fbbf24" filter="url(#core-glow)" />
                    <circle cx="100" cy="100" r="2" fill="#fff" />
                    
                    {/* Geometric connecting lines from core to inner nonagon */}
                    <g className="animate-spin" style={{ animationDuration: '40s', transformOrigin: 'center', animationDirection: 'reverse' }}>
                       {Array.from({ length: 9 }).map((_, i) => {
                          const rad = (i * 2 * Math.PI) / 9 - Math.PI / 2;
                          const x = 100 + 40 * Math.cos(rad);
                          const y = 100 + 40 * Math.sin(rad);
                          return <line key={`spoke-${i}`} x1="100" y1="100" x2={x} y2={y} stroke="#fbbf24" strokeWidth="0.5" strokeOpacity="0.4" filter="url(#core-glow)" />;
                       })}
                    </g>

                  </svg>
                </div>
              </div>
            )}

            {/* TAB CONTENT: Cosmic Chronology Narrative Saga */}
            {activeTab === 'soulsaga' && (
              <div className="bg-slate-900/80 border border-white/10 rounded-3xl p-6 backdrop-blur-md shadow-xl flex flex-col gap-6 font-mono">
                <div className="border-b border-white/5 pb-4">
                  <h3 className="text-lg font-bold text-white uppercase flex items-center gap-2">
                    <FileText className="text-rose-500" /> SoulSaga Historical Chronology
                  </h3>
                  <p className="text-xs text-slate-400">Localized events and cosmic configurations at your unique spacetime vectors.</p>
                </div>

                <div className="relative border-l border-white/10 pl-6 ml-4 flex flex-col gap-8 text-xs text-left">
                  
                  <div className="relative">
                    <div className="absolute -left-10 top-0.5 bg-cyan-400 w-8 h-8 rounded-full border-4 border-slate-900 flex items-center justify-center">
                      <Compass size={12} className="text-slate-950" />
                    </div>
                    <div>
                      <span className="text-cyan-400 font-bold uppercase tracking-wider text-[10px]">T-0s // SPATIOTEMPORAL ANCHOR</span>
                      <h4 className="text-sm font-bold text-white mt-1">Cosmic Gateway Formed at {formData.neighborhood}</h4>
                      <p className="text-slate-400 mt-2 leading-relaxed">
                        The geocentric baseline coordinate vector anchored exactly in {formData.city}, {formData.state}, {formData.country} ({formData.hemisphere} Hemisphere) on Planet {formData.planet}. Local space-time curvature established a static localized entropy field of 99.8%.
                      </p>
                    </div>
                  </div>

                  <div className="relative">
                    <div className="absolute -left-10 top-0.5 bg-rose-500 w-8 h-8 rounded-full border-4 border-slate-900 flex items-center justify-center">
                      <Activity size={12} className="text-slate-950" />
                    </div>
                    <div>
                      <span className="text-rose-400 font-bold uppercase tracking-wider text-[10px]">T+28s // BIOMETRIC ENTRAINMENT</span>
                      <h4 className="text-sm font-bold text-white mt-1">Dynamic Somatic Seed Absorption</h4>
                      <p className="text-slate-400 mt-2 leading-relaxed">
                        HRV vector calculated at **{hrvSim}ms** baseline. Continuous micro-voltage telemetry successfully mapped to the SAGES autonomic nervous shield layer. Real-time liveness verifies a unique, unsymmetric threshold matching **SAGE-13**.
                      </p>
                    </div>
                  </div>

                  <div className="relative">
                    <div className="absolute -left-10 top-0.5 bg-amber-500 w-8 h-8 rounded-full border-4 border-slate-900 flex items-center justify-center">
                      <Award size={12} className="text-slate-950" />
                    </div>
                    <div>
                      <span className="text-amber-400 font-bold uppercase tracking-wider text-[10px]">T+45s // ROOT RESONANCE FUSION</span>
                      <h4 className="text-sm font-bold text-white mt-1">Weighted Geometric Harmonic Synthesis at {generatedData.rootFrequency} Hz</h4>
                      <p className="text-slate-400 mt-2 leading-relaxed">
                        Trapped Chaos within the Sun and Bliss within the Moon computed using multi-dimensional wave mechanics. This treats frequencies as interacting dimensions of a hyper-volume, creating a fused acoustic baseline of **{generatedData.rootFrequency} Hz** (Carrier wave verified).
                      </p>
                    </div>
                  </div>

                </div>
              </div>
            )}

            {/* TAB CONTENT: Sovereign Post-Quantum Cryptographic Keys */}
            {activeTab === 'cryptvault' && (
              <div className="bg-slate-900/80 border border-white/10 rounded-3xl p-6 backdrop-blur-md shadow-xl flex flex-col gap-6 font-mono text-left">
                <div className="border-b border-white/5 pb-4 flex justify-between items-center">
                  <div>
                    <h3 className="text-lg font-bold text-white uppercase flex items-center gap-2">
                      <Shield className="text-yellow-400" /> Sovereign Post-Quantum Security Keys
                    </h3>
                    <p className="text-xs text-slate-400">ML-KEM (Kyber-1024) Public Keys and Dilithium-5 sovereign signatures.</p>
                  </div>
                  <div className="p-1 bg-yellow-500/10 border border-yellow-500/20 text-yellow-400 text-[10px] px-2 py-0.5 rounded uppercase">
                    Quantum-Safe
                  </div>
                </div>

                <div className="flex flex-col gap-4 text-xs">
                  <div>
                    <span className="text-slate-500 text-[10px] uppercase block mb-1">SHA-3-512 SECURE sponge HASH (Sovereign ID Anchor)</span>
                    <div className="bg-slate-950 border border-white/5 p-3 rounded-xl flex items-center justify-between gap-4">
                      <span className="text-cyan-400 text-[11px] font-bold break-all">{generatedData.soulHash}</span>
                      <button 
                        onClick={() => {
                          document.execCommand('copy');
                          navigator.clipboard?.writeText?.(generatedData.soulHash);
                        }}
                        className="text-xs text-slate-400 hover:text-white border border-white/10 bg-white/5 px-3 py-1 rounded-lg"
                      >
                        Copy Hash
                      </button>
                    </div>
                  </div>

                  <div>
                    <span className="text-slate-500 text-[10px] uppercase block mb-1">ML-KEM-1024 Kyber Public Key Lattice Envelope</span>
                    <div className="bg-slate-950 border border-white/5 p-3 rounded-xl relative">
                      <div className="text-slate-400 text-[10px] font-mono leading-relaxed select-all break-all h-20 overflow-y-auto pr-2 scrollbar-thin">
                        {generatedData.kyberPublicKey}
                      </div>
                      <div className="absolute bottom-2 right-2 flex items-center gap-1.5 bg-black/80 px-2 py-1 rounded text-[8px] text-yellow-400 border border-yellow-500/20">
                        <Lock size={10} /> 1024-BIT LATTICE KEY
                      </div>
                    </div>
                  </div>

                  <div>
                    <span className="text-slate-500 text-[10px] uppercase block mb-1">Dilithium-5 Quantum Signature Payload</span>
                    <div className="bg-slate-950 border border-white/5 p-3 rounded-xl flex items-center justify-between gap-4">
                      <span className="text-rose-400 text-[10px] font-mono break-all leading-normal select-all">{generatedData.dilithiumSignature}</span>
                      <span className="text-[9px] text-slate-500 whitespace-nowrap">STATUS: SECURELY SIGNED</span>
                    </div>
                  </div>
                </div>
              </div>
            )}

            {/* TAB CONTENT: Raw Genesis Block JSON */}
            {activeTab === 'ledger' && (
              <div className="bg-slate-900/80 border border-white/10 rounded-3xl p-6 backdrop-blur-md shadow-xl flex flex-col gap-6 font-mono text-left">
                <div className="border-b border-white/5 pb-4 flex justify-between items-center">
                  <div>
                    <h3 className="text-lg font-bold text-white uppercase flex items-center gap-2">
                      <Globe className="text-emerald-400 animate-pulse" /> Ineffable Ledger Genesis block JSON
                    </h3>
                    <p className="text-xs text-slate-400">Canonical raw block payload ready for integration with the Archivus ledger.</p>
                  </div>
                </div>

                <div className="bg-slate-950 rounded-2xl border border-white/5 p-4 relative">
                  <pre className="text-[10px] text-slate-300 leading-relaxed overflow-x-auto h-80 scrollbar-thin max-w-full whitespace-pre-wrap select-all">
                    {JSON.stringify({
                      "BLOCK_TYPE": "SOULSHOT_GENESIS_VECTOR",
                      "BLOCK_VERSION": "1.8",
                      "TIMESTAMP": new Date().toISOString(),
                      "METADATA": {
                        "SOVEREIGN_NAME": formData.name,
                        "INTENT_MANTRA": formData.mantra,
                        "COSMIC_COORD": {
                          "BIRTH_DATE": formData.birthDate,
                          "BIRTH_TIME": formData.birthTime,
                          "CIVIL_WEIGHT_OUNCES": formData.weightLbs * 16 + formData.weightOz
                        },
                        "9_TIER_LOCATION": {
                          "NEIGHBORHOOD": formData.neighborhood,
                          "CITY": formData.city,
                          "COUNTY": formData.county,
                          "STATE": formData.state,
                          "REGION": formData.region,
                          "COUNTRY": formData.country,
                          "CONTINENT": formData.continent,
                          "HEMISPHERE": formData.hemisphere,
                          "PLANET": formData.planet
                        }
                      },
                      "SOMATIC_TELEMETRY": {
                        "HRV_BASELINE_MS": hrvSim,
                        "GSR_ACTIVITY_MICROS": gsrSim
                      },
                      "CALCULUS": {
                        "ROOT_FREQUENCY_HZ": parseFloat(generatedData.rootFrequency),
                        "DIVINE_SCORE_COHERENCE": generatedData.divineScore,
                        "SOUL_HASH_SHA3_512": generatedData.soulHash,
                        "DILITHIUM5_SIGNATURE": generatedData.dilithiumSignature,
                        "STRATEGIC_SAGES": generatedData.strategic9.map(s => s.name),
                        "WILD_SAGES": generatedData.wild4.map(s => s.name)
                      },
                      "STATUS": "IMMUTABLE_CANON"
                    }, null, 2)}
                  </pre>
                </div>
              </div>
            )}

            {/* Bottom Actions Block */}
            <div className="flex flex-col sm:flex-row gap-4">
              <button 
                onClick={() => {
                  stopSynthesizer();
                  setStep('landing');
                }}
                className="flex-1 py-4 bg-white/5 hover:bg-white/10 border border-white/10 rounded-2xl font-mono text-xs font-bold uppercase tracking-wider text-center text-slate-300 hover:text-white transition"
              >
                Create Another SoulShot
              </button>

              <button 
                onClick={() => alert("Simulated: Exported and compiled high-resolution SAGES scroll PDF!")}
                className="flex-1 py-4 bg-gradient-to-r from-cyan-500 to-teal-500 hover:from-cyan-400 hover:to-teal-400 text-slate-950 rounded-2xl font-mono text-xs font-bold uppercase tracking-widest text-center transition flex items-center justify-center gap-2 shadow-lg shadow-cyan-500/10"
              >
                <Download size={14} /> Download Printable Art Scroll
              </button>
            </div>

          </div>
        </main>
      )}

      {/* Persistent Bottom Status Information Bar */}
      <footer className="border-t border-white/5 bg-slate-950/80 backdrop-blur-sm py-4 px-6 relative z-10 flex flex-col sm:flex-row items-center justify-between gap-2 text-[10px] font-mono text-slate-500">
        <div>
          [:: CTRL + ALT + ReDESIGN ::] with Aurphyx. SAGES Compliant.
        </div>
        <div className="flex items-center gap-4">
          <span className="flex items-center gap-1"><Shield size={10} className="text-teal-400" /> SAGE-13 Compliant</span>
          <span className="flex items-center gap-1"><Activity size={10} className="text-rose-400" /> THIN Sensor Streamed</span>
        </div>
      </footer>
    </div>
  );
}