import React, { useState } from 'react';
import { 
  Sparkles, Hexagon, Fingerprint, Activity, 
  Disc3, Info, Lock, Flame, Loader2, X
} from 'lucide-react';

const SoulSyncCreatorLore = () => {
  const [isOpen, setIsOpen] = useState(false);
  const [loreData, setLoreData] = useState(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState(null);

  const apiKey = ""; // API Key provided by environment

  // Mock V.A.P. Data for a Rezz ID Track
  const trackVapData = {
    title: "Hypno-Death (Unreleased ID)",
    artist: "REZZ",
    pillars: {
      structural: "90 BPM, Industrial Half-time, Heavy Sub-bass transient",
      affective: "High Arousal, Negative Valence (Dark/Hypnotic)",
      photometric: "Hex #FF0000 (Pure Red), Strobe Trigger Active",
      contextual: "The Filter, Cyberpunk Club, Dissociation"
    },
    credits: {
      writer: "Isabelle Rezazadeh",
      hardware: "Moog Sub 37, Ableton Live"
    }
  };

  // Exponential Backoff Fetcher
  const fetchWithBackoff = async (url, options, retries = 5) => {
    const delays = [1000, 2000, 4000, 8000, 16000];
    for (let i = 0; i < retries; i++) {
      try {
        const res = await fetch(url, options);
        if (!res.ok) throw new Error(`HTTP Error: ${res.status}`);
        return await res.json();
      } catch (err) {
        if (i === retries - 1) throw err;
        await new Promise(resolve => setTimeout(resolve, delays[i]));
      }
    }
  };

  const generateLore = async () => {
    setIsLoading(true);
    setError(null);
    setIsOpen(true);

    const prompt = `
      You are Audry, the AI archivist for the SoulSync Vibe Audio Protocol (V.A.P.).
      Your task is to generate an "About the Song / Creator Lore" drop for the listener.
      
      Track Data:
      Artist: ${trackVapData.artist}
      Title: ${trackVapData.title}
      V.A.P. Structural: ${trackVapData.pillars.structural}
      V.A.P. Affective: ${trackVapData.pillars.affective}
      V.A.P. Photometric: ${trackVapData.pillars.photometric}
      Credits: Writer - ${trackVapData.credits.writer}, Gear - ${trackVapData.credits.hardware}

      Instructions:
      Write a 3-paragraph "Lore Drop". 
      1. Paragraph 1: The "Vibe Origin". Describe the dark, hypnotic, industrial energy of the track as if you were describing a scene in a cyberpunk movie. Mention the Rezz goggles and the red strobes.
      2. Paragraph 2: The "Verified Credits & Gear". Mention Isabelle writing this on a Moog Sub 37 to achieve that specific sub-bass pressure.
      3. Paragraph 3: The "Biometric Impact". Explain to the listener what this track is mathematically designed to do to their heart rate and brainwaves (inducing a trance state).
      Keep the tone edgy, immersive, and highly analytical. Format with clean spacing. Do not use markdown headers.
    `;

    const payload = {
      contents: [{ parts: [{ text: prompt }] }],
      systemInstruction: { 
        parts: [{ text: "You are Audry, the hyper-intelligent, slightly chaotic AI archivist for the Aurphyx ecosystem. Provide immersive, lore-rich music analysis." }] 
      }
    };

    try {
      const url = `https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash-preview-09-2025:generateContent?key=${apiKey}`;
      const data = await fetchWithBackoff(url, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(payload)
      });

      const generatedText = data.candidates?.[0]?.content?.parts?.[0]?.text;
      if (generatedText) {
        setLoreData(generatedText);
      } else {
        throw new Error("No lore generated.");
      }
    } catch (err) {
      setError("Audry is currently shielding the Ineffable Ledger. Unable to fetch track lore. Please try again.");
      console.error("Lore Generation Error:", err);
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="min-h-screen bg-[#050505] text-zinc-200 font-sans flex items-center justify-center p-4 relative overflow-hidden">
      
      {/* Background Rezz-style Hypnotic Rings */}
      <div className="absolute inset-0 flex items-center justify-center opacity-20 pointer-events-none">
        <div className="w-[800px] h-[800px] border-4 border-red-600/30 rounded-full animate-[spin_10s_linear_infinite] border-dashed"></div>
        <div className="absolute w-[600px] h-[600px] border-4 border-red-500/20 rounded-full animate-[spin_8s_linear_infinite_reverse] border-dotted"></div>
        <div className="absolute w-[400px] h-[400px] border-2 border-red-500/40 rounded-full animate-[spin_5s_linear_infinite]"></div>
      </div>

      {/* Main Player Card */}
      <div className="relative z-10 w-full max-w-md bg-[#0a0a0c] border border-red-900/30 rounded-3xl p-6 shadow-[0_0_50px_rgba(220,38,38,0.1)]">
        
        {/* Album Art Placeholder */}
        <div className="w-full aspect-square bg-gradient-to-br from-red-900 via-black to-black rounded-2xl mb-6 relative overflow-hidden flex items-center justify-center border border-red-900/50 group">
           <div className="absolute inset-0 bg-[url('data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSI0IiBoZWlnaHQ9IjQiPgo8cmVjdCB3aWR0aD0iNCIgaGVpZ2h0PSI0IiBmaWxsPSIjMDAwIj48L3JlY3Q+CjxwYXRoIGQ9Ik0wIDBMMCA0TDEgNEwwIDEiIGZpbGw9IiMzMzMiPjwvcGF0aD4KPC9zdmc+')] opacity-50 mix-blend-overlay"></div>
           <div className="flex gap-4">
             {/* Rezz Goggles Simulation */}
             <div className="w-16 h-16 rounded-full border-4 border-red-500 flex items-center justify-center animate-pulse shadow-[0_0_20px_#ef4444]">
               <div className="w-8 h-8 rounded-full border-2 border-red-400 border-dashed animate-[spin_2s_linear_infinite]"></div>
             </div>
             <div className="w-16 h-16 rounded-full border-4 border-red-500 flex items-center justify-center animate-pulse shadow-[0_0_20px_#ef4444]">
               <div className="w-8 h-8 rounded-full border-2 border-red-400 border-dashed animate-[spin_2s_linear_infinite_reverse]"></div>
             </div>
           </div>
        </div>

        {/* Track Info */}
        <div className="text-center mb-6">
          <h2 className="text-2xl font-black text-white tracking-tighter uppercase">{trackVapData.title}</h2>
          <p className="text-red-500 font-bold tracking-widest text-sm mt-1">{trackVapData.artist}</p>
        </div>

        {/* VAP Stats Mini */}
        <div className="flex justify-between items-center bg-black/50 border border-zinc-800 rounded-xl p-3 mb-6">
          <div className="flex items-center gap-2">
            <Activity size={14} className="text-red-500" />
            <span className="text-[10px] font-bold text-zinc-400 uppercase">90 BPM</span>
          </div>
          <div className="flex items-center gap-2">
            <Disc3 size={14} className="text-zinc-500" />
            <span className="text-[10px] font-bold text-zinc-400 uppercase">Sub-Bass</span>
          </div>
          <div className="flex items-center gap-2">
            <Hexagon size={14} className="text-red-500" />
            <span className="text-[10px] font-bold text-zinc-400 uppercase">#FF0000</span>
          </div>
        </div>

        {/* Action Button */}
        <button 
          onClick={generateLore}
          className="w-full py-4 rounded-xl bg-gradient-to-r from-red-900 to-black border border-red-600/50 hover:border-red-500 hover:shadow-[0_0_20px_rgba(220,38,38,0.3)] transition-all flex items-center justify-center gap-3 group"
        >
          <Sparkles size={18} className="text-red-400 group-hover:animate-spin" />
          <span className="text-xs font-black text-red-100 uppercase tracking-widest">Invoke V.A.P. Creator Lore</span>
        </button>

      </div>

      {/* LLM Lore Modal Overlay */}
      {isOpen && (
        <div className="fixed inset-0 z-50 flex items-end sm:items-center justify-center p-0 sm:p-4 bg-black/90 backdrop-blur-md animate-in fade-in duration-300">
          <div className="w-full max-w-2xl bg-[#0a0a0a] border-t sm:border border-red-900/50 sm:rounded-3xl rounded-t-3xl overflow-hidden shadow-[0_0_100px_rgba(220,38,38,0.15)] flex flex-col max-h-[85vh]">
            
            {/* Header */}
            <div className="p-5 border-b border-zinc-900 flex justify-between items-center bg-black sticky top-0 z-10">
              <div className="flex items-center gap-3">
                <Fingerprint size={18} className="text-red-500" />
                <div>
                  <h3 className="text-sm font-black text-white uppercase tracking-wider">Behind The Vibe</h3>
                  <div className="text-[9px] text-zinc-500 uppercase tracking-widest font-mono">V.A.P. Pillar 9.4 • Audry Synthesis</div>
                </div>
              </div>
              <button onClick={() => setIsOpen(false)} className="p-2 bg-zinc-900 rounded-full text-zinc-400 hover:text-white transition-colors">
                <X size={16} />
              </button>
            </div>

            {/* Content Area */}
            <div className="p-6 overflow-y-auto custom-scrollbar">
              {isLoading ? (
                <div className="py-20 flex flex-col items-center justify-center text-center space-y-4">
                  <Loader2 size={32} className="text-red-500 animate-spin" />
                  <p className="text-xs font-bold text-red-400/70 uppercase tracking-widest animate-pulse">
                    Audry is querying the Ineffable Ledger...<br/>Extracting Creator Logic...
                  </p>
                </div>
              ) : error ? (
                <div className="py-10 text-center">
                  <Info size={32} className="text-zinc-600 mx-auto mb-4" />
                  <p className="text-sm text-zinc-400 font-medium">{error}</p>
                </div>
              ) : (
                <div className="space-y-6">
                  {/* Verified Credits Box */}
                  <div className="bg-red-950/10 border border-red-900/30 rounded-2xl p-4 flex flex-col sm:flex-row gap-4 items-start sm:items-center">
                    <div className="w-12 h-12 rounded-full bg-red-900/20 flex items-center justify-center border border-red-500/20 shrink-0">
                      <Lock size={16} className="text-red-400" />
                    </div>
                    <div>
                      <div className="text-[10px] font-bold text-red-500 uppercase tracking-widest mb-1">Cryptographically Verified Origin</div>
                      <p className="text-xs text-zinc-300">Written & Produced by <span className="text-white font-bold">{trackVapData.credits.writer}</span></p>
                      <p className="text-[10px] text-zinc-500 mt-0.5">Hardware: {trackVapData.credits.hardware}</p>
                    </div>
                  </div>

                  {/* Generated Lore text */}
                  <div className="text-zinc-300 text-sm leading-relaxed space-y-5 whitespace-pre-wrap font-medium">
                    {loreData}
                  </div>
                  
                  {/* Validation Footer */}
                  <div className="mt-8 pt-4 border-t border-zinc-900 flex justify-between items-center text-[9px] font-bold text-zinc-600 uppercase tracking-widest">
                    <span className="flex items-center gap-1"><Flame size={10} className="text-red-500"/> Sentient Synthesis Validated</span>
                    <span>V.A.P. v3.1</span>
                  </div>
                </div>
              )}
            </div>
          </div>
        </div>
      )}

      {/* Global styles for custom scrollbar within this component scope */}
      <style dangerouslySetCreateElement={{__html: `
        .custom-scrollbar::-webkit-scrollbar { width: 6px; }
        .custom-scrollbar::-webkit-scrollbar-track { background: transparent; }
        .custom-scrollbar::-webkit-scrollbar-thumb { background: #3f3f46; border-radius: 10px; }
        .custom-scrollbar::-webkit-scrollbar-thumb:hover { background: #ef4444; }
      `}} />
    </div>
  );
};

export default SoulSyncCreatorLore;