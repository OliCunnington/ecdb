use axum::{
    routing::get,
    routing::post, 
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
        .route("/customers", get(check::get_customers))
        .route("/products", get(check::get_products))
        .route("/product/:id", get(check::get_product))
        .route("/signup", post(check::sign_up))
        .route("/signin", post(check::sign_in))
        .route("/signout", get(check::sign_out))
        .route("/session", get(check::session));

    Router::new()
        .nest("/api", api_routes)
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive()) // currently permissive -> edit before PROD
        .with_state(app_state)
}
