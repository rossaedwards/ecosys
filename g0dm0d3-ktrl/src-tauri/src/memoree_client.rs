//! Memoree HTTP client. Local :7042 first, then Agora, then ecosystem.

use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Mutex;
use std::time::Duration;

const PROJECT: &str = "g0dm0d3";
const LOCAL: &str = "http://127.0.0.1:7042";
const AGORA: &str = "https://memoree.g0dm0d3.org";
const ECOSYSTEM: &str = "https://memoree.aurphyx.org";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoreeHealth {
    pub ok: bool,
    pub base: Option<String>,
    pub source: String,
    pub detail: String,
}

pub struct MemoreeClient {
    http: Client,
    base: Mutex<Option<String>>,
}

impl MemoreeClient {
    pub fn new(http: Client) -> Self {
        Self {
            http,
            base: Mutex::new(None),
        }
    }

    pub async fn health(&self) -> MemoreeHealth {
        for (source, url) in [
            ("local", LOCAL),
            ("agora", AGORA),
            ("ecosystem", ECOSYSTEM),
        ] {
            let timeout = if source == "local" {
                Duration::from_millis(800)
            } else {
                Duration::from_secs(3)
            };
            match self
                .http
                .get(format!("{url}/health"))
                .timeout(timeout)
                .send()
                .await
            {
                Ok(resp) if resp.status().is_success() => {
                    if let Ok(mut guard) = self.base.lock() {
                        *guard = Some(url.to_string());
                    }
                    let body = resp.text().await.unwrap_or_default();
                    return MemoreeHealth {
                        ok: true,
                        base: Some(url.to_string()),
                        source: source.to_string(),
                        detail: body,
                    };
                }
                Ok(resp) => {
                    let _ = resp;
                }
                Err(_) => {}
            }
        }
        if let Ok(mut guard) = self.base.lock() {
            *guard = None;
        }
        MemoreeHealth {
            ok: false,
            base: None,
            source: "none".into(),
            detail: "Memoree unreachable on :7042, memoree.g0dm0d3.org, and memoree.aurphyx.org. Local daemon: python memoree_service.py. MEM-0: memoree/schemas.py currently starts with YAML and may SyntaxError before the service boots.".into(),
        }
    }

    fn base_url(&self) -> Option<String> {
        self.base.lock().ok().and_then(|g| g.clone())
    }

    pub async fn ensure_base(&self) -> Result<String, String> {
        if let Some(b) = self.base_url() {
            return Ok(b);
        }
        let h = self.health().await;
        h.base.ok_or_else(|| h.detail)
    }

    pub async fn projects(&self) -> Result<Value, String> {
        let base = self.ensure_base().await?;
        let resp = self
            .http
            .get(format!("{base}/projects"))
            .timeout(Duration::from_secs(8))
            .send()
            .await
            .map_err(|e| e.to_string())?;
        resp.json().await.map_err(|e| e.to_string())
    }

    pub async fn context_active(&self, llm: &str) -> Result<Value, String> {
        let base = self.ensure_base().await?;
        let resp = self
            .http
            .get(format!("{base}/context/active"))
            .query(&[("project", PROJECT), ("llm", llm)])
            .timeout(Duration::from_secs(12))
            .send()
            .await
            .map_err(|e| e.to_string())?;
        if !resp.status().is_success() {
            return Err(format!("context/active {}", resp.status()));
        }
        resp.json().await.map_err(|e| e.to_string())
    }

    pub async fn write_event(
        &self,
        session_id: &str,
        llm: &str,
        role: &str,
        content: &str,
        parent_id: Option<&str>,
        tags: &[&str],
        intent: Option<&str>,
    ) -> Result<Value, String> {
        let base = self.ensure_base().await?;
        let body = json!({
            "session_id": session_id,
            "project": PROJECT,
            "role": role,
            "content": content,
            "llm": llm,
            "parent_id": parent_id,
            "tags": tags,
            "intent": intent,
            "memory_type": "episodic",
        });
        let resp = self
            .http
            .post(format!("{base}/memories/events"))
            .json(&body)
            .timeout(Duration::from_secs(12))
            .send()
            .await
            .map_err(|e| e.to_string())?;
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        if !status.is_success() {
            return Err(format!("memories/events {status}: {text}"));
        }
        serde_json::from_str(&text).or_else(|_| Ok(json!({ "raw": text })))
    }

    pub async fn sync_assistant(
        &self,
        llm: &str,
        session_id: &str,
        planet: &str,
    ) -> Result<Value, String> {
        let base = self.ensure_base().await?;
        let body = json!({
            "llm": llm,
            "session_id": session_id,
            "capabilities": { "planet": planet, "console": "g0dm0d3-ktrl" },
        });
        let resp = self
            .http
            .post(format!("{base}/assistants/sync"))
            .json(&body)
            .timeout(Duration::from_secs(8))
            .send()
            .await
            .map_err(|e| e.to_string())?;
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        if !status.is_success() {
            return Err(format!("assistants/sync {status}: {text}"));
        }
        serde_json::from_str(&text).or_else(|_| Ok(json!({ "raw": text })))
    }
}

pub fn memoree_llm_for_hub(hub: &str) -> &'static str {
    match hub {
        "grok" | "supergrok" => "supergrok",
        "claude" => "claude",
        "gemini" => "gemini",
        "copilot" | "openai" | "chatgpt" => "openai",
        "perplexity" => "perplexity",
        "ollama" => "ollama",
        "hermes" | "lechat" | "mistral" => "unknown",
        _ => "unknown",
    }
}
