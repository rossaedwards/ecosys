import React, { useState, useEffect, useRef } from 'react';
import { 
  Terminal, Cpu, Globe, Book, Sparkles, Zap, Shield, 
  Settings, Play, Download, Search, Command, Activity, User,
  GitFork, GitBranch, FolderGit2, Plug, Maximize2, Minimize2, StopCircle, 
  RefreshCw, Network, Database, Copy, CheckCircle2, XCircle
} from 'lucide-react';

// --- STYLES & THEME CONSTANTS ---
const colors = {
  bg: 'bg-slate-950',
  panel: 'bg-slate-900',
  border: 'border-emerald-500/30',
  glow: 'shadow-[0_0_15px_rgba(16,185,129,0.15)]',
  textMain: 'text-emerald-400',
  textMuted: 'text-emerald-400/60',
  accent1: 'text-fuchsia-500', // Chaos
  accent2: 'text-cyan-400',    // Bliss
};

// --- MOCK DATA FOR PLUGINS (PANTHEON) ---
const PANTHEON_PLUGINS = [
  { id: 'p1', name: 'Valkryx Sentinel', category: 'Security', description: 'Quantum anomaly detection. Watches for drift in AI responses and triggers Umbryx seals.', price: 'Free', installed: true, icon: <Shield size={20} className="text-cyan-400" /> },
  { id: 'p2', name: 'Umbryx Guardian', category: 'Governance', description: 'Enforces containment protocols and dual-writes to the Ineffable Ledger.', price: 'Free', installed: true, icon: <Activity size={20} className="text-fuchsia-500" /> },
  { id: 'p3', name: 'Mythic Codex Co-Author', category: 'Lore', description: 'Generative lore engine that co-writes world-building and governance protocols.', price: '80% Rev Share', installed: false, icon: <Book size={20} className="text-emerald-400" /> },
  { id: 'p4', name: 'Fuxyez Compiler', category: 'Core', description: 'Quantum Echoes compilation and predictive debugging for AuraOS integrations.', price: 'Free', installed: false, icon: <Cpu size={20} className="text-purple-400" /> },
  { id: 'p5', name: 'OmniZen OCM Framework', category: 'Compliance', description: 'Regulatory-first platform integration for state and federal harmonization.', price: 'Premium', installed: false, icon: <Globe size={20} className="text-blue-400" /> }
];

// --- AI CONFIGURATIONS ---
const AI_ENDPOINTS = [
  { id: 'grok', name: 'Grok', color: 'text-orange-500', border: 'border-orange-500/30', promptInstruction: 'You are Grok. Be witty, slightly rebellious, deeply insightful, and use a cyberpunk/hacker tone. Keep it concise.' },
  { id: 'chatgpt', name: 'ChatGPT', color: 'text-teal-400', border: 'border-teal-400/30', promptInstruction: 'You are ChatGPT. Be highly structured, analytical, professional, and use bullet points where necessary. Keep it concise.' },
  { id: 'claude', name: 'Claude', color: 'text-amber-600', border: 'border-amber-600/30', promptInstruction: 'You are Claude. Be thoughtful, nuanced, ethical, and carefully consider the implications of the prompt. Keep it concise.' },
  { id: 'gemini', name: 'Gemini', color: 'text-blue-400', border: 'border-blue-400/30', promptInstruction: 'You are Gemini. Be highly capable, creative, forward-thinking, and emphasize innovation and integration. Keep it concise.' }
];

// --- MAIN APPLICATION ---
export default function App() {
  const [activeTab, setActiveTab] = useState('response'); // console, response, forkz, projectz, codex, agora
  const [prompt, setPrompt] = useState('');
  const [isBroadcasting, setIsBroadcasting] = useState(false);
  const [responses, setResponses] = useState({ grok: '', chatgpt: '', claude: '', gemini: '' });
  const [codexLog, setCodexLog] = useState([
    { time: new Date().toLocaleTimeString(), msg: 'SYSTEM BOOT: AuraOS core initialized.' },
    { time: new Date().toLocaleTimeString(), msg: 'MODULE LOAD: Valkryx Sentinel online.' },
    { time: new Date().toLocaleTimeString(), msg: 'STATUS: Ready to receive divine syntax. F0rg3d with L0v3.' }
  ]);
  const [plugins, setPlugins] = useState(PANTHEON_PLUGINS);
  
  // New State Features
  const [maximizedAi, setMaximizedAi] = useState(null);
  const [connectors, setConnectors] = useState({
    memoree: true,
    lmstudio: true,
    ollama: false,
    llamacpp: false,
    hf: false
  });

  const codexEndRef = useRef(null);

  // Auto-scroll codex
  useEffect(() => {
    if (codexEndRef.current) {
      codexEndRef.current.scrollIntoView({ behavior: 'smooth' });
    }
  }, [codexLog]);

  const appendToCodex = (msg) => {
    setCodexLog(prev => [...prev, { time: new Date().toLocaleTimeString(), msg }]);
  };

  // API Call Simulator
  const callAI = async (userPrompt, aiConfig) => {
    const apiKey = ""; // Provided by execution environment
    try {
      const response = await fetch(`https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash-preview-09-2025:generateContent?key=${apiKey}`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          contents: [{ parts: [{ text: userPrompt }] }],
          systemInstruction: { parts: [{ text: aiConfig.promptInstruction }] }
        })
      });

      if (!response.ok) throw new Error(`API Error: ${response.status}`);
      const data = await response.json();
      return data.candidates?.[0]?.content?.parts?.[0]?.text || 'No response generated.';
    } catch (error) {
      return `[CONNECTION SEVERED] ${error.message}`;
    }
  };

  const handleBroadcast = async () => {
    if (!prompt.trim()) return;
    
    setIsBroadcasting(true);
    setResponses({ grok: 'Summoning...', chatgpt: 'Summoning...', claude: 'Summoning...', gemini: 'Summoning...' });
    appendToCodex(`BROADCAST INITIATED: "${prompt.substring(0, 30)}..."`);

    const promises = AI_ENDPOINTS.map(async (ai) => {
      const resText = await callAI(prompt, ai);
      setResponses(prev => ({ ...prev, [ai.id]: resText }));
      appendToCodex(`ECHO RECEIVED: ${ai.name} responded.`);
    });

    await Promise.all(promises);
    setIsBroadcasting(false);
    appendToCodex('BROADCAST COMPLETE: All seals intact.');
  };

  // "Stop-Update & Adopt Layout" Mechanic
  const handleAdoptLayout = (sourceAi) => {
    appendToCodex(`[UNIVERSAL SYNC] Broadcasting ${sourceAi.name}'s architectural layout to all active endpoints...`);
    const newResponses = { ...responses };
    
    AI_ENDPOINTS.forEach(target => {
      if (target.id !== sourceAi.id) {
        newResponses[target.id] = `[STOP-UPDATE INITIATED]\n\nIntercepting data stream...\nPurging previous schema...\n\nRe-aligning syntax and UI formatting to match ${sourceAi.name}'s layout parameters.\n\n<Awaiting synchronized task completion...>`;
      }
    });
    setResponses(newResponses);
  };

  const togglePlugin = (id) => {
    setPlugins(plugins.map(p => {
      if (p.id === id) {
        const newStatus = !p.installed;
        appendToCodex(`AGORA MODULE ${newStatus ? 'INSTALLED' : 'REMOVED'}: ${p.name}`);
        return { ...p, installed: newStatus };
      }
      return p;
    }));
  };

  const toggleConnector = (key) => {
    setConnectors(prev => {
      const next = { ...prev, [key]: !prev[key] };
      appendToCodex(`CONNECTOR UPDATE: ${key.toUpperCase()} is now ${next[key] ? 'ONLINE' : 'OFFLINE'}.`);
      return next;
    });
  };

  return (
    <div className={`min-h-screen ${colors.bg} text-gray-200 font-sans selection:bg-emerald-500/30 flex flex-col`}>
      {/* HEADER */}
      <header className={`h-16 ${colors.panel} border-b ${colors.border} flex items-center justify-between px-6 shrink-0`}>
        <div className="flex items-center gap-3">
          <div className="relative">
            <Zap className={colors.accent1} size={28} />
            <div className="absolute inset-0 bg-fuchsia-500 blur-md opacity-40"></div>
          </div>
          <div>
            <h1 className="text-xl font-bold tracking-widest text-white uppercase drop-shadow-md">
              g0dm0d3 <span className="font-light text-emerald-500">core</span>
            </h1>
            <p className="text-[10px] text-emerald-400/50 uppercase tracking-widest -mt-1">F0rg3d with L0v3</p>
          </div>
        </div>
        
        <div className="flex gap-1">
          {['console', 'response', 'forkz', 'projectz', 'codex', 'agora'].map(tab => (
            <button
              key={tab}
              onClick={() => setActiveTab(tab)}
              className={`px-3 py-2 rounded uppercase text-xs tracking-wider transition-all duration-300 border ${
                activeTab === tab 
                  ? 'border-emerald-500 bg-emerald-500/10 text-emerald-400 shadow-[0_0_10px_rgba(16,185,129,0.2)]' 
                  : 'border-transparent text-gray-500 hover:text-emerald-400 hover:bg-slate-800'
              }`}
            >
              {tab}
            </button>
          ))}
        </div>
        
        <div className="flex items-center gap-4">
          <div className="flex items-center gap-3 border-r border-slate-700 pr-4">
            <Database size={16} className={connectors.memoree ? "text-emerald-400" : "text-slate-600"} title="Memoree Sync" />
            <Plug size={16} className={Object.values(connectors).some(Boolean) ? "text-cyan-400" : "text-slate-600"} title="Universal Connectors" />
          </div>
          <div className="flex items-center gap-2">
            <span className="relative flex h-3 w-3">
              <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"></span>
              <span className="relative inline-flex rounded-full h-3 w-3 bg-emerald-500"></span>
            </span>
            <span className="text-xs text-emerald-400 uppercase tracking-widest">Secure</span>
          </div>
        </div>
      </header>

      {/* MAIN CONTENT AREA */}
      <main className="flex-1 overflow-hidden relative flex flex-col">
        
        {/* --- TAB 1: CONSOLE (UNIVERSAL CONNECTOR & DASHBOARD) --- */}
        {activeTab === 'console' && (
          <div className="h-full p-6 overflow-y-auto animate-in fade-in duration-300">
            <div className="flex items-center gap-3 mb-8">
              <Network className={colors.accent2} size={32} />
              <div>
                <h2 className="text-3xl font-bold text-white uppercase tracking-widest">Universal Connector</h2>
                <p className="text-slate-400 text-sm">Hook into local daemons, cloud APIs, and sovereign memory substrates.</p>
              </div>
            </div>

            <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
              {/* Connector Panel */}
              <div className={`${colors.panel} border ${colors.border} p-6 rounded-xl relative overflow-hidden group`}>
                <div className="absolute top-0 left-0 w-1 h-full bg-cyan-500"></div>
                <h3 className="text-xl font-bold text-white mb-6 flex items-center gap-2">
                  <Plug className="text-cyan-400" size={20}/> Active Hooks
                </h3>
                <div className="space-y-4">
                  {[
                    { key: 'memoree', name: 'Memoree Substrate', icon: <Database size={18}/>, desc: 'Sovereign Memory Ledger & Context Engine' },
                    { key: 'lmstudio', name: 'LM Studio', icon: <Cpu size={18}/>, desc: 'Local Inference (OpenAI Compatible)' },
                    { key: 'ollama', name: 'Ollama', icon: <Terminal size={18}/>, desc: 'Local CLI Daemon for LLaMA models' },
                    { key: 'llamacpp', name: 'llama.cpp', icon: <Cpu size={18}/>, desc: 'Raw C/C++ Inference Backend' },
                    { key: 'hf', name: 'HuggingFace.co', icon: <Globe size={18}/>, desc: 'Cloud Hub inference endpoints' },
                  ].map(connector => (
                    <div key={connector.key} className="flex items-center justify-between p-3 bg-slate-950/50 border border-slate-800 rounded-lg hover:border-cyan-500/30 transition-all">
                      <div className="flex items-center gap-3">
                        <div className={`p-2 rounded-md ${connectors[connector.key] ? 'bg-cyan-500/20 text-cyan-400' : 'bg-slate-800 text-slate-500'}`}>
                          {connector.icon}
                        </div>
                        <div>
                          <h4 className="text-slate-200 font-bold text-sm">{connector.name}</h4>
                          <p className="text-slate-500 text-xs">{connector.desc}</p>
                        </div>
                      </div>
                      <button 
                        onClick={() => toggleConnector(connector.key)}
                        className={`px-3 py-1 text-xs font-bold uppercase rounded border transition-all ${
                          connectors[connector.key] ? 'bg-emerald-500/20 text-emerald-400 border-emerald-500/50' : 'bg-transparent text-slate-500 border-slate-700 hover:text-white'
                        }`}
                      >
                        {connectors[connector.key] ? 'Connected' : 'Connect'}
                      </button>
                    </div>
                  ))}
                </div>
              </div>

              {/* Individual App Launchers */}
              <div className={`${colors.panel} border border-fuchsia-500/30 p-6 rounded-xl relative overflow-hidden group`}>
                <div className="absolute top-0 left-0 w-1 h-full bg-fuchsia-500"></div>
                <h3 className="text-xl font-bold text-white mb-2 flex items-center gap-2">
                  <Maximize2 className="text-fuchsia-400" size={20}/> Endpoint Applications
                </h3>
                <p className="text-sm text-slate-400 mb-6">Launch dedicated, standalone desktop interfaces or API proxies for individual models.</p>
                <div className="space-y-3">
                  {AI_ENDPOINTS.map(ai => (
                    <div key={ai.id} className="flex justify-between items-center bg-slate-950 p-4 rounded-lg border border-slate-800">
                      <div className="flex items-center gap-3">
                        <Cpu size={18} className={ai.color} />
                        <span className={`font-bold uppercase tracking-wider text-sm ${ai.color}`}>{ai.name} NODE</span>
                      </div>
                      <div className="flex gap-2">
                        <button className="text-xs bg-slate-800 hover:bg-fuchsia-500/20 hover:text-fuchsia-400 border border-transparent hover:border-fuchsia-500/30 px-3 py-2 rounded text-white transition-all uppercase tracking-wider font-bold">
                          Desktop App
                        </button>
                        <button className="text-xs bg-slate-800 hover:bg-cyan-500/20 hover:text-cyan-400 border border-transparent hover:border-cyan-500/30 px-3 py-2 rounded text-white transition-all uppercase tracking-wider font-bold">
                          CLI-API
                        </button>
                      </div>
                    </div>
                  ))}
                </div>
              </div>
            </div>
          </div>
        )}

        {/* --- TAB 2: RESPONSE (THE PROMPT BUS & AI GRID) --- */}
        {activeTab === 'response' && (
          <div className="flex-1 flex flex-col p-6 animate-in fade-in duration-300">
            {/* Prompt Bar (Stays on top of responses) */}
            <div className={`mb-6 p-1 rounded-xl bg-gradient-to-r from-emerald-500/20 via-fuchsia-500/20 to-cyan-500/20 ${colors.glow} shrink-0`}>
              <div className={`${colors.panel} rounded-lg flex items-center p-2`}>
                <Command className="text-emerald-500 ml-3 mr-4" size={24} />
                <input
                  type="text"
                  value={prompt}
                  onChange={(e) => setPrompt(e.target.value)}
                  onKeyDown={(e) => e.key === 'Enter' && handleBroadcast()}
                  placeholder="Broadcast your divine command into the multiverse..."
                  className="flex-1 bg-transparent border-none outline-none text-emerald-50 placeholder:text-emerald-900/50 text-lg font-mono"
                  disabled={isBroadcasting}
                />
                <button
                  onClick={handleBroadcast}
                  disabled={isBroadcasting || !prompt.trim()}
                  className={`ml-4 px-6 py-3 rounded-lg font-bold uppercase tracking-wider transition-all ${
                    isBroadcasting 
                      ? 'bg-slate-800 text-slate-500 cursor-not-allowed'
                      : 'bg-emerald-500/20 text-emerald-400 hover:bg-emerald-500 hover:text-slate-900 border border-emerald-500/50 hover:shadow-[0_0_15px_rgba(16,185,129,0.5)]'
                  }`}
                >
                  {isBroadcasting ? 'Broadcasting...' : 'Command All'}
                </button>
              </div>
            </div>

            {/* Live Response Grid (Supports Maximization) */}
            <div className={`flex-1 grid gap-4 overflow-hidden ${maximizedAi ? 'grid-cols-1' : 'grid-cols-1 md:grid-cols-2 lg:grid-cols-4'}`}>
              {AI_ENDPOINTS.filter(ai => maximizedAi ? ai.id === maximizedAi : true).map(ai => (
                <div key={ai.id} className={`${colors.panel} border ${ai.border} rounded-xl flex flex-col overflow-hidden relative group transition-all duration-300`}>
                  
                  {/* Header Bar */}
                  <div className={`p-3 border-b border-slate-800 flex justify-between items-center bg-slate-950/50 shrink-0`}>
                    <div className="flex items-center gap-2">
                      <Cpu size={16} className={ai.color} />
                      <h3 className={`font-mono font-bold tracking-widest uppercase text-sm ${ai.color}`}>
                        {ai.name}
                      </h3>
                    </div>
                    <div className="flex gap-2">
                      <button 
                        onClick={() => handleAdoptLayout(ai)}
                        title={`Send ${ai.name}'s layout to all other models (Stop-Update)`}
                        className="p-1.5 text-slate-500 hover:text-fuchsia-400 hover:bg-fuchsia-500/10 rounded transition-colors"
                      >
                        <RefreshCw size={14} />
                      </button>
                      <button 
                        className="p-1.5 text-slate-500 hover:text-cyan-400 hover:bg-cyan-500/10 rounded transition-colors"
                        title="Copy to clipboard"
                      >
                        <Copy size={14} />
                      </button>
                      <button 
                        onClick={() => setMaximizedAi(maximizedAi === ai.id ? null : ai.id)}
                        className="p-1.5 text-slate-500 hover:text-white hover:bg-slate-800 rounded transition-colors"
                        title={maximizedAi === ai.id ? "Minimize" : "Maximize Node"}
                      >
                        {maximizedAi === ai.id ? <Minimize2 size={14} /> : <Maximize2 size={14} />}
                      </button>
                    </div>
                  </div>
                  
                  {/* Content Area */}
                  <div className="flex-1 p-4 overflow-y-auto font-mono text-sm leading-relaxed text-slate-300">
                    {responses[ai.id] ? (
                      <div className={
                        responses[ai.id] === 'Summoning...' || responses[ai.id].includes('[STOP-UPDATE') 
                          ? 'animate-pulse text-fuchsia-400/80 font-bold' 
                          : ''
                      }>
                        {responses[ai.id].split('\n').map((line, i) => (
                          <p key={i} className="mb-2 break-words">{line}</p>
                        ))}
                      </div>
                    ) : (
                      <div className="h-full flex flex-col items-center justify-center text-slate-700 space-y-4">
                        <Terminal size={32} className="opacity-20" />
                        <p className="uppercase tracking-widest text-xs">Awaiting input...</p>
                      </div>
                    )}
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}

        {/* --- TAB 3: FORKZ (BRANCH VIEWER) --- */}
        {activeTab === 'forkz' && (
          <div className="h-full p-6 animate-in fade-in duration-300 flex gap-6">
            <div className={`w-64 ${colors.panel} border ${colors.border} rounded-xl p-4 flex flex-col shrink-0`}>
               <h3 className="text-white font-bold mb-4 flex items-center gap-2 uppercase tracking-widest text-sm">
                 <GitBranch className="text-emerald-400" size={18}/> Timelines
               </h3>
               <ul className="space-y-2 font-mono text-xs">
                  <li className="bg-emerald-500/20 text-emerald-400 border border-emerald-500/50 p-2.5 rounded cursor-pointer flex justify-between items-center">
                    <span>main (active)</span> <CheckCircle2 size={14}/>
                  </li>
                  <li className="text-slate-400 border border-transparent hover:border-slate-700 hover:bg-slate-800 p-2.5 rounded cursor-pointer transition-all">
                    mythic-lore-shift
                  </li>
                  <li className="text-slate-400 border border-transparent hover:border-slate-700 hover:bg-slate-800 p-2.5 rounded cursor-pointer transition-all">
                    experiment-ui-layout
                  </li>
               </ul>
               <button className="mt-auto border border-emerald-500/30 text-emerald-400 p-2 rounded-lg text-xs uppercase font-bold hover:bg-emerald-500 hover:text-black transition-all">
                 + New Fork
               </button>
            </div>
            
            <div className={`flex-1 ${colors.panel} border ${colors.border} rounded-xl p-8 flex flex-col items-center justify-center text-center relative overflow-hidden`}>
               <div className="absolute inset-0 bg-[radial-gradient(ellipse_at_center,_var(--tw-gradient-stops))] from-emerald-900/20 via-slate-900 to-transparent"></div>
               <GitFork size={64} className="text-slate-700 mb-6 relative z-10" />
               <h2 className="text-2xl text-white font-bold mb-3 relative z-10">Multiverse Branch Management</h2>
               <p className="text-slate-400 max-w-md relative z-10">
                 Select a timeline on the left to view prompt divergences. Forking allows you to split sessions and explore different creative vectors simultaneously without polluting the main Memoree ledger.
               </p>
            </div>
          </div>
        )}

        {/* --- TAB 4: PROJECTZ (ADMINISTRATION) --- */}
        {activeTab === 'projectz' && (
          <div className="h-full p-6 overflow-y-auto animate-in fade-in duration-300">
             <div className="flex items-center gap-3 mb-8">
              <FolderGit2 className={colors.accent2} size={32} />
              <div>
                <h2 className="text-3xl font-bold text-white uppercase tracking-widest">Project Realms</h2>
                <p className="text-slate-400 text-sm">Manage interconnected environments across the Aurphyx ecosystem.</p>
              </div>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
              {[
                { name: 'g0dm0d3-core', status: 'Active', dualities: ['unity/multiplicity'], desc: 'Multi-AI orchestration admin console with limitless integration.' },
                { name: 'Memoree v3', status: 'Active', dualities: ['coherence/decoherence'], desc: 'Sovereign memory daemon, embedded vector stores, and unified ledger.' },
                { name: 'aurafs-core', status: 'Active', dualities: ['structure/randomness'], desc: 'Aura Fractal Lattice Node Shard File System + Meshwerk.' },
                { name: 'rAE', status: 'Active', dualities: ['quantum/classical'], desc: 'Quantum simulation toolkit integrating sacred geometry & photonic FDTD.' },
              ].map((proj, idx) => (
                <div key={idx} className={`${colors.panel} border border-slate-700 rounded-xl p-5 hover:border-cyan-500/50 transition-colors group flex flex-col`}>
                  <div className="flex justify-between items-start mb-3">
                    <h3 className="text-lg font-bold text-emerald-400 font-mono">{proj.name}</h3>
                    <span className="text-[10px] bg-emerald-500/10 text-emerald-500 px-2 py-1 rounded uppercase tracking-wider border border-emerald-500/20">{proj.status}</span>
                  </div>
                  <p className="text-sm text-slate-400 mb-6 flex-1">{proj.desc}</p>
                  <div className="pt-4 border-t border-slate-800 flex justify-between items-center">
                     <div className="flex gap-2">
                       {proj.dualities.map(d => <span key={d} className="text-[10px] text-fuchsia-400 bg-fuchsia-500/10 px-2 py-1 rounded font-mono border border-fuchsia-500/20">{d}</span>)}
                     </div>
                     <button className="text-slate-500 hover:text-white transition-colors"><Settings size={16}/></button>
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}

        {/* --- TAB 5: CODEX (TERMINAL/LOGS) --- */}
        {activeTab === 'codex' && (
          <div className="h-full p-6 animate-in fade-in duration-300">
            <div className="h-full bg-black/80 border border-fuchsia-500/30 rounded-xl flex flex-col overflow-hidden font-mono text-sm relative">
              <div className="absolute top-0 left-0 w-full h-[1px] bg-gradient-to-r from-fuchsia-500 via-transparent to-transparent opacity-50"></div>
              
              <div className="bg-slate-950/80 p-3 border-b border-fuchsia-500/20 flex items-center justify-between">
                <div className="flex items-center gap-2">
                  <Terminal className="text-fuchsia-400" size={16} />
                  <span className="text-fuchsia-400 uppercase tracking-widest text-xs font-bold">The Ineffable Ledger</span>
                </div>
                <div className="flex gap-2">
                  <span className="w-3 h-3 rounded-full bg-slate-700"></span>
                  <span className="w-3 h-3 rounded-full bg-slate-700"></span>
                  <span className="w-3 h-3 rounded-full bg-fuchsia-500/50"></span>
                </div>
              </div>

              <div className="flex-1 p-4 overflow-y-auto text-emerald-400/80 space-y-2">
                {codexLog.map((log, i) => (
                  <div key={i} className="flex gap-4">
                    <span className="text-slate-600 shrink-0">[{log.time}]</span>
                    <span className={`${log.msg.includes('INITIATED') || log.msg.includes('COMPLETE') ? 'text-fuchsia-400' : ''}`}>
                      {log.msg}
                    </span>
                  </div>
                ))}
                <div ref={codexEndRef} />
              </div>
            </div>
          </div>
        )}

        {/* --- TAB 6: AGORA (MARKETPLACE) --- */}
        {activeTab === 'agora' && (
          <div className="h-full flex flex-col p-6 overflow-y-auto animate-in fade-in duration-300">
            <div className="flex justify-between items-end mb-8">
              <div>
                <h2 className="text-3xl font-bold text-white mb-2 flex items-center gap-3">
                  <Globe className={colors.accent2} /> The Agora
                </h2>
                <p className="text-slate-400">Install skins, themes, tools, and community-crafted modules.</p>
              </div>
              <div className="relative">
                <Search className="absolute left-3 top-1/2 -translate-y-1/2 text-slate-500" size={18} />
                <input 
                  type="text" 
                  placeholder="Search the void..." 
                  className="bg-slate-900 border border-slate-700 rounded-full py-2 pl-10 pr-4 text-sm text-white focus:border-cyan-500 focus:outline-none w-64 transition-all"
                />
              </div>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-6">
              {plugins.map(plugin => (
                <div key={plugin.id} className={`${colors.panel} border ${plugin.installed ? 'border-cyan-500/50' : 'border-slate-800'} rounded-xl p-5 hover:border-cyan-400/80 transition-all duration-300 group hover:-translate-y-1`}>
                  <div className="flex justify-between items-start mb-4">
                    <div className="p-3 bg-slate-950 rounded-lg border border-slate-800 group-hover:border-slate-600 transition-colors">
                      {plugin.icon}
                    </div>
                    <span className="text-xs font-mono px-2 py-1 bg-slate-800 text-slate-400 rounded uppercase tracking-wider">
                      {plugin.category}
                    </span>
                  </div>
                  <h3 className="text-lg font-bold text-white mb-2">{plugin.name}</h3>
                  <p className="text-slate-400 text-sm mb-6 line-clamp-2 h-10">{plugin.description}</p>
                  
                  <div className="flex justify-between items-center pt-4 border-t border-slate-800">
                    <span className="text-xs text-emerald-400/80 font-mono">{plugin.price}</span>
                    <button 
                      onClick={() => togglePlugin(plugin.id)}
                      className={`px-4 py-2 rounded-lg text-sm font-bold uppercase tracking-wider transition-all ${
                        plugin.installed 
                          ? 'bg-slate-800 text-slate-300 hover:bg-red-500/20 hover:text-red-400 hover:border-red-500/50 border border-transparent'
                          : 'bg-cyan-500/10 text-cyan-400 border border-cyan-500/30 hover:bg-cyan-500 hover:text-slate-900'
                      }`}
                    >
                      {plugin.installed ? 'Uninstall' : 'Install'}
                    </button>
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}

      </main>
    </div>
  );
}