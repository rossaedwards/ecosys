import React, { useState, useEffect } from 'react';
import { 
  Ear, EyeOff, Activity, Waves, Mic2, 
  Zap, Heart, Cpu, ShieldCheck, Play, Pause, Settings, Info
} from 'lucide-react';

const UniversalAccessibilityLayer = () => {
  const [isPlaying, setIsPlaying] = useState(false);
  const [activeMode, setActiveMode] = useState('haptic'); // 'haptic' or 'auditory'
  const [intensity, setIntensity] = useState(0);

  // Simulated Track: Stuca - Jump In (from the Golden Set)
  const activeTrack = {
    title: "Jump In",
    artist: "Stuca",
    vap: {
      structural: { bpm: 140, kick_transient: "Hard Clip", syncopation: 0.8 },
      affective: { arousal: 0.9, valence: -0.5, tension: "Hydraulic Pressure" },
      photometric: { primary_hex: "#FF4500", strobe: "Polyrhythmic", temp: "Toxic" },
      kinetic: { target_hr: "130-150", movement: "Drop Squat" }
    }
  };

  // Simulate V.A.P. data driving hardware intensity
  useEffect(() => {
    let interval;
    if (isPlaying) {
      interval = setInterval(() => {
        // Syncopated pulsing based on the 140 BPM / Hard Clip profile
        setIntensity(Math.random() > 0.3 ? Math.floor(Math.random() * 40) + 60 : 10);
      }, 428); // Roughly 140 BPM in ms
    } else {
      setIntensity(0);
    }
    return () => clearInterval(interval);
  }, [isPlaying]);

  return (
    <div className="min-h-screen bg-[#050505] text-zinc-100 font-sans p-4 md:p-8 selection:bg-cyan-500/30">
      
      {/* Header */}
      <header className="max-w-6xl mx-auto flex flex-col md:flex-row justify-between items-start md:items-center mb-10 gap-6 border-b border-zinc-900 pb-6">
        <div>
          <div className="flex items-center gap-2 mb-2">
            <ShieldCheck size={16} className="text-cyan-400" />
            <span className="text-[10px] font-black tracking-[0.2em] text-cyan-400 uppercase">Universal Accessibility Layer Active</span>
          </div>
          <h1 className="text-3xl font-black tracking-tighter">
            SOULSYNC <span className="text-zinc-600">U.A.L.</span>
          </h1>
          <p className="text-xs text-zinc-500 font-bold uppercase tracking-widest mt-1">Translating Vibration to Sensation</p>
        </div>

        <div className="flex bg-zinc-900 p-1 rounded-xl border border-zinc-800">
          <button 
            onClick={() => setActiveMode('haptic')}
            className={`flex items-center gap-2 px-6 py-2 rounded-lg text-xs font-bold transition-all ${activeMode === 'haptic' ? 'bg-cyan-500/20 text-cyan-400 border border-cyan-500/50' : 'text-zinc-500 hover:text-zinc-300'}`}
          >
            <Ear size={16} /> DEAF / HAPTIC
          </button>
          <button 
            onClick={() => setActiveMode('auditory')}
            className={`flex items-center gap-2 px-6 py-2 rounded-lg text-xs font-bold transition-all ${activeMode === 'auditory' ? 'bg-purple-500/20 text-purple-400 border border-purple-500/50' : 'text-zinc-500 hover:text-zinc-300'}`}
          >
            <EyeOff size={16} /> BLIND / AUDITORY
          </button>
        </div>
      </header>

      <div className="max-w-6xl mx-auto grid grid-cols-1 lg:grid-cols-12 gap-8">
        
        {/* Left Column: Player & Active V.A.P. */}
        <div className="lg:col-span-4 space-y-6">
          <div className="bg-zinc-900/40 border border-zinc-800 rounded-3xl p-6 relative overflow-hidden group">
            {/* Visualizer Background based on track hex */}
            <div 
              className="absolute inset-0 opacity-20 transition-opacity duration-75"
              style={{ backgroundColor: activeTrack.vap.photometric.primary_hex, opacity: intensity / 200 }}
            />
            
            <div className="relative z-10">
              <div className="flex justify-between items-start mb-8">
                <div>
                  <h3 className="text-[10px] font-bold text-zinc-500 uppercase tracking-widest mb-1">Source Stream</h3>
                  <div className="text-xl font-black">{activeTrack.title}</div>
                  <div className="text-sm font-bold text-zinc-400">{activeTrack.artist}</div>
                </div>
                <button 
                  onClick={() => setIsPlaying(!isPlaying)}
                  className="w-12 h-12 rounded-full bg-white text-black flex items-center justify-center hover:scale-105 transition-transform shadow-[0_0_20px_rgba(255,255,255,0.2)]"
                >
                  {isPlaying ? <Pause fill="black" /> : <Play fill="black" className="ml-1" />}
                </button>
              </div>

              <div className="space-y-4">
                <div className="bg-black/50 p-3 rounded-xl border border-zinc-800">
                  <div className="text-[10px] font-bold text-zinc-500 uppercase mb-1 flex items-center gap-2"><Activity size={12}/> Structural Base</div>
                  <div className="text-sm font-bold">{activeTrack.vap.structural.bpm} BPM • {activeTrack.vap.structural.kick_transient}</div>
                </div>
                <div className="bg-black/50 p-3 rounded-xl border border-zinc-800">
                  <div className="text-[10px] font-bold text-zinc-500 uppercase mb-1 flex items-center gap-2"><Zap size={12}/> Affective Tension</div>
                  <div className="text-sm font-bold">{activeTrack.vap.affective.tension}</div>
                </div>
              </div>
            </div>
          </div>

          <div className="bg-black border border-zinc-800 rounded-3xl p-6">
             <div className="flex items-center justify-between mb-4">
               <h3 className="text-xs font-bold text-zinc-400 uppercase tracking-widest flex items-center gap-2">
                 <Cpu size={14} className="text-green-500" /> Hardware Link
               </h3>
               <span className="flex items-center gap-1 text-[9px] text-green-500 uppercase font-bold tracking-widest"><div className="w-1.5 h-1.5 bg-green-500 rounded-full animate-pulse"/> Connected</span>
             </div>
             <p className="text-xs text-zinc-500 leading-relaxed font-medium">
               IBS Ultra-thin Nanomembrane interface detected. Biometric skin adhesion confirmed. Neural routing active.
             </p>
          </div>
        </div>

        {/* Right Column: U.A.L. Translation Engine */}
        <div className="lg:col-span-8">
          
          {/* HAPTIC MODE (DEAF ACCESSIBILITY) */}
          {activeMode === 'haptic' && (
            <div className="space-y-6 animate-in fade-in slide-in-from-bottom-4 duration-500">
              <div className="bg-gradient-to-br from-cyan-900/20 to-black border border-cyan-500/20 rounded-3xl p-8">
                <div className="flex items-center gap-3 mb-6">
                  <div className="p-3 bg-cyan-500/10 rounded-xl border border-cyan-500/30">
                    <Waves className="text-cyan-400" size={24} />
                  </div>
                  <div>
                    <h2 className="text-xl font-black text-white">Neuro-Haptic Translation Matrix</h2>
                    <p className="text-xs font-bold text-cyan-500 uppercase tracking-widest">Pillar 1 & 8 → Somatosensory Cortex</p>
                  </div>
                </div>

                <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                  {/* Spine / Core Bass */}
                  <div className="bg-black/60 border border-zinc-800 rounded-2xl p-6 relative overflow-hidden">
                    <div className="absolute bottom-0 left-0 w-full h-1 bg-zinc-900">
                      <div className="h-full bg-cyan-500 transition-all duration-75" style={{ width: `${intensity}%` }} />
                    </div>
                    <div className="flex justify-between items-start mb-4">
                      <div className="text-[10px] font-bold text-zinc-500 uppercase tracking-widest">Sub-Bass Routing</div>
                      <div className="px-2 py-1 bg-cyan-500/10 text-cyan-400 text-[9px] font-black rounded uppercase">Spinal Membrane</div>
                    </div>
                    <div className="text-2xl font-black text-white mb-1">{intensity > 50 ? 'HEAVY PRESSURE' : 'IDLE'}</div>
                    <p className="text-xs text-zinc-500">Translating "Hard Clip" kick transient into deep, sustained lumbar vibrations.</p>
                  </div>

                  {/* Peripheral / Hi-Hats */}
                  <div className="bg-black/60 border border-zinc-800 rounded-2xl p-6 relative overflow-hidden">
                     <div className="absolute bottom-0 left-0 w-full h-1 bg-zinc-900">
                      <div className="h-full bg-blue-500 transition-all duration-75" style={{ width: `${intensity > 70 ? 100 : 0}%` }} />
                    </div>
                    <div className="flex justify-between items-start mb-4">
                      <div className="text-[10px] font-bold text-zinc-500 uppercase tracking-widest">Syncopation Routing</div>
                      <div className="px-2 py-1 bg-blue-500/10 text-blue-400 text-[9px] font-black rounded uppercase">Wrist / Radial</div>
                    </div>
                    <div className="text-2xl font-black text-white mb-1">{intensity > 70 ? 'STACCATO TAP' : 'IDLE'}</div>
                    <p className="text-xs text-zinc-500">Mapping polyrhythmic upper-frequency data to sharp, localized peripheral taps.</p>
                  </div>
                </div>

                {/* Biometric Safety Override */}
                <div className="mt-6 flex items-center justify-between p-4 bg-zinc-900/50 rounded-xl border border-zinc-800">
                  <div className="flex items-center gap-3">
                    <Heart size={16} className="text-red-500" />
                    <div>
                      <div className="text-[10px] font-bold text-zinc-400 uppercase">S.A.G.E.S. Bio-Limit</div>
                      <div className="text-xs text-zinc-300">Max haptic intensity capped at 85% to prevent neuro-fatigue.</div>
                    </div>
                  </div>
                  <Settings size={16} className="text-zinc-600 hover:text-white cursor-pointer" />
                </div>
              </div>
            </div>
          )}

          {/* AUDITORY MODE (BLIND ACCESSIBILITY) */}
          {activeMode === 'auditory' && (
            <div className="space-y-6 animate-in fade-in slide-in-from-bottom-4 duration-500">
               <div className="bg-gradient-to-br from-purple-900/20 to-black border border-purple-500/20 rounded-3xl p-8">
                <div className="flex items-center gap-3 mb-6">
                  <div className="p-3 bg-purple-500/10 rounded-xl border border-purple-500/30">
                    <Mic2 className="text-purple-400" size={24} />
                  </div>
                  <div>
                    <h2 className="text-xl font-black text-white">Audry Synth-Description Engine</h2>
                    <p className="text-xs font-bold text-purple-500 uppercase tracking-widest">Pillar 6 & 7 → Spatital Audio TTS</p>
                  </div>
                </div>

                <div className="bg-black/60 border border-zinc-800 rounded-2xl p-6">
                  <div className="flex justify-between items-center mb-6">
                    <h3 className="text-[10px] font-bold text-zinc-500 uppercase tracking-widest">Live Generative Transcript</h3>
                    <div className="flex items-center gap-2">
                       <span className="relative flex h-2 w-2">
                        <span className={`animate-ping absolute inline-flex h-full w-full rounded-full bg-purple-400 opacity-75 ${isPlaying ? 'block' : 'hidden'}`}></span>
                        <span className={`relative inline-flex rounded-full h-2 w-2 ${isPlaying ? 'bg-purple-500' : 'bg-zinc-700'}`}></span>
                      </span>
                      <span className="text-[9px] font-black text-purple-400 uppercase">Live Synthesis</span>
                    </div>
                  </div>

                  {/* Simulated Audry TTS Output */}
                  <div className="space-y-4">
                    <div className={`p-4 rounded-xl border transition-all ${isPlaying ? 'bg-purple-900/20 border-purple-500/30' : 'bg-zinc-900/50 border-zinc-800'}`}>
                      <div className="text-[10px] font-bold text-purple-400 mb-2 uppercase flex items-center gap-2">
                        <Info size={12}/> Vibe Context Established
                      </div>
                      <p className="text-sm text-zinc-200 leading-relaxed font-medium italic">
                        "The atmosphere shifts to an aggressive, industrial rave setting. The primary visual color is a toxic, burnt orange—hex code FF4500. It feels like mechanical pressure building."
                      </p>
                    </div>

                    <div className={`p-4 rounded-xl border transition-all ${isPlaying && intensity > 50 ? 'bg-red-900/20 border-red-500/30' : 'bg-zinc-900/50 border-zinc-800'}`}>
                       <div className="text-[10px] font-bold text-red-400 mb-2 uppercase flex items-center gap-2">
                        <Zap size={12}/> Kinetic & Photometric Event
                      </div>
                      <p className="text-sm text-zinc-200 leading-relaxed font-medium italic">
                        "The drop hits. A polyrhythmic strobe effect flashes violently across the room, perfectly synced to the heavy sub-bass distortion. The energy requires intense physical grounding."
                      </p>
                    </div>
                  </div>
                </div>

                {/* Spatial Audio Controls */}
                <div className="mt-6 flex justify-between items-center">
                  <div className="flex gap-4">
                    <div className="flex flex-col">
                      <span className="text-[9px] font-bold text-zinc-500 uppercase mb-1">Voice Profile</span>
                      <span className="text-xs font-black text-white">Audry (Phoenix Bliss)</span>
                    </div>
                    <div className="h-8 w-px bg-zinc-800"></div>
                    <div className="flex flex-col">
                      <span className="text-[9px] font-bold text-zinc-500 uppercase mb-1">Audio Mix</span>
                      <span className="text-xs font-black text-white">80% Music / 20% TTS Ducking</span>
                    </div>
                  </div>
                  <button className="px-4 py-2 bg-zinc-900 border border-zinc-700 hover:bg-zinc-800 rounded-lg text-[10px] font-bold uppercase text-white transition-colors">
                    Calibrate Mix
                  </button>
                </div>
              </div>
            </div>
          )}

        </div>
      </div>
    </div>
  );
};

export default UniversalAccessibilityLayer;