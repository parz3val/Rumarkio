use serde::{Serialize, Deserialize};
use axum::Json;
use axum::response::IntoResponse;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation, TokenData};
use bcrypt::{verify, BcryptError};
// generic types for the api response
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ApiResponse<T: Serialize> {
    pub data: T
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

pub type ErrorType = (http::StatusCode, Json<ErrorResponse>);

pub fn api_response<T: Serialize>(data: T) -> impl IntoResponse {
    (http::StatusCode::OK, Json(ApiResponse { data }))
}

pub fn api_error(status: http::StatusCode, message: String) -> ErrorType {
   (status, Json(ErrorResponse { status: "FAIL".to_string(), message }))
}

// JWT HELPERS 

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims<T>
{
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
    pub payload: T,
}

pub fn encode_jwt<T>(
    claims: &T,
    key: &EncodingKey,
) -> Result<String, jsonwebtoken::errors::Error>
    where
        T: Serialize + for<'de> Deserialize<'de>,
{
    let header = Header::default();
    let token = encode(&header, claims, key)?;
    Ok(token)
}


pub fn decode_jwt<'a, T>(
    token: String,
    key: &DecodingKey,
    validation: &Validation,
) -> Result<TokenData<T>, jsonwebtoken::errors::Error>
    where
        T: for<'de> Deserialize<'de> + Serialize,
{
    let token_data = decode::<T>(token.as_str(), key, validation)?;
    Ok(token_data)
}

pub fn create_auth_token<T>(data: T, encoding_key: &str) -> Result<String, jsonwebtoken::errors::Error>
where T: Serialize + for<'de> Deserialize<'de>
{
    let key = EncodingKey::from_secret(encoding_key.as_bytes());
    let sub = "AUTH_TOKEN";
    let iat = chrono::Utc::now().timestamp() as usize;
    let exp = iat + (60 * 60 * 24);
    let claims = TokenClaims {
        sub: sub.to_string(),
        iat,
        exp,
        payload: data,
    };
    encode_jwt(&claims, &key)
}

// PASSWORD HASH HELPERS

pub(crate) fn verify_password(
    password_string: String,
    password_hash: String,
) -> Result<bool, BcryptError> {
    verify(password_string, &password_hash)
}
