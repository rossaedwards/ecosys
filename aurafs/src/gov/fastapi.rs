//! AuraFS Governance FastAPI-compatible REST API Layer
//! f0rg3d in l0v3 by Ross Edwards & Aurphyx

use axum::{
    routing::{get, post},
    Router,
    Json,
    response::IntoResponse,
    extract::{Path, State, Query},
    http::StatusCode,
};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tower_http::cors::CorsLayer;

use crate::{
    GovernanceSystem, ProposalStatus, VoteOption,
    ProposalType, SoulProof,
};

/// Shared application state for axum handlers
#[derive(Clone)]
pub struct AppState {
    pub governance: Arc<GovernanceSystem>,
}

// === Request Models ===

#[derive(Deserialize)]
pub struct CreateProposalRequest {
    pub creator_bliss_id: String,
    pub title: String,
    pub description: String,
    pub proposal_type: ProposalType,
    pub voting_period_hours: i64,
}

#[derive(Deserialize)]
pub struct CastVoteRequest {
    pub proposal_id: String,
    pub bliss_id: String,
    pub option: VoteOption,
    pub quantum_signature: String,
    pub soul_proof: SoulProof,
}

#[derive(Deserialize)]
pub struct QueryParams {
    pub status: Option<ProposalStatus>,
}

// === Response Models ===

#[derive(Serialize)]
struct ApiResponse<T> {
    success: bool,
    data: T,
}

// === Handlers ===

async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "aurafs-governance-fastapi",
        "version": "1.0.0"
    }))
}

async fn create_proposal(
    State(state): State<AppState>,
    Json(payload): Json<CreateProposalRequest>,
) -> Result<Json<ApiResponse<serde_json::Value>>, (StatusCode, String)> {
    let proposal = state.governance.proposal_manager.create_proposal(
        payload.creator_bliss_id,
        payload.title,
        payload.description,
        payload.proposal_type,
        payload.voting_period_hours,
        None,
    ).map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    Ok(Json(ApiResponse {
        success: true,
        data: serde_json::to_value(proposal).unwrap(),
    }))
}

async fn list_proposals(
    State(state): State<AppState>,
    Query(query): Query<QueryParams>,
) -> Json<ApiResponse<Vec<serde_json::Value>>> {
    let proposals = state.governance.proposal_manager.list_proposals(query.status);
    let data = proposals.into_iter()
        .map(|p| serde_json::to_value(p).unwrap())
        .collect();

    Json(ApiResponse {
        success: true,
        data,
    })
}

async fn cast_vote(
    State(state): State<AppState>,
    Json(payload): Json<CastVoteRequest>,
) -> Result<Json<ApiResponse<serde_json::Value>>, (StatusCode, String)> {
    let signature = hex::decode(&payload.quantum_signature)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid signature encoding: {}", e)))?;

    let vote = state.governance.voting_engine.cast_vote(
        payload.proposal_id,
        payload.bliss_id,
        payload.option,
        signature,
        payload.soul_proof,
        None,
    ).await.map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(ApiResponse {
        success: true,
        data: serde_json::to_value(vote).unwrap(),
    }))
}

// === Router Setup ===

/// Build axum Router configured for AuraFS governance API
pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/proposals", post(create_proposal))
        .route("/proposals", get(list_proposals))
        .route("/votes", post(cast_vote))
        .with_state(state)
        .layer(CorsLayer::permissive())
}

/// Start the FastAPI-compatible governance server at given port
pub async fn start_server(governance: Arc<GovernanceSystem>, port: u16) -> Result<(), hyper::Error> {
    use std::net::SocketAddr;
    use axum::Server;

    let state = AppState { governance };
    let app = build_router(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("🚀 AuraFS Governance FastAPI serving at {}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
}