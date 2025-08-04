use crate::api::response::ApiError;
use crate::auth::contract::UserDetails;

pub trait UserDetailsFinder<TUserDetails>
where
    TUserDetails: UserDetails,
{
    fn find_by_username(
        &self,
        username: &str,
    ) -> impl Future<Output = Result<Option<TUserDetails>, ApiError>>;
}
