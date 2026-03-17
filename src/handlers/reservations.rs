use axum::{
    response::Json,
    routing::{delete, get, post},
    Router,
};

use crate::{errors::Result, AppState};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/reserve", post(create_reservation))
        .route("/my-reservations", get(get_my_reservations))
        .route("/reservations/:id", delete(cancel_reservation))
        .route("/queue/:book_id", get(get_reservation_queue))
}

async fn create_reservation() -> Result<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({
        "message": "Create reservation - Coming soon"
    })))
}

async fn get_my_reservations() -> Result<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({
        "message": "My reservations - Coming soon"
    })))
}

async fn cancel_reservation() -> Result<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({
        "message": "Cancel reservation - Coming soon"
    })))
}

async fn get_reservation_queue() -> Result<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({
        "message": "Reservation queue - Coming soon"
    })))
}