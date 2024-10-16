use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};
use argon2::{self};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2
};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2.hash_password(password.as_bytes(), &salt).unwrap().to_string()
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    let parsed_hash = PasswordHash::new(hash).unwrap();
    Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok()
}

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

pub fn validate_token(token: &str) -> Result<i64, jsonwebtoken::errors::Error> {
    println!("Validating token: {}", token);
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    if token.len() < 10 {  // Arbitrary minimum length
        println!("Token too short, likely invalid");
        return Err(jsonwebtoken::errors::Error::from(jsonwebtoken::errors::ErrorKind::InvalidToken));
    }
    let token_data = match decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    ) {
        Ok(data) => data,
        Err(e) => {
            println!("Token validation error: {:?}", e);
            return Err(e);
        }
    };

    println!("Token validated successfully");
    Ok(token_data.claims.sub.parse().unwrap())
}