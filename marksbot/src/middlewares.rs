use crate::{dto, helpers};
use axum::{body::Body, extract::Request, http, http::Response, middleware::Next};
use http::StatusCode;
use jsonwebtoken::{Algorithm, DecodingKey, Validation};

pub async fn authorization_middleware(
    mut req: Request,
    next: Next,
) -> Result<Response<Body>, StatusCode> {
    let auth_header = req.headers_mut().get(http::header::AUTHORIZATION);
    let auth_header = match auth_header {
        Some(header) => header.to_str().map_err(|_| StatusCode::FORBIDDEN)?,
        None => {
            dbg!("No header");
            return Err(StatusCode::UNAUTHORIZED);
        }
    };
    let mut header = auth_header.split_whitespace();
    let (_, token) = (header.next(), header.next());
    let key = DecodingKey::from_secret("SECRET_KEY".as_bytes());
    let validation = Validation::new(Algorithm::HS256);
    match helpers::decode_jwt::<helpers::TokenClaims<dto::AuthUser>>(
        token.unwrap().to_string(),
        &key,
        &validation,
    ) {
        Ok(data) => {
            req.extensions_mut().insert(data.claims.payload);
        }
        Err(e) => {
            dbg!(e);
            return Err(StatusCode::UNAUTHORIZED);
        }
    };
    Ok(next.run(req).await)
}
