# Echos of Life | Colleen Marie Olson

> **An interactive voice companion, autobiography co-author, and family heirloom publishing archive** designed to capture, preserve, and illuminate the life story, generational wisdom, and maternal reflections of Colleen Marie Olson (legal name: Colleen Ott) for her children (Amy, Mandy, Ross, Jacob, Holly), the Edwards & Lindemann family lines, and future generations.

---

## 🌟 Overview & Purpose

**Echos of Life** bridges the gap between fragile oral history and enduring literature. Many rich family stories, childhood memories, Minnesota cultural traditions, and maternal insights are lost over time simply because writing a complete autobiography is daunting. 

**Echos of Life** transforms spoken memory into written legacy. Colleen can converse naturally with **Clara**—an empathetic AI memoirist—or speak freely into a specialized **Voice Recording Studio**. Powered by **Google Gemini 3.5 Flash**, the application transcribes Colleen's exact words, weaves them into polished memoir chapters with evocative pull quotes and reflections, grounds historical details with real-time web search, and binds them into an interactive digital heirloom.

---

## ✨ Key Features

### 🎙️ 1. Cosmic Voice Recording Studio
- **Word-for-Word Audio Transcription**: Record spoken stories directly on device. The audio is processed via Gemini's multimodal engine to generate accurate, verbatim transcripts.
- **Live Waveform & Amplitude Visualizer**: Real-time multi-bar dynamic audio visualizer providing immediate feedback that the microphone is capturing Colleen's voice clearly.
- **Cosmic Sacred Geometry Visualizer**: A custom canvas-rendered breathing mandala and glowing aura pulse synchronously with Colleen's voice amplitude.
- **1-Tap Chapter Conversion**: Transform any spoken recording directly into a formatted, published chapter in Colleen's Memoir Book.

### 💬 2. Interactive Memoirist Companion ("Clara")
- **Empathetic Multi-Turn Dialogue**: Clara listens attentively, asks gentle, evocative follow-up questions, and helps Colleen unlock long-forgotten details about her childhood, family milestones, and personal journey.
- **Biographical Grounding**: Grounded in Colleen's true Minnesota roots (Colleen, James, Amy, Mandy, Ross, Jacob, Holly, and their ancestors), her maiden name Colleen Marie Olson / legal name Colleen Ott, and the Edwards and Lindemann heritage.
- **Voice-Enabled Interface**: Full bidirectional speech support—speak hands-free via on-device Speech-to-Text and listen to Clara's replies via natural Text-to-Speech (TTS).

### 📖 3. In-App Multimedia E-Reader
- **Accessible Typography**: Toggle **OpenDyslexic** / accessible font, Literary Serif, or Modern Sans.
- **E-Reader Theme Tones**: Seamlessly switch between *Parchment Light*, *Warm Sepia*, and *Cosmic Dark* modes.
- **Tap-to-Listen Audio Playback**: Every paragraph features an individual tap-to-listen speaker icon with interactive highlighting and audio narration.
- **Tagging & Folder Organization**: Filter and categorize chapters by individual child (*Amy, Mandy, Ross, Jacob, Holly*), Minnesota roots, or Edwards & Lindemann lore.

### 🖨️ 4. Professional Publishing Pipelines
- **Typeset PDF Generation**: Compiles publication-ready PDF documents formatted with illuminated drop caps, elegant chapter headers, pull quotes, and Minnesota archival styling.
- **Standard EPUB eBook**: Produces standard reflowable eBooks compatible with Apple Books, Kindle, Kobo, and e-readers.
- **Synced Audio-Story Archive (JSON)**: Exports comprehensive structured archives linking transcripts, metadata, timestamps, and audio snippet paths.

### 🧠 5. Fail-Open Memory Daemon Sync (Memoree)
- **Zero-Latency Capture**: Dispatches conversational turns (`POST /memories/events`) and synthesized chapters (`POST /memories/creative`) asynchronously to local and remote Memoree daemon endpoints.
- **Fail-Open Resilience**: If the daemon endpoint is unreachable, Room SQLite and Gemini continue operating smoothly without blocking UI execution.

---

## 🛠️ Tech Stack & Architecture

### **Core Platform & Language**
- **Target OS**: Android (API 24 to API 36 / Android 16)
- **Language**: 100% Kotlin with Coroutines & Reactive Kotlin Flow
- **Architecture**: MVVM (Model-View-ViewModel) + Repository Pattern + Clean Architecture

### **UI & Design System**
- **Jetpack Compose**: Declarative UI built with Material Design 3 (M3).
- **Cosmic Aurora Theme**: Custom design system combining deep indigo canvases (`#060812`, `#0A0F22`), glowing cyan and violet auroras (`#22D3EE`, `#9D67FF`), warm amber accents (`#FFB787`), and parchment cards.
- **Hardware-Accelerated Canvas**: Mathematical sacred geometry rendering for 16-petal lotus mandalas, radial gradients, and audio amplitude waveforms.

### **Artificial Intelligence & Multimodal Audio**
- **Google Gemini API**:
  - `gemini-3.5-flash`: Fast, high-fidelity conversational agent and voice-to-text audio transcription.
  - `gemini-3.1-pro-preview`: In-depth literary chapter synthesis and biographical formatting.
  - `gemini-3.1-flash-lite-preview`: Low-latency conversational turn responses.
- **Multimodal Audio Processing**: Encodes on-device audio into Base64 MP4 payloads for zero-loss Gemini transcription.
- **Google Search Grounding**: `googleSearch` tool integration for dynamic fact-checking and era-specific historical grounding.

### **Local Persistence & Networking**
- **Room Database (SQLite)**: Schema-managed local storage with Kotlin Symbol Processing (`KSP`) for DAOs, entities, and type converters.
- **Retrofit 2 & OkHttp 3**: REST networking with connection pooling, custom logging interceptors, and timeout handling.
- **Moshi Kotlin Codegen**: High-performance JSON serialization for Gemini API payloads and responses.

---

## 📄 License & Attribution

Developed with ❤️ by **Aurphyx LLC** for Colleen Marie Olson & the Olson/Ott Family.
Copyright © 2026 Aurphyx LLC. All rights reserved.
