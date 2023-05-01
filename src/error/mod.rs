#[derive(Debug, PartialEq)]
pub enum Code {
    Unknown,
    InvalidArgument,
    NotFound,
    AlreadyExists,
    PermissionDenied,
    Internal,
    Unauthenticated,
    DatabaseError,
    SQLError,
}

#[derive(Debug)]
pub struct AppError {
    pub code: Code,
    pub message: String,
}

impl AppError {
    pub fn new(code: Code, message: impl Into<String>) -> AppError {
        AppError {
            code,
            message: message.into(),
        }
    }
}
