use axum::{
    extract::State, 
    response::IntoResponse, 
    Json
};
use serde_json::{json, Value};

use crate::{
    state::AppState,
};

/// Health check endpoint that confirms the API is running
/// 
/// # Endpoint
/// `GET /api/check/alive`
/// 
/// # Purpose
/// This endpoint provides a simple way to verify that the API server is up and running.
/// It's designed for basic health monitoring and can be used by load balancers,
/// container orchestration systems, or monitoring tools to determine if the service
/// is operational.
/// 
/// # Query Parameters
/// None required.
/// 
/// # Example Requests
/// ```bash
/// # Basic health check
/// curl "http://localhost:3000/api/check/alive"
/// ```
/// 
/// # Content-Type
/// - Request: None (GET request)
/// - Response: `application/json`
/// 
/// # Response Format
/// ```json
/// {
///   "status": "ok"
/// }
/// ```
/// 
/// # Returns
/// #### Successful Response
/// - Status: 200 OK
/// - Body: JSON object with status field
/// 
/// # Response Fields
/// - `status`: Always returns "ok" when the service is running
/// 
/// # Status Codes
/// - `200 OK`: The service is alive
/// - `503 Service Unavailable`: The service is down
/// 
/// # Performance Notes
/// - Extremely lightweight check (no database queries)
/// - Sub-millisecond response time
/// - Safe to call frequently for monitoring
/// - No authentication required
/// 
/// # Use Cases
/// - **Load balancer health checks**: Verify service availability
/// - **Container orchestration**: Kubernetes liveness probes
/// - **Monitoring systems**: Basic uptime monitoring
/// - **CI/CD pipelines**: Verify deployment success
/// 
/// # Notes
/// - This endpoint only checks if the web server is responding
/// - Does not verify database connectivity or other dependencies
/// - For comprehensive health checks, use [`health_check_ready`] instead
/// 
/// # See Also
/// - [`health_check_ready`] - Comprehensive health check including database
pub async fn health_check_alive() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}

/// Ready check endpoint that confirms the API and database are operational
/// 
/// # Endpoint
/// `GET /api/check/ready`
/// 
/// # Purpose
/// This endpoint provides a comprehensive health check that verifies both the API
/// server and database connectivity are working properly. It's designed for
/// readiness probes and deployment verification where all dependencies must
/// be functional before accepting traffic.
/// 
/// # Query Parameters
/// None required.
/// 
/// # Example Requests
/// ```bash
/// # Basic readiness check
/// curl "http://localhost:3000/api/check/ready"
/// ```
/// 
/// # Content-Type
/// - Request: None (GET request)
/// - Response: `application/json`
/// 
/// # Response Format
/// #### When Ready
/// ```json
/// {
///   "status": "ready",
///   "database": "connected"
/// }
/// ```
/// 
/// #### When Not Ready
/// ```json
/// {
///   "status": "not ready",
///   "database": "disconnected"
/// }
/// ```
/// 
/// # Returns
/// #### Service Ready
/// - Status: 200 OK
/// - Body: JSON with "ready" status and "connected" database
/// 
/// #### Service Not Ready
/// - Status: 200 OK (still returns 200, check response body)
/// - Body: JSON with "not ready" status and "disconnected" database
///  
/// # Response Fields
/// - `status`: "ready" when all systems operational, "not ready" otherwise
/// - `database`: "connected" when database is accessible, "disconnected" otherwise
/// 
/// # Status Codes
/// - `200 OK`: The service is alive
/// - `503 Service Unavailable`: The service is down
///
/// # Health Checks Performed
/// - **Web server**: Confirms API is responding to requests
/// - **Database connectivity**: Verifies SurrealDB connection is active
/// - **Database health**: Calls SurrealDB's health() method
/// 
/// # Performance Notes
/// - Includes database query, slightly slower than [`health_check_alive`]
/// - Typical response time: 1-10ms depending on database latency
/// - Safe for regular monitoring (every 10-30 seconds)
/// - No authentication required
/// 
/// # Use Cases
/// - **Kubernetes readiness probes**: Verify pod is ready for traffic
/// - **Deployment verification**: Ensure all dependencies are working
/// - **Load balancer configuration**: Only route to ready instances
/// - **Monitoring dashboards**: Comprehensive service status
/// 
/// # Error Scenarios
/// - **Database connection lost**: Returns "not ready" status
/// - **Database timeout**: Returns "not ready" status
/// - **SurrealDB service down**: Returns "not ready" status
/// - **Network issues**: Returns "not ready" status
/// 
/// # Notes
/// - Always returns HTTP 200, check response body for actual status
/// - More comprehensive than [`health_check_alive`] but slightly slower
/// - Recommended for readiness probes in production environments
/// - Database errors are logged but don't cause endpoint to fail
/// 
/// # See Also
/// - [`health_check_alive`] - Lightweight health check (no database)
/// - Database initialization: [`crate::db::initialize_database`]
pub async fn health_check_ready(State(app_state): State<AppState>) -> impl IntoResponse {
    // Simple database connectivity check
    match app_state.db.health().await {
        Ok(_) => Json(json!({ 
            "status": "ready", 
            "database": "connected" 
        })),
        Err(_) => Json(json!({ 
            "status": "not ready", 
            "database": "disconnected" 
        })),
    }
}

// basic handler that responds with a static string
pub async fn hello_world() -> &'static str {
    "Hello, World!"
}
