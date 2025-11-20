use crate::auth::backend;
use crate::auth::contract::{ClaimsMaker, UserDetails, UserDetailsFinder};
use crate::auth::types::AuthenticationToken;
use crate::http::ApiError;
use serde::Serialize;

/// Authenticates a user and generates an authentication token.
///
/// This function orchestrates the authentication process:
/// 1. Verifies user credentials using the backend.
/// 2. Creates claims from the user details.
/// 3. Encodes the claims into a JWT access token.
///
/// # Arguments
///
/// * `user_details_finder` - A reference to an object implementing `UserDetailsFinder`.
/// * `claims_maker` - A reference to an object implementing `ClaimsMaker`.
/// * `username` - The username to authenticate.
/// * `password` - The password to verify.
///
/// # Returns
///
/// * `Ok(AuthenticationToken)` - The generated authentication token if successful.
/// * `Err(ApiError)` - An error if authentication fails.
pub async fn authenticate<TClaims, TUserDetails>(
    user_details_finder: &impl UserDetailsFinder<TUserDetails>,
    claims_maker: &impl ClaimsMaker<TUserDetails, TClaims>,
    username: &str,
    password: &str,
) -> Result<AuthenticationToken, ApiError>
where
    TClaims: Serialize,
    TUserDetails: UserDetails,
{
    let user_details = backend::authenticate(user_details_finder, username, password).await?;

    let claims = claims_maker.make(user_details);

    let auth_token = backend::encode(claims)?;

    Ok(AuthenticationToken::new_bearer(auth_token))
}
