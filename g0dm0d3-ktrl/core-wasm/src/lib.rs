//! Shared orchestration types. Desktop talks JSON over Tauri invoke;
//! wasm32 can bind this crate later for graph layout.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HubId {
    Grok,
    Claude,
    Gemini,
    Copilot,
    Hermes,
    Perplexity,
    Lechat,
    Ollama,
    Openai,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PlanetState {
    Dark,
    Orbit,
    Surface,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Clip {
    pub id: String,
    pub source_hub: HubId,
    pub text: String,
    pub range: Option<(usize, usize)>,
    pub parent_turn: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TopologyKind {
    Link,
    Chain,
    Ritual,
    Forkz,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RitualGraphNode {
    pub id: String,
    pub kind: TopologyKind,
    pub hub: Option<HubId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RitualGraph {
    pub nodes: Vec<RitualGraphNode>,
    pub edges: Vec<(String, String)>,
}

impl PlanetState {
    pub fn as_str(self) -> &'static str {
        match self {
            PlanetState::Dark => "dark",
            PlanetState::Orbit => "orbit",
            PlanetState::Surface => "surface",
        }
    }
}

impl HubId {
    pub fn as_str(self) -> &'static str {
        match self {
            HubId::Grok => "grok",
            HubId::Claude => "claude",
            HubId::Gemini => "gemini",
            HubId::Copilot => "copilot",
            HubId::Hermes => "hermes",
            HubId::Perplexity => "perplexity",
            HubId::Lechat => "lechat",
            HubId::Ollama => "ollama",
            HubId::Openai => "openai",
        }
    }

    /// Memoree LLMProvider mapping until schemas.py gains copilot/hermes/lechat.
    pub fn memoree_llm(self) -> &'static str {
        match self {
            HubId::Grok => "supergrok",
            HubId::Claude => "claude",
            HubId::Gemini => "gemini",
            HubId::Copilot => "openai",
            HubId::Hermes => "unknown",
            HubId::Perplexity => "perplexity",
            HubId::Lechat => "unknown",
            HubId::Ollama => "ollama",
            HubId::Openai => "openai",
        }
    }
}
