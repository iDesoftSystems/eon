use crate::auth::backend;
use crate::http::ApiError;
use axum::extract::Request;
use axum::http;
use axum::middleware::Next;
use axum::response::Response;
use serde::de::DeserializeOwned;

/// JWT authentication middleware.
///
/// This middleware extracts the JWT token from the `Authorization` header,
/// validates it, and decodes the claims. If valid, the claims are added to the
/// request extensions.
///
/// # Arguments
///
/// * `req` - The incoming HTTP request.
/// * `next` - The next middleware or handler in the chain.
///
/// # Returns
///
/// * `Ok(Response)` - The response from the next handler if authentication is successful.
/// * `Err(ApiError)` - An error if authentication fails (e.g., missing header, invalid token).
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
