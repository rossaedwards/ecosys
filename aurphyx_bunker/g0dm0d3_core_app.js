// g0dm0d3-core/src/app/layout.tsx
import type { Metadata } from 'next'
import { Inter } from 'next/font/google'
import './globals.css'
import { Toaster } from 'react-hot-toast'

const inter = Inter({ subsets: ['latin'] })

export const metadata: Metadata = {
  title: 'g0dm0d3 - Divine AI Orchestration Console',
  description: 'Command multiple AIs through cosmic interface. Every feature forged with love.',
  keywords: ['ai', 'productivity', 'creator tools', 'aurphyx', 'g0dm0d3']
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en">
      <body className={`${inter.className} bg-gray-900 text-white`}>
        <div className="starfield"></div>
        {children}
        <Toaster 
          position="top-right"
          toastOptions={{
            style: {
              background: '#1a1a1a',
              color: '#0FFFAA',
              border: '1px solid #0FFFAA'
            }
          }}
        />
      </body>
    </html>
  )
}

// g0dm0d3-core/src/app/page.tsx
'use client'

import React, { useState } from 'react'
import { motion } from 'framer-motion'
import { CosmicInterface } from '../components/CosmicInterface'
import { BroadcastInterface } from '../components/BroadcastInterface'
import { CreatorSuite } from '../components/CreatorSuite'
import { Zap, Video, Eye, Settings } from 'lucide-react'

export default function Home() {
  const [activeTab, setActiveTab] = useState('cosmic')

  return (
    <div className="min-h-screen">
      {/* Header */}
      <motion.header 
        className="bg-black/90 backdrop-blur-lg border-b-2 border-cyan-500 p-4 sticky top-0 z-50"
        initial={{ y: -100 }}
        animate={{ y: 0 }}
        transition={{ duration: 0.8 }}
      >
        <div className="max-w-7xl mx-auto flex items-center justify-between">
          <div className="flex items-center gap-4">
            <motion.h1 
              className="text-3xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-cyan-400 to-purple-500"
              whileHover={{ scale: 1.05 }}
            >
              g0dm0d3
            </motion.h1>
            <span className="text-gray-400 text-lg">Divine AI Orchestration Console</span>
          </div>
          <div className="flex items-center gap-4">
            <span className="text-sm text-gray-400 px-3 py-1 bg-purple-900/30 rounded-full">
              Powered by Aurphyx
            </span>
            <Settings className="w-6 h-6 text-gray-500 hover:text-cyan-400 cursor-pointer transition-colors" />
          </div>
        </div>
      </motion.header>

      {/* Navigation */}
      <nav className="bg-gray-800/90 backdrop-blur-lg border-b border-gray-700">
        <div className="max-w-7xl mx-auto">
          <div className="flex">
            {[
              { key: 'cosmic', label: 'Cosmic Interface', icon: <Zap className="w-4 h-4" /> },
              { key: 'broadcast', label: 'AI Broadcast', icon: <Zap className="w-4 h-4" /> },
              { key: 'creator', label: 'Creator Suite', icon: <Video className="w-4 h-4" /> },
              { key: 'analytics', label: 'Analytics', icon: <Eye className="w-4 h-4" /> }
            ].map(tab => (
              <motion.button
                key={tab.key}
                onClick={() => setActiveTab(tab.key)}
                className={`flex items-center gap-2 px-6 py-4 font-medium transition-all relative ${
                  activeTab === tab.key
                    ? 'text-cyan-400'
                    : 'text-gray-400 hover:text-white'
                }`}
                whileHover={{ y: -2 }}
                whileTap={{ scale: 0.98 }}
              >
                {tab.icon}
                {tab.label}
                {activeTab === tab.key && (
                  <motion.div
                    className="absolute bottom-0 left-0 right-0 h-0.5 bg-cyan-400"
                    layoutId="activeTab"
                    transition={{ type: "spring", stiffness: 300, damping: 30 }}
                  />
                )}
              </motion.button>
            ))}
          </div>
        </div>
      </nav>

      {/* Main Content */}
      <main className="max-w-7xl mx-auto p-6">
        <motion.div
          key={activeTab}
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.5 }}
        >
          {activeTab === 'cosmic' && <CosmicInterface />}
          {activeTab === 'broadcast' && <BroadcastInterface />}
          {activeTab === 'creator' && <CreatorSuite />}
          {activeTab === 'analytics' && <AnalyticsView />}
        </motion.div>
      </main>

      {/* Footer */}
      <footer className="mt-20 border-t border-gray-800 p-6">
        <div className="max-w-7xl mx-auto text-center">
          <p className="text-gray-500">
            Forged with ∞ Love by{' '}
            <span className="text-cyan-400 font-bold">Ross Edwards</span> • 
            Powered by <span className="text-purple-400">Aurphyx</span>
          </p>
        </div>
      </footer>
    </div>
  )
}

const AnalyticsView = () => (
  <div className="space-y-6">
    <h2 className="text-2xl font-bold text-cyan-400">Divine Analytics</h2>
    <div className="grid gap-4 md:grid-cols-3">
      {[
        { label: 'AI Broadcasts Today', value: '47', change: '+12%' },
        { label: 'Total Responses', value: '1,204', change: '+8%' },
        { label: 'Divine Insights', value: '∞', change: 'Always Growing' }
      ].map(stat => (
        <motion.div
          key={stat.label}
          className="bg-gray-800 p-6 rounded-lg border border-gray-700"
          whileHover={{ scale: 1.02, borderColor: '#0FFFAA' }}
        >
          <h3 className="text-sm text-gray-400 mb-2">{stat.label}</h3>
          <p className="text-3xl font-bold text-white">{stat.value}</p>
          <p className="text-sm text-green-400">{stat.change}</p>
        </motion.div>
      ))}
    </div>
  </div>
)

// g0dm0d3-core/src/app/globals.css
@tailwind base;
@tailwind components;
@tailwind utilities;

@import url('https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@400;700&display=swap');

* {
  box-sizing: border-box;
}

body {
  margin: 0;
  font-family: 'Inter', system-ui, -apple-system, sans-serif;
  background: radial-gradient(ellipse at center, #1a1a2e 0%, #16213e 50%, #0f0f23 100%);
  color: #eeeeee;
  overflow-x: hidden;
}

.starfield {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-image: 
    radial-gradient(2px 2px at 20px 30px, #eee, transparent),
    radial-gradient(1px 1px at 40px 70px, #fff, transparent),
    radial-gradient(1px 1px at 90px 40px, #fff, transparent),
    radial-gradient(1px 1px at 130px 80px, #fff, transparent),
    radial-gradient(2px 2px at 160px 30px, #eee, transparent);
  background-repeat: repeat;
  background-size: 200px 100px;
  animation: twinkle 20s linear infinite;
  z-index: -1;
  opacity: 0.6;
}

@keyframes twinkle {
  from { background-position: 0 0; }
  to { background-position: -200px -100px; }
}

@keyframes pulse-cosmic {
  0%, 100% { opacity: 1; transform: scale(1); }
  50% { opacity: 0.8; transform: scale(1.05); }
}

@keyframes float {
  0%, 100% { transform: translateY(0px); }
  50% { transform: translateY(-10px); }
}

@keyframes rotate-slow {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

.cosmic-glow {
  box-shadow: 0 0 20px #0FFFAA44, 0 0 40px #0FFFAA22;
}

.cosmic-button {
  @apply bg-gradient-to-r from-cyan-400 to-purple-500 text-black font-bold py-3 px-6 rounded-lg;
  @apply hover:scale-105 transform transition-all duration-300;
  @apply shadow-lg hover:shadow-cyan-400/25;
}

.cosmic-input {
  @apply bg-gray-800/80 border-2 border-cyan-500/50 text-white p-3 rounded-lg;
  @apply focus:border-cyan-400 focus:ring-2 focus:ring-cyan-400/20 outline-none;
  @apply backdrop-blur-sm transition-all duration-300;
}

/* Custom scrollbar */
::-webkit-scrollbar {
  width: 8px;
}

::-webkit-scrollbar-track {
  background: #1a1a1a;
}

::-webkit-scrollbar-thumb {
  background: linear-gradient(45deg, #0FFFAA, #00CC88);
  border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
  background: linear-gradient(45deg, #00CC88, #0FFFAA);
}

// g0dm0d3-core/next.config.js
/** @type {import('next').NextConfig} */
const nextConfig = {
  experimental: {
    appDir: true,
  },
  images: {
    domains: ['localhost'],
  },
  env: {
    CUSTOM_KEY: process.env.CUSTOM_KEY,
  },
}

module.exports = nextConfig

// g0dm0d3-core/tailwind.config.js
/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    './src/pages/**/*.{js,ts,jsx,tsx,mdx}',
    './src/components/**/*.{js,ts,jsx,tsx,mdx}',
    './src/app/**/*.{js,ts,jsx,tsx,mdx}',
  ],
  theme: {
    extend: {
      colors: {
        cosmic: {
          50: '#f0fdfa',
          400: '#2dd4bf', 
          500: '#0FFFAA',
          600: '#0d9488',
          900: '#042f2e'
        },
        void: {
          900: '#0A0A0A',
          800: '#1A1A1A', 
          700: '#2A2A2A'
        }
      },
      fontFamily: {
        mono: ['JetBrains Mono', 'monospace'],
        sans: ['Inter', 'system-ui', 'sans-serif']
      },
      animation: {
        'pulse-cosmic': 'pulse-cosmic 2s ease-in-out infinite alternate',
        'float': 'float 3s ease-in-out infinite',
        'rotate-slow': 'rotate-slow 20s linear infinite'
      },
      backgroundImage: {
        'cosmic-gradient': 'radial-gradient(ellipse at center, #1a1a2e 0%, #16213e 50%, #0f0f23 100%)',
        'divine-gradient': 'linear-gradient(135deg, #0FFFAA, #00CC88, #667eea)'
      }
    },
  },
  plugins: [],
}

// g0dm0d3-core/src/components/BroadcastInterface.tsx
'use client'

import React, { useState } from 'react'
import { motion } from 'framer-motion'
import { Zap, Send, Loader2 } from 'lucide-react'
import toast from 'react-hot-toast'

interface AIResponse {
  [key: string]: string
}

export const BroadcastInterface = () => {
  const [prompt, setPrompt] = useState('')
  const [responses, setResponses] = useState<AIResponse>({})
  const [loading, setLoading] = useState(false)
  const [selectedAIs, setSelectedAIs] = useState(['gpt', 'claude', 'gemini'])

  const availableAIs = {
    gpt: { name: 'ChatGPT', color: '#10a37f', icon: '💬' },
    claude: { name: 'Claude', color: '#cc785c', icon: '🤖' },
    gemini: { name: 'Gemini', color: '#4285f4', icon: '🦉' },
    grok: { name: 'Grok', color: '#ff6b35', icon: '🧠' }
  }

  const handleBroadcast = async () => {
    if (!prompt.trim()) {
      toast.error('Please enter a prompt')
      return
    }

    setLoading(true)
    setResponses({})

    try {
      // Simulate AI responses
      const mockResponses: AIResponse = {}
      
      for (const aiKey of selectedAIs) {
        await new Promise(resolve => setTimeout(resolve, Math.random() * 2000 + 500))
        
        const ai = availableAIs[aiKey as keyof typeof availableAIs]
        mockResponses[aiKey] = `${ai.icon} ${ai.name} responds: "${prompt}" - Here's my perspective on this topic with detailed insights and actionable recommendations.`
      }

      setResponses(mockResponses)
      toast.success('Broadcast complete!')
      
    } catch (error) {
      toast.error('Broadcast failed')
    } finally {
      setLoading(false)
    }
  }

  const toggleAI = (aiKey: string) => {
    setSelectedAIs(prev => 
      prev.includes(aiKey) 
        ? prev.filter(ai => ai !== aiKey)
        : [...prev, aiKey]
    )
  }

  return (
    <div className="space-y-6">
      {/* AI Selection */}
      <motion.div 
        className="bg-gray-800/50 backdrop-blur-sm p-6 rounded-xl border border-gray-700"
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
      >
        <h3 className="text-xl font-bold text-cyan-400 mb-4">🤖 Divine AI Arsenal</h3>
        <div className="grid grid-cols-2 md:grid-cols-4 gap-3">
          {Object.entries(availableAIs).map(([key, ai]) => (
            <motion.button
              key={key}
              onClick={() => toggleAI(key)}
              className={`p-3 rounded-lg border-2 transition-all flex items-center gap-2 ${
                selectedAIs.includes(key)
                  ? 'border-cyan-400 bg-cyan-900/30 text-white'
                  : 'border-gray-600 bg-gray-700/50 text-gray-300 hover:border-gray-500'
              }`}
              whileHover={{ scale: 1.02 }}
              whileTap={{ scale: 0.98 }}
            >
              <span className="text-lg">{ai.icon}</span>
              <span className="font-medium">{ai.name}</span>
            </motion.button>
          ))}
        </div>
        <p className="text-sm text-gray-400 mt-3">
          {selectedAIs.length} AI(s) selected for broadcast
        </p>
      </motion.div>

      {/* Broadcast Input */}
      <motion.div 
        className="bg-gray-800/50 backdrop-blur-sm p-6 rounded-xl border border-gray-700"
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: 0.1 }}
      >
        <div className="flex gap-3">
          <input
            type="text"
            value={prompt}
            onChange={(e) => setPrompt(e.target.value)}
            placeholder="Enter your divine command..."
            className="flex-1 cosmic-input"
            onKeyPress={(e) => e.key === 'Enter' && !loading && handleBroadcast()}
            disabled={loading}
          />
          <motion.button
            onClick={handleBroadcast}
            disabled={!prompt.trim() || selectedAIs.length === 0 || loading}
            className="cosmic-button flex items-center gap-2 disabled:opacity-50 disabled:cursor-not-allowed"
            whileHover={{ scale: loading ? 1 : 1.05 }}
            whileTap={{ scale: loading ? 1 : 0.95 }}
          >
            {loading ? (
              <Loader2 className="w-4 h-4 animate-spin" />
            ) : (
              <Zap className="w-4 h-4" />
            )}
            {loading ? 'Broadcasting...' : 'BROADCAST'}
          </motion.button>
        </div>
      </motion.div>

      {/* Responses */}
      {Object.keys(responses).length > 0 && (
        <motion.div 
          className="grid gap-4 md:grid-cols-2 lg:grid-cols-3"
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          transition={{ delay: 0.2 }}
        >
          {selectedAIs.map(aiKey => {
            const ai = availableAIs[aiKey as keyof typeof availableAIs]
            return (
              <motion.div
                key={aiKey}
                className="bg-gray-800/70 backdrop-blur-sm border-2 rounded-xl overflow-hidden"
                style={{ borderColor: ai.color + '40' }}
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                whileHover={{ 
                  scale: 1.02, 
                  borderColor: ai.color + '80',
                  boxShadow: `0 10px 30px ${ai.color}20`
                }}
              >
                <div 
                  className="p-4 flex items-center gap-3"
                  style={{ backgroundColor: ai.color + '20' }}
                >
                  <span className="text-2xl">{ai.icon}</span>
                  <div>
                    <h3 className="font-bold text-white">{ai.name}</h3>
                    <p className="text-xs text-gray-400">Divine Response</p>
                  </div>
                </div>
                <div className="p-4">
                  {responses[aiKey] ? (
                    <p className="text-gray-300 text-sm leading-relaxed">
                      {responses[aiKey]}
                    </p>
                  ) : loading ? (
                    <div className="flex items-center gap-2 text-gray-500">
                      <Loader2 className="w-4 h-4 animate-spin" />
                      Thinking...
                    </div>
                  ) : (
                    <p className="text-gray-500 text-sm">Waiting for broadcast...</p>
                  )}
                </div>
              </motion.div>
            )
          })}
        </motion.div>
      )}
    </div>
  )
}

// g0dm0d3-core/src/components/CosmicInterface.tsx
'use client'

import React, { useState, useRef, useEffect } from 'react'
import { motion } from 'framer-motion'
import { Hand, Eye, Mic, Settings, Zap } from 'lucide-react'

export const CosmicInterface = () => {
  const [selectedPlanet, setSelectedPlanet] = useState<string | null>(null)
  const [gestureMode, setGestureMode] = useState(false)
  const [eyeTracking, setEyeTracking] = useState(false)
  const [voiceMode, setVoiceMode] = useState(false)
  const [responses, setResponses] = useState<{[key: string]: string}>({})
  const containerRef = useRef<HTMLDivElement>(null)
  const [mousePosition, setMousePosition] = useState({ x: 0, y: 0 })

  const planets = {
    grok: {
      id: 'grok',
      name: 'Grok',
      persona: 'The Rebel Oracle',
      avatar: '🧠',
      color: '#ff6b35',
      position: { x: 200, y: 150 },
      size: 80
    },
    chatgpt: {
      id: 'chatgpt', 
      name: 'ChatGPT',
      persona: 'The Structured Sage',
      avatar: '💬',
      color: '#10a37f',
      position: { x: 400, y: 200 },
      size: 85
    },
    gemini: {
      id: 'gemini',
      name: 'Gemini', 
      persona: 'The Creative Twin',
      avatar: '🦉',
      color: '#4285f4',
      position: { x: 300, y: 100 },
      size: 75
    },
    claude: {
      id: 'claude',
      name: 'Claude',
      persona: 'The Thoughtful Guardian',
      avatar: '🤖',
      color: '#cc785c',
      position: { x: 150, y: 250 },
      size: 70
    }
  }

  useEffect(() => {
    const handleMouseMove = (e: MouseEvent) => {
      if (containerRef.current && gestureMode) {
        const rect = containerRef.current.getBoundingClientRect()
        setMousePosition({
          x: e.clientX - rect.left,
          y: e.clientY - rect.top
        })
      }
    }

    if (gestureMode) {
      window.addEventListener('mousemove', handleMouseMove)
      return () => window.removeEventListener('mousemove', handleMouseMove)
    }
  }, [gestureMode])

  const PlanetAvatar = ({ planet }: { planet: any }) => {
    const isSelected = selectedPlanet === planet.id

    return (
      <motion.div
        className="absolute cursor-pointer"
        style={{
          left: `${planet.position.x}px`,
          top: `${planet.position.y}px`,
          width: `${planet.size}px`,
          height: `${planet.size}px`
        }}
        onClick={() => setSelectedPlanet(planet.id)}
        whileHover={{ scale: 1.2 }}
        animate={{ 
          scale: isSelected ? 1.2 : 1,
          rotate: 360
        }}
        transition={{ 
          rotate: { duration: 20, repeat: Infinity, ease: "linear" }
        }}
      >
        <div
          className="w-full h-full rounded-full flex items-center justify-center text-2xl border-2 transition-all duration-300"
          style={{
            background: `radial-gradient(circle at 30% 30%, ${planet.color}aa, ${planet.color}22, #000)`,
            borderColor: isSelected ? planet.color : planet.color + '66',
            boxShadow: isSelected 
              ? `0 0 30px ${planet.color}88, inset 0 0 20px ${planet.color}44`
              : `0 0 15px ${planet.color}44`
          }}
        >
          <span>{planet.avatar}</span>
        </div>
        
        {/* Atmosphere */}
        <div
          className="absolute top-0 left-0 w-full h-full rounded-full pointer-events-none"
          style={{
            background: `radial-gradient(circle, ${planet.color}11, transparent)`,
            transform: 'scale(1.3)'
          }}
        />
        
        {/* Response indicator */}
        {responses[planet.id] && (
          <div 
            className="absolute -top-1 -right-1 w-3 h-3 rounded-full animate-pulse"
            style={{ backgroundColor: '#0FFFAA' }}
          />
        )}
      </motion.div>
    )
  }

  const ResponseWindow = ({ planet }: { planet: any }) => {
    if (!planet || !responses[planet.id]) return null

    return (
      <motion.div
        className="absolute z-10 w-80 bg-gray-800/90 backdrop-blur-lg border-2 rounded-xl p-4"
        style={{
          left: `${planet.position.x + planet.size + 20}px`,
          top: `${planet.position.y}px`,
          borderColor: planet.color
        }}
        initial={{ opacity: 0, x: -20 }}
        animate={{ opacity: 1, x: 0 }}
        exit={{ opacity: 0, x: -20 }}
      >
        <div className="flex items-center justify-between mb-3">
          <div className="flex items-center gap-2">
            <span className="text-lg">{planet.avatar}</span>
            <div>
              <h3 className="font-bold text-white">{planet.persona}</h3>
              <p className="text-xs text-gray-400">{planet.name}</p>
            </div>
          </div>
          <button
            onClick={() => setSelectedPlanet(null)}
            className="text-gray-400 hover:text-white"
          >
            ✕
          </button>
        </div>
        
        <div className="text-gray-300 text-sm leading-relaxed">
          {responses[planet.id]}
        </div>
        
        <div className="mt-3 flex gap-2">
          <button 
            className="px-3 py-1 text-xs rounded-full border"
            style={{ 
              borderColor: planet.color,
              backgroundColor: planet.color + '20',
              color: planet.color
            }}
          >
            👍 Helpful
          </button>
          <button className="px-3 py-1 text-xs rounded-full border border-orange-500 bg-orange-500/20 text-orange-400">
            🔄 Refine  
          </button>
        </div>
      </motion.div>
    )
  }

  return (
    <div 
      ref={containerRef}
      className="relative w-full h-[600px] bg-gradient-to-b from-purple-900/20 to-blue-900/20 rounded-xl border border-gray-700 overflow-hidden"
    >
      {/* Control Panel */}
      <motion.div 
        className="absolute top-4 right-4 bg-gray-800/90 backdrop-blur-lg border border-gray-600 rounded-lg p-4 z-20"
        initial={{ opacity: 0, x: 20 }}
        animate={{ opacity: 1, x: 0 }}
      >
        <h3 className="text-cyan-400 font-bold mb-3 flex items-center gap-2">
          <Settings className="w-4 h-4" />
          Neural Controls
        </h3>
        
        {[
          { key: 'gesture', label: 'Gesture Control', icon: Hand, state: gestureMode, setState: setGestureMode },
          { key: 'eye', label: 'Eye Tracking', icon: Eye, state: eyeTracking, setState: setEyeTracking },
          { key: 'voice', label: 'Voice Commands', icon: Mic, state: voiceMode, setState: setVoiceMode }
        ].map(control => (
          <label key={control.key} className="flex items-center gap-2 mb-2 cursor-pointer text-sm">
            <input
              type="checkbox"
              checked={control.state}
              onChange={(e) => control.setState(e.target.checked)}
              className="rounded"
            />
            <control.icon className="w-4 h-4" />
            <span className="text-gray-300">{control.label}</span>
          </label>
        ))}
      </motion.div>

      {/* Gesture Cursor */}
      {gestureMode && (
        <motion.div
          className="absolute w-8 h-8 rounded-full border-2 border-cyan-400 bg-cyan-400/20 pointer-events-none z-30"
          style={{
            left: `${mousePosition.x - 16}px`,
            top: `${mousePosition.y - 16}px`
          }}
          animate={{ scale: [1, 1.2, 1] }}
          transition={{ duration: 1, repeat: Infinity }}
        />
      )}

      {/* AI Planets */}
      {Object.values(planets).map(planet => (
        <PlanetAvatar key={planet.id} planet={planet} />
      ))}

      {/* Response Windows */}
      {selectedPlanet && (
        <ResponseWindow planet={planets[selectedPlanet as keyof typeof planets]} />
      )}

      {/* Command Interface */}
      <motion.div 
        className="absolute bottom-4 left-1/2 transform -translate-x-1/2 w-4/5 max-w-lg bg-gray-800/90 backdrop-blur-lg border border-gray-600 rounded-xl p-4 z-20"
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
      >
        <div className="flex items-center gap-3 mb-3">
          <Zap className="w-5 h-5 text-cyan-400" />
          <h3 className="font-bold text-cyan-400">Cosmic Command Center</h3>
        </div>
        
        <div className="flex gap-2">
          <input
            type="text"
            placeholder="Broadcast to cosmic entities..."
            className="flex-1 cosmic-input text-sm"
          />
          <button className="cosmic-button px-4 py-2 text-sm">
            BROADCAST
          </button>
        </div>
        
        <div className="mt-2 text-xs text-gray-400">
          Click planets to interact • {Object.keys(responses).length} responses ready
        </div>
      </motion.div>
    </div>
  )
}

// g0dm0d3-core/src/components/CreatorSuite.tsx
'use client'

import React, { useState } from 'react'
import { motion } from 'framer-motion'
import { Video, Camera, Hash, Users, TrendingUp, Zap } from 'lucide-react'

export const CreatorSuite = () => {
  const [creatorMode, setCreatorMode] = useState('youtube')

  const modes = [
    { key: 'youtube', label: 'YouTube', icon: Video, color: '#FF0000' },
    { key: 'instagram', label: 'Instagram', icon: Camera, color: '#E4405F' },
    { key: 'tiktok', label: 'TikTok', icon: Hash, color: '#000000' },
    { key: 'multi', label: 'Multi-Platform', icon: Users, color: '#6366F1' }
  ]

  const youtubeFeatures = [
    { key: 'ideas', label: 'Video Ideas', description: 'AI-generated viral concepts', icon: '💡' },
    { key: 'scripts', label: 'Script Outline', description: 'Structured video scripts', icon: '📝' },
    { key: 'thumbnails', label: 'Thumbnails', description: 'Eye-catching designs', icon: '🎨' },
    { key: 'seo', label: 'SEO Optimizer', description: 'Title, tags & descriptions', icon: '🔍' },
    { key: 'analytics', label: 'Analytics', description: 'Performance insights', icon: '📊' },
    { key: 'trending', label: 'Trending Topics', description: 'What\'s hot right now', icon: '🔥' }
  ]

  return (
    <div className="space-y-6">
      {/* Mode Selector */}
      <motion.div 
        className="flex gap-3 mb-6"
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
      >
        {modes.map(mode => (
          <motion.button
            key={mode.key}
            onClick={() => setCreatorMode(mode.key)}
            className={`flex items-center gap-2 px-4 py-3 rounded-lg font-medium transition-all ${
              creatorMode === mode.key
                ? 'bg-cyan-500 text-black'
                : 'bg-gray-700 text-gray-300 hover:bg-gray-600'
            }`}
            whileHover={{ scale: 1.05 }}
            whileTap={{ scale: 0.95 }}
          >
            <mode.icon className="w-4 h-4" />
            {mode.label}
          </motion.button>
        ))}
      </motion.div>

      {/* Features Grid */}
      <motion.div 
        className="grid gap-4 md:grid-cols-2 lg:grid-cols-3"
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        transition={{ delay: 0.1 }}
      >
        {youtubeFeatures.map((feature, index) => (
          <motion.div
            key={feature.key}
            className="bg-gray-800/50 backdrop-blur-sm border border-gray-700 rounded-xl p-6 hover:border-cyan-400 cursor-pointer group"
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.1 + index * 0.05 }}
            whileHover={{ 
              scale: 1.02,
              boxShadow: '0 10px 30px rgba(6, 182, 212, 0.1)'
            }}
          >
            <div className="flex items-center gap-3 mb-3">
              <span className="text-2xl">{feature.icon}</span>
              <h3 className="font-bold text-white group-hover:text-cyan-400 transition-colors">
                {feature.label}
              </h3>
            </div>
            <p className="text-gray-400 text-sm mb-4">
              {feature.description}
            </p>
            <button className="w-full bg-gradient-to-r from-cyan-500 to-blue-500 text-white font-bold py-2 rounded-lg hover:from-cyan-400 hover:to-blue-400 transition-all">
              Launch Feature
            </button>
          </motion.div>
        ))}
      </motion.div>

      {/* Ross Edwards Channel Prep */}
      <motion.div 
        className="mt-8 p-6 bg-gradient-to-r from-purple-900/30 to-blue-900/30 border border-purple-500/50 rounded-xl"
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: 0.3 }}
      >
        <h2 className="text-2xl font-bold text-white mb-4 flex items-center gap-2">
          🚀 "Ross Edwards AI" Channel Launch Pad
        </h2>
        
        <div className="grid gap-6 md:grid-cols-2">
          <div>
            <h3 className="text-lg font-bold text-purple-300 mb-3">Channel Setup Progress</h3>
            <div className="space-y-2">
              {[
                { task: 'Channel concept defined', status: 'complete' },
                { task: 'Aurphyx branding ready', status: 'complete' },
                { task: 'First 5 video ideas', status: 'progress' },
                { task: 'Upload schedule planned', status: 'progress' },
                { task: 'Community guidelines', status: 'pending' }
              ].map(item => (
                <div key={item.task} className="flex items-center gap-2 text-sm">
                  <span className={`w-3 h-3 rounded-full ${
                    item.status === 'complete' ? 'bg-green-400' :
                    item.status === 'progress' ? 'bg-yellow-400' : 'bg-gray-400'
                  }`} />
                  <span className="text-gray-300">{item.task}</span>
                </div>
              ))}
            </div>
          </div>
          
          <div>
            <h3 className="text-lg font-bold text-purple-300 mb-3">Content Pipeline</h3>
            <div className="grid grid-cols-2 gap-4">
              {[
                { label: 'Scripts Ready', value: '3', icon: '📝' },
                { label: 'Thumbnails Created', value: '5', icon: '🎨' },
                { label: 'SEO Optimized', value: '3', icon: '🔍' },
                { label: 'Videos Planned', value: '12', icon: '🎬' }
              ].map(stat => (
                <div key={stat.label} className="text-center p-3 bg-gray-800/50 rounded-lg">
                  <div className="text-2xl mb-1">{stat.icon}</div>
                  <div className="text-2xl font-bold text-cyan-400">{stat.value}</div>
                  <div className="text-xs text-gray-400">{stat.label}</div>
                </div>
              ))}
            </div>
          </div>
        </div>
        
        <motion.button 
          className="mt-6 w-full bg-gradient-to-r from-purple-500 to-pink-500 text-white font-bold py-3 rounded-lg"
          whileHover={{ scale: 1.02 }}
          whileTap={{ scale: 0.98 }}
        >
          🎬 Generate Complete Video Series
        </motion.button>
      </motion.div>
    </div>
  )
}