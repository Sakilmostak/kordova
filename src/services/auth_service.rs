use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    auth::AuthService,
    errors::{AppError, Result},
    models::{CreateUserRequest, LoginRequest, LoginResponse, User, UserProfile, UserType},
};

pub struct AuthServiceImpl {
    db: PgPool,
    auth: AuthService,
}

impl AuthServiceImpl {
    pub fn new(db: PgPool, auth: AuthService) -> Self {
        Self { db, auth }
    }

    pub async fn register_user(&self, request: CreateUserRequest) -> Result<UserProfile> {
        // Check if user already exists
        let existing_user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE email = $1"
        )
        .bind(&request.email)
        .fetch_optional(&self.db)
        .await?;

        if existing_user.is_some() {
            return Err(AppError::Conflict("User already exists".to_string()));
        }

        // Hash password
        let password_hash = self.auth.hash_password(&request.password)?;

        // Create user
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (email, password_hash, first_name, last_name, user_type, student_id, department, phone, address)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING *
            "#
        )
        .bind(&request.email)
        .bind(&password_hash)
        .bind(&request.first_name)
        .bind(&request.last_name)
        .bind(&request.user_type)
        .bind(&request.student_id)
        .bind(&request.department)
        .bind(&request.phone)
        .bind(&request.address)
        .fetch_one(&self.db)
        .await?;

        Ok(UserProfile {
            id: user.id,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            user_type: user.user_type,
            student_id: user.student_id,
            department: user.department,
        })
    }

    pub async fn login(&self, request: LoginRequest) -> Result<LoginResponse> {
        // Find user by email
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE email = $1 AND is_active = true"
        )
        .bind(&request.email)
        .fetch_optional(&self.db)
        .await?
        .ok_or_else(|| AppError::Authentication("Invalid credentials".to_string()))?;

        // Verify password
        if !self.auth.verify_password(&request.password, &user.password_hash)? {
            return Err(AppError::Authentication("Invalid credentials".to_string()));
        }

        // Generate token
        let token = self.auth.generate_token(user.id, &user.email, user.user_type.clone())?;

        Ok(LoginResponse {
            access_token: token,
            token_type: "Bearer".to_string(),
            expires_in: 3600,
            user: UserProfile {
                id: user.id,
                email: user.email,
                first_name: user.first_name,
                last_name: user.last_name,
                user_type: user.user_type,
                student_id: user.student_id,
                department: user.department,
            },
        })
    }

    pub async fn get_user_profile(&self, user_id: Uuid) -> Result<UserProfile> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE id = $1 AND is_active = true"
        )
        .bind(user_id)
        .fetch_optional(&self.db)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        Ok(UserProfile {
            id: user.id,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            user_type: user.user_type,
            student_id: user.student_id,
            department: user.department,
        })
    }
}