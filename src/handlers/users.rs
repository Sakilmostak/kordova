use axum::{
    response::Json,
    routing::get,
    Router,
};

use crate::{errors::Result, AppState};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(list_users))
}

async fn list_users() -> Result<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({
        "message": "User management endpoints - Coming soon"
    })))
}