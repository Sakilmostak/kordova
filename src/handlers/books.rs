use axum::{
    extract::{Path, Query, Request, State},
    middleware,
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    errors::Result,
    middleware::auth_middleware,
    models::{CreateBookRequest, SearchBooksRequest},
    services::BookService,
    AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(search_books).post(create_book))
        .route("/:id", get(get_book).put(update_book).delete(delete_book))
        .route("/:id/copies", get(get_book_copies))
        .layer(middleware::from_fn_with_state(
            AppState::default(), // This will be replaced with proper state in main.rs
            auth_middleware,
        ))
}

async fn search_books(
    State(app_state): State<AppState>,
    Query(params): Query<SearchBooksRequest>,
) -> Result<Json<serde_json::Value>> {
    let book_service = BookService::new(app_state.db);
    let result = book_service.search_books(params).await?;

    Ok(Json(serde_json::json!(result)))
}

async fn get_book(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>> {
    let book_service = BookService::new(app_state.db);
    let book = book_service.get_book(id).await?;

    Ok(Json(serde_json::json!(book)))
}

async fn create_book(
    State(app_state): State<AppState>,
    Json(request): Json<CreateBookRequest>,
) -> Result<Json<serde_json::Value>> {
    request.validate()?;
    
    let book_service = BookService::new(app_state.db);
    let book = book_service.create_book(request).await?;

    Ok(Json(serde_json::json!(book)))
}

async fn update_book(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(request): Json<CreateBookRequest>,
) -> Result<Json<serde_json::Value>> {
    request.validate()?;
    
    let book_service = BookService::new(app_state.db);
    let book = book_service.update_book(id, request).await?;

    Ok(Json(serde_json::json!(book)))
}

async fn delete_book(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>> {
    let book_service = BookService::new(app_state.db);
    book_service.delete_book(id).await?;

    Ok(Json(serde_json::json!({
        "message": "Book deleted successfully"
    })))
}

async fn get_book_copies(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>> {
    let book_service = BookService::new(app_state.db);
    let copies = book_service.get_book_copies(id).await?;

    Ok(Json(serde_json::json!(copies)))
}