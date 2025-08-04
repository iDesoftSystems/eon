use crate::auth::contract::UserDetails;
use serde::Serialize;

pub trait ClaimsMaker<TUserDetails, TClaims>
where
    TClaims: Serialize,
    TUserDetails: UserDetails,
{
    fn make(&self, user_details: TUserDetails) -> TClaims;
}
