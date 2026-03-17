use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::{Validate, ValidationError};

// Password validation function for complex password requirements
fn validate_password_complexity(password: &str) -> Result<(), ValidationError> {
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_digit = password.chars().any(|c| c.is_numeric());
    let has_special = password.chars().any(|c| !c.is_alphanumeric());
    
    if !has_lowercase {
        return Err(ValidationError::new("Password must contain at least one lowercase letter"));
    }
    if !has_uppercase {
        return Err(ValidationError::new("Password must contain at least one uppercase letter"));
    }
    if !has_digit {
        return Err(ValidationError::new("Password must contain at least one number"));
    }
    if !has_special {
        return Err(ValidationError::new("Password must contain at least one special character"));
    }
    
    Ok(())
}

// User-related models
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
    pub user_type: UserType,
    pub student_id: Option<String>,
    pub department: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "user_type", rename_all = "lowercase")]
pub enum UserType {
    Student,
    Faculty,
    Staff,
    Admin,
}

#[derive(Debug, Validate, Deserialize)]
pub struct CreateUserRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 12), custom = "validate_password_complexity")]
    pub password: String,
    #[validate(length(min = 1))]
    pub first_name: String,
    #[validate(length(min = 1))]
    pub last_name: String,
    pub user_type: UserType,
    pub student_id: Option<String>,
    pub department: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Validate, Deserialize)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub user: UserProfile,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub user_type: UserType,
    pub student_id: Option<String>,
    pub department: Option<String>,
}

// Book-related models
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Book {
    pub id: Uuid,
    pub isbn_13: Option<String>,
    pub isbn_10: Option<String>,
    pub title: String,
    pub subtitle: Option<String>,
    pub authors: Vec<String>,
    pub publisher: Option<String>,
    pub publication_year: Option<i32>,
    pub edition: Option<String>,
    pub language: String,
    pub pages: Option<i32>,
    pub format: String,
    pub genre: Option<String>,
    pub subjects: Option<Vec<String>>,
    pub description: Option<String>,
    pub cover_image_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Validate, Deserialize)]
pub struct CreateBookRequest {
    pub isbn_13: Option<String>,
    pub isbn_10: Option<String>,
    #[validate(length(min = 1))]
    pub title: String,
    pub subtitle: Option<String>,
    #[validate(length(min = 1))]
    pub authors: Vec<String>,
    pub publisher: Option<String>,
    pub publication_year: Option<i32>,
    pub edition: Option<String>,
    pub language: Option<String>,
    pub pages: Option<i32>,
    pub format: Option<String>,
    pub genre: Option<String>,
    pub subjects: Option<Vec<String>>,
    pub description: Option<String>,
    pub cover_image_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct BookCopy {
    pub id: Uuid,
    pub book_id: Uuid,
    pub barcode: String,
    pub location: String,
    pub condition: BookCondition,
    pub acquisition_date: Option<NaiveDate>,
    pub price: Option<rust_decimal::Decimal>,
    pub status: CopyStatus,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "book_condition", rename_all = "lowercase")]
pub enum BookCondition {
    Excellent,
    Good,
    Fair,
    Poor,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "copy_status", rename_all = "snake_case")]
pub enum CopyStatus {
    Available,
    CheckedOut,
    Reserved,
    Maintenance,
    Lost,
}

// Transaction-related models
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Transaction {
    pub id: Uuid,
    pub copy_id: Uuid,
    pub user_id: Uuid,
    pub transaction_type: TransactionType,
    pub checkout_date: NaiveDate,
    pub due_date: NaiveDate,
    pub return_date: Option<NaiveDate>,
    pub renewed_count: i32,
    pub fine_amount: rust_decimal::Decimal,
    pub fine_paid: bool,
    pub staff_id: Option<Uuid>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "transaction_type", rename_all = "lowercase")]
pub enum TransactionType {
    Checkout,
    Return,
    Renew,
    Reserve,
}

#[derive(Debug, Validate, Deserialize)]
pub struct CheckoutRequest {
    pub copy_id: Uuid,
    pub user_id: Uuid,
    pub due_date: Option<NaiveDate>,
}

#[derive(Debug, Validate, Deserialize)]
pub struct ReturnRequest {
    pub copy_id: Uuid,
}

// Reservation-related models
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Reservation {
    pub id: Uuid,
    pub book_id: Uuid,
    pub user_id: Uuid,
    pub reservation_date: DateTime<Utc>,
    pub expiry_date: DateTime<Utc>,
    pub status: ReservationStatus,
    pub position_in_queue: Option<i32>,
    pub notification_sent: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "reservation_status", rename_all = "lowercase")]
pub enum ReservationStatus {
    Active,
    Fulfilled,
    Cancelled,
    Expired,
}

#[derive(Debug, Validate, Deserialize)]
pub struct CreateReservationRequest {
    pub book_id: Uuid,
}

// Search and pagination
#[derive(Debug, Deserialize)]
pub struct SearchBooksRequest {
    pub q: Option<String>,
    pub author: Option<String>,
    pub genre: Option<String>,
    pub year: Option<i32>,
    pub language: Option<String>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub pagination: PaginationInfo,
}

#[derive(Debug, Serialize)]
pub struct PaginationInfo {
    pub page: u32,
    pub limit: u32,
    pub total: i64,
    pub total_pages: u32,
    pub has_next: bool,
    pub has_prev: bool,
}