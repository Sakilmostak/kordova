use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub redis_url: String,
    pub jwt_secret: String,
    pub port: u16,
    pub environment: String,
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, env::VarError> {
        dotenvy::dotenv().ok(); // Load .env file if present
        
        // CRITICAL SECURITY: JWT_SECRET must be provided in production
        let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| {
            let environment = env::var("ENVIRONMENT").unwrap_or_else(|| "development".to_string());
            if environment == "production" {
                panic!("JWT_SECRET environment variable is required in production. This is a critical security requirement.");
            }
            eprintln!("WARNING: Using default JWT secret in development. Set JWT_SECRET environment variable for security.");
            "your-secret-key-dev-only-change-in-production".to_string()
        });
        
        // Validate JWT secret strength
        if jwt_secret.len() < 32 {
            if env::var("ENVIRONMENT").unwrap_or_else(|| "development".to_string()) == "production" {
                panic!("JWT_SECRET must be at least 32 characters long for production security.");
            }
            eprintln!("WARNING: JWT_SECRET should be at least 32 characters for better security.");
        }
        
        Ok(AppConfig {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgresql://localhost/library_management".to_string()),
            redis_url: env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".to_string()),
            jwt_secret,
            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .unwrap_or(3000),
            environment: env::var("ENVIRONMENT")
                .unwrap_or_else(|_| "development".to_string()),
            smtp_host: env::var("SMTP_HOST")
                .unwrap_or_else(|_| "localhost".to_string()),
            smtp_port: env::var("SMTP_PORT")
                .unwrap_or_else(|_| "587".to_string())
                .parse()
                .unwrap_or(587),
            smtp_username: env::var("SMTP_USERNAME")
                .unwrap_or_else(|_| "admin@library.com".to_string()),
            smtp_password: env::var("SMTP_PASSWORD")
                .unwrap_or_else(|_| "password".to_string()),
        })
    }
}