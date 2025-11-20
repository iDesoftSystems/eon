use serde::Serialize;

/// Represents a validation error for a specific field.
#[derive(Serialize)]
pub struct Field {
    pub field: String,
    pub reason: String,
    pub code: String,
}

impl Field {
    pub fn new(field: &str, reason: &str, code: &str) -> Self {
        Self {
            field: field.to_string(),
            reason: reason.to_string(),
            code: code.to_string(),
        }
    }
}
