use serde::{Deserialize, Serialize};
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Validation};
use once_cell::sync::Lazy;
use serde_json::json;
use std::fmt::Display;
use axum::{
    async_trait,
    extract::FromRequestParts,
    response::{IntoResponse, Response},
    Json, RequestPartsExt,
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};

use axum::http::{
    request::Parts, 
    StatusCode, 
};
// use axum_extra::{
//     headers::{authorization::Bearer, Authorization},
//     TypedHeader,
// };


#[derive(Deserialize, sqlx::FromRow)]
pub struct AuthPayload {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct Claims {
    pub sub: String,
    pub exp: u64,
}

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

pub static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});


impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "username: {} expiration: {}", self.sub, self.exp)
    }
}


#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;
        // Decode the user data
        let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = Json(json!({
            "detail": error_message,
        }));
        (status, body).into_response()
    }
}

// pub struct Keys {
//     pub encoding: EncodingKey,
//     pub decoding: DecodingKey,
// }

// impl Keys {
//     pub fn new(secret: &[u8]) -> Self {
//         Self {
//             encoding: EncodingKey::from_secret(secret),
//             decoding: DecodingKey::from_secret(secret),
//         }
//     }
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Claims {
//     pub sub: String,
//     pub exp: usize,
// }

// #[derive(Debug, Serialize)]
// pub struct AuthBody {
//     pub access_token: String,
//     pub token_type: String,
//     pub username: String,
// }

// #[derive(Debug, Deserialize)]
// pub struct AuthPayload {
//     pub username: String,
//     pub password: String,
// }

// impl AuthBody {
//     pub fn new(access_token: String, username: String) -> Self {
//         Self {
//             access_token,
//             token_type: "Bearer".to_string(),
//             username,
//         }
//     }
// }

#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}