//! Phase 6 — window + main loop.
//!
//! Wires audio capture (Phase 5) -> DSP/VAP runtime + Nexus Vibez (vap-core)
//! -> two-pass GL renderer (Phase 4) into a single winit/glium application.

mod audio;
mod renderer;

use std::path::Path;
use std::sync::mpsc;
use std::time::{Duration, Instant};

use glium::backend::glutin::SimpleWindowBuilder;
use winit::event::{ElementState, Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::keyboard::{Key, NamedKey};
use winit::window::Fullscreen;

use vap_core::{DspEngine, NexusVibezEngine, NexusVibezFrame, VapRuntime};

use renderer::RenderState;

const TARGET_FPS: f64 = 120.0;

fn target_frame_time() -> Duration {
    Duration::from_secs_f64(1.0 / TARGET_FPS)
}

fn main() -> anyhow::Result<()> {
    let audio_path = std::env::args().nth(1);

    let mut vap = match &audio_path {
        Some(path) => {
            let (vap, source) = vap_core::load_vap(Path::new(path));
            eprintln!("[vap] {}", source.description());
            vap
        }
        None => VapRuntime::default(),
    };

    let (tx, rx) = mpsc::channel::<Vec<f32>>();
    let audio_capture = match audio::start_capture(tx) {
        Ok(capture) => Some(capture),
        Err(e) => {
            eprintln!("[audio] capture unavailable ({e}); running with a silent field");
            None
        }
    };
    let (channels, sample_rate) = audio_capture
        .as_ref()
        .map(|a| (a.channels, a.sample_rate))
        .unwrap_or((2, 44_100));
    // Kept alive for the duration of the program: dropping it stops the stream.
    let _audio_capture = audio_capture;

    let mut dsp = DspEngine::new();
    let mut nexus = NexusVibezEngine::new();
    let mut nexus_frame = NexusVibezFrame {
        tsl_x: 0.0,
        tsl_y: 0.0,
        tsl_z: 0.0,
        phase_align: 0.0,
        lattice_rot: 0.0,
    };

    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);

    let (window, display) = SimpleWindowBuilder::new()
        .with_title("Vibe Visualizer — V.A.P. v3.1 | Aurphyx")
        .with_inner_size(1280, 720)
        .build(&event_loop);

    let inner = window.inner_size();
    let mut renderer_state = RenderState::new(&display, inner.width, inner.height)?;

    let mut last_frame = Instant::now();
    let mut fullscreen = false;
    let frame_time = target_frame_time();

    event_loop.run(move |event, elwt| match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => elwt.exit(),

            WindowEvent::Resized(size) => {
                if size.width > 0 && size.height > 0 {
                    display.resize((size.width, size.height));
                    if let Err(e) = renderer_state.resize(&display, size.width, size.height) {
                        eprintln!("[render] resize failed: {e}");
                    }
                }
            }

            WindowEvent::KeyboardInput {
                event: key_event, ..
            } => {
                if key_event.state == ElementState::Pressed {
                    match key_event.logical_key.as_ref() {
                        Key::Character("f" | "F") => {
                            fullscreen = !fullscreen;
                            window.set_fullscreen(
                                fullscreen.then_some(Fullscreen::Borderless(None)),
                            );
                        }
                        Key::Character("g" | "G") => {
                            renderer_state.visual_noise = !renderer_state.visual_noise;
                        }
                        Key::Character("b" | "B") => {
                            renderer_state.bloom_enabled = !renderer_state.bloom_enabled;
                        }
                        Key::Named(NamedKey::Escape) => elwt.exit(),
                        _ => {}
                    }
                }
            }

            WindowEvent::RedrawRequested => {
                let now = Instant::now();
                let dt = (now - last_frame).as_secs_f32().max(1e-4);
                last_frame = now;

                // Drain every audio chunk that arrived since the last frame;
                // fold each through the DSP pipeline, keep the most recent
                // raw chunk for the Nexus Vibez stereo-phase analysis.
                let mut latest_chunk: Option<Vec<f32>> = None;
                while let Ok(chunk) = rx.try_recv() {
                    if let Some(frame) = dsp.process(&chunk, channels, sample_rate, dt) {
                        vap_core::apply_frame(&mut vap, &frame, sample_rate, dt);
                    }
                    latest_chunk = Some(chunk);
                }

                if let Some(chunk) = latest_chunk {
                    if channels == 2 {
                        nexus_frame = nexus.process(&chunk, &vap.chroma_energy, dt);
                    }
                }

                if let Err(e) = renderer_state.render(&display, &vap, &nexus_frame, dt) {
                    eprintln!("[render] frame failed: {e}");
                }
            }

            _ => {}
        },

        Event::AboutToWait => {
            let elapsed = last_frame.elapsed();
            if elapsed < frame_time {
                std::thread::sleep(frame_time - elapsed);
            }
            window.request_redraw();
        }

        _ => {}
    })?;

    Ok(())
}
