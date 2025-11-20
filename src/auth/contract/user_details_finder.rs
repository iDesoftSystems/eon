use crate::auth::contract::UserDetails;
use crate::http::ApiError;

/// A trait for retrieving user details from a data source (e.g., database).
pub trait UserDetailsFinder<TUserDetails>
where
    TUserDetails: UserDetails,
{
    /// Finds user details by username.
    ///
    /// # Arguments
    ///
    /// * `username` - The username to search for.
    ///
    /// # Returns
    ///
    /// * `Result<Option<TUserDetails>, ApiError>` - The user details if found, or `None` if not found.
    fn find_by_username(
        &self,
        username: &str,
    ) -> impl Future<Output = Result<Option<TUserDetails>, ApiError>>;
}
