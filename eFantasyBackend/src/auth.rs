use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};
use argon2::{self, password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString}, Argon2};

/// JWT claims structure
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

/// Hashes a password using Argon2
pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2.hash_password(password.as_bytes(), &salt).unwrap().to_string()
}

/// Verifies a password against its hash
pub fn verify_password(password: &str, hash: &str) -> bool {
    let parsed_hash = PasswordHash::new(hash).unwrap();
    Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok()
}

/// Generates a JWT token for a user
pub fn generate_token(user_id: i64) -> Result<String, String> {
    let secret = match std::env::var("JWT_SECRET") {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to get JWT_SECRET: {:?}", e);
            return Err("JWT_SECRET not set or inaccessible".to_string());
        }
    };

    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() + 3600; // 1 hour expiration

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
        .map_err(|e| format!("Token generation failed: {:?}", e))
}

/// Validates a JWT token
pub fn validate_token(token: &str) -> Result<i64, jsonwebtoken::errors::Error> {
    println!("auth::validate_token: Validating token");
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;

    println!("auth::validate_token: Token validated successfully");
    Ok(token_data.claims.sub.parse().unwrap())
}