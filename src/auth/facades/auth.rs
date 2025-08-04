use crate::api::response::ApiError;
use crate::auth::backend;
use crate::auth::contract::{ClaimsMaker, UserDetails, UserDetailsFinder};
use crate::auth::types::AuthenticationToken;
use serde::Serialize;

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

    let auth_token = backend::encode(claims).await?;

    Ok(AuthenticationToken::new_bearer(auth_token))
}
