use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router, middleware,
};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
    compression::CompressionLayer,
};
use tracing::{info, error};
use sqlx::PgPool;

mod config;
mod database;
mod auth;
mod models;
mod handlers;
mod services;
mod middleware;
mod errors;

use config::AppConfig;
use database::Database;
use auth::AuthService;

pub struct AppState {
    pub db: PgPool,
    pub auth: AuthService,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("Starting Library Management System server...");

    // Load configuration
    let config = AppConfig::from_env()?;
    
    // Initialize database
    let database = Database::new(&config.database_url).await?;
    let db_pool = database.pool();
    
    // Run migrations
    database.migrate().await?;
    
    // Initialize authentication service
    let auth_service = AuthService::new(config.jwt_secret.clone());
    
    // Create application state
    let app_state = AppState {
        db: db_pool,
        auth: auth_service,
    };

    // Build application router
    let app = create_app().with_state(app_state);

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    let listener = TcpListener::bind(addr).await?;
    
    info!("Server listening on {}", addr);
    
    axum::serve(listener, app).await?;

    Ok(())
}

fn create_app() -> Router<AppState> {
    Router::new()
        .route("/", get(health_check))
        .nest("/api/v1", api_routes())
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive())
                .layer(CompressionLayer::new())
        )
}

fn api_routes() -> Router<AppState> {
    Router::new()
        .nest("/auth", 
            handlers::auth::routes()
                .layer(middleware::from_fn(crate::middleware::auth_rate_limit_middleware))
        )
        .nest("/books", 
            handlers::books::routes()
                .layer(middleware::from_fn(crate::middleware::general_rate_limit_middleware))
        )
        .nest("/users", 
            handlers::users::routes()
                .layer(middleware::from_fn(crate::middleware::general_rate_limit_middleware))
        )
        .nest("/circulation", 
            handlers::circulation::routes()
                .layer(middleware::from_fn(crate::middleware::general_rate_limit_middleware))
        )
        .nest("/reservations", 
            handlers::reservations::routes()
                .layer(middleware::from_fn(crate::middleware::general_rate_limit_middleware))
        )
        .nest("/reports", 
            handlers::reports::routes()
                .layer(middleware::from_fn(crate::middleware::general_rate_limit_middleware))
        )
        .nest("/admin", 
            handlers::admin::routes()
                .layer(middleware::from_fn(crate::middleware::general_rate_limit_middleware))
        )
}

async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "Library Management System",
        "version": env!("CARGO_PKG_VERSION")
    }))
}