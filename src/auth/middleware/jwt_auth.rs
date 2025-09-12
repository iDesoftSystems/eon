use crate::auth::backend;
use crate::http::ApiError;
use axum::extract::Request;
use axum::http;
use axum::middleware::Next;
use axum::response::Response;
use serde::de::DeserializeOwned;

/// JWT authentication middleware
pub async fn jwt_auth<TClaims>(mut req: Request, next: Next) -> Result<Response, ApiError>
where
    TClaims: DeserializeOwned + Clone + Send + Sync + 'static,
{
    let header_value = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .ok_or(ApiError::Unauthorized)?;

    let auth_header = header_value.to_str().map_err(|err| {
        tracing::error!(?err, "Error parsing authorization header");
        ApiError::Unauthorized
    })?;

    let mut header = auth_header.split_whitespace();

    let (_, maybe_token) = (header.next(), header.next());

    let Some(token) = maybe_token else {
        tracing::error!("Authorization header is missing");
        return Err(ApiError::Unauthorized);
    };

    let claims = backend::decode::<TClaims>(token)?;

    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}
