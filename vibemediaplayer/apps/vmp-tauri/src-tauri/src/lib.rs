//! Vibe Media Player — Tauri backend (native decode/play + VAP tags).

use parking_lot::Mutex;
use serde::Serialize;
use std::path::PathBuf;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use tauri::{Emitter, Manager, State};
use vmp_audio::{
    list_devices, load_media_tags, probe_media, save_vap_for_media, scan_folder, DeviceInventory,
    MediaTagBundle, PlayerEngine, PlayerStatus, SaveReport, OPEN_DIALOG_FILTER,
};
use vmp_core::{AppMode, FileMenuAction, ModuleLayout};
use vmp_dsp::{AudioAnalyzer, EqMode, EqStateSnapshot};
use vmp_v01d::{binding_for_vinyl_vibez, info as v01d_info, V01dInfo};
use vmp_vap::VapObject;
use vmp_viz::{AuraphyxEngine, VapRuntime};

pub struct AppState {
    pub player: Mutex<PlayerEngine>,
    pub last_vap: Mutex<Option<VapObject>>,
    pub media_path: Mutex<Option<PathBuf>>,
    pub app_mode: Mutex<AppMode>,
    pub vap_runtime: Arc<Mutex<VapRuntime>>,
    pub auraphyx: Arc<Mutex<AuraphyxEngine>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            player: Mutex::new(PlayerEngine::new()),
            last_vap: Mutex::new(None),
            media_path: Mutex::new(None),
            app_mode: Mutex::new(AppMode::Player),
            vap_runtime: Arc::new(Mutex::new(VapRuntime::init())),
            auraphyx: Arc::new(Mutex::new(AuraphyxEngine::new())),
        }
    }
}

/// Event name the Auraphyx analysis thread emits `vmp_viz::ShaderUniforms`
/// JSON on. Frontend: `listen('auraphyx-frame', ...)`.
const AURAPHYX_EVENT: &str = "auraphyx-frame";

/// Spawn the background PCM -> FFT -> VapRuntime/Auraphyx -> event pipeline.
/// One thread for the app's lifetime; reads live output format off the
/// player's `Shared` handle each iteration rather than re-locking the player.
fn spawn_auraphyx_pipeline(app: &tauri::App) {
    let state = app.state::<AppState>();
    let rx = state.player.lock().subscribe_pcm(64);
    let shared = state.player.lock().shared_handle();
    let vap_runtime = state.vap_runtime.clone();
    let auraphyx = state.auraphyx.clone();
    let app_handle = app.handle().clone();

    std::thread::spawn(move || {
        let mut analyzer = AudioAnalyzer::new();
        // Throttle emits independently of PCM arrival rate.
        const EMIT_INTERVAL: std::time::Duration = std::time::Duration::from_millis(1000 / 60);
        let mut last_emit = std::time::Instant::now();
        let mut last_tick = std::time::Instant::now();

        while let Ok(pcm) = rx.recv() {
            let now = std::time::Instant::now();
            let dt = now.duration_since(last_tick).as_secs_f32().max(1e-4);
            last_tick = now;

            let sample_rate = shared.out_sample_rate.load(Ordering::Relaxed) as u32;
            let channels = shared.out_channels.load(Ordering::Relaxed) as usize;
            if sample_rate == 0 || channels == 0 {
                continue;
            }

            let Some(frame) = analyzer.process(&pcm, channels, dt) else {
                continue;
            };

            let uniforms = {
                let mut rt = vap_runtime.lock();
                rt.apply_analysis_frame(&frame, sample_rate, dt);
                let chrom = rt.photometric.chrom_energy;
                let aura_frame = if channels == 2 {
                    auraphyx.lock().process(&pcm, &chrom, dt)
                } else {
                    vmp_viz::AuraphyxFrame::default()
                };
                rt.shader_uniforms(&aura_frame)
            };

            if now.duration_since(last_emit) >= EMIT_INTERVAL {
                last_emit = now;
                let _ = app_handle.emit(AURAPHYX_EVENT, &uniforms);
            }
        }
    });
}

#[derive(Serialize)]
pub struct OpenResult {
    pub status: PlayerStatus,
    pub vap: VapObject,
    pub vap_source: String,
    pub format: String,
    pub can_embed: bool,
}

#[tauri::command]
fn vmp_version() -> serde_json::Value {
    serde_json::json!({
        "product": "Vibe Media Player",
        "vap": "3.1",
        "v01d": v01d_info(),
        "filter": OPEN_DIALOG_FILTER,
    })
}

#[tauri::command]
fn file_menu_items() -> Vec<serde_json::Value> {
    FileMenuAction::all()
        .iter()
        .map(|a| {
            serde_json::json!({
                "id": format!("{:?}", a).to_ascii_lowercase(),
                "label": a.label(),
                "shortcut": a.shortcut(),
            })
        })
        .collect()
}

#[tauri::command]
fn open_file(path: String, state: State<'_, AppState>) -> Result<OpenResult, String> {
    let status = state
        .player
        .lock()
        .load(&path)
        .map_err(|e| e.to_string())?;

    let bundle: MediaTagBundle = load_media_tags(&path).map_err(|e| e.to_string())?;
    *state.last_vap.lock() = Some(bundle.vap.clone());
    *state.media_path.lock() = Some(PathBuf::from(&path));

    Ok(OpenResult {
        status,
        vap: bundle.vap,
        vap_source: bundle.vap_source,
        format: bundle.format.label().into(),
        can_embed: bundle.can_embed,
    })
}

#[tauri::command]
fn open_folder(path: String) -> Result<Vec<String>, String> {
    let files = scan_folder(&path).map_err(|e| e.to_string())?;
    Ok(files
        .into_iter()
        .map(|p| p.display().to_string())
        .collect())
}

#[tauri::command]
fn probe(path: String) -> Result<serde_json::Value, String> {
    let p = probe_media(&path).map_err(|e| e.to_string())?;
    serde_json::to_value(p).map_err(|e| e.to_string())
}

#[tauri::command]
fn play(state: State<'_, AppState>) -> PlayerStatus {
    let eng = state.player.lock();
    eng.play();
    eng.status()
}

#[tauri::command]
fn pause(state: State<'_, AppState>) -> PlayerStatus {
    let eng = state.player.lock();
    eng.pause();
    eng.status()
}

#[tauri::command]
fn toggle(state: State<'_, AppState>) -> PlayerStatus {
    let eng = state.player.lock();
    eng.toggle();
    eng.status()
}

#[tauri::command]
fn stop(state: State<'_, AppState>) -> PlayerStatus {
    let eng = state.player.lock();
    eng.stop();
    eng.status()
}

#[tauri::command]
fn seek(sec: f64, state: State<'_, AppState>) -> PlayerStatus {
    let eng = state.player.lock();
    eng.seek_sec(sec);
    eng.status()
}

#[tauri::command]
fn set_volume(volume: f32, state: State<'_, AppState>) -> PlayerStatus {
    let eng = state.player.lock();
    eng.set_volume(volume);
    eng.status()
}

#[tauri::command]
fn status(state: State<'_, AppState>) -> PlayerStatus {
    state.player.lock().status()
}

#[tauri::command]
fn set_eq_band(index: usize, gain_db: f32, state: State<'_, AppState>) -> EqStateSnapshot {
    let eng = state.player.lock();
    eng.set_eq_band(index, gain_db);
    eng.eq_snapshot()
}

#[tauri::command]
fn set_eq_mode(mode: String, state: State<'_, AppState>) -> Result<EqStateSnapshot, String> {
    let m = match mode.as_str() {
        "graphic10" => EqMode::Graphic10,
        "graphic31" => EqMode::Graphic31,
        "parametric" => EqMode::Parametric,
        "vap_guided" => EqMode::VapGuided,
        "context_linked" => EqMode::ContextLinked,
        "bypass" => EqMode::Bypass,
        _ => return Err(format!("unknown eq mode {mode}")),
    };
    let eng = state.player.lock();
    eng.set_eq_mode(m);
    Ok(eng.eq_snapshot())
}

#[tauri::command]
fn get_vap(state: State<'_, AppState>) -> Option<VapObject> {
    state.last_vap.lock().clone()
}

#[tauri::command]
fn set_vap(vap: VapObject, state: State<'_, AppState>) {
    *state.last_vap.lock() = Some(vap);
}

#[tauri::command]
fn save_vap(embed: bool, state: State<'_, AppState>) -> Result<SaveReport, String> {
    let path = state
        .media_path
        .lock()
        .clone()
        .ok_or_else(|| "no media loaded".to_string())?;
    let vap = state
        .last_vap
        .lock()
        .clone()
        .ok_or_else(|| "no VAP object".to_string())?;
    save_vap_for_media(path, &vap, embed).map_err(|e| e.to_string())
}

#[tauri::command]
fn set_app_mode(mode: String, state: State<'_, AppState>) -> Result<ModuleLayout, String> {
    let m = match mode.as_str() {
        "player" => AppMode::Player,
        "vinyl" | "vinyl_vibez" | "vinylvibez" => AppMode::VinylVibez,
        _ => return Err(format!("unknown mode {mode}")),
    };
    *state.app_mode.lock() = m;
    Ok(match m {
        AppMode::Player => ModuleLayout::default_player(),
        AppMode::VinylVibez => ModuleLayout::vinyl_vibez(),
    })
}

#[tauri::command]
fn v01d_info_cmd() -> V01dInfo {
    v01d_info()
}

#[tauri::command]
fn vinyl_binding() -> serde_json::Value {
    serde_json::to_value(binding_for_vinyl_vibez()).unwrap_or_default()
}

#[tauri::command]
fn list_audio_devices() -> DeviceInventory {
    list_devices()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(AppState::default())
        .setup(|app| {
            spawn_auraphyx_pipeline(app);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            vmp_version,
            file_menu_items,
            open_file,
            open_folder,
            probe,
            play,
            pause,
            toggle,
            stop,
            seek,
            set_volume,
            status,
            set_eq_band,
            set_eq_mode,
            get_vap,
            set_vap,
            save_vap,
            set_app_mode,
            v01d_info_cmd,
            vinyl_binding,
            list_audio_devices,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Vibe Media Player");
}
