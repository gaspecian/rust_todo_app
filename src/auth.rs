use crate::AppState; // Import AppState from the crate root
use axum::{
    extract::{FromRequestParts},
    http::{request::Parts, StatusCode},
    RequestPartsExt,
};
use axum_extra::{
  extract::TypedHeader,
  headers::authorization::Bearer,
  headers::Authorization
};
use jsonwebtoken::{decode, Validation, Algorithm};
use serde::{Deserialize, Serialize};

// Make the Claims struct public
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub user_id: i64,
}

impl FromRequestParts<AppState> for Claims {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {

      // Extract the token from the authorization header
      let TypedHeader(Authorization(bearer)) = parts
        .extract::<TypedHeader<Authorization<Bearer>>>()
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

      // Decode the user data
      let token_data = decode::<Claims>(
        bearer.token(),
        &state.decoding_key,
        &Validation::new(Algorithm::HS256),
      )
      .map_err(|_| StatusCode::UNAUTHORIZED)?;

      Ok(token_data.claims)
    }
}