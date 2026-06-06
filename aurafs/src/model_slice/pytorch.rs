//! ═══════════════════════════════════════════════════════════════════
//! ✨ [:: f0rg3d with l0v3 by Aurphyx Quantum Division ::] ✨
//! 💎 AuraFS PyTorch Model Slicer - The Neural Harvester
//! 🔥 FFI Bridge + Tensor Fractal Decomposition + Kagome Injection
//! ═══════════════════════════════════════════════════════════════════

#![warn(missing_docs)]
#![allow(improper_ctypes_definitions)]

use crate::{
    crypto::hash::{Blake3Digest, blake3_hash},
    model_slice::{
        ModelShard, SliceConfig, SliceError, FractalLineage,
    },
    shard::{ShardId, ShardMetadata, ShardLayer, LatticeGeometry, ShardMetadataTrait, CoreShardMetadata},
    compression::CompressionAlgorithm,
    gov::BlissId,
};
use serde::{Deserialize, Serialize};
use std::{
    ffi::{CStr, CString},
    path::PathBuf,
    ptr,
    time::SystemTime,
};
use thiserror::Error;
use tracing::{info, warn, debug, instrument};

// ═══════════════════════════════════════════════════════════════════
// FFI Bindings (The Bridge to LibTorch)
// ═══════════════════════════════════════════════════════════════════

/// Opaque pointer to C++ PyTorch model
#[repr(C)]
pub struct PyTorchModel { _private: [u8; 0] }

/// PyTorch C++ FFI bindings for tensor extraction & serialization
extern "C" {
    fn load_pytorch_model(path: *const std::os::raw::c_char) -> *mut PyTorchModel;
    fn free_pytorch_model(model: *mut PyTorchModel);
    fn get_layer_count(model: *const PyTorchModel) -> u32;
    fn get_layer_name(model: *const PyTorchModel, layer_idx: u32) -> *const std::os::raw::c_char;
    fn get_layer_size(model: *const PyTorchModel, layer_idx: u32) -> usize;
    fn extract_layer(
        model: *const PyTorchModel,
        layer_idx: u32,
        buffer: *mut u8,
        buffer_len: usize,
    ) -> i32;
    // New Phase II FFI: Get tensor gradients for frustration calculation
    fn get_layer_frustration(model: *const PyTorchModel, layer_idx: u32) -> f64;
}

/// Safe RAII Wrapper for PyTorch Model
pub struct SafeTorchModel(*mut PyTorchModel);

impl Drop for SafeTorchModel {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { free_pytorch_model(self.0) };
        }
    }
}

// ═══════════════════════════════════════════════════════════════════
// The Slicer Engine
// ═══════════════════════════════════════════════════════════════════

/// PyTorch Model Slicer
pub struct TorchModelSlicer {
    config: SliceConfig,
}

impl TorchModelSlicer {
    /// Create new slicer
    pub fn new(config: SliceConfig) -> Self {
        Self { config }
    }

    /// 🔮 Slice a PyTorch model into Kagome Lattice Shards
    #[instrument(skip(self))]
    pub async fn slice_pytorch_model(
        &self,
        path: &PathBuf,
        config: &SliceConfig,
    ) -> Result<Vec<ModelShard>, SliceError> {
        let path_str = path.to_str()
            .ok_or_else(|| SliceError::LoadError("Invalid path encoding".to_string()))?;
        let c_path = CString::new(path_str)
            .map_err(|e| SliceError::LoadError(format!("Path error: {}", e)))?;

        // 1. Load Model (Unsafe FFI)
        let model_ptr = unsafe { load_pytorch_model(c_path.as_ptr()) };
        if model_ptr.is_null() {
            return Err(SliceError::LoadError("Failed to load PyTorch model via FFI".to_string()));
        }
        let safe_model = SafeTorchModel(model_ptr);

        let layer_count = unsafe { get_layer_count(safe_model.0) };
        info!("🔮 Analyzing PyTorch Model: {} layers detected", layer_count);

        let mut shards = Vec::new();
        let mut lineage = FractalLineage::default();

        // 2. Iterate Layers (The Harvester Loop)
        for i in 0..layer_count {
            let layer_size = unsafe { get_layer_size(safe_model.0, i) };
            
            // Get layer name for metadata
            let name_ptr = unsafe { get_layer_name(safe_model.0, i) };
            let layer_name = if !name_ptr.is_null() {
                unsafe { CStr::from_ptr(name_ptr).to_string_lossy().into_owned() }
            } else {
                format!("layer_{}", i)
            };

            // Allocate buffer
            let mut buffer = vec![0u8; layer_size];
            
            // Extract weights
            let result = unsafe { 
                extract_layer(safe_model.0, i, buffer.as_mut_ptr(), layer_size) 
            };

            if result != 0 {
                return Err(SliceError::Serialization(format!("Failed to extract layer {}", i)));
            }

            // 3. Compress (Adaptive)
            // Use config to decide compression
            let compressed_data = if config.compression != CompressionAlgorithm::None {
                config.compression.compress(&buffer).await
                    .map_err(|e| SliceError::Serialization(e.to_string()))?
            } else {
                buffer
            };

            // 4. 🟢 Phase II: Calculate Frustration
            // In a real implementation, we'd use gradients.
            // Here we check if the layer is "dense" or "conv" to guess computational complexity.
            let frustration_index = unsafe { get_layer_frustration(safe_model.0, i) };
            
            // 5. Create Shard Identity
            // Note: We use ShardLayer::Compute to force Kagome geometry
            let shard_id = ShardId::from_content_with_layer(&compressed_data, ShardLayer::Compute);
            
            // 6. Forge Metadata with Bio-Resonance
            let mut core_meta = CoreShardMetadata::new(
                shard_id.clone(), 
                compressed_data.len() as u64, 
                ShardLayer::Compute
            );
            
            // 🟢 FORCE KAGOME LATTICE
            // Even if the default logic suggests otherwise, Neural Weights MUST be Kagome.
            core_meta.geometry = LatticeGeometry::Kagome;
            
            // Add tags
            core_meta.tags.push("pytorch_weight".to_string());
            core_meta.tags.push(layer_name.clone());
            if frustration_index > 0.8 {
                core_meta.tags.push("high_frustration".to_string());
            }

            // Create Full Metadata wrapper (Generic for now, assuming ObjectShardMetadata as container)
            // In a perfect world, we'd use `ComputeShardMetadata` defined in `metadata.rs`
            // But ModelShard struct expects `ShardMetadata` enum wrapper.
            // Let's assume ShardMetadata can wrap this.
            
            // Note: Adapting to the legacy ShardMetadata struct usage in ModelShard
            // We map our Phase II CoreMetadata into the ShardMetadata struct
            let mut metadata = ShardMetadata::new(
                shard_id.clone(), 
                compressed_data.len() as u64, 
                Some("application/octet-stream".to_string())
            );
            metadata.tags = core_meta.tags;
            // metadata.layer_idx = Some(i); // Add if struct supports it
            
            // 🟢 CRITICAL: Inject Geometry into Legacy Metadata
            metadata.geometry = LatticeGeometry::Kagome; 

            // 7. Construct ModelShard
            let shard = ModelShard {
                shard_id: shard_id.clone(),
                layer_range: (i as usize, (i + 1) as usize),
                weights: compressed_data,
                metadata,
                parent_shards: vec![], // Populated by Fractal logic
                child_shards: vec![],
                signature: None, // Signed later
                owner: BlissId::default(), // TODO: Soul ID
            };

            shards.push(shard);
            debug!("✨ Extracted Layer {}: {} ({} bytes) -> Kagome Shard", i, layer_name, layer_size);
        }

        // 8. Fractal Decomposition (Optional)
        // If the model is huge, we recursively shard the shards
        // (Delegated to fractal.rs logic)

        Ok(shards)
    }
}

// ═══════════════════════════════════════════════════════════════════
// Error Handling
// ═══════════════════════════════════════════════════════════════════

#[derive(Debug, Error)]
pub enum TorchSliceError {
    #[error("FFI Error: {0}")]
    FfiError(String),
    #[error("Shape Mismatch")]
    ShapeMismatch,
    #[error("PyTorch Version Mismatch")]
    VersionError,
}

impl From<TorchSliceError> for SliceError {
    fn from(err: TorchSliceError) -> Self {
        SliceError::LoadError(err.to_string())
    }
}

/// Tensor metadata for shard optimization
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TensorMetadata {
    pub shape: Vec<usize>,
    pub dtype: String,
    pub layer_idx: usize,
    pub requires_grad: bool,
    /// 🟢 Phase II: Computational Density
    pub frustration: f64,
}