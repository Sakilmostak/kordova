use sqlx::{PgPool, Row};
use uuid::Uuid;
use chrono::{DateTime, NaiveDate, Utc};

use library_management_system::{
    auth::AuthService,
    models::*,
    database::Database,
    config::AppConfig,
};

pub mod test_db;
pub mod factories;

pub use test_db::TestDatabase;
pub use factories::*;

/// Test utility for creating application state
pub async fn create_test_app_state() -> (PgPool, AuthService) {
    let test_db = TestDatabase::new().await;
    let pool = test_db.pool();
    let auth_service = AuthService::new("test_secret_key".to_string());
    (pool, auth_service)
}

/// Clean up test database tables
pub async fn cleanup_database(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM transactions").execute(pool).await?;
    sqlx::query("DELETE FROM reservations").execute(pool).await?;
    sqlx::query("DELETE FROM book_copies").execute(pool).await?;
    sqlx::query("DELETE FROM books").execute(pool).await?;
    sqlx::query("DELETE FROM users").execute(pool).await?;
    Ok(())
}

/// Assert that a user exists in the database
pub async fn assert_user_exists(pool: &PgPool, user_id: Uuid) -> bool {
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_one(pool)
        .await
        .unwrap();
    count > 0
}

/// Assert that a book exists in the database
pub async fn assert_book_exists(pool: &PgPool, book_id: Uuid) -> bool {
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM books WHERE id = $1")
        .bind(book_id)
        .fetch_one(pool)
        .await
        .unwrap();
    count > 0
}

/// Assert that a transaction exists in the database
pub async fn assert_transaction_exists(pool: &PgPool, transaction_id: Uuid) -> bool {
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM transactions WHERE id = $1")
        .bind(transaction_id)
        .fetch_one(pool)
        .await
        .unwrap();
    count > 0
}

/// Get transaction count for a user
pub async fn get_user_transaction_count(pool: &PgPool, user_id: Uuid) -> i64 {
    sqlx::query_scalar("SELECT COUNT(*) FROM transactions WHERE user_id = $1")
        .bind(user_id)
        .fetch_one(pool)
        .await
        .unwrap()
}

/// Check if a book copy is available
pub async fn is_copy_available(pool: &PgPool, copy_id: Uuid) -> bool {
    let status: Option<String> = sqlx::query_scalar(
        "SELECT status::text FROM book_copies WHERE id = $1"
    )
    .bind(copy_id)
    .fetch_optional(pool)
    .await
    .unwrap();
    
    status.map_or(false, |s| s == "available")
}