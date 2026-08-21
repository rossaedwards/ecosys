//! Core transmutation engine components

pub mod engine;
pub mod context;
pub mod pipeline;

pub use engine::TransmutationEngine;
pub use context::{TransmutationContext, CeremonialMode, Symbol, SymbolKind, Dependency};
pub use pipeline::{TransformationPipeline, PipelineStage};