#[tokio::main]
async fn main() {
    // Initialize tracing
    ecdb::init_tracing();

    // Run the Server
    if let Err(e) = ecdb::run_server().await {
        tracing::error!("Server startup failed: {}", e);
        std::process::exit(1);
    }
}

