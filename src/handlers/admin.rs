use axum::{
    response::Json,
    routing::{get, post},
    Router,
};

use crate::{errors::Result, AppState};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/dashboard", get(get_dashboard))
        .route("/system-status", get(get_system_status))
        .route("/bulk-import", post(bulk_import_books))
}

async fn get_dashboard() -> Result<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({
        "message": "Admin dashboard - Coming soon"
    })))
}

async fn get_system_status() -> Result<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({
        "message": "System status - Coming soon"
    })))
}

async fn bulk_import_books() -> Result<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({
        "message": "Bulk import - Coming soon"
    })))
}