use sqlx::{PgPool, Postgres, migrate::MigrateDatabase};
use std::sync::Once;

static INIT: Once = Once::new();

pub struct TestDatabase {
    pool: PgPool,
    database_url: String,
}

impl TestDatabase {
    pub async fn new() -> Self {
        INIT.call_once(|| {
            dotenvy::dotenv().ok();
        });

        let database_url = std::env::var("TEST_DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://postgres:password@localhost/lms_test".to_string());

        // Create database if it doesn't exist
        if !Postgres::database_exists(&database_url).await.unwrap_or(false) {
            Postgres::create_database(&database_url).await.unwrap();
        }

        let pool = PgPool::connect(&database_url).await.unwrap();
        
        // Run migrations
        sqlx::migrate!("../migrations").run(&pool).await.unwrap();

        Self {
            pool,
            database_url,
        }
    }

    pub fn pool(&self) -> PgPool {
        self.pool.clone()
    }

    pub async fn cleanup(&self) {
        // Drop and recreate database for clean state
        sqlx::query("DROP SCHEMA public CASCADE; CREATE SCHEMA public;")
            .execute(&self.pool)
            .await
            .unwrap();
        
        // Re-run migrations
        sqlx::migrate!("../migrations").run(&self.pool).await.unwrap();
    }
}

impl Drop for TestDatabase {
    fn drop(&mut self) {
        // Cleanup is handled by the cleanup method
    }
}