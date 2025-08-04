pub trait UserDetails {
    fn password_hashed(&self) -> String;
    fn is_enabled(&self) -> bool;
}
