import React, { useState } from 'react';
import { 
  CheckCircle2, Music, Fingerprint, Network, 
  Sparkles, Layers, Eye, BookOpen, Mic2, 
  Save, Shield, FileVideo, Activity
} from 'lucide-react';

const AdoreCreatorHub = () => {
  const [activeTab, setActiveTab] = useState('catalog');
  const [loreOutput, setLoreOutput] = useState('');
  const [isGenerating, setIsGenerating] = useState(false);
  const apiKey = ""; // Environment API Key

  // Mock Artist Data (Verified)
  const artistData = {
    name: "REZZ",
    blissId: "0xR3ZZ...99F4",
    verifications: [
      { org: "TheMLC", status: "Verified", icon: <Shield size={14} className="text-green-400" /> },
      { org: "BMI", status: "Verified", icon: <Shield size={14} className="text-green-400" /> },
      { org: "Ineffable Ledger", status: "Immutable", icon: <Network size={14} className="text-purple-400" /> }
    ],
    catalog: [
      { id: "ID-01", title: "Edge", vapStatus: "100%", ualStatus: "Synced" },
      { id: "ID-02", title: "Hypno-Death (ID)", vapStatus: "Draft", ualStatus: "Pending" }
    ]
  };

  const generateLoreAndVideo = async () => {
    setIsGenerating(true);
    // Simulate Gemini API Call for Alternative Music Video / Lore Generation
    const prompt = `
      You are Audry, the creative AI agent for the Adoré Creator Studio.
      Generate a short, vivid treatment for an 'Alternative Music Video' and 'Accessible Lore Description' 
      for a dark, industrial bass track by REZZ. 
      Include a visual description for deaf users (to be mapped to nanomembrane haptics) 
      and an audio-descriptive lore script for blind users.
    `;

    try {
      const response = await fetch(`https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash-preview-09-2025:generateContent?key=${apiKey}`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ contents: [{ parts: [{ text: prompt }] }] })
      });
      const data = await response.json();
      setLoreOutput(data.candidates?.[0]?.content?.parts?.[0]?.text || "Lore generated successfully.");
    } catch (e) {
      setLoreOutput("SYSTEM OVERRIDE: \n\n[Visual Protocol]\nScene opens in a neon-lit cybernetic factory. Heavy sub-bass triggers red strobe pulses.\n\n[UAL Deaf Haptic Map]\nNanomembrane routing: Heavy continuous pressure on lower spine (sub-bass). Sharp staccato taps on wrists (hi-hats).\n\n[UAL Blind Audio Description]\nAudry TTS: 'The environment feels toxic and electric. A deep, mechanical growl reverberates as the color of the room shifts to a harsh, blinding crimson.'");
    } finally {
      setIsGenerating(false);
    }
  };

  return (
    <div className="min-h-screen bg-[#050505] text-zinc-200 font-sans flex overflow-hidden">
      
      {/* Sidebar Navigation */}
      <div className="w-64 bg-[#0a0a0a] border-r border-zinc-900 flex flex-col">
        <div className="p-6 border-b border-zinc-900">
          <h1 className="text-2xl font-black text-white tracking-tighter">
            ADORÉ <span className="text-purple-500">STUDIO</span>
          </h1>
          <div className="text-[10px] text-zinc-500 font-bold tracking-widest uppercase mt-1">Creator Hub v1.0</div>
        </div>
        
        <nav className="p-4 space-y-2 flex-1">
          {[
            { id: 'catalog', icon: <Music size={18}/>, label: 'My Catalog' },
            { id: 'vap_editor', icon: <Layers size={18}/>, label: 'V.A.P. Editor' },
            { id: 'ual_sync', icon: <Activity size={18}/>, label: 'UAL / Accessibility' },
            { id: 'ai_agents', icon: <Sparkles size={18}/>, label: 'Lore & AI Agents' },
          ].map(item => (
            <button 
              key={item.id}
              onClick={() => setActiveTab(item.id)}
              className={`w-full flex items-center gap-3 px-4 py-3 rounded-xl text-sm font-bold transition-all ${
                activeTab === item.id ? 'bg-purple-600/20 text-purple-400 border border-purple-500/30' : 'text-zinc-500 hover:text-zinc-300 hover:bg-zinc-900'
              }`}
            >
              {item.icon} {item.label}
            </button>
          ))}
        </nav>

        <div className="p-4 border-t border-zinc-900">
          <div className="bg-green-950/20 border border-green-900/30 rounded-xl p-4">
            <div className="flex items-center gap-2 mb-2">
              <CheckCircle2 size={16} className="text-green-500" />
              <span className="text-xs font-black text-white uppercase">BlissID Verified</span>
            </div>
            <div className="text-[10px] text-zinc-500 font-mono break-all">{artistData.blissId}</div>
          </div>
        </div>
      </div>

      {/* Main Content Area */}
      <div className="flex-1 overflow-y-auto p-8 relative">
        {/* Background ambient glow */}
        <div className="absolute top-0 right-0 w-96 h-96 bg-purple-900/10 rounded-full blur-3xl pointer-events-none -translate-y-1/2 translate-x-1/2"></div>

        {/* Top Bar: Verifications */}
        <div className="flex justify-between items-center mb-8">
          <h2 className="text-3xl font-black text-white">Welcome, {artistData.name}</h2>
          <div className="flex gap-3">
            {artistData.verifications.map((v, i) => (
              <div key={i} className="flex items-center gap-2 px-3 py-1.5 bg-zinc-900/50 border border-zinc-800 rounded-full text-[10px] font-bold uppercase text-zinc-400">
                {v.icon} {v.org}: {v.status}
              </div>
            ))}
          </div>
        </div>

        {/* Dynamic Content based on Tab */}
        {activeTab === 'ai_agents' && (
          <div className="space-y-6 max-w-4xl">
            <div className="bg-zinc-900/40 border border-zinc-800 rounded-3xl p-8">
              <div className="flex justify-between items-start mb-6">
                <div>
                  <h3 className="text-xl font-black text-white flex items-center gap-2">
                    <FileVideo size={24} className="text-purple-500" /> AI Lore & Video Treatment Agent
                  </h3>
                  <p className="text-sm text-zinc-400 mt-1">Generate infinite alternative music videos and UAL accessibility descriptions for your tracks.</p>
                </div>
                <div className="px-3 py-1 bg-purple-900/30 border border-purple-500/30 rounded-lg text-[10px] font-bold text-purple-400 uppercase">Powered by Audry</div>
              </div>

              <div className="space-y-4">
                <div className="bg-black/50 border border-zinc-800 rounded-xl p-4">
                  <label className="text-[10px] font-bold text-zinc-500 uppercase tracking-widest block mb-2">Select Track</label>
                  <select className="w-full bg-zinc-900 border border-zinc-700 rounded-lg p-3 text-white text-sm focus:outline-none focus:border-purple-500">
                    <option>Hypno-Death (ID) - Unreleased</option>
                    <option>Edge - Mastered</option>
                  </select>
                </div>

                <button 
                  onClick={generateLoreAndVideo}
                  disabled={isGenerating}
                  className="w-full py-4 bg-white text-black rounded-xl font-black text-sm uppercase tracking-widest hover:bg-zinc-200 transition-all flex justify-center items-center gap-2 disabled:opacity-50"
                >
                  {isGenerating ? <Sparkles size={18} className="animate-spin" /> : <Sparkles size={18} />}
                  {isGenerating ? 'Synthesizing Lore...' : 'Generate Alternative Video & UAL Data'}
                </button>

                {loreOutput && (
                  <div className="mt-6 bg-black border border-purple-900/50 rounded-2xl p-6 relative overflow-hidden">
                    <div className="absolute top-0 left-0 w-1 h-full bg-gradient-to-b from-purple-500 to-cyan-500"></div>
                    <h4 className="text-xs font-black text-purple-400 uppercase tracking-widest mb-4 flex items-center gap-2">
                      <BookOpen size={14} /> Agent Output Log
                    </h4>
                    <div className="text-zinc-300 text-sm leading-relaxed whitespace-pre-wrap font-medium">
                      {loreOutput}
                    </div>
                    <div className="mt-6 flex justify-end gap-3">
                      <button className="px-4 py-2 bg-zinc-900 text-white rounded-lg text-xs font-bold hover:bg-zinc-800 transition-colors flex items-center gap-2">
                        <Mic2 size={14}/> Send to Audry TTS
                      </button>
                      <button className="px-4 py-2 bg-purple-600 text-white rounded-lg text-xs font-bold hover:bg-purple-500 transition-colors flex items-center gap-2">
                        <Save size={14}/> Commit to Ineffable Ledger
                      </button>
                    </div>
                  </div>
                )}
              </div>
            </div>
          </div>
        )}

        {/* Placeholder for other tabs */}
        {activeTab !== 'ai_agents' && (
          <div className="flex flex-col items-center justify-center h-64 text-center border-2 border-dashed border-zinc-800 rounded-3xl">
            <Layers size={48} className="text-zinc-700 mb-4" />
            <h3 className="text-lg font-bold text-zinc-500">Module Initializing...</h3>
            <p className="text-sm text-zinc-600 mt-2">The {activeTab} dashboard is being synced with the Chakra Cores.</p>
          </div>
        )}

      </div>
    </div>
  );
};

export default AdoreCreatorHub;