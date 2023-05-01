use tonic::Status;

use crate::error::*;

pub fn app_error_to_grpc_error(error: AppError) -> Status {
    match error.code {
        Code::InvalidArgument => Status::new(tonic::Code::InvalidArgument, error.message),
        Code::NotFound => Status::new(tonic::Code::NotFound, error.message),
        Code::AlreadyExists => Status::new(tonic::Code::AlreadyExists, error.message),
        Code::PermissionDenied => Status::new(tonic::Code::PermissionDenied, error.message),
        Code::Unauthenticated => Status::new(tonic::Code::Unauthenticated, error.message),
        Code::Internal => Status::new(tonic::Code::Internal, "Internal error"),
        Code::Unknown => Status::new(tonic::Code::Unknown, "Unknown error"),
        Code::DatabaseError => Status::new(tonic::Code::Internal, "Internal error"),
        Code::SQLError => Status::new(tonic::Code::Internal, "Internal error"),
    }
}
