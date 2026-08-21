use crate::prelude::*;
use axum::Router;
use tower_http::cors::CorsLayer;

pub async fn create_app(
    _db: crate::database::Database,
    _redis: crate::database::Redis,
    _config: crate::config::Config,
) -> Result<Router> {
    let app = Router::new()
        .layer(CorsLayer::permissive())
        .route("/health", axum::routing::get(health_check));

    Ok(app)
}

async fn health_check() -> &'static str {
    "OK"
}

