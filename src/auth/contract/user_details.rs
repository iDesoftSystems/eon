/// A trait representing user details required for authentication.
pub trait UserDetails {
    /// Returns the hashed password of the user.
    fn password_hashed(&self) -> String;

    /// Returns whether the user account is enabled.
    fn is_enabled(&self) -> bool;
}
