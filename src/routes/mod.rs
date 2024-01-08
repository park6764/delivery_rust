use diesel::RunQueryDsl;
use std::error::Error;

use axum::{http::{StatusCode, HeaderMap}, extract::{State, Request}, middleware::Next, response::Response};
use deadpool_diesel::postgres::Pool;
use diesel::{QueryDsl, ExpressionMethods, BoolExpressionMethods};
use jsonwebtoken as jwt;

use crate::models::{Claims, Owner, Customer};

pub mod customers;
pub mod orders;
pub mod owners;
pub mod restaurants;
pub mod login;

pub fn internal_error<E: Error>(error: E) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
}

pub async fn require_authentication(
    State(pool): State<Pool>,
    headers: HeaderMap,
    mut request: Request,
    next: Next
) -> Result<Response, (StatusCode, String)> {
    let header_auth = headers.get("authorization")
        .ok_or((StatusCode::UNAUTHORIZED, String::from("not authenticated from header")))
        .map(|t| t.to_str().map_err(internal_error))??;

    let header_token = header_auth.split(" ").nth(1).expect("Auth token must be \"Bearer ...\"");

    let conn = pool.get().await.map_err(internal_error)?;

    let token_secret = std::env::var("JWT_SECRET").map_err(internal_error)?;

    let claims = verify_token(&token_secret, header_token)?;

    use diesel::OptionalExtension;

    let header_token2 = String::from(header_token);

   let user = if claims.is_owner {
    use crate::schema::owners::dsl::*;

    let o: Option<Owner> = conn.interact(move |conn| {
        owners.filter(id.eq(claims.id).and(token.eq(Some(header_token2))))
        .first(conn) 
        .optional()
    }).await
    .map_err(internal_error)?
    .map_err(internal_error)?;

    let o2 = o.ok_or((
        StatusCode::UNAUTHORIZED,
        String::from("You are not authorized owner for this")
    ))?;

    Ok(o2)
   } else {
    use crate::schema::customers::dsl::*;

    let c: Option<Customer> = conn.interact(move |conn| {
        customers.filter(id.eq(claims.id).and(token.eq(Some(header_token2))))
        .first(conn)
        .optional()
    }).await
    .map_err(internal_error)?
    .map_err(internal_error)?;

    let c2 = c.ok_or((
        StatusCode::UNAUTHORIZED,
        String::from("You are not authorized owner for this")
    ))?;
    
    Err(c2)
   };

   request.extensions_mut().insert(user);

   Ok(next.run(request).await)
}

pub fn create_token(secret: &str, id: i32, is_owner: bool) -> Result<String, (StatusCode, String)> {
    let claims = Claims { id, exp: (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp() as usize, is_owner };
    let key = jwt::EncodingKey::from_secret(secret.as_bytes());

    jwt::encode(&jwt::Header::default(), &claims, &key)
        .map_err(internal_error)

}

pub fn verify_token(secret: &str, token: &str) -> Result<Claims, (StatusCode, String)> {
    let key = jwt::DecodingKey::from_secret(secret.as_bytes());
    let validation = jwt::Validation::new(jwt::Algorithm::HS256);
    match jwt::decode::<Claims>(token, &key, &validation) {
            Ok(claim) => Ok(claim.claims),
            Err(error) => match error.kind() {
                jsonwebtoken::errors::ErrorKind::InvalidToken
                | jsonwebtoken::errors::ErrorKind::InvalidSignature
                | jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                    Err((StatusCode::UNAUTHORIZED, String::from("not authenticated!")))
                },
                _ => {
                    Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Error validating token: {:?}", error)))
                }
            }
        }
}