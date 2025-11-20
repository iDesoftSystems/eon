use serde::Serialize;

/// Represents an authentication token response.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationToken {
    /// The JWT access token.
    pub access_token: String,
    /// The token type (e.g., "Bearer").
    pub token_type: String,
}

impl AuthenticationToken {
    /// Creates a new `AuthenticationToken` with type "Bearer".
    ///
    /// # Arguments
    ///
    /// * `access_token` - The JWT access token string.
    ///
    /// # Returns
    ///
    /// * `Self` - A new `AuthenticationToken` instance.
    pub fn new_bearer(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}
