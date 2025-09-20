use crate::{modules::common::ErrorResponse, AppState}; // Import AppState from the crate root
use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    RequestPartsExt,
};
use axum_extra::{extract::TypedHeader, headers::authorization::Bearer, headers::Authorization};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, Algorithm, Validation};
use jsonwebtoken::{encode, EncodingKey, Header};
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
            bearer.token(),
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

pub fn generate_token(
    session_duration: i64,
    user_id: i64,
    enconding_key: &EncodingKey,
) -> Result<String, ErrorResponse> {
    let now = Utc::now();
    let exp = now + Duration::minutes(session_duration);
    let claims = Claims {
        user_id,
        iat: now.timestamp(),
        exp: exp.timestamp(),
    };

    let token = match encode(&Header::default(), &claims, enconding_key) {
        Ok(token) => token,
        Err(e) => {
            tracing::warn!("Error generating JWT token: {0}", e);
            return Err(ErrorResponse {
                message: "Failed to generate JWT token.".to_string(),
            });
        }
    };

    Ok(token)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::unreadable_literal)]
mod tests {
    use super::*;
    use jsonwebtoken::{DecodingKey, Validation};

    #[test]
    fn test_generate_token_success() {
        let secret = "test_secret";
        let encoding_key = EncodingKey::from_secret(secret.as_ref());
        let user_id = 123;
        let session_duration = 60;

        let result = generate_token(session_duration, user_id, &encoding_key);
        assert!(result.is_ok());

        let token = result.unwrap();
        assert!(!token.is_empty());

        // Verify the token can be decoded
        let decoding_key = DecodingKey::from_secret(secret.as_ref());
        let validation = Validation::new(Algorithm::HS256);
        let decoded = decode::<Claims>(&token, &decoding_key, &validation);
        assert!(decoded.is_ok());

        let claims = decoded.unwrap().claims;
        assert_eq!(claims.user_id, user_id);
    }

    #[test]
    fn test_claims_structure() {
        let claims = Claims {
            iat: 1234567890,
            exp: 1234567950,
            user_id: 42,
        };

        assert_eq!(claims.iat, 1234567890);
        assert_eq!(claims.exp, 1234567950);
        assert_eq!(claims.user_id, 42);
    }

    #[test]
    fn test_generate_token_different_users() {
        let secret = "test_secret";
        let encoding_key = EncodingKey::from_secret(secret.as_ref());
        let session_duration = 60;

        let token1 = generate_token(session_duration, 1, &encoding_key).unwrap();
        let token2 = generate_token(session_duration, 2, &encoding_key).unwrap();

        assert_ne!(token1, token2);

        // Verify both tokens contain correct user IDs
        let decoding_key = DecodingKey::from_secret(secret.as_ref());
        let validation = Validation::new(Algorithm::HS256);

        let claims1 = decode::<Claims>(&token1, &decoding_key, &validation)
            .unwrap()
            .claims;
        let claims2 = decode::<Claims>(&token2, &decoding_key, &validation)
            .unwrap()
            .claims;

        assert_eq!(claims1.user_id, 1);
        assert_eq!(claims2.user_id, 2);
    }

    #[test]
    fn test_invalid_token_format() {
        let invalid_token = "invalid.token.format";
        let decoding_key = DecodingKey::from_secret("secret".as_ref());
        let validation = Validation::default();

        let result = decode::<Claims>(invalid_token, &decoding_key, &validation);
        assert!(result.is_err());
    }

    #[test]
    fn test_expired_token() {
        let claims = Claims {
            user_id: 1,
            iat: chrono::Utc::now().timestamp(),
            exp: (chrono::Utc::now() - chrono::Duration::hours(1)).timestamp(),
        };

        let encoding_key = EncodingKey::from_secret("secret".as_ref());
        let token = encode(&Header::default(), &claims, &encoding_key).unwrap();

        let decoding_key = DecodingKey::from_secret("secret".as_ref());
        let validation = Validation::default();
        let result = decode::<Claims>(&token, &decoding_key, &validation);
        assert!(result.is_err());
    }

    #[test]
    fn test_wrong_secret() {
        let encoding_key = EncodingKey::from_secret("secret".as_ref());
        let token = generate_token(60, 1, &encoding_key).unwrap();
        let wrong_key = DecodingKey::from_secret("wrong_secret".as_ref());
        let validation = Validation::default();

        let result = decode::<Claims>(&token, &wrong_key, &validation);
        assert!(result.is_err());
    }
}
