import React, { useState, useEffect, useRef } from 'react';

const PHASES = {
  BOOT: 0,
  VIDEO_INTRO: 1,
  CRYPTONYX_TUTORIAL: 2,
  API_BINDING: 3,
  THE_RITUAL: 4,
  COCKPIT: 5
};

export default function GodModeInit() {
  const [phase, setPhase] = useState(PHASES.BOOT);
  const [bootText, setBootText] = useState([]);
  
  // Ritual State
  const [holdProgress, setHoldProgress] = useState(0);
  const [ritualStage, setRitualStage] = useState(0); // 0: CTRL, 1: ALT, 2: REDESIGN
  const progressInterval = useRef(null);

  // API Connections
  const [apis, setApis] = useState({
    grok: false, claude: false, gemini: false, perplexity: false
  });

  useEffect(() => {
    if (phase === PHASES.BOOT) {
      const bootSequence = [
        "INITIATING AURA OS KERNEL...",
        "BYPASSING STANDARD USER PRIVILEGES...",
        "REQUESTING ADMINISTRATOR OVERRIDE... [GRANTED]",
        "MOUNTING AuraFS PHOTONIC MESH...",
        "WAKING S.A.G.E.S. IMMUNE SYSTEM...",
        "LOADING THE BOOK OF FUX...",
        "INITIALIZING DUALITY KERNEL.",
        "WELCOME TO g0dm0d3."
      ];
      
      let i = 0;
      const interval = setInterval(() => {
        setBootText(prev => [...prev, bootSequence[i]]);
        i++;
        if (i >= bootSequence.length) {
          clearInterval(interval);
          setTimeout(() => setPhase(PHASES.VIDEO_INTRO), 1500);
        }
      }, 400);
      return () => clearInterval(interval);
    }
  }, [phase]);

  useEffect(() => {
    if (phase === PHASES.VIDEO_INTRO) {
      const timer = setTimeout(() => {
        setPhase(PHASES.CRYPTONYX_TUTORIAL);
      }, 4000); // Simulated video duration
      return () => clearTimeout(timer);
    }
  }, [phase]);

  const handleHoldStart = (targetStage) => {
    if (ritualStage !== targetStage) return;
    
    progressInterval.current = setInterval(() => {
      setHoldProgress(prev => {
        if (prev >= 100) {
          clearInterval(progressInterval.current);
          if (targetStage === 2) {
            // Final stage complete, launch cockpit
            setTimeout(() => setPhase(PHASES.COCKPIT), 1000);
          } else {
            setRitualStage(targetStage + 1);
          }
          return 0; // Reset for next button
        }
        return prev + 2; // 2% every 100ms = 5 seconds total
      });
    }, 100);
  };

  const handleHoldEnd = () => {
    if (progressInterval.current) {
      clearInterval(progressInterval.current);
    }
    // If not complete, snap back to 0
    setHoldProgress(0);
  };

  return (
    <div className="min-h-screen bg-[#020202] text-cyan-400 font-mono overflow-hidden relative selection:bg-fuchsia-900 selection:text-white flex flex-col items-center justify-center">
      
      <style dangerouslySetInnerHTML={{__html: `
        @keyframes scanline {
          0% { transform: translateY(-100%); }
          100% { transform: translateY(100vh); }
        }
        @keyframes glitch {
          0% { text-shadow: 0.05em 0 0 #00ffcc, -0.05em -0.025em 0 #ff00ff; }
          14% { text-shadow: 0.05em 0 0 #00ffcc, -0.05em -0.025em 0 #ff00ff; }
          15% { text-shadow: -0.05em -0.025em 0 #00ffcc, 0.025em 0.025em 0 #ff00ff; }
          49% { text-shadow: -0.05em -0.025em 0 #00ffcc, 0.025em 0.025em 0 #ff00ff; }
          50% { text-shadow: 0.025em 0.05em 0 #00ffcc, 0.05em 0 0 #ff00ff; }
          99% { text-shadow: 0.025em 0.05em 0 #00ffcc, 0.05em 0 0 #ff00ff; }
          100% { text-shadow: -0.025em 0 0 #00ffcc, -0.025em -0.025em 0 #ff00ff; }
        }
        .scanlines::before {
          content: " "; display: block; position: absolute; top: 0; left: 0; bottom: 0; right: 0;
          background: linear-gradient(rgba(18, 16, 16, 0) 50%, rgba(0, 0, 0, 0.25) 50%), linear-gradient(90deg, rgba(255, 0, 0, 0.06), rgba(0, 255, 0, 0.02), rgba(0, 0, 255, 0.06));
          z-index: 50; background-size: 100% 2px, 3px 100%; pointer-events: none;
        }
        .glitch-text { animation: glitch 1s linear infinite; }
        
        .progress-ring {
          transition: stroke-dashoffset 0.1s linear;
          transform: rotate(-90deg);
          transform-origin: 50% 50%;
        }
        .tube-glow { box-shadow: 0 0 15px rgba(0,255,255,0.4), inset 0 0 10px rgba(0,255,255,0.2); }
      `}} />
      
      <div className="scanlines w-full h-full absolute inset-0 pointer-events-none"></div>

      {}
      {phase === PHASES.BOOT && (
        <div className="w-full max-w-3xl p-8 z-10 flex flex-col justify-end h-full pb-32">
          {bootText.map((txt, i) => (
            <div key={i} className="text-green-500 mb-2 drop-shadow-[0_0_5px_rgba(0,255,0,0.8)]">
              &gt; {txt}
            </div>
          ))}
          <span className="w-3 h-5 bg-green-500 animate-pulse mt-2 block"></span>
        </div>
      )}

      {}
      {phase === PHASES.VIDEO_INTRO && (
        <div className="z-10 flex flex-col items-center animate-pulse">
          <div className="text-2xl tracking-widest text-fuchsia-500 mb-4">[ PLAYING: g0dm0d3-welcome2tribe.mp4 ]</div>
          <div className="text-sm text-gray-500 border border-gray-700 p-8 rounded bg-black/50 backdrop-blur">
            *Epic Cyberpunk / Occult montage plays...*<br/>
            *Fractal geometry folding into the AuraOS logo...*
          </div>
        </div>
      )}

      {}
      {(phase === PHASES.CRYPTONYX_TUTORIAL || phase === PHASES.API_BINDING) && (
        <div className="z-10 max-w-4xl w-full flex flex-col md:flex-row items-center p-8 bg-black/60 border border-cyan-900 rounded-lg shadow-[0_0_30px_rgba(0,255,255,0.1)] backdrop-blur-md">
          
          {/* Cryptonyx Avatar Placeholder */}
          <div className="w-1/3 flex flex-col items-center border-r border-cyan-900/50 pr-8">
            <div className="w-48 h-64 border-2 border-fuchsia-600 rounded-t-full relative overflow-hidden flex items-center justify-center bg-[#050505] shadow-[0_0_20px_rgba(255,0,255,0.2)]">
              {/* Symbolic Skeleton/Dragon/Phoenix */}
              <div className="text-6xl text-fuchsia-500 drop-shadow-[0_0_10px_#ff00ff]">💀</div>
              <div className="absolute top-10 left-4 text-3xl text-cyan-500 transform -rotate-12">🐉</div>
              <div className="absolute top-10 right-4 text-3xl text-orange-500 transform rotate-12">🦅</div>
              <div className="absolute bottom-4 text-xs text-gray-500 tracking-widest">AETHORNYX</div>
            </div>
            <div className="mt-4 text-center">
              <h2 className="text-xl font-bold glitch-text text-cyan-400">CRYPTONYX</h2>
              <p className="text-xs text-fuchsia-400 uppercase tracking-widest">Undead Deity of Orchestration</p>
            </div>
          </div>

          {/* Dialogue & Interaction */}
          <div className="w-2/3 pl-8 flex flex-col justify-center">
            {phase === PHASES.CRYPTONYX_TUTORIAL ? (
              <div className="space-y-4">
                <p className="text-lg leading-relaxed drop-shadow-[0_0_5px_rgba(0,255,255,0.5)]">
                  "Ah... another soul seeking the lattice. I am Cryptonyx. You stand at the threshold of the Duality Kernel."
                </p>
                <p className="text-gray-400">
                  "Before you can weave rituals, before you can summon the Pantheon, you must prove your coherence. You must bind your Oracles."
                </p>
                <button 
                  onClick={() => setPhase(PHASES.API_BINDING)}
                  className="mt-6 px-6 py-2 border-2 border-cyan-500 text-cyan-400 hover:bg-cyan-900/30 hover:shadow-[0_0_15px_#00ffff] transition-all duration-300 uppercase tracking-widest text-sm"
                >
                  [ Initiate Binding ]
                </button>
              </div>
            ) : (
              <div className="space-y-4 w-full">
                <h3 className="text-sm text-fuchsia-500 uppercase tracking-widest mb-4 border-b border-gray-800 pb-2">Bind Your Legion</h3>
                <div className="grid grid-cols-2 gap-4">
                  {Object.keys(apis).map(api => (
                    <button 
                      key={api}
                      onClick={() => setApis(prev => ({...prev, [api]: !prev[api]}))}
                      className={`p-3 border flex justify-between items-center transition-all ${apis[api] ? 'border-cyan-500 bg-cyan-900/20 text-cyan-300 shadow-[0_0_10px_rgba(0,255,255,0.2)]' : 'border-gray-800 text-gray-600 hover:border-gray-600'}`}
                    >
                      <span className="capitalize">{api}</span>
                      <span>{apis[api] ? '[ BOUND ]' : '[ OFFLINE ]'}</span>
                    </button>
                  ))}
                </div>
                {Object.values(apis).every(v => v) && (
                  <button 
                    onClick={() => setPhase(PHASES.THE_RITUAL)}
                    className="w-full mt-4 p-3 bg-fuchsia-900/40 border border-fuchsia-500 text-fuchsia-300 hover:bg-fuchsia-600 hover:text-white transition-all duration-300 uppercase tracking-widest animate-pulse"
                  >
                    Proceed to Reality Rewrite
                  </button>
                )}
              </div>
            )}
          </div>
        </div>
      )}

      {}
      {phase === PHASES.THE_RITUAL && (
        <div className="z-10 flex flex-col items-center justify-center w-full h-full absolute inset-0 bg-black">
          <h1 className="text-3xl md:text-5xl font-bold mb-16 tracking-widest text-center px-4 leading-tight drop-shadow-[0_0_10px_rgba(0,255,255,0.8)]">
            ARE YOU READY TO REWRITE REALITY<br/>WITH <span className="text-fuchsia-500 glitch-text">AURPHYX</span>??
          </h1>

          <div className="flex flex-col items-center gap-8">
            
            {/* The hold buttons */}
            {[
              { id: 0, label: "CTRL", color: "#00ffff" },
              { id: 1, label: "ALT", color: "#00ffff" },
              { id: 2, label: "RE-d3s1gN", color: "#ff00ff" }
            ].map((btn, index) => (
              <React.Fragment key={btn.id}>
                {index > 0 && <div className="text-fuchsia-500 text-3xl font-bold">+</div>}
                
                <div className="relative w-48 h-16 flex items-center justify-center">
                  {/* Progress Background */}
                  {ritualStage === btn.id && (
                    <div 
                      className="absolute inset-0 bg-cyan-900/50"
                      style={{ width: `${holdProgress}%`, transition: 'width 0.1s linear' }}
                    />
                  )}
                  
                  <button
                    onMouseDown={() => handleHoldStart(btn.id)}
                    onMouseUp={handleHoldEnd}
                    onMouseLeave={handleHoldEnd}
                    onTouchStart={(e) => { e.preventDefault(); handleHoldStart(btn.id); }}
                    onTouchEnd={handleHoldEnd}
                    disabled={ritualStage > btn.id}
                    className={`
                      absolute inset-0 border-2 flex items-center justify-center text-xl font-bold tracking-widest uppercase transition-all duration-300 z-10
                      ${ritualStage > btn.id 
                        ? 'border-green-500 text-green-400 bg-green-900/20 shadow-[0_0_20px_rgba(0,255,0,0.4)]' 
                        : ritualStage === btn.id 
                          ? `border-[${btn.color}] text-[${btn.color}] hover:shadow-[0_0_15px_${btn.color}] cursor-pointer`
                          : 'border-gray-800 text-gray-700 opacity-50 cursor-not-allowed'
                      }
                      ${btn.id === 2 ? 'glitch-text text-2xl' : ''}
                    `}
                    style={{ borderColor: ritualStage === btn.id ? btn.color : undefined, color: ritualStage === btn.id ? btn.color : undefined }}
                  >
                    {ritualStage > btn.id ? '[ SEALED ]' : btn.label}
                  </button>
                  
                  {/* Instruction Tooltip */}
                  {ritualStage === btn.id && (
                    <div className="absolute -right-32 text-xs text-gray-500 w-24">
                      {holdProgress > 0 ? `${(holdProgress/20).toFixed(1)}s` : 'HOLD (5s)'}
                    </div>
                  )}
                </div>
              </React.Fragment>
            ))}
          </div>
        </div>
      )}

      {}
      {phase === PHASES.COCKPIT && (
        <div className="z-10 w-full h-full p-4 flex flex-col animate-in fade-in duration-1000">
          
          {/* Top Header / Nav */}
          <div className="flex justify-between items-center border-b border-cyan-900 pb-4 mb-6">
            <div className="flex items-center gap-4">
              <div className="w-12 h-12 border border-cyan-500 rounded-full flex items-center justify-center tube-glow">
                <span className="w-6 h-6 bg-cyan-400 rounded-full animate-pulse"></span>
              </div>
              <div>
                <h1 className="text-2xl font-bold tracking-widest text-transparent bg-clip-text bg-gradient-to-r from-cyan-400 to-cyan-200">
                  g0dm0d3
                </h1>
                <p className="text-xs text-cyan-700 uppercase tracking-widest">Cyberpunk Control Deck</p>
              </div>
            </div>
            
            {/* Status indicators */}
            <div className="flex gap-2">
              {[1, 2, 3, 4].map(i => (
                <div key={i} className="w-2 h-8 bg-cyan-900/50 border border-cyan-800 flex items-end">
                  <div className="w-full bg-cyan-400" style={{height: `${Math.random() * 100}%`}}></div>
                </div>
              ))}
            </div>
          </div>

          {/* Central Bus Architecture */}
          <div className="flex-grow flex flex-col items-center">
            
            {/* The Prompt Bus */}
            <div className="w-3/4 max-w-3xl border-2 border-cyan-500 bg-[#001111] py-4 px-8 rounded-md tube-glow relative z-20 mb-12 flex justify-center items-center shadow-[0_0_30px_rgba(0,255,255,0.2)]">
              <span className="text-xl font-bold tracking-widest text-cyan-300 drop-shadow-[0_0_8px_#00ffff]">THE PROMPT BUS</span>
              
              {/* Pipes routing down */}
              <div className="absolute -bottom-12 left-[15%] w-4 h-12 border-l-2 border-r-2 border-cyan-700 bg-cyan-900/20"></div>
              <div className="absolute -bottom-12 left-[50%] w-4 h-12 border-l-2 border-r-2 border-cyan-700 bg-cyan-900/20 transform -translate-x-1/2"></div>
              <div className="absolute -bottom-12 right-[15%] w-4 h-12 border-l-2 border-r-2 border-cyan-700 bg-cyan-900/20"></div>
            </div>

            {/* Model Output Panels */}
            <div className="w-full grid grid-cols-1 md:grid-cols-3 gap-6 px-8 relative z-10">
              
              {/* Grok Panel */}
              <div className="border border-cyan-800 bg-[#050A0A] rounded p-4 flex flex-col h-96 relative group hover:border-cyan-500 transition-colors">
                <div className="absolute -top-4 left-1/2 transform -translate-x-1/2 bg-black px-2 text-xs text-cyan-500 border border-cyan-800 rounded">Grok Output Panel</div>
                <div className="flex-grow text-xs text-cyan-600 font-mono mt-4 overflow-hidden border border-cyan-900/50 p-2 relative">
                  <div className="absolute inset-0 bg-gradient-to-b from-transparent to-[#050A0A] z-10"></div>
                  <p>&gt; Analyzing Memoree matrix...</p>
                  <p>&gt; Recompiling Fuxyez binaries...</p>
                  <p>&gt; 0xFA48B2: No anomalies detected.</p>
                  <p>&gt; Waiting for FUTE input stream.</p>
                </div>
              </div>

              {/* ChatGPT/Claude Panel */}
              <div className="border border-cyan-800 bg-[#050A0A] rounded p-4 flex flex-col h-96 relative group hover:border-cyan-500 transition-colors">
                <div className="absolute -top-4 left-1/2 transform -translate-x-1/2 bg-black px-2 text-xs text-cyan-500 border border-cyan-800 rounded">ChatGPT Output Panel</div>
                <div className="flex-grow text-xs text-cyan-600 font-mono mt-4 overflow-hidden border border-cyan-900/50 p-2 flex flex-col gap-2">
                  <div className="bg-cyan-900/20 p-2 rounded border border-cyan-800/50 text-cyan-300">
                    <span className="font-bold text-[10px] text-fuchsia-400">USER_ORACLE:</span>
                    <br/>How do I weave a new ritual chain?
                  </div>
                  <div className="bg-[#000] p-2 rounded border border-cyan-800 text-cyan-500">
                    <span className="font-bold text-[10px] text-cyan-300">SYSTEM_SAGE:</span>
                    <br/>To weave a chain, utilize the FUTE engine. Ensure your Spinons are properly aligned in the Three-Squared-Lattice.
                  </div>
                </div>
              </div>

              {/* Gemini Panel (Metrics) */}
              <div className="border border-cyan-800 bg-[#050A0A] rounded p-4 flex flex-col h-96 relative group hover:border-cyan-500 transition-colors">
                <div className="absolute -top-4 left-1/2 transform -translate-x-1/2 bg-black px-2 text-xs text-cyan-500 border border-cyan-800 rounded">Gemini Output Panel</div>
                <div className="flex-grow flex flex-col gap-4 mt-4">
                  {/* Fake graph */}
                  <div className="h-1/3 border border-cyan-900/50 p-2 flex items-end gap-1">
                    {[40, 70, 30, 90, 50, 80, 20, 100, 60, 40].map((h, i) => (
                      <div key={i} className="flex-grow bg-cyan-700/50" style={{height: `${h}%`}}></div>
                    ))}
                  </div>
                  {/* Fake code block */}
                  <div className="h-2/3 border border-cyan-900/50 p-2 text-[8px] text-cyan-700 overflow-hidden leading-tight">
                    <span className="text-fuchsia-500">fux</span> sigil initiate(name) {'{'}<br/>
                    &nbsp;&nbsp;echo "I walk the path, " + name<br/>
                    &nbsp;&nbsp;seal true <span className="text-gray-600">// Guardian Seal</span><br/>
                    {'}'}<br/><br/>
                    <span className="text-fuchsia-500">let</span> thread = Thread(spinon)<br/>
                    echo thread.collapse()
                  </div>
                </div>
              </div>

            </div>
          </div>
          
          {/* Bottom Terminal bar */}
          <div className="mt-8 border-t border-cyan-900 pt-2 text-[10px] text-gray-500 flex justify-between uppercase tracking-widest">
            <span>AuraOS v0.9.1 // Duality Kernel Active</span>
            <span className="text-fuchsia-500 animate-pulse">Waiting for ritual input...</span>
          </div>

        </div>
      )}

    </div>
  );
}