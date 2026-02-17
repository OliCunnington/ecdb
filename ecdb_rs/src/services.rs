pub fn init_tracing() {
    use tracing_subscriber::{
        filter::LevelFilter, fmt, prelude::*, EnvFilter,
    };

    let rust_log = std::env::var(EnvFilter::DEFAULT_ENV)
        .unwrap_or_else(|_| "sqlx=info,tower_http=debug,info".to_string());

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .parse_lossy(rust_log),
        )
        .init();

    tracing::info!("Tracing initialized successfully");
}

pub async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = crate::config::Config::from_env();
    
    // Initialize database with config
    let db = crate::db::connection::initialize_database(&config).await?;

    // Create application state
    let state = crate::state::AppState::new(config, db);

    // Check migration status before starting server
    // crate::db::migration::migration_check(&state).await?;

    // Create router with state
    let app = crate::router::create_router(state.clone()).await;
    tracing::info!("Router created...");

    // Create TCP listener
    let listener = tokio::net::TcpListener::bind(&state.config.bind_addr).await?;
    tracing::info!("Server listening on {}", state.config.bind_addr);

    // Start server with graceful shutdown
    axum::serve(listener, app)
        .with_graceful_shutdown(handle_shutdown())
        .await?;

    tracing::info!("Server shutdown complete");

    Ok(())
}

async fn handle_shutdown() {
    // Wait for shutdown signals
    let mut sigint = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::interrupt())
        .expect("Failed to listen for SIGINT");
    let mut sigterm = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
        .expect("Failed to listen for SIGTERM");
    let mut sigquit = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::quit())
        .expect("Failed to listen for SIGQUIT");

    tokio::select! {
        _ = sigint.recv() => {
            tracing::info!("SIGINT received. Initiating graceful shutdown...");
        }
        _ = sigterm.recv() => {
            tracing::info!("SIGTERM received. Initiating graceful shutdown...");
        }
        _ = sigquit.recv() => {
            tracing::info!("SIGQUIT received. Initiating graceful shutdown...");
        }
    }
}
