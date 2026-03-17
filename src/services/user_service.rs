use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    errors::{AppError, Result},
    models::{User, UserType, CreateUserRequest, UserProfile},
};

pub struct UserService {
    db: PgPool,
}

impl UserService {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    pub async fn get_user(&self, id: Uuid) -> Result<User> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.db)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        Ok(user)
    }

    pub async fn list_users(&self, page: u32, limit: u32) -> Result<Vec<UserProfile>> {
        let offset = (page - 1) * limit;
        
        let users = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE is_active = true ORDER BY created_at DESC LIMIT $1 OFFSET $2"
        )
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&self.db)
        .await?;

        let profiles = users.into_iter().map(|user| UserProfile {
            id: user.id,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            user_type: user.user_type,
            student_id: user.student_id,
            department: user.department,
        }).collect();

        Ok(profiles)
    }

    pub async fn update_user(&self, id: Uuid, request: CreateUserRequest) -> Result<UserProfile> {
        let user = sqlx::query_as::<_, User>(
            r#"
            UPDATE users SET
                email = $2, first_name = $3, last_name = $4, user_type = $5,
                student_id = $6, department = $7, phone = $8, address = $9,
                updated_at = NOW()
            WHERE id = $1 AND is_active = true
            RETURNING *
            "#
        )
        .bind(id)
        .bind(&request.email)
        .bind(&request.first_name)
        .bind(&request.last_name)
        .bind(&request.user_type)
        .bind(&request.student_id)
        .bind(&request.department)
        .bind(&request.phone)
        .bind(&request.address)
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

    pub async fn deactivate_user(&self, id: Uuid) -> Result<()> {
        let result = sqlx::query(
            "UPDATE users SET is_active = false, updated_at = NOW() WHERE id = $1"
        )
        .bind(id)
        .execute(&self.db)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("User not found".to_string()));
        }

        Ok(())
    }
}