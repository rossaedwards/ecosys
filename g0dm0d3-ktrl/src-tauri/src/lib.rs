#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod connectors;
mod keyring_store;
mod memoree_client;

use connectors::{ask_hub, hub_configured, HubReply};
use ktrl_core_wasm::{HubId, PlanetState};
use memoree_client::{memoree_llm_for_hub, MemoreeClient, MemoreeHealth};
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use tauri::{AppHandle, Manager, State};
use uuid::Uuid;

pub struct AppState {
    http: reqwest::Client,
    memoree: MemoreeClient,
    session_id: String,
}

#[derive(Debug, Serialize)]
pub struct OracleStatus {
    pub hub: String,
    pub bound: bool,
}

fn context_preamble(ctx: &Value) -> String {
    let mut bits = Vec::new();
    if let Some(n) = ctx.get("total_memories").and_then(|v| v.as_u64()) {
        bits.push(format!("Memoree memories: {n}"));
    }
    if let Some(s) = ctx.get("last_summary").and_then(|v| v.as_str()) {
        bits.push(format!("Last summary: {s}"));
    }
    if bits.is_empty() {
        "Memoree project g0dm0d3 is paired.".into()
    } else {
        bits.join("\n")
    }
}

#[tauri::command]
fn bind_oracle(hub: HubId, key: String) -> Result<String, String> {
    let hub = hub.as_str();
    let trimmed = key.trim();
    if trimmed.is_empty() {
        keyring_store::delete_key(hub)?;
        return Ok(format!("{hub} unbound"));
    }
    keyring_store::set_key(hub, trimmed)?;
    Ok(format!("{hub} bound"))
}

#[tauri::command]
fn oracle_status() -> Vec<OracleStatus> {
    connectors::all_hubs()
        .iter()
        .map(|h| OracleStatus {
            hub: (*h).to_string(),
            bound: hub_configured(h),
        })
        .collect()
}

#[tauri::command]
async fn set_kiosk(app: AppHandle, on: bool) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        window.set_fullscreen(on).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
async fn memoree_health(state: State<'_, AppState>) -> Result<MemoreeHealth, String> {
    Ok(state.memoree.health().await)
}

#[tauri::command]
async fn memoree_projects(state: State<'_, AppState>) -> Result<Value, String> {
    state.memoree.projects().await
}

#[tauri::command]
async fn memoree_save_clip(
    state: State<'_, AppState>,
    text: String,
    source_hub: HubId,
    parent_id: Option<String>,
) -> Result<Value, String> {
    let llm = memoree_llm_for_hub(source_hub.as_str());
    state
        .memoree
        .write_event(
            &state.session_id,
            llm,
            "user",
            &text,
            parent_id.as_deref(),
            &["memory-link", "g0dm0d3-ktrl"],
            Some("save"),
        )
        .await
}

#[tauri::command]
async fn memoree_sync(
    state: State<'_, AppState>,
    hub: HubId,
    planet: PlanetState,
) -> Result<Value, String> {
    let hub = hub.as_str();
    let llm = memoree_llm_for_hub(hub);
    state
        .memoree
        .sync_assistant(llm, &state.session_id, planet.as_str())
        .await
}

#[tauri::command]
async fn broadcast_prompt(
    state: State<'_, AppState>,
    prompt: String,
    targets: Vec<HubId>,
) -> Result<Vec<HubReply>, String> {
    let prompt = prompt.trim().to_string();
    if prompt.is_empty() {
        return Err("empty prompt".into());
    }
    let mut futs = Vec::new();
    for raw in &targets {
        let hub = raw.as_str();
        let http = state.http.clone();
        let mem = &state.memoree;
        let llm = memoree_llm_for_hub(hub);
        let system = match mem.context_active(llm).await {
            Ok(ctx) => Some(context_preamble(&ctx)),
            Err(_) => None,
        };
        let p = prompt.clone();
        futs.push(async move { ask_hub(&http, hub, &p, system.as_deref()).await });
        let _ = mem
            .sync_assistant(llm, &state.session_id, "surface")
            .await;
        let _ = mem
            .write_event(
                &state.session_id,
                llm,
                "user",
                &prompt,
                None,
                &["broadcast", "prompt-bus"],
                Some("broadcast"),
            )
            .await;
    }
    let replies = futures::future::join_all(futs).await;
    for r in &replies {
        if r.ok {
            let llm = memoree_llm_for_hub(&r.hub);
            let _ = state
                .memoree
                .write_event(
                    &state.session_id,
                    llm,
                    "assistant",
                    &r.text,
                    None,
                    &["broadcast", "reply"],
                    Some("broadcast"),
                )
                .await;
        }
    }
    Ok(replies)
}

#[tauri::command]
async fn route_clip(
    state: State<'_, AppState>,
    clip: String,
    source_hub: HubId,
    targets: Vec<HubId>,
    extra_prompt: Option<String>,
) -> Result<Vec<HubReply>, String> {
    let clip = clip.trim().to_string();
    if clip.is_empty() {
        return Err("empty clip".into());
    }
    let source = source_hub.as_str();
    let extra = extra_prompt.unwrap_or_default();
    let prompt = if extra.trim().is_empty() {
        format!("Chain-link from {source}. Continue from this clip:\n\n{clip}")
    } else {
        format!("{extra}\n\nChain-link clip from {source}:\n\n{clip}")
    };

    let parent = state
        .memoree
        .write_event(
            &state.session_id,
            memoree_llm_for_hub(source),
            "user",
            &clip,
            None,
            &["chain-link", "clip"],
            Some("chain-link"),
        )
        .await
        .ok()
        .and_then(|v| v.get("id").and_then(|id| id.as_str()).map(|s| s.to_string()));

    let mut futs = Vec::new();
    for raw in &targets {
        let hub = raw.as_str();
        let http = state.http.clone();
        let p = prompt.clone();
        let llm = memoree_llm_for_hub(hub);
        let system = match state.memoree.context_active(llm).await {
            Ok(ctx) => Some(context_preamble(&ctx)),
            Err(_) => None,
        };
        let _ = state
            .memoree
            .sync_assistant(llm, &state.session_id, "orbit")
            .await;
        futs.push(async move { ask_hub(&http, hub, &p, system.as_deref()).await });
    }
    let replies = futures::future::join_all(futs).await;
    for r in &replies {
        if r.ok {
            let llm = memoree_llm_for_hub(&r.hub);
            let _ = state
                .memoree
                .write_event(
                    &state.session_id,
                    llm,
                    "assistant",
                    &r.text,
                    parent.as_deref(),
                    &["chain-link", "reply"],
                    Some("chain-link"),
                )
                .await;
        }
    }
    Ok(replies)
}

#[tauri::command]
fn session_id(state: State<'_, AppState>) -> String {
    state.session_id.clone()
}

#[tauri::command]
fn bound_map() -> HashMap<String, bool> {
    connectors::all_hubs()
        .iter()
        .map(|h| ((*h).to_string(), hub_configured(h)))
        .collect()
}

pub fn run() {
    let http = reqwest::Client::builder()
        .user_agent("g0dm0d3-ktrl/0.1")
        .build()
        .expect("http client");
    let memoree = MemoreeClient::new(http.clone());
    let new_session_id = Uuid::new_v4().to_string();

    tauri::Builder::default()
        .manage(AppState {
            http,
            memoree,
            session_id: new_session_id,
        })
        .invoke_handler(tauri::generate_handler![
            bind_oracle,
            oracle_status,
            set_kiosk,
            memoree_health,
            memoree_projects,
            memoree_save_clip,
            memoree_sync,
            broadcast_prompt,
            route_clip,
            session_id,
            bound_map,
        ])
        .setup(|app| {
            let _ = app;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running g0dm0d3-ktrl");
}
