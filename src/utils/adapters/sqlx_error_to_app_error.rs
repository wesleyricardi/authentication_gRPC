use crate::error::{AppError, Code};
use sqlx::Error;

pub fn sqlx_error_to_app_error(error: Error) -> AppError {
    match error.as_database_error() {
        Some(err) => match err.code().as_ref().map(|s| &**s) {
            Some("23505") => AppError::new(Code::AlreadyExists, err.message()),
            Some(code) => AppError::new(Code::Unknown, format!("Error code: {}", code)),

            None => AppError::new(Code::Unknown, "Null error code".to_string()),
        },
        None => AppError::new(Code::Unknown, "Null error".to_string()),
    }
}
