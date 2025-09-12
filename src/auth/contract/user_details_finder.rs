use crate::auth::contract::UserDetails;
use crate::http::ApiError;

pub trait UserDetailsFinder<TUserDetails>
where
    TUserDetails: UserDetails,
{
    fn find_by_username(
        &self,
        username: &str,
    ) -> impl Future<Output = Result<Option<TUserDetails>, ApiError>>;
}
