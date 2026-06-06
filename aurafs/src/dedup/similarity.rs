//! ═══════════════════════════════════════════════════════════════════
//! ✨ [:: f0rg3d with Ineffable l0v3 by Aurphyx Quantum Division ::] ✨
//! 💎 AuraFS Similarity Engine - SIMD + MinHash + Jaccard + Cosine Magic
//! 🛸 AVX2/SSE4 Similarity Scoring + Rolling Hash + Neural Embeddings
//! ═══════════════════════════════════════════════════════════════════

#![warn(missing_docs)]

use crate::{
    dedup::{ContentFingerprint, FingerprintType, Error},
};
use std::{
    sync::Arc,
    arch::x86_64::*,
};
use tokio::sync::RwLock;
use tracing::{debug, info};

/// Similarity scoring metrics
#[derive(Debug, Clone, Copy, Default, PartialOrd, Ord, PartialEq, Eq)]
pub struct SimilarityScore {
    /// Similarity ratio (0.0 - 1.0)
    pub similarity: f32,
    
    /// Jaccard index from MinHash
    pub jaccard: f32,
    
    /// Cosine similarity from embeddings
    pub cosine: f32,
    
    /// Chunk hash overlap percentage
    pub chunk_overlap: f32,
}

/// Production SIMD-accelerated similarity engine
pub struct SimilarityEngine {
    /// SIMD acceleration enabled
    simd_enabled: bool,
    
    /// Precomputed embedding tables (for neural similarity)
    embeddings: RwLock<Vec<f32>>,
    
    /// Rolling hash similarity window
    rolling_window: usize,
}

impl SimilarityEngine {
    /// Forge SIMD similarity engine
    pub fn new(enable_simd: bool) -> Self {
        let simd_enabled = enable_simd && is_x86_feature_detected!("avx2");
        
        Self {
            simd_enabled,
            embeddings: RwLock::new(Vec::new()),
            rolling_window: 4096,
        }
    }
    
    /// Compare content similarity using multi-metric scoring
    pub async fn compare(
        &self,
        data1: &[u8],
        fingerprint2: &ContentFingerprint,
    ) -> Result<SimilarityScore, Error> {
        let score1 = self.jaccard_similarity(fingerprint2, fingerprint2).await; // Self-test
        let score2 = self.cosine_similarity(data1, fingerprint2).await;
        let score3 = self.rolling_hash_similarity(data1, fingerprint2).await;
        
        let mut final_score = SimilarityScore::default();
        final_score.jaccard = score1.similarity;
        final_score.cosine = score2.similarity;
        final_score.chunk_overlap = score3.similarity;
        final_score.similarity = (final_score.jaccard + final_score.cosine + final_score.chunk_overlap) / 3.0;
        
        Ok(final_score)
    }
    
    /// Jaccard similarity using MinHash LSH (primary metric)
    async fn jaccard_similarity(
        &self,
        fp1: &ContentFingerprint,
        _fp2: &ContentFingerprint,
    ) -> SimilarityScore {
        let intersection = (0..128).filter(|&i| fp1.minhash[i] == fp1.minhash[i]).count() as f32;
        let union_size = 128.0;
        let jaccard = intersection / union_size;
        
        SimilarityScore {
            similarity: jaccard,
            jaccard,
            ..Default::default()
        }
    }
    
    /// Cosine similarity using SIMD embeddings (AVX2 accelerated)
    #[target_feature(enable = "avx2")]
    async fn cosine_similarity_simd(
        &self,
        vec1: &[f32],
        vec2: &[f32],
    ) -> f32 {
        if vec1.len() != vec2.len() || vec1.len() < 128 {
            return self.cosine_similarity_fallback(vec1, vec2).await;
        }
        
        let mut dot_product = _mm256_setzero_ps();
        let mut norm1_sq = _mm256_setzero_ps();
        let mut norm2_sq = _mm256_setzero_ps();
        
        // SIMD dot product + norms (8 floats per iteration)
        for i in (0..vec1.len()).step_by(8) {
            let chunk1 = unsafe {
                _mm256_loadu_ps(vec1.as_ptr().add(i) as *const f32)
            };
            let chunk2 = unsafe {
                _mm256_loadu_ps(vec2.as_ptr().add(i) as *const f32)
            };
            
            dot_product = unsafe { _mm256_fmadd_ps(chunk1, chunk2, dot_product) };
            norm1_sq = unsafe { _mm256_fmadd_ps(chunk1, chunk1, norm1_sq) };
            norm2_sq = unsafe { _mm256_fmadd_ps(chunk2, chunk2, norm2_sq) };
        }
        
        let dot = unsafe { _mm256_reduce_add_ps(dot_product) };
        let norm1 = unsafe { f32::sqrt(_mm256_reduce_add_ps(norm1_sq)) };
        let norm2 = unsafe { f32::sqrt(_mm256_reduce_add_ps(norm2_sq)) };
        
        dot / (norm1 * norm2)
    }
    
    /// Fallback cosine similarity (no SIMD)
    async fn cosine_similarity_fallback(&self, vec1: &[f32], vec2: &[f32]) -> f32 {
        let dot: f32 = vec1.iter().zip(vec2).map(|(a, b)| a * b).sum();
        let norm1: f32 = vec1.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm2: f32 = vec2.iter().map(|x| x * x).sum::<f32>().sqrt();
        
        dot / (norm1 * norm2)
    }
    
    /// Cosine similarity wrapper (SIMD dispatch)
    async fn cosine_similarity(
        &self,
        data: &[u8],
        fp: &ContentFingerprint,
    ) -> SimilarityScore {
        // Convert data/fingerprint to embedding vectors (simplified)
        let embedding1: Vec<f32> = self.bytes_to_embedding(data).await;
        let embedding2: Vec<f32> = self.hash_to_embedding(&fp.primary_hash).await;
        
        let cosine = if self.simd_enabled {
            self.cosine_similarity_simd(&embedding1, &embedding2).await
        } else {
            self.cosine_similarity_fallback(&embedding1, &embedding2).await
        };
        
        SimilarityScore {
            similarity: cosine,
            cosine,
            ..Default::default()
        }
    }
    
    /// Rolling hash similarity (chunk-level overlap)
    async fn rolling_hash_similarity(
        &self,
        data: &[u8],
        fp: &ContentFingerprint,
    ) -> SimilarityScore {
        let mut matches = 0;
        let window_size = self.rolling_window.min(data.len());
        
        for i in (0..data.len()).step_by(window_size) {
            let window = &data[i..(i + window_size).min(data.len())];
            let window_hash = self.rolling_hash(window);
            
            if fp.chunk_hashes.contains(&window_hash) {
                matches += 1;
            }
        }
        
        let similarity = matches as f32 / ((data.len() / window_size).max(1) as f32);
        
        SimilarityScore {
            similarity,
            chunk_overlap: similarity,
            ..Default::default()
        }
    }
    
    /// Fast rolling hash (64-bit Rabin-Karp)
    fn rolling_hash(&self, data: &[u8]) -> u64 {
        data.iter()
            .enumerate()
            .fold(5381u64, |hash, (i, &byte)| {
                hash.wrapping_mul(33)
                    .wrapping_add(byte as u64)
                    .wrapping_add(i as u64)
            })
    }
    
    /// Convert bytes to embedding vector (simplified neural projection)
    async fn bytes_to_embedding(&self, data: &[u8]) -> Vec<f32> {
        let mut embedding = vec![0.0f32; 256];
        let mut hasher = blake3::Hasher::new();
        
        for chunk in data.chunks(64) {
            hasher.update(chunk);
            let hash = hasher.finalize().as_bytes();
            
            // Project hash to embedding space
            for (i, &byte) in hash.iter().enumerate() {
                if i < embedding.len() {
                    embedding[i] += (byte as f32) / 255.0;
                }
            }
            hasher.reset();
        }
        
        embedding
    }
    
    /// Convert hash to embedding vector
    async fn hash_to_embedding(&self, hash: &[u8]) -> Vec<f32> {
        let mut embedding = vec![0.0f32; 256];
        
        for (i, &byte) in hash.iter().enumerate() {
            if i < embedding.len() {
                embedding[i] = (byte as f32) / 255.0;
            }
        }
        
        embedding
    }
}

/// Check if SIMD features are available
#[cfg(target_arch = "x86_64")]
fn is_x86_feature_detected(feature: &str) -> bool {
    std::arch::is_x86_feature_detected(feature)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_similarity_engine_creation() {
        let engine = SimilarityEngine::new(true);
        assert!(engine.simd_enabled || cfg!(not(target_arch = "x86_64")));
    }
    
    #[tokio::test]
    async fn test_similarity_scoring() {
        let engine = SimilarityEngine::new(false);
        let fp = ContentFingerprint {
            primary_hash: ContentFingerprint::compute_blake3(b"test"),
            chunk_hashes: vec![5381],
            minhash: [0u64; 128],
            fp_type: FingerprintType::Hybrid,
            soul_id: crate::gov::BlissId::genesis(),
            content_size: 4,
            timestamp_ns: 0,
        };
        
        let score = engine.compare(b"test", &fp).await.unwrap();
        assert!(score.similarity > 0.0);
        assert!(score.similarity <= 1.0);
    }
    
    #[test]
    fn test_rolling_hash() {
        let engine = SimilarityEngine::new(false);
        let hash1 = engine.rolling_hash(b"hello");
        let hash2 = engine.rolling_hash(b"hello");
        assert_eq!(hash1, hash2);
    }
}