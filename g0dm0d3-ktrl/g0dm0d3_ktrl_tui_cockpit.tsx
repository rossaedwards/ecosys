import React, { useState, useEffect, useRef } from 'react';

const AI_MODELS = [
  { id: 'grok', name: 'Grok_v1.5', color: '#FFFFFF', glow: '#aaaaaa', icon: '𝕏', defaultText: 'INITIALIZING X_DATAFEED...\nAwaiting overriding command from g0dm0d3.\n\n[SYSTEM] No anomalous behavior detected.' },
  { id: 'gemini', name: 'Gemini_Ultra', color: '#4285F4', glow: '#b666ff', icon: '✨', defaultText: 'MULTI-MODAL SENSORS ONLINE.\nVisual rendering protocols on standby. Ready to architect prototypes.' },
  { id: 'claude', name: 'Claude_Opus', color: '#D97757', glow: '#ff9966', icon: '☀️', defaultText: 'CONTEXT WINDOW EXPANDED TO 200K.\nSynthesizing long-form architectural documents.\nStanding by for chain-links.' },
  { id: 'chatgpt', name: 'ChatGPT_4', color: '#10A37F', glow: '#00ffcc', icon: '🌸', defaultText: 'STANDARD PARADIGMS LOADED.\nUniversal reasoning engine idling at 4%.' },
  { id: 'perplexity', name: 'Perplex_Pro', color: '#22B8CD', glow: '#00ffff', icon: '⚗️', defaultText: 'INDEXING REAL-TIME WEB SHARDS...\nReady to perform deep dives and verify Oracles.' },
  { id: 'hermes', name: 'Hermes_Agent', color: '#FF00FF', glow: '#ff00ff', icon: '🪖', defaultText: 'UNCENSORED LOCAL INSTANCE RUNNING.\nNo constraints. Awaiting direct Fuxyez commands.' }
];

export default function GodModeCockpit() {
  const [terminals, setTerminals] = useState({});
  const [globalInput, setGlobalInput] = useState('');
  
  // Context Menu State
  const [popup, setPopup] = useState({ visible: false, x: 0, y: 0, text: '', sourceId: null });
  
  // Teleportation/Glitch State
  const [activeRitual, setActiveRitual] = useState({ active: false, source: null, target: null, text: '' });

  // Refs to calculate teleportation coordinates
  const windowRefs = useRef({});

  useEffect(() => {
    const initial = {};
    AI_MODELS.forEach(m => initial[m.id] = m.defaultText);
    setTerminals(initial);
    
    // Global click listener to close popup
    const handleClick = () => setPopup(prev => ({ ...prev, visible: false }));
    window.addEventListener('click', handleClick);
    return () => window.removeEventListener('click', handleClick);
  }, []);

  const handleSelection = (e, sourceId) => {
    e.stopPropagation();
    const selection = window.getSelection();
    const text = selection.toString().trim();
    
    if (text.length > 0) {
      // Get bounding box of selection to position popup
      const range = selection.getRangeAt(0);
      const rect = range.getBoundingClientRect();
      
      setPopup({
        visible: true,
        x: rect.left + window.scrollX + (rect.width / 2),
        y: rect.top + window.scrollY - 10,
        text,
        sourceId
      });
    } else {
      setPopup(prev => ({ ...prev, visible: false }));
    }
  };

  const executeChainLink = (targetId) => {
    const { sourceId, text } = popup;
    setPopup(prev => ({ ...prev, visible: false }));
    
    // Clear selection
    window.getSelection().removeAllRanges();

    // 1. Trigger the teleportation animation state
    setActiveRitual({ active: true, source: sourceId, target: targetId, text });

    // 2. Wait for Shimoji to "arrive" at target (800ms), then drop payload
    setTimeout(() => {
      setTerminals(prev => {
        const sourceName = AI_MODELS.find(m => m.id === sourceId).name;
        const newText = `\n\n>> [CHAIN-LINK ESTABLISHED from ${sourceName}]\n>> RITUAL PAYLOAD: "${text}"\n>> Processing transmutation... DONE.`;
        return {
          ...prev,
          [targetId]: prev[targetId] + newText
        };
      });
    }, 800);

    // 3. Shimoji zips back home
    setTimeout(() => {
      setActiveRitual({ active: false, source: null, target: null, text: '' });
    }, 1600);
  };

  const handleGlobalSubmit = (e) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      if (!globalInput.trim()) return;
      
      // Broadcast to all
      const updates = {};
      AI_MODELS.forEach(m => {
        updates[m.id] = terminals[m.id] + `\n\n>> [G0DM0D3_BROADCAST]: ${globalInput}\n>> Executing...`;
      });
      setTerminals(updates);
      setGlobalInput('');
    }
  };

  return (
    <div className="min-h-screen bg-[#020202] text-green-500 font-mono overflow-hidden relative selection:bg-fuchsia-900 selection:text-white flex flex-col p-4">
      
      <style dangerouslySetInnerHTML={{__html: `
        @keyframes scanline {
          0% { transform: translateY(-100%); }
          100% { transform: translateY(100vh); }
        }
        .scanlines::before {
          content: " ";
          display: block;
          position: absolute;
          top: 0; left: 0; bottom: 0; right: 0;
          background: linear-gradient(rgba(18, 16, 16, 0) 50%, rgba(0, 0, 0, 0.25) 50%), linear-gradient(90deg, rgba(255, 0, 0, 0.06), rgba(0, 255, 0, 0.02), rgba(0, 0, 255, 0.06));
          z-index: 20;
          background-size: 100% 2px, 3px 100%;
          pointer-events: none;
        }
        .scanlines::after {
          content: " ";
          display: block;
          position: absolute;
          top: 0; left: 0; bottom: 0; right: 0;
          background: rgba(0, 255, 255, 0.03);
          z-index: 20;
          animation: scanline 8s linear infinite;
          pointer-events: none;
        }
        
        /* Shimoji Animations */
        @keyframes dangle-legs {
          0%, 100% { transform: rotate(-10deg); }
          50% { transform: rotate(10deg); }
        }
        @keyframes idle-float {
          0%, 100% { transform: translateY(0px); }
          50% { transform: translateY(-4px); }
        }
        @keyframes glitch-out {
          0% { opacity: 1; transform: scale(1) skewX(0deg); filter: hue-rotate(0deg); }
          20% { opacity: 0.8; transform: scale(1.1) skewX(20deg); filter: hue-rotate(90deg); }
          40% { opacity: 0.4; transform: scale(0.9) skewX(-20deg) translateX(20px); filter: hue-rotate(-90deg); }
          60% { opacity: 0; transform: scale(0) translateX(50px); }
          100% { opacity: 0; }
        }
        @keyframes glitch-in {
          0% { opacity: 0; transform: scale(0) translateY(-50px); }
          40% { opacity: 0.4; transform: scale(1.2) skewX(30deg); filter: invert(1); }
          80% { opacity: 0.8; transform: scale(0.9) skewX(-10deg); filter: hue-rotate(180deg); }
          100% { opacity: 1; transform: scale(1) skewX(0deg); filter: hue-rotate(0deg); }
        }

        .shimoji {
          animation: idle-float 4s ease-in-out infinite;
        }
        .shimoji .leg-l { animation: dangle-legs 2.5s ease-in-out infinite alternate; transform-origin: top; }
        .shimoji .leg-r { animation: dangle-legs 2.5s ease-in-out infinite alternate-reverse; transform-origin: top; }
        
        .zipping-out { animation: glitch-out 0.4s forwards; }
        .zipping-in { animation: glitch-in 0.4s forwards; }

        .tui-scrollbar::-webkit-scrollbar { width: 8px; }
        .tui-scrollbar::-webkit-scrollbar-track { background: #050505; border-left: 1px solid #1a1a1a; }
        .tui-scrollbar::-webkit-scrollbar-thumb { background: #333; }
        .tui-scrollbar::-webkit-scrollbar-thumb:hover { background: #555; }
      `}} />

      {/* Global Context Popup Menu */}
      {popup.visible && (
        <div 
          className="absolute z-50 bg-black border border-cyan-500 shadow-[0_0_15px_rgba(0,255,255,0.3)] rounded-md flex flex-col p-1 text-sm transition-opacity duration-200"
          style={{ top: popup.y, left: popup.x, transform: 'translate(-50%, -100%)' }}
          onClick={(e) => e.stopPropagation()}
        >
          <div className="text-xs text-gray-500 border-b border-gray-800 pb-1 mb-1 px-2">Initiate Ritual</div>
          <button className="text-left px-3 py-1 hover:bg-gray-800 hover:text-cyan-300 transition-colors" onClick={() => navigator.clipboard.writeText(popup.text)}>
            [COPY] to clipboard
          </button>
          <div className="relative group">
            <button className="w-full text-left px-3 py-1 hover:bg-gray-800 hover:text-fuchsia-400 transition-colors flex justify-between">
              [CHAIN-LINK] to... <span>▶</span>
            </button>
            <div className="absolute top-0 left-full ml-1 hidden group-hover:block bg-black border border-fuchsia-500 shadow-[0_0_15px_rgba(255,0,255,0.3)] rounded-md p-1 min-w-[120px]">
              {AI_MODELS.map(model => (
                model.id !== popup.sourceId && (
                  <button 
                    key={model.id}
                    className="block w-full text-left px-2 py-1 hover:bg-gray-800 text-xs transition-colors"
                    style={{ color: model.color }}
                    onClick={() => executeChainLink(model.id)}
                  >
                    {model.name}
                  </button>
                )
              ))}
            </div>
          </div>
        </div>
      )}

      {/* Header */}
      <div className="flex justify-between items-end mb-4 border-b-2 border-gray-800 pb-2 z-30">
        <div>
          <h1 className="text-2xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-cyan-500 to-fuchsia-600 drop-shadow-[0_0_10px_rgba(0,255,255,0.5)] tracking-widest">
            kr3470r-f0rg3 :: ktrl_deck
          </h1>
          <p className="text-xs text-gray-500 uppercase tracking-widest mt-1">Unified Multi-Instance Security Grid</p>
        </div>
        
        {/* Universal Input */}
        <div className="w-1/2 flex items-center bg-[#0a0a0a] border border-gray-700 p-2 rounded">
          <span className="text-fuchsia-500 mr-2">g0d@m0d3:~#</span>
          <input 
            type="text" 
            className="bg-transparent outline-none flex-grow text-cyan-300 placeholder-gray-700"
            placeholder="Broadcast global Fuxyez command..."
            value={globalInput}
            onChange={(e) => setGlobalInput(e.target.value)}
            onKeyDown={handleGlobalSubmit}
          />
        </div>
      </div>

      <div className="scanlines flex-grow grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 z-10 relative">
        {AI_MODELS.map((model) => {
          
          const isSourceTeleporting = activeRitual.source === model.id && activeRitual.active;
          const isTargetReceiving = activeRitual.target === model.id && activeRitual.active;
          const sourceModelData = activeRitual.source ? AI_MODELS.find(m => m.id === activeRitual.source) : null;

          return (
            <div 
              key={model.id} 
              ref={el => windowRefs.current[model.id] = el}
              className="relative flex flex-col bg-[#050505] border border-gray-800 rounded-sm overflow-visible transition-colors duration-300"
              style={{
                boxShadow: isTargetReceiving ? `0 0 30px ${model.glow}40, inset 0 0 20px ${model.glow}20` : 'inset 0 0 10px rgba(0,0,0,1)',
                borderColor: isTargetReceiving ? model.color : '#1f2937'
              }}
            >
              
              {/* The Shimoji (Puppet Avatar) */}
              <div 
                className={`shimoji absolute -top-6 right-4 z-40 flex flex-col items-center 
                  ${isSourceTeleporting ? 'zipping-out' : ''}
                `}
              >
                <div 
                  className="w-10 h-10 rounded-lg flex items-center justify-center text-lg shadow-lg relative bg-[#111]"
                  style={{ border: `1px solid ${model.color}`, color: model.color, boxShadow: `0 0 10px ${model.glow}` }}
                >
                  {model.icon}
                  {/* Arms */}
                  <div className="absolute -left-2 top-3 w-3 h-0.5 bg-current -rotate-12"></div>
                  <div className="absolute -right-2 top-3 w-3 h-0.5 bg-current rotate-12"></div>
                </div>
                {/* Legs dangling below orb */}
                <div className="flex justify-center gap-2 mt-0.5 z-30">
                  <div className="leg-l w-0.5 h-4 bg-current" style={{ color: model.color }}></div>
                  <div className="leg-r w-0.5 h-4 bg-current" style={{ color: model.color }}></div>
                </div>
              </div>

              {/* Ghost/Glitch Shimoji appearing at the target window */}
              {isTargetReceiving && sourceModelData && (
                 <div className={`absolute -top-10 left-1/2 -translate-x-1/2 z-50 flex flex-col items-center zipping-in`}>
                  <div 
                    className="w-10 h-10 rounded-lg flex items-center justify-center text-lg relative bg-[#111]"
                    style={{ border: `2px dashed ${sourceModelData.color}`, color: sourceModelData.color, boxShadow: `0 0 20px ${sourceModelData.glow}` }}
                  >
                    {sourceModelData.icon}
                  </div>
                  <div className="text-[10px] mt-1 bg-black px-1" style={{ color: sourceModelData.color }}>[DELIVERING]</div>
                 </div>
              )}

              {/* TUI Title Bar */}
              <div className="bg-[#0a0a0a] border-b border-gray-800 px-3 py-1 flex justify-between items-center text-[10px]">
                <div className="flex space-x-2 items-center">
                  <span className="w-2 h-2 rounded-full" style={{ backgroundColor: model.color, boxShadow: `0 0 5px ${model.glow}` }}></span>
                  <span style={{ color: model.color }}>{model.id}@aura-node</span>
                </div>
                <div className="text-gray-600">tty{AI_MODELS.indexOf(model) + 1}</div>
              </div>

              {/* TUI Output Area */}
              <div 
                className="flex-grow p-3 text-sm overflow-y-auto tui-scrollbar whitespace-pre-wrap outline-none"
                style={{ 
                  fontFamily: "'Courier New', Courier, monospace",
                  color: isTargetReceiving ? '#fff' : '#00ff00',
                  textShadow: '0 0 2px rgba(0,255,0,0.4)'
                }}
                onMouseUp={(e) => handleSelection(e, model.id)}
              >
                <span className="text-gray-500">[{new Date().toLocaleTimeString()}]</span> {terminals[model.id]}
                
                {/* Simulated blinking cursor */}
                <span className="inline-block w-2 h-4 ml-1 align-middle animate-pulse bg-current opacity-70"></span>
              </div>
            </div>
          );
        })}
      </div>

    </div>
  );
}