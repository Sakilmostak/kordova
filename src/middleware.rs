use axum::{
    extract::{Request, State, ConnectInfo},
    http::{header::AUTHORIZATION, HeaderMap},
    middleware::Next,
    response::Response,
};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use uuid::Uuid;

use crate::{auth::Claims, errors::AppError, AppState};

// Simple rate limiting structure
#[derive(Clone)]
struct RateLimitEntry {
    count: u32,
    window_start: Instant,
}

lazy_static::lazy_static! {
    static ref AUTH_RATE_LIMITER: Arc<Mutex<HashMap<String, RateLimitEntry>>> = 
        Arc::new(Mutex::new(HashMap::new()));
    static ref GENERAL_RATE_LIMITER: Arc<Mutex<HashMap<String, RateLimitEntry>>> = 
        Arc::new(Mutex::new(HashMap::new()));
}

pub async fn auth_middleware(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let auth_header = headers.get(AUTHORIZATION)
        .and_then(|value| value.to_str().ok());

    let token = app_state.auth.extract_token_from_header(auth_header)
        .map_err(|err| AppError::Authentication(err.to_string()))?;

    let claims = app_state.auth.verify_token(token)
        .map_err(|_| AppError::Authentication("Invalid token".to_string()))?;

    // Add claims to request extensions for use in handlers
    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}

pub async fn admin_middleware(
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let claims = request.extensions().get::<Claims>()
        .ok_or_else(|| AppError::Authorization("Missing authentication".to_string()))?;

    match claims.user_type {
        crate::models::UserType::Admin => Ok(next.run(request).await),
        _ => Err(AppError::Authorization("Admin access required".to_string())),
    }
}

pub async fn staff_middleware(
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let claims = request.extensions().get::<Claims>()
        .ok_or_else(|| AppError::Authorization("Missing authentication".to_string()))?;

    match claims.user_type {
        crate::models::UserType::Admin | crate::models::UserType::Staff => {
            Ok(next.run(request).await)
        }
        _ => Err(AppError::Authorization("Staff access required".to_string())),
    }
}

// Helper function to extract user ID from request
pub fn get_current_user_id(request: &Request) -> Result<Uuid, AppError> {
    let claims = request.extensions().get::<Claims>()
        .ok_or_else(|| AppError::Authorization("Missing authentication".to_string()))?;
    
    Uuid::parse_str(&claims.sub)
        .map_err(|_| AppError::Internal("Invalid user ID in token".to_string()))
}

// Helper function to extract current user claims
pub fn get_current_user_claims(request: &Request) -> Result<&Claims, AppError> {
    request.extensions().get::<Claims>()
        .ok_or_else(|| AppError::Authorization("Missing authentication".to_string()))
}

/// Check rate limit for a given IP and limits
fn check_rate_limit(
    limiter: &Arc<Mutex<HashMap<String, RateLimitEntry>>>,
    ip: &str,
    max_requests: u32,
    window_duration: Duration,
) -> Result<(), AppError> {
    let mut limiter = limiter.lock().unwrap();
    let now = Instant::now();
    
    // Clean up expired entries
    limiter.retain(|_, entry| now.duration_since(entry.window_start) < window_duration);
    
    let entry = limiter.entry(ip.to_string()).or_insert(RateLimitEntry {
        count: 0,
        window_start: now,
    });
    
    // Reset count if window expired
    if now.duration_since(entry.window_start) >= window_duration {
        entry.count = 0;
        entry.window_start = now;
    }
    
    if entry.count >= max_requests {
        return Err(AppError::RateLimit("Rate limit exceeded".to_string()));
    }
    
    entry.count += 1;
    Ok(())
}

/// Rate limiting middleware for authentication endpoints (login, register)
pub async fn auth_rate_limit_middleware(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let client_ip = addr.ip().to_string();
    
    // Allow 5 login attempts per minute per IP
    check_rate_limit(&AUTH_RATE_LIMITER, &client_ip, 5, Duration::from_secs(60))?;
    
    Ok(next.run(request).await)
}

/// General rate limiting middleware for API endpoints
pub async fn general_rate_limit_middleware(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let client_ip = addr.ip().to_string();
    
    // Allow 100 requests per minute per IP for general endpoints
    check_rate_limit(&GENERAL_RATE_LIMITER, &client_ip, 100, Duration::from_secs(60))?;
    
    Ok(next.run(request).await)
}