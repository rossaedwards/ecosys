use aurphyx_casino::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("aurphyx_casino=debug,info")
        .init();

    // Load configuration
    let config = Config::load()?;

    // Initialize database connections
    let db = Database::connect(&config.database_url).await?;
    let redis = Redis::connect(&config.redis_url).await?;

    // Start API server
    let app = api::rest::server::create_app(db, redis, config.clone()).await?;
    
    let listener = tokio::net::TcpListener::bind(&config.bind_address).await?;
    tracing::info!("🚀 Aurphyx Casino server starting on {}", config.bind_address);
    
    axum::serve(listener, app).await?;

    Ok(())
}

