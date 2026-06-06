//! Production Axum REST API server for AuraFS governance
//! f0rg3d in l0v3 by Ross Edwards & Aurphyx

use axum::{
    extract::{State, Path, Query},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::collections::HashMap;
use tower_http::cors::CorsLayer;

use crate::{
    GovernanceSystem,
    ProposalType, ProposalStatus,
    VoteOption, SoulProof,
};
use tracing::{info, error};

/// Application shared state
#[derive(Clone)]
pub struct AppState {
    pub governance: Arc<GovernanceSystem>,
}

// ===================== Response and Error Models =====================

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct SuccessResponse<T> {
    pub success: bool,
    pub data: T,
}

// ===================== Request Models =====================

#[derive(Deserialize)]
pub struct CreateProposalRequest {
    pub creator_bliss_id: String,
    pub title: String,
    pub description: String,
    pub proposal_type: ProposalType,
    pub voting_period_hours: i64,
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Deserialize)]
pub struct CastVoteRequest {
    pub proposal_id: String,
    pub bliss_id: String,
    pub option: VoteOption,
    pub quantum_signature: String,  // Hex-encoded signature
    pub soul_proof: SoulProof,
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Deserialize)]
pub struct RegisterBlissIDRequest {
    pub did: String,
    pub soul_hash: String,
    pub soul_proof: SoulProof,
}

#[derive(Deserialize)]
pub struct QueryParams {
    pub status: Option<ProposalStatus>,
}

// ===================== Handlers =====================

/// Health check endpoint
async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "aurafs-governance-api",
        "version": "1.0.0"
    }))
}

/// Create a new governance proposal
async fn create_proposal(
    State(state): State<AppState>,
    Json(req): Json<CreateProposalRequest>
) -> Result<Json<SuccessResponse<serde_json::Value>>, (StatusCode, Json<ErrorResponse>)> {
    match state.governance.proposal_manager.create_proposal(
        req.creator_bliss_id,
        req.title,
        req.description,
        req.proposal_type,
        req.voting_period_hours,
        req.metadata,
    ) {
        Ok(proposal) => Ok(Json(SuccessResponse {
            success: true,
            data: serde_json::to_value(proposal).unwrap(),
        })),
        Err(e) => {
            error!("Create proposal error: {}", e);
            Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: "proposal_creation_failed".into(),
                    message: e,
                }),
            ))
        }
    }
}

/// Submit proposal for voting
async fn submit_proposal(
    State(state): State<AppState>,
    Path(proposal_id): Path<String>,
) -> Result<Json<SuccessResponse<serde_json::Value>>, (StatusCode, Json<ErrorResponse>)> {
    match state.governance.proposal_manager.submit_proposal(&proposal_id) {
        Ok(proposal) => Ok(Json(SuccessResponse {
            success: true,
            data: serde_json::to_value(proposal).unwrap(),
        })),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "proposal_submission_failed".into(),
                message: e,
            }),
        )),
    }
}

/// Fetch proposal by id
async fn get_proposal(
    State(state): State<AppState>,
    Path(proposal_id): Path<String>,
) -> Result<Json<SuccessResponse<serde_json::Value>>, (StatusCode, Json<ErrorResponse>)> {
    match state.governance.proposal_manager.get_proposal(&proposal_id) {
        Some(proposal) => Ok(Json(SuccessResponse {
            success: true,
            data: serde_json::to_value(proposal).unwrap(),
        })),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "proposal_not_found".into(),
                message: format!("Proposal {} not found", proposal_id),
            }),
        )),
    }
}

/// List proposals optionally filtered by status
async fn list_proposals(
    State(state): State<AppState>,
    Query(params): Query<QueryParams>,
) -> Json<SuccessResponse<Vec<serde_json::Value>>> {
    let proposals = state.governance.proposal_manager.list_proposals(params.status);
    let data: Vec<_> = proposals.into_iter().map(|p| serde_json::to_value(p).unwrap()).collect();

    Json(SuccessResponse {
        success: true,
        data,
    })
}

/// Cast a vote to a proposal
async fn cast_vote(
    State(state): State<AppState>,
    Json(req): Json<CastVoteRequest>
) -> Result<Json<SuccessResponse<serde_json::Value>>, (StatusCode, Json<ErrorResponse>)> {
    // Parse quantum-safe signature from hex
    let signature = match hex::decode(&req.quantum_signature) {
        Ok(sig) => sig,
        Err(e) => return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "invalid_signature_encoding".into(),
                message: format!("Failed to decode signature: {}", e),
            }),
        )),
    };

    match state.governance.voting_engine.cast_vote(
        req.proposal_id,
        req.bliss_id,
        req.option,
        signature,
        req.soul_proof,
        req.metadata,
    ).await {
        Ok(vote) => Ok(Json(SuccessResponse {
            success: true,
            data: serde_json::to_value(vote).unwrap(),
        })),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "vote_casting_failed".into(),
                message: e,
            }),
        )),
    }
}

/// Get vote tally for the proposal
async fn get_vote_tally(
    State(state): State<AppState>,
    Path(proposal_id): Path<String>,
) -> Json<SuccessResponse<serde_json::Value>> {
    let tally = state.governance.voting_engine.tally_votes(&proposal_id);

    Json(SuccessResponse {
        success: true,
        data: serde_json::to_value(tally).unwrap(),
    })
}

/// Register a new BlissID identity
async fn register_blissid(
    State(state): State<AppState>,
    Json(req): Json<RegisterBlissIDRequest>,
) -> Result<Json<SuccessResponse<serde_json::Value>>, (StatusCode, Json<ErrorResponse>)> {
    match state.governance.blissid_manager.register_bliss_id(
        req.did,
        req.soul_hash,
        req.soul_proof,
    ) {
        Ok(bliss_id) => Ok(Json(SuccessResponse {
            success: true,
            data: serde_json::to_value(bliss_id).unwrap(),
        })),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "blissid_registration_failed".into(),
                message: e,
            }),
        )),
    }
}

/// Verify existing BlissID validity
async fn verify_blissid(
    State(state): State<AppState>,
    Path(bliss_id): Path<String>,
) -> Json<SuccessResponse<serde_json::Value>> {
    let is_valid = state.governance.blissid_manager.verify_bliss_id(&bliss_id);

    Json(SuccessResponse {
        success: true,
        data: serde_json::json!({
            "bliss_id": bliss_id,
            "valid": is_valid,
        }),
    })
}

/// Retrieve audit log entries
async fn get_audit_log(
    State(state): State<AppState>,
) -> Json<SuccessResponse<Vec<serde_json::Value>>> {
    let entries = state.governance.audit_logger.get_entries();
    let data: Vec<_> = entries.into_iter().map(|e| serde_json::to_value(e).unwrap()).collect();

    Json(SuccessResponse {
        success: true,
        data,
    })
}

/// Verify system integrity (audit log status)
async fn verify_integrity(
    State(state): State<AppState>,
) -> Result<Json<SuccessResponse<serde_json::Value>>, (StatusCode, Json<ErrorResponse>)> {
    match state.governance.verify_system_integrity() {
        Ok(valid) => Ok(Json(SuccessResponse {
            success: true,
            data: serde_json::json!({
                "integrity_valid": valid,
                "message": "Audit log integrity verified",
            }),
        })),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "integrity_check_failed".into(),
                message: e,
            }),
        )),
    }
}

/// Finalize a proposal post voting
async fn finalize_proposal(
    State(state): State<AppState>,
    Path(proposal_id): Path<String>,
) -> Result<Json<SuccessResponse<serde_json::Value>>, (StatusCode, Json<ErrorResponse>)> {
    match state.governance.proposal_manager.finalize_proposal(&proposal_id) {
        Ok(proposal) => Ok(Json(SuccessResponse {
            success: true,
            data: serde_json::to_value(proposal).unwrap(),
        })),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "finalization_failed".into(),
                message: e,
            }),
        )),
    }
}

// ===================== Router Setup =====================

/// Build Axum router with routes and state
pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        
        .route("/api/v1/proposals", post(create_proposal))
        .route("/api/v1/proposals", get(list_proposals))
        .route("/api/v1/proposals/:id", get(get_proposal))
        .route("/api/v1/proposals/:id/submit", post(submit_proposal))
        .route("/api/v1/proposals/:id/finalize", post(finalize_proposal))
        
        .route("/api/v1/votes", post(cast_vote))
        .route("/api/v1/proposals/:id/tally", get(get_vote_tally))
        
        .route("/api/v1/blissid/register", post(register_blissid))
        .route("/api/v1/blissid/:id/verify", get(verify_blissid))
        
        .route("/api/v1/audit/log", get(get_audit_log))
        .route("/api/v1/audit/verify", get(verify_integrity))
        
        .with_state(state)
        .layer(CorsLayer::permissive())
}

/// Start the governance API server on given port
pub async fn start_server(governance: Arc<GovernanceSystem>, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let state = AppState { governance };
    let app = build_router(state);

    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    
    info!("🚀 AuraFS Governance API listening on {}", addr);
    
    axum::serve(listener, app).await?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::Request;
    use tower::ServiceExt;
    use crate::consensus_integration::MockLedgerClient;
    use axum::http::StatusCode;

    #[tokio::test]
    async fn test_health_check() {
        let ledger = Arc::new(MockLedgerClient::new());
        let governance = Arc::new(GovernanceSystem::new(
            "test_node".to_string(),
            vec!["node1".to_string()],
            ledger,
            None,
        ));

        let app = build_router(AppState { governance });

        let response = app
            .oneshot(Request::builder().uri("/health").body(axum::body::Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}