use crate::http::types::Field;
use serde::Serialize;

#[derive(Serialize)]
pub struct ProblemDetails {
    pub detail: String,
    pub errors: Vec<Field>,
}

impl From<String> for ProblemDetails {
    fn from(value: String) -> Self {
        Self {
            detail: value,
            errors: vec![],
        }
    }
}

impl From<Vec<Field>> for ProblemDetails {
    fn from(value: Vec<Field>) -> Self {
        let first_reason = value
            .first()
            .map(|field| field.reason.to_string())
            .unwrap_or("Your request parameters didn't pass validation".into());

        Self {
            detail: first_reason,
            errors: value,
        }
    }
}
