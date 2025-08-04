use crate::api::response::ApiError;
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use serde::Serialize;

pub async fn encode<TClaims>(claims: TClaims) -> Result<String, ApiError>
where
    TClaims: Serialize,
{
    let jwt_secret_encoded = std::env::var("JWT_SECRET").map_err(|err| {
        tracing::error!(?err, "failed to read JWT_SECRET");
        ApiError::Unexpected(Box::new(err))
    })?;

    let jwt_secret = BASE64_STANDARD
        .decode(jwt_secret_encoded.as_bytes())
        .map_err(|err| {
            tracing::error!(?err, "failed to decode JWT_SECRET");
            ApiError::Unexpected(Box::new(err))
        })?;

    let encoding_key = EncodingKey::from_secret(&jwt_secret);

    let access_token = jsonwebtoken::encode(&Header::new(Algorithm::HS512), &claims, &encoding_key)
        .map_err(|err| {
            tracing::error!(?err, "failed to encode user details");
            ApiError::Unexpected(Box::new(err))
        })?;

    Ok(access_token)
}
