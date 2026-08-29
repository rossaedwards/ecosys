//! Phase 4 — GL renderer, using `glium`. Two-pass pipeline mirroring
//! `src/gl_renderer.c`: Pass 1 renders the Chladni field (vibe.frag) to an
//! off-screen FBO texture; Pass 2 runs the bloom post-process
//! (post_bloom.frag) reading that texture and composing to the screen.

use glium::backend::glutin::Display;
use glium::glutin::surface::WindowSurface;
use glium::index::{NoIndices, PrimitiveType};
use glium::texture::Texture2d;
use glium::uniforms::{MagnifySamplerFilter, MinifySamplerFilter};
use glium::{implement_vertex, uniform, Program, Surface, VertexBuffer};

use vap_core::{NexusVibezFrame, VapRuntime};

const VERT_SRC: &str = include_str!("../shaders/vibe.vert");
const SCENE_FRAG_SRC: &str = include_str!("../shaders/vibe.frag");
const BLOOM_FRAG_SRC: &str = include_str!("../shaders/post_bloom.frag");

#[derive(Copy, Clone)]
struct Vertex {
    a_pos: [f32; 2],
}
implement_vertex!(Vertex, a_pos);

// Two triangles covering the full [-1,1] clip-space quad.
const QUAD: [Vertex; 6] = [
    Vertex { a_pos: [-1.0, -1.0] },
    Vertex { a_pos: [1.0, -1.0] },
    Vertex { a_pos: [1.0, 1.0] },
    Vertex { a_pos: [-1.0, -1.0] },
    Vertex { a_pos: [1.0, 1.0] },
    Vertex { a_pos: [-1.0, 1.0] },
];

pub struct RenderState {
    width: u32,
    height: u32,
    pub bloom_enabled: bool,
    pub visual_noise: bool,
    scene_program: Program,
    bloom_program: Program,
    quad_vbo: VertexBuffer<Vertex>,
    scene_texture: Texture2d,
    time_accum: f32,
}

impl RenderState {
    pub fn new(display: &Display<WindowSurface>, width: u32, height: u32) -> anyhow::Result<Self> {
        let scene_program = Program::from_source(display, VERT_SRC, SCENE_FRAG_SRC, None)
            .map_err(|e| anyhow::anyhow!("failed to compile scene shader: {e}"))?;
        let bloom_program = Program::from_source(display, VERT_SRC, BLOOM_FRAG_SRC, None)
            .map_err(|e| anyhow::anyhow!("failed to compile bloom shader: {e}"))?;
        let quad_vbo = VertexBuffer::new(display, &QUAD)?;
        let scene_texture = Texture2d::empty(display, width.max(1), height.max(1))?;

        Ok(RenderState {
            width: width.max(1),
            height: height.max(1),
            bloom_enabled: true,
            visual_noise: false,
            scene_program,
            bloom_program,
            quad_vbo,
            scene_texture,
            time_accum: 0.0,
        })
    }

    pub fn resize(&mut self, display: &Display<WindowSurface>, width: u32, height: u32) -> anyhow::Result<()> {
        self.width = width.max(1);
        self.height = height.max(1);
        self.scene_texture = Texture2d::empty(display, self.width, self.height)?;
        Ok(())
    }

    /// Render one frame: Pass 1 (Chladni + Nexus Vibez field) to the FBO,
    /// Pass 2 (bloom composite) to the screen.
    pub fn render(
        &mut self,
        display: &Display<WindowSurface>,
        vap: &VapRuntime,
        nexus: &NexusVibezFrame,
        dt: f32,
    ) -> anyhow::Result<()> {
        self.time_accum += dt;
        let resolution = [self.width as f32, self.height as f32];
        let indices = NoIndices(PrimitiveType::TrianglesList);

        // ── Pass 1: Cymatic field render -> FBO color texture ──────────
        {
            let mut fbo = glium::framebuffer::SimpleFrameBuffer::new(display, &self.scene_texture)?;
            fbo.clear_color(0.0, 0.0, 0.0, 1.0);

            let chrom = vap.chroma_energy;
            let uniforms = uniform! {
                u_time: self.time_accum,
                u_resolution: resolution,
                u_centroid: vap.spectral_centroid_hz,
                u_saturation: vap.saturation_index,
                u_syncopation: vap.syncopation_index,
                u_bpm_norm: vap.bpm_norm(),
                u_groove: vap.groove_quantization,
                u_dissonance: vap.dissonance_density,
                u_valence: vap.affective.valence,
                u_arousal: vap.affective.arousal,
                u_scenario_fog: vap.scenario_fog(),
                u_primary_rgb: vap.photometric.primary_hex,
                u_secondary_rgb: vap.photometric.secondary_hex,
                u_brightness_floor: vap.photometric.brightness_floor,
                u_brightness_ceiling: vap.photometric.brightness_ceiling,
                u_strobe_trigger: vap.photometric.strobe_threshold,
                u_fog_density: vap.photometric.fog_density,
                u_visual_noise: if self.visual_noise { 1.0f32 } else { 0.0f32 },
                u_entrainment: vap.entrainment_factor,
                u_tsl_x: nexus.tsl_x,
                u_tsl_y: nexus.tsl_y,
                u_tsl_z: nexus.tsl_z,
                u_phase_align: nexus.phase_align,
                u_lattice_rot: nexus.lattice_rot,
            }
            // `uniform float u_chrom_energy[4]` is a GLSL array, which OpenGL
            // addresses per-element by name ("name[i]") — glium's fixed-size
            // [f32; N] AsUniformValue impls map to vecN instead, so each
            // element is bound individually here to match the array uniform.
            .add("u_chrom_energy[0]", chrom[0])
            .add("u_chrom_energy[1]", chrom[1])
            .add("u_chrom_energy[2]", chrom[2])
            .add("u_chrom_energy[3]", chrom[3]);

            fbo.draw(
                &self.quad_vbo,
                indices,
                &self.scene_program,
                &uniforms,
                &Default::default(),
            )?;
        }

        // ── Pass 2: Bloom post-process -> screen ────────────────────────
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        let sampler = self
            .scene_texture
            .sampled()
            .magnify_filter(MagnifySamplerFilter::Linear)
            .minify_filter(MinifySamplerFilter::Linear);
        let bloom_strength = if self.bloom_enabled { vap.bloom_strength() } else { 0.0 };
        let bloom_uniforms = uniform! {
            u_scene: sampler,
            u_resolution: resolution,
            u_bloom_strength: bloom_strength,
            u_fade_amount: 1.0f32,
        };
        target.draw(
            &self.quad_vbo,
            indices,
            &self.bloom_program,
            &bloom_uniforms,
            &Default::default(),
        )?;
        target.finish()?;

        Ok(())
    }
}
