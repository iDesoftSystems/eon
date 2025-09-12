use crate::http::ApiError;
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use serde::Serialize;

pub fn encode<TClaims>(claims: TClaims) -> Result<String, ApiError>
where
    TClaims: Serialize,
{
    let jwt_secret_encoded = std::env::var("JWT_SECRET").map_err(|err| {
        tracing::error!(?err, "failed to read JWT_SECRET");
        ApiError::Unexpected(Box::new(err))
    })?;

    let encoding_key = EncodingKey::from_base64_secret(&jwt_secret_encoded).map_err(|err| {
        tracing::error!(?err, "failed to decode JWT_SECRET");
        ApiError::Unexpected(Box::new(err))
    })?;

    let access_token = jsonwebtoken::encode(&Header::new(Algorithm::HS512), &claims, &encoding_key)
        .map_err(|err| {
            tracing::error!(?err, "failed to encode user details");
            ApiError::Unexpected(Box::new(err))
        })?;

    Ok(access_token)
}
