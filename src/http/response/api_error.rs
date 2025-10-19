use crate::http::response::BadRequest;
use crate::http::response::ConflictWithMessage;
use crate::http::response::Forbidden;
use crate::http::response::InternalServer;
use crate::http::response::Unauthorized;
use crate::http::response::UnprocessableEntity;
use crate::http::ResourceNotFound;
use crate::http::{EntityCodeNotFound, EntityIdNotFound, ResourceId};
use axum::response::{IntoResponse, Response};
use std::fmt::Debug;
use validator::ValidationErrors;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error(transparent)]
    Unexpected(#[from] Box<dyn std::error::Error + Send + Sync>),

    #[error("Forbidden access for user")]
    Forbidden,

    #[error("Unauthorized access")]
    Unauthorized,

    #[cfg(feature = "orm")]
    #[error(transparent)]
    DbError(#[from] sea_orm::DbErr),

    #[error(transparent)]
    Validation(#[from] ValidationErrors),

    #[error("Entity not found with id: {0}")]
    #[deprecated(since = "0.5.0", note = "Use ResourceNotFound instead")]
    EntityIdNotFound(i64),

    #[error("Entity not found with code: {0}")]
    #[deprecated(since = "0.5.0", note = "Use ResourceNotFound instead")]
    EntityCodeNotFound(String),

    #[error("Resource not found")]
    ResourceNotFound(ResourceId),

    #[error("{0}")]
    Message(String),

    #[error("{0}")]
    MessageStr(&'static str),

    #[error("{0}")]
    ConflictWithMessageStr(&'static str),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            #[cfg(feature = "orm")]
            ApiError::DbError(err) => db_err_into_response(err),
            ApiError::Forbidden => {
                tracing::error!("Forbidden access for current user");
                Forbidden.into_response()
            }
            ApiError::Validation(errs) => BadRequest(errs).into_response(),
            ApiError::Message(msg) => UnprocessableEntity(msg).into_response(),
            ApiError::MessageStr(msg) => UnprocessableEntity(msg.to_string()).into_response(),
            ApiError::Unexpected(err) => {
                tracing::error!(?err, "Internal error");
                InternalServer.into_response()
            }
            ApiError::Unauthorized => Unauthorized.into_response(),
            ApiError::ConflictWithMessageStr(msg) => {
                ConflictWithMessage(msg.to_string()).into_response()
            }
            ApiError::ResourceNotFound(resource_id) => {
                ResourceNotFound(resource_id).into_response()
            }
            ApiError::EntityIdNotFound(id) => EntityIdNotFound(id).into_response(),
            ApiError::EntityCodeNotFound(code) => EntityCodeNotFound(code).into_response(),
        }
    }
}

#[cfg(feature = "orm")]
fn db_err_into_response(err: sea_orm::DbErr) -> Response {
    tracing::error!("DbErr: {:?}", err);

    match err {
        sea_orm::DbErr::Exec(sea_orm::RuntimeErr::SqlxError(sea_orm::sqlx::Error::Database(
            database_err,
        ))) => match database_err.kind() {
            sea_orm::sqlx::error::ErrorKind::UniqueViolation => {
                crate::http::response::Conflict.into_response()
            }
            sea_orm::sqlx::error::ErrorKind::ForeignKeyViolation => {
                crate::http::response::UnprocessableEntity(
                    "Cannot delete or update record because it is referenced by another record."
                        .to_string(),
                )
                .into_response()
            }
            _ => crate::http::response::InternalServer.into_response(),
        },
        _ => crate::http::response::InternalServer.into_response(),
    }
}
