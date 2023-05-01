use redis::RedisError;

use crate::error::{AppError, Code};

pub fn redis_error_to_app_error(error: RedisError) -> AppError {
    match error.kind() {
        redis::ErrorKind::ResponseError => AppError::new(Code::Internal, "The server generated an invalid response."),
        redis::ErrorKind::AuthenticationFailed => AppError::new(Code::Internal, "The server generated an invalid response."),
        redis::ErrorKind::TypeError => AppError::new(Code::Internal, "Operation failed because of a type mismatch."),
        redis::ErrorKind::ExecAbortError => AppError::new(Code::Internal, "A script execution was aborted."),
        redis::ErrorKind::BusyLoadingError => AppError::new(Code::Internal, "The server cannot response because it's loading a dump."),
        redis::ErrorKind::NoScriptError => AppError::new(Code::Internal, "A script that was requested does not actually exist."),
        redis::ErrorKind::InvalidClientConfig => AppError::new(Code::Internal, "An error that was caused because the parameter to the client were wrong."),
        redis::ErrorKind::Moved => AppError::new(Code::Internal, "Raised if a key moved to a different node."),
        redis::ErrorKind::Ask => AppError::new(Code::Internal, "Raised if a key moved to a different node but we need to ask."),
        redis::ErrorKind::TryAgain => AppError::new(Code::Internal, "Raised if a request needs to be retried."),
        redis::ErrorKind::ClusterDown => AppError::new(Code::Internal, "Raised if a redis cluster is down."),
        redis::ErrorKind::CrossSlot => AppError::new(Code::Internal, "A request spans multiple slots."),
        redis::ErrorKind::MasterDown => AppError::new(Code::Internal, "A cluster master is unavailable."),
        redis::ErrorKind::IoError => AppError::new(Code::Internal, "This kind is returned if the redis error is one that is not native to the system. This is usually the case if the cause is another error."),
        redis::ErrorKind::ClientError => AppError::new(Code::Internal, "An error raised that was identified on the client before execution."),
        redis::ErrorKind::ExtensionError => AppError::new(Code::Internal, "An extension error. This is an error created by the server that is not directly understood by the library."),
        redis::ErrorKind::ReadOnly => AppError::new(Code::Internal, "An extension error. This is an error created by the server that is not directly understood by the library."),
        _ => AppError::new(Code::Unknown, "Error unknown"),
    }
}
