use crate::api::response::ApiError;
use crate::auth::contract::UserDetails;
use crate::auth::contract::UserDetailsFinder;

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
