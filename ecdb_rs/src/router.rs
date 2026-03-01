use axum::{
    routing::get, 
    Router,
};
use tower_http::{
    trace::TraceLayer,
    cors::CorsLayer,
};

use crate::{
    api::check,
    state::AppState,
};

pub async fn create_router(app_state: AppState) -> Router {
    let api_routes = Router::new()
        .route("/check/alive", get(check::health_check_alive))
        .route("/check/ready", get(check::health_check_ready))
        .route("/test", get(check::hello_world))
        .route("/customers", get(check::get_customers));

    Router::new()
        .nest("/api", api_routes)
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive()) // currently permissive -> edit before PROD
        .with_state(app_state)
}
