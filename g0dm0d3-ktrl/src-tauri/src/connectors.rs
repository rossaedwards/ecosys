//! Parallel LLM connectors. Missing key → "not configured"; other hubs still run.

use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::time::Duration;

use crate::keyring_store;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HubReply {
    pub hub: String,
    pub ok: bool,
    pub text: String,
    pub configured: bool,
}

pub fn normalize_hub(raw: &str) -> String {
    match raw.trim().to_ascii_lowercase().as_str() {
        "supergrok" | "xai" | "grok" => "grok".into(),
        "anthropic" | "claude" => "claude".into(),
        "google" | "gemini" => "gemini".into(),
        "github" | "copilot" => "copilot".into(),
        "nous" | "hermes" => "hermes".into(),
        "pplx" | "perplexity" => "perplexity".into(),
        "mistral" | "lechat" => "lechat".into(),
        "local" | "ollama" => "ollama".into(),
        "openai" | "chatgpt" => "openai".into(),
        other => other.to_string(),
    }
}

pub fn all_hubs() -> &'static [&'static str] {
    &[
        "grok",
        "claude",
        "gemini",
        "copilot",
        "hermes",
        "perplexity",
        "lechat",
        "ollama",
        "openai",
    ]
}

fn timeout() -> Duration {
    Duration::from_secs(45)
}

async fn openai_compat(
    http: &Client,
    url: &str,
    bearer: &str,
    extra_headers: &[(&str, &str)],
    model: &str,
    prompt: &str,
    system: Option<&str>,
) -> Result<String, String> {
    let mut messages = Vec::new();
    if let Some(sys) = system.filter(|s| !s.is_empty()) {
        messages.push(json!({ "role": "system", "content": sys }));
    }
    messages.push(json!({ "role": "user", "content": prompt }));

    let mut req = http
        .post(url)
        .header("Authorization", format!("Bearer {bearer}"))
        .header("Content-Type", "application/json")
        .json(&json!({
            "model": model,
            "messages": messages,
            "stream": false,
        }))
        .timeout(timeout());
    for (k, v) in extra_headers {
        req = req.header(*k, *v);
    }
    let resp = req.send().await.map_err(|e| e.to_string())?;
    let status = resp.status();
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    if !status.is_success() {
        return Err(format!("{status}: {body}"));
    }
    body.pointer("/choices/0/message/content")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("unexpected response: {body}"))
}

async fn ask_grok(http: &Client, key: &str, prompt: &str, system: Option<&str>) -> Result<String, String> {
    openai_compat(
        http,
        "https://api.x.ai/v1/chat/completions",
        key,
        &[],
        "grok-3",
        prompt,
        system,
    )
    .await
}

async fn ask_claude(http: &Client, key: &str, prompt: &str, system: Option<&str>) -> Result<String, String> {
    let mut body = json!({
        "model": "claude-sonnet-4-20250514",
        "max_tokens": 2048,
        "messages": [{ "role": "user", "content": prompt }],
    });
    if let Some(sys) = system.filter(|s| !s.is_empty()) {
        body["system"] = json!(sys);
    }
    let resp = http
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&body)
        .timeout(timeout())
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let status = resp.status();
    let v: Value = resp.json().await.map_err(|e| e.to_string())?;
    if !status.is_success() {
        return Err(format!("{status}: {v}"));
    }
    v.pointer("/content/0/text")
        .and_then(|t| t.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("unexpected response: {v}"))
}

async fn ask_gemini(http: &Client, key: &str, prompt: &str, system: Option<&str>) -> Result<String, String> {
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent?key={key}"
    );
    let mut body = json!({
        "contents": [{ "role": "user", "parts": [{ "text": prompt }] }]
    });
    if let Some(sys) = system.filter(|s| !s.is_empty()) {
        body["systemInstruction"] = json!({ "parts": [{ "text": sys }] });
    }
    let resp = http
        .post(url)
        .json(&body)
        .timeout(timeout())
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let status = resp.status();
    let v: Value = resp.json().await.map_err(|e| e.to_string())?;
    if !status.is_success() {
        return Err(format!("{status}: {v}"));
    }
    v.pointer("/candidates/0/content/parts/0/text")
        .and_then(|t| t.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("unexpected response: {v}"))
}

async fn ask_copilot(http: &Client, key: &str, prompt: &str, system: Option<&str>) -> Result<String, String> {
    if key.starts_with("sk-") {
        return openai_compat(
            http,
            "https://api.openai.com/v1/chat/completions",
            key,
            &[],
            "gpt-4o",
            prompt,
            system,
        )
        .await;
    }
    openai_compat(
        http,
        "https://models.github.ai/inference/chat/completions",
        key,
        &[("Accept", "application/vnd.github+json")],
        "openai/gpt-4o",
        prompt,
        system,
    )
    .await
}

async fn ask_hermes(http: &Client, key: &str, prompt: &str, system: Option<&str>) -> Result<String, String> {
    openai_compat(
        http,
        "https://openrouter.ai/api/v1/chat/completions",
        key,
        &[
            ("HTTP-Referer", "https://g0dm0d3.org"),
            ("X-Title", "g0dm0d3-ktrl"),
        ],
        "nousresearch/hermes-3-llama-3.1-70b",
        prompt,
        system,
    )
    .await
}

async fn ask_perplexity(http: &Client, key: &str, prompt: &str, system: Option<&str>) -> Result<String, String> {
    openai_compat(
        http,
        "https://api.perplexity.ai/chat/completions",
        key,
        &[],
        "sonar",
        prompt,
        system,
    )
    .await
}

async fn ask_lechat(http: &Client, key: &str, prompt: &str, system: Option<&str>) -> Result<String, String> {
    openai_compat(
        http,
        "https://api.mistral.ai/v1/chat/completions",
        key,
        &[],
        "mistral-large-latest",
        prompt,
        system,
    )
    .await
}

async fn ask_ollama(http: &Client, model: &str, prompt: &str, system: Option<&str>) -> Result<String, String> {
    let mut messages = Vec::new();
    if let Some(sys) = system.filter(|s| !s.is_empty()) {
        messages.push(json!({ "role": "system", "content": sys }));
    }
    messages.push(json!({ "role": "user", "content": prompt }));
    let resp = http
        .post("http://127.0.0.1:11434/api/chat")
        .json(&json!({
            "model": if model.is_empty() { "llama3.2" } else { model },
            "messages": messages,
            "stream": false,
        }))
        .timeout(timeout())
        .send()
        .await
        .map_err(|e| format!("ollama :11434 — {e}"))?;
    let status = resp.status();
    let v: Value = resp.json().await.map_err(|e| e.to_string())?;
    if !status.is_success() {
        return Err(format!("{status}: {v}"));
    }
    v.pointer("/message/content")
        .and_then(|t| t.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("unexpected response: {v}"))
}

async fn ask_openai(http: &Client, key: &str, prompt: &str, system: Option<&str>) -> Result<String, String> {
    openai_compat(
        http,
        "https://api.openai.com/v1/chat/completions",
        key,
        &[],
        "gpt-4o",
        prompt,
        system,
    )
    .await
}

pub fn hub_configured(hub: &str) -> bool {
    let h = normalize_hub(hub);
    if h == "ollama" {
        return true;
    }
    keyring_store::is_bound(&h)
}

pub async fn ask_hub(
    http: &Client,
    hub: &str,
    prompt: &str,
    system: Option<&str>,
) -> HubReply {
    let hub = normalize_hub(hub);
    if hub != "ollama" && !keyring_store::is_bound(&hub) {
        return HubReply {
            hub: hub.clone(),
            ok: false,
            configured: false,
            text: "not configured".into(),
        };
    }
    let key = if hub == "ollama" {
        keyring_store::get_key("ollama").unwrap_or_else(|_| "llama3.2".into())
    } else {
        match keyring_store::get_key(&hub) {
            Ok(k) => k,
            Err(_) => {
                return HubReply {
                    hub: hub.clone(),
                    ok: false,
                    configured: false,
                    text: "not configured".into(),
                };
            }
        }
    };

    let result = match hub.as_str() {
        "grok" => ask_grok(http, &key, prompt, system).await,
        "claude" => ask_claude(http, &key, prompt, system).await,
        "gemini" => ask_gemini(http, &key, prompt, system).await,
        "copilot" => ask_copilot(http, &key, prompt, system).await,
        "hermes" => ask_hermes(http, &key, prompt, system).await,
        "perplexity" => ask_perplexity(http, &key, prompt, system).await,
        "lechat" => ask_lechat(http, &key, prompt, system).await,
        "ollama" => ask_ollama(http, &key, prompt, system).await,
        "openai" => ask_openai(http, &key, prompt, system).await,
        other => Err(format!("unknown hub {other}")),
    };

    match result {
        Ok(text) => HubReply {
            hub,
            ok: true,
            configured: true,
            text,
        },
        Err(e) => HubReply {
            hub,
            ok: false,
            configured: true,
            text: format!("error: {e}"),
        },
    }
}
