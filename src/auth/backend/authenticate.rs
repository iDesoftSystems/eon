use crate::auth::contract::UserDetails;
use crate::auth::contract::UserDetailsFinder;
use crate::http::ApiError;

/// Authenticates a user by verifying their username and password.
///
/// # Arguments
///
/// * `finder` - A reference to an object implementing `UserDetailsFinder`.
/// * `username` - The username to authenticate.
/// * `password` - The password to verify.
///
/// # Returns
///
/// * `Ok(TUserDetails)` - The user details if authentication is successful.
/// * `Err(ApiError)` - An error if authentication fails (e.g., invalid credentials, user disabled).
pub async fn authenticate<TUserDetails>(
    finder: &impl UserDetailsFinder<TUserDetails>,
    username: &str,
    password: &str,
) -> Result<TUserDetails, ApiError>
where
    TUserDetails: UserDetails,
{
    let user_details = finder
        .find_by_username(username)
        .await?
        .ok_or(ApiError::Unauthorized)?;

    let pwd_valid = bcrypt::verify(password, &user_details.password_hashed()).map_err(|err| {
        tracing::error!(?err, "failed to verify password");
        ApiError::Unexpected(Box::new(err))
    })?;

    if !pwd_valid {
        return Err(ApiError::Unauthorized);
    }

    if !user_details.is_enabled() {
        return Err(ApiError::Forbidden);
    }

    Ok(user_details)
}
