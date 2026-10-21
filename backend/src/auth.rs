use crate::errors::AppError;
use crate::models::AuthClaims;
use actix_web::dev::Payload;
use actix_web::{Error, FromRequest, HttpRequest};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use futures_util::future::{ready, Ready};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use std::time::{SystemTime, UNIX_EPOCH};

#[allow(dead_code)]
pub fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|e| AppError::InternalError(e.to_string()))
}

pub fn verify_password(password: &str, password_hash: &str) -> bool {
    let parsed_hash = match PasswordHash::new(password_hash) {
        Ok(h) => h,
        Err(_) => return false,
    };
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

pub fn generate_jwt(username: &str, secret: &str) -> Result<String, AppError> {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| AppError::InternalError(e.to_string()))?
        .as_secs()
        + 24 * 3600;

    let claims = AuthClaims {
        sub: username.to_string(),
        exp: expiration as usize,
        role: "admin".to_string(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::InternalError(e.to_string()))
}

pub fn decode_jwt(token: &str, secret: &str) -> Result<AuthClaims, AppError> {
    let validation = Validation::default();
    let token_data = decode::<AuthClaims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )
    .map_err(|e| AppError::Unauthorized(format!("Invalid token: {}", e)))?;

    Ok(token_data.claims)
}

pub struct AuthenticatedAdmin {
    #[allow(dead_code)]
    pub username: String,
}

impl FromRequest for AuthenticatedAdmin {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let auth_header = req.headers().get("Authorization");
        let cookie = req.cookie("admin_token");

        let token = if let Some(header_val) = auth_header {
            if let Ok(str_val) = header_val.to_str() {
                if str_val.starts_with("Bearer ") {
                    Some(str_val[7..].to_string())
                } else {
                    None
                }
            } else {
                None
            }
        } else if let Some(cookie_val) = cookie {
            Some(cookie_val.value().to_string())
        } else {
            None
        };

        if let Some(t) = token {
            let secret = std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "super_secret_luxury_shoe_brand_jwt_key_2026_aurelia".to_string());

            match decode_jwt(&t, &secret) {
                Ok(claims) => ready(Ok(AuthenticatedAdmin {
                    username: claims.sub,
                })),
                Err(err) => ready(Err(err.into())),
            }
        } else {
            ready(Err(AppError::Unauthorized("Authorization token required".to_string()).into()))
        }
    }
}
