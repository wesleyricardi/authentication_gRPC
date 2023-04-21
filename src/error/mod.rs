#[derive(Debug, PartialEq)]
pub enum Code {
    Ok = 0,
    Cancelled = 1,
    Unknown = 2,
    InvalidArgument = 3,
    DeadlineExceeded = 4,
    NotFound = 5,
    AlreadyExists = 6,
    PermissionDenied = 7,
    ResourceExhausted = 8,
    FailedPrecondition = 9,
    Aborted = 10,
    OutOfRange = 11,
    Unimplemented = 12,
    Internal = 13,
    Unavailable = 14,
    DataLoss = 15,
    Unauthenticated = 16,
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
