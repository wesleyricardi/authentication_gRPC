use tonic::Status;

use crate::error::*;

pub fn app_error_to_grpc_error(error: AppError) -> Status {
    match error.code {
        Code::Ok => Status::new(tonic::Code::Ok, error.message),
        Code::Cancelled => Status::new(tonic::Code::Cancelled, error.message),
        Code::Unknown => Status::new(tonic::Code::Unknown, error.message),
        Code::InvalidArgument => Status::new(tonic::Code::InvalidArgument, error.message),
        Code::DeadlineExceeded => Status::new(tonic::Code::DeadlineExceeded, error.message),
        Code::NotFound => Status::new(tonic::Code::NotFound, error.message),
        Code::AlreadyExists => Status::new(tonic::Code::AlreadyExists, error.message),
        Code::PermissionDenied => Status::new(tonic::Code::PermissionDenied, error.message),
        Code::ResourceExhausted => Status::new(tonic::Code::ResourceExhausted, error.message),
        Code::FailedPrecondition => Status::new(tonic::Code::FailedPrecondition, error.message),
        Code::Aborted => Status::new(tonic::Code::Aborted, error.message),
        Code::OutOfRange => Status::new(tonic::Code::OutOfRange, error.message),
        Code::Unimplemented => Status::new(tonic::Code::Unimplemented, error.message),
        Code::Internal => Status::new(tonic::Code::Internal, error.message),
        Code::Unavailable => Status::new(tonic::Code::Unavailable, error.message),
        Code::DataLoss => Status::new(tonic::Code::DataLoss, error.message),
        Code::Unauthenticated => Status::new(tonic::Code::Unauthenticated, error.message),
    }
}
