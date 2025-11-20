use crate::auth::contract::UserDetails;
use serde::Serialize;

/// A trait for creating JWT claims from user details.
///
/// This trait allows customizing how user information is mapped to JWT claims.
pub trait ClaimsMaker<TUserDetails, TClaims>
where
    TClaims: Serialize,
    TUserDetails: UserDetails,
{
    /// Creates claims from the given user details.
    ///
    /// # Arguments
    ///
    /// * `user_details` - The user details to convert into claims.
    ///
    /// # Returns
    ///
    /// * `TClaims` - The generated claims.
    fn make(&self, user_details: TUserDetails) -> TClaims;
}
