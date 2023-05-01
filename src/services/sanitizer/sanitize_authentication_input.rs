use crate::error::*;
use mockall::automock;
use sanitizer::prelude::*;

#[automock]
pub trait SanitizeAuthentication: Sync + Send {
    fn sanitize_username_input(&self, username: String) -> Result<String, AppError>;
    fn sanitize_email_input(&self, email: String) -> Result<String, AppError>;
    fn sanitize_password_input(&self, password: String) -> Result<String, AppError>;
}

pub struct SanitizeUser;

impl SanitizeAuthentication for SanitizeUser {
    fn sanitize_username_input(&self, username: String) -> Result<String, AppError> {
        if username.is_empty() {
            return Err(AppError::new(Code::InvalidArgument, "Username is empty"));
        };

        let mut instance = StringSanitizer::from(username);
        instance.trim().alphanumeric();

        let username_sanitized = instance.get();
        if username_sanitized.is_empty() {
            return Err(AppError::new(
                Code::Internal,
                "Username is empty after sanitize",
            ));
        };

        Ok(username_sanitized)
    }

    fn sanitize_email_input(&self, email: String) -> Result<String, AppError> {
        if email.is_empty() {
            return Err(AppError::new(Code::InvalidArgument, "Email is empty"));
        }

        let mut instance = StringSanitizer::from(email);
        instance.trim().to_lowercase();

        let email_sanitized = instance.get();

        if email_sanitized.is_empty() {
            return Err(AppError::new(
                Code::InvalidArgument,
                "Email is empty after sanitize",
            ));
        };

        Ok(email_sanitized)
    }

    fn sanitize_password_input(&self, password: String) -> Result<String, AppError> {
        if password.is_empty() {
            return Err(AppError::new(Code::InvalidArgument, "Password is empty"));
        }

        let mut instance = StringSanitizer::from(password);
        instance.trim();

        let password_sanitized = instance.get();

        if password_sanitized.is_empty() {
            return Err(AppError::new(
                Code::InvalidArgument,
                "Password is empty after sanitize",
            ));
        };

        Ok(password_sanitized)
    }
}
