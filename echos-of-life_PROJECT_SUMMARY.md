# PROJECT SUMMARY: Echos of Life (Colleen Marie Olson Memoir)

> **Internal Context Guide & System Specification**  
> **Confidential — For Aurphyx LLC AI Employees, Autonomous Agents, and Engineering Teams**  
> **Last Updated:** August 2026

---

## 1. Executive Overview & Mission

**Echos of Life** is an AI-powered conversational memoir companion and autobiographical archiving platform engineered specifically for **Colleen Marie Olson** (legal name: **Colleen Ott**). The primary business objective is to empower Colleen to effortlessly record, preserve, and synthesize her lifetime of memories, maternal wisdom, and family lore into a cohesive, beautifully bound digital autobiography for her children, grandchildren, and descendants.

### Core Value Proposition
- **Frictionless Oral Capture**: Eliminates the cognitive burden of typing or manual drafting by providing intuitive voice-to-text recording and real-time audio transcription with live visual waveforms.
- **Empathetic AI Co-Authoring**: Uses Google Gemini to act as a sensitive biographical interviewer ("Clara"), asking tailored follow-up questions grounded in Colleen's genuine Minnesota history.
- **Heirloom Synthesis & Multimedia E-Reader**: Transforms spontaneous oral reminiscences into structured literary chapters with accessible OpenDyslexic typography, theme modes, and per-paragraph "tap-to-listen" audio playback.
- **Multi-Format Publishing**: Typeset PDF books with drop caps, standard EPUB eBooks, and synced audio-story archive exports.
- **Fail-Open Memoree Daemon Sync**: Background memory capture (`POST /memories/events` and `POST /memories/creative`) that never blocks user interaction.

---

## 2. Stakeholder & Family Domain Context

To maintain empathetic and authentic interactions, all AI agents and developers must adhere strictly to these biographical ground truths:

| Entity | Role / Significance | Key Biographical Anchor Points |
| :--- | :--- | :--- |
| **Colleen Marie Olson** | Primary Subject / Mother / Author | Maiden/birth name: **Colleen Marie Olson**. Legal name: **Colleen Ott**. Born, raised, and rooted in **Minnesota**. Matriarch of 5 children. |
| **James** | Husband / Father | Born and raised in **Minnesota**. Built their life and family in Minnesota. |
| **Edwards & Lindemann Lines** | Family Heritage | Core maternal and paternal ancestral roots and traditions. |
| **Amy** | Oldest Daughter (1st Child) | Born in **Minnesota in 1972**. Major milestone chapter in Colleen's early motherhood journey. |
| **Mandy** | Daughter (2nd Child) | Born and raised in **Minnesota**. Cooking/baking traditions and heartwarming memories. |
| **Ross** | Son (3rd Child) | Born and raised in **Minnesota**. Boyhood adventures, energy, humor, and school milestones. |
| **Jacob** | Son (4th Child) | Born and raised in **Minnesota**. Sports, fishing, lake adventures, and family bonds. |
| **Holly** | Youngest Daughter (5th Child) | Born and raised in **Minnesota**. The cherished baby of all five siblings. |
| **Grandchildren** | Next Generation (Minnesota) | Cherished grandchildren in Minnesota; maternal wisdom, love, and life lessons. |
| **Clara** | AI Memoir Co-Author | Empathetic, supportive, loving best friend and co-author. |

> **CRITICAL BIOGRAPHICAL DIRECTIVE:** Colleen, James, all five children (Amy, Mandy, Ross, Jacob, Holly), and their parents/grandparents were born and raised in **Minnesota**. Never refer to Pennsylvania as their origin or birthplace.

---

## 3. Architecture & Technical Design

```
┌────────────────────────────────────────────────────────┐
│                   Jetpack Compose UI                   │
│  (HomeScreen, VoiceStudioScreen, VoiceConversation,    │
│   MemoirBookScreen [E-Reader], Settings, ExportDialog) │
└───────────────────────────▲────────────────────────────┘
                            │ StateFlow / Events
┌───────────────────────────┴────────────────────────────┐
│                    MemoirViewModel                     │
│  (State coordinator, Speech/Audio orchestrator)        │
└─────────────▲────────────────────────────▲─────────────┘
              │                            │
┌─────────────┴───────────────┐ ┌──────────┴─────────────┐
│      SpeechManager &        │ │    MemoirRepository    │
│   AudioRecorderManager      │ │  (Single Source Truth) │
└─────────────────────────────┘ └──────▲──────────▲──────┘
                                       │          │
         ┌─────────────────────────────┴────┐  ┌──┴──────────────────────┐
         │       External / Network         │  │  MemoirDatabase (Room)  │
         │  • GeminiClient (3.5 Flash REST) │  │  • memoir_chapters      │
         │  • MemoreeClient (Fail-Open)     │  │  • conversation_msgs    │
         │  • PublishingPipeline (PDF/EPUB) │  │  • companion_profile    │
         └──────────────────────────────────┘  │  • family_prompts       │
                                               └─────────────────────────┘
```

### 3.1 Persistence & Data Architecture
- **Room Database Schema v3**: Supports string list type converters (`Converters.kt`), thematic tags, child attribution (`childTag`), lineage metadata, and attached audio URIs.
- **Publishing Pipeline**: Native `PdfDocument` rendering with canvas text layout and custom EPUB ZIP package generator with `mimetype`, `META-INF/container.xml`, `content.opf`, and `toc.ncx`.
- **FileProvider Integration**: Secure content URI generation for direct sharing via Android Intent.

### 3.2 Memoree REST Client Specifications
- **Endpoints**: Local daemon (`http://127.0.0.1:7042`) with fallback to Cloudflare Tunnel.
- **Fail-Open Policy**: Network timeouts set strictly to 1.5s connect / 3.0s read/write. If daemon is unreachable or responds with error, operations fail silently in background without impeding Room SQLite or Gemini AI execution.
- **Data Schemas**:
  - Conversational Turns: `POST /memories/events` with role, content, tags, intent.
  - Synthesized Chapters: `POST /memories/creative` with `medium: "prose"`, title, themes, characters, tags.
  - Never sends raw audio bytes to Memoree.

---

## 4. Verification & Testing

- Compile verification: `compile_applet`
- Unit and JVM tests: `gradle :app:testDebugUnitTest`
- Screenshot testing: `gradle :app:verifyRoborazziDebug`

---

## 5. Aurphyx LLC Ownership & Confidentiality

- **Product Owner**: Aurphyx AI Life Legacy Division
- **Client**: Colleen Marie Olson / Colleen Ott
- **Lead AI Engineer**: Google AI Studio Agent
