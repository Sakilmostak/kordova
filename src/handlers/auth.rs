use axum::{
    extract::{Request, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use validator::Validate;

use crate::{
    errors::{AppError, Result},
    middleware::get_current_user_id,
    models::{CreateUserRequest, LoginRequest},
    services::AuthServiceImpl,
    AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        .route("/profile", get(get_profile))
}

async fn login(
    State(app_state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<serde_json::Value>> {
    request.validate()?;
    
    let auth_service = AuthServiceImpl::new(app_state.db, app_state.auth);
    let response = auth_service.login(request).await?;

    Ok(Json(serde_json::json!(response)))
}

async fn register(
    State(app_state): State<AppState>,
    Json(request): Json<CreateUserRequest>,
) -> Result<Json<serde_json::Value>> {
    request.validate()?;
    
    let auth_service = AuthServiceImpl::new(app_state.db, app_state.auth);
    let user_profile = auth_service.register_user(request).await?;

    Ok(Json(serde_json::json!({
        "user": user_profile,
        "message": "User registered successfully"
    })))
}

async fn get_profile(
    State(app_state): State<AppState>,
    request: Request,
) -> Result<Json<serde_json::Value>> {
    let user_id = get_current_user_id(&request)?;
    
    let auth_service = AuthServiceImpl::new(app_state.db, app_state.auth);
    let profile = auth_service.get_user_profile(user_id).await?;

    Ok(Json(serde_json::json!(profile)))
}