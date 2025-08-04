use crate::api::response::ApiError;
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use serde::de::DeserializeOwned;

pub fn decode<TClaims>(access_token: &str) -> Result<TClaims, ApiError>
where
    TClaims: DeserializeOwned,
{
    let jwt_secret_encoded = std::env::var("JWT_SECRET").map_err(|err| {
        tracing::error!(?err, "failed to read JWT_SECRET");
        ApiError::Unexpected(Box::new(err))
    })?;

    let jwt_audience = std::env::var("JWT_AUDIENCE").map_err(|err| {
        tracing::error!(?err, "failed to read JWT_AUDIENCE");
        ApiError::Unexpected(Box::new(err))
    })?;

    let mut validation = Validation::new(Algorithm::HS512);
    validation.set_required_spec_claims(&["exp", "sub", "iat", "iss", "aud"]);
    validation.set_audience(&[jwt_audience]);

    let decoding_key = DecodingKey::from_base64_secret(&jwt_secret_encoded).map_err(|err| {
        tracing::error!(?err, "failed to decode JWT_SECRET");
        ApiError::Unexpected(Box::new(err))
    })?;

    let token_data = jsonwebtoken::decode::<TClaims>(access_token, &decoding_key, &validation)
        .map_err(|err| {
            tracing::error!(?err, "failed to decode JWT");
            ApiError::Unauthorized
        })?;

    Ok(token_data.claims)
}
