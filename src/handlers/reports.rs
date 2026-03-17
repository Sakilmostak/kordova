use axum::{
    response::Json,
    routing::get,
    Router,
};

use crate::{errors::Result, AppState};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/circulation-stats", get(get_circulation_stats))
        .route("/popular-books", get(get_popular_books))
        .route("/overdue-report", get(get_overdue_report))
        .route("/user-activity", get(get_user_activity))
}

async fn get_circulation_stats() -> Result<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({
        "message": "Circulation statistics - Coming soon"
    })))
}

async fn get_popular_books() -> Result<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({
        "message": "Popular books report - Coming soon"
    })))
}

async fn get_overdue_report() -> Result<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({
        "message": "Overdue report - Coming soon"
    })))
}

async fn get_user_activity() -> Result<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({
        "message": "User activity report - Coming soon"
    })))
}