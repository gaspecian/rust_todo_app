use crate::AppState; // Import AppState from the crate root
use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    RequestPartsExt,
};
use axum_extra::{extract::TypedHeader, headers::authorization::Bearer, headers::Authorization};
use jsonwebtoken::{decode, Algorithm, Validation};
use serde::{Deserialize, Serialize};

use chrono::Utc;

// Make the Claims struct public
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub iat: i64,
    pub user_id: i64,
}

impl FromRequestParts<AppState> for Claims {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let session_duration = state.session_duration_minutes;
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        // Decode the user data
        let token_data = decode::<Self>(
            bearer.token(),
            &state.decoding_key,
            &Validation::new(Algorithm::HS256),
        )
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

        if token_data.claims.iat + (session_duration * 60) < Utc::now().timestamp() {
            return Err(StatusCode::UNAUTHORIZED);
        }

        Ok(token_data.claims)
    }
}
