use crate::AppState; // Import AppState from the crate root
use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    RequestPartsExt,
};
use axum_extra::{extract::TypedHeader, headers::authorization::Bearer, headers::Authorization};
use jsonwebtoken::{decode, Algorithm, Validation};
use serde::{Deserialize, Serialize};

// Make the Claims struct public
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub iat: i64,
    pub exp: i64,
    pub user_id: i64,
}

impl FromRequestParts<AppState> for Claims {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) =
            match parts.extract::<TypedHeader<Authorization<Bearer>>>().await {
                Ok(extracted) => extracted,
                Err(e) => {
                    tracing::warn!("Failed to extract bearer header: {0}", &e);
                    return Err(StatusCode::UNAUTHORIZED);
                }
            };

        // Decode the user data
        let token_data = match decode::<Self>(
            &bearer.token(),
            &state.decoding_key,
            &Validation::new(Algorithm::HS256),
        ) {
            Ok(data) => data,
            Err(e) => {
                tracing::warn!("Failed to decode token: {0}", &e);
                return Err(StatusCode::UNAUTHORIZED);
            }
        };

        Ok(token_data.claims)
    }
}
