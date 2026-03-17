use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use crate::models::{UserType};

#[derive(Debug, Clone)]
pub struct AuthService {
    jwt_secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user_id
    pub email: String,
    pub user_type: UserType,
    pub exp: usize,
    pub iat: usize,
}

impl AuthService {
    pub fn new(jwt_secret: String) -> Self {
        Self { jwt_secret }
    }

    pub fn hash_password(&self, password: &str) -> Result<String, argon2::password_hash::Error> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
        Ok(password_hash.to_string())
    }

    pub fn verify_password(&self, password: &str, hash: &str) -> Result<bool, argon2::password_hash::Error> {
        let parsed_hash = PasswordHash::new(hash)?;
        let argon2 = Argon2::default();
        match argon2.verify_password(password.as_bytes(), &parsed_hash) {
            Ok(_) => Ok(true),
            Err(argon2::password_hash::Error::Password) => Ok(false),
            Err(err) => Err(err),
        }
    }

    pub fn generate_token(&self, user_id: Uuid, email: &str, user_type: UserType) -> Result<String, jsonwebtoken::errors::Error> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as usize;

        let claims = Claims {
            sub: user_id.to_string(),
            email: email.to_string(),
            user_type,
            exp: now + 3600, // 1 hour expiry
            iat: now,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_ref()),
            &Validation::default(),
        )
        .map(|data| data.claims)
    }

    pub fn extract_token_from_header(&self, auth_header: Option<&str>) -> Result<&str, &'static str> {
        let header = auth_header.ok_or("Missing authorization header")?;
        
        if !header.starts_with("Bearer ") {
            return Err("Invalid authorization header format");
        }

        Ok(&header[7..]) // Remove "Bearer " prefix
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::UserType;

    #[test]
    fn test_password_hashing() {
        let auth = AuthService::new("test_secret".to_string());
        let password = "test_password123";
        
        let hash = auth.hash_password(password).unwrap();
        assert!(auth.verify_password(password, &hash).unwrap());
        assert!(!auth.verify_password("wrong_password", &hash).unwrap());
    }

    #[test]
    fn test_jwt_token() {
        let auth = AuthService::new("test_secret".to_string());
        let user_id = Uuid::new_v4();
        let email = "test@example.com";
        let user_type = UserType::Student;
        
        let token = auth.generate_token(user_id, email, user_type.clone()).unwrap();
        let claims = auth.verify_token(&token).unwrap();
        
        assert_eq!(claims.sub, user_id.to_string());
        assert_eq!(claims.email, email);
        assert!(matches!(claims.user_type, UserType::Student));
    }
}