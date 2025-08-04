use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationToken {
    pub access_token: String,
    pub token_type: String,
}

impl AuthenticationToken {
    pub fn new_bearer(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}
