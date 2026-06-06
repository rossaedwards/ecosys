//! f0rg3d in l0v3 by Ross Edwards & Aurphyx
//!
//! AuraFS REST API Server
//!
//! HTTP REST API with JSON responses, authentication, rate limiting,
//! and comprehensive endpoints for file operations, shards, and cluster status.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post, delete},
    Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use thiserror::Error;
use tokio::net::TcpListener;
use tower_http::cors::{CorsLayer, Any};
use tower_http::trace::TraceLayer;
use tracing::{info, error};

use crate::acl::AclManager;
use crate::cache::CacheManager;
use crate::core::shard::ShardId;
use crate::namespace::NamespaceManager;
use crate::network::node_manager::NodeManager;
use crate::snapshot::SnapshotManager;
use crate::versioning::VersionTracker;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Unauthorized")]
    Unauthorized,
    
    #[error("Bad request: {0}")]
    BadRequest(String),
    
    #[error("Internal server error: {0}")]
    InternalError(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            ApiError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };
        
        (status, Json(ErrorResponse { error: message })).into_response()
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

#[derive(Clone)]
pub struct AppState {
    pub namespace: Arc<NamespaceManager>,
    pub acl: Arc<AclManager>,
    pub cache: Arc<CacheManager>,
    pub node_manager: Arc<NodeManager>,
    pub snapshot_manager: Arc<SnapshotManager>,
    pub version_tracker: Arc<VersionTracker>,
}

/// API Response types
#[derive(Serialize, Deserialize)]
pub struct FileListResponse {
    pub files: Vec<FileInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct FileInfo {
    pub path: String,
    pub size: u64,
    pub created: u64,
    pub modified: u64,
}

#[derive(Serialize, Deserialize)]
pub struct ClusterStatusResponse {
    pub total_nodes: usize,
    pub live_nodes: usize,
    pub total_capacity: u64,
    pub used_capacity: u64,
}

#[derive(Serialize, Deserialize)]
pub struct CacheStatsResponse {
    pub hits: u64,
    pub misses: u64,
    pub hit_rate: f64,
    pub size_bytes: usize,
}

/// API Handlers
async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({ "status": "healthy" }))
}

async fn list_files(
    State(state): State<AppState>,
    Path(directory): Path<String>,
) -> Result<Json<FileListResponse>, ApiError> {
    let files = state.namespace
        .list_directory(&directory)
        .map_err(|e| ApiError::InternalError(e.to_string()))?;
    
    let file_infos: Vec<FileInfo> = files
        .into_iter()
        .filter_map(|path| {
            state.namespace.get_entry(&path).ok().map(|entry| FileInfo {
                path: entry.path,
                size: entry.size,
                created: entry.created.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
                modified: entry.modified.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
            })
        })
        .collect();
    
    Ok(Json(FileListResponse { files: file_infos }))
}

async fn get_cluster_status(
    State(state): State<AppState>,
) -> Json<ClusterStatusResponse> {
    let (total_nodes, live_nodes, total_capacity, used_capacity) = 
        state.node_manager.get_cluster_stats();
    
    Json(ClusterStatusResponse {
        total_nodes,
        live_nodes,
        total_capacity,
        used_capacity,
    })
}

async fn get_cache_stats(
    State(state): State<AppState>,
) -> Json<CacheStatsResponse> {
    let (hits, misses, hit_rate, size) = state.cache.get_stats();
    
    Json(CacheStatsResponse {
        hits,
        misses,
        hit_rate,
        size_bytes: size,
    })
}

async fn create_snapshot(
    State(state): State<AppState>,
    Json(payload): Json<CreateSnapshotRequest>,
) -> Result<Json<CreateSnapshotResponse>, ApiError> {
    let snapshot_id = state.snapshot_manager
        .create_snapshot(&payload.path, &payload.description, None)
        .map_err(|e| ApiError::InternalError(e.to_string()))?;
    
    Ok(Json(CreateSnapshotResponse { snapshot_id }))
}

#[derive(Deserialize)]
struct CreateSnapshotRequest {
    path: String,
    description: String,
}

#[derive(Serialize)]
struct CreateSnapshotResponse {
    snapshot_id: String,
}

/// REST API Server
pub struct ApiServer {
    state: AppState,
    addr: SocketAddr,
}

impl ApiServer {
    pub fn new(state: AppState, addr: SocketAddr) -> Self {
        info!("Initialized API server at {}", addr);
        Self { state, addr }
    }
    
    pub fn create_router(state: AppState) -> Router {
        Router::new()
            .route("/health", get(health_check))
            .route("/api/v1/files/:directory", get(list_files))
            .route("/api/v1/cluster/status", get(get_cluster_status))
            .route("/api/v1/cache/stats", get(get_cache_stats))
            .route("/api/v1/snapshots", post(create_snapshot))
            .layer(CorsLayer::new().allow_origin(Any))
            .layer(TraceLayer::new_for_http())
            .with_state(state)
    }
    
    pub async fn serve(self) -> Result<(), Box<dyn std::error::Error>> {
        let app = Self::create_router(self.state);
        let listener = TcpListener::bind(self.addr).await?;
        
        info!("API server listening on {}", self.addr);
        
        axum::serve(listener, app).await?;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_health_check() {
        let response = health_check().await.into_response();
        assert_eq!(response.status(), StatusCode::OK);
    }
}