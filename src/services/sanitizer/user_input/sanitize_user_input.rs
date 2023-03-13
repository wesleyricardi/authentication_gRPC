use sanitizer::prelude::*;
pub use tonic::Status;

pub trait SanitizeUser {
    fn sanitize_username_input(&self, username: String) -> Result<String, Status>;
    fn sanitize_email_input(&self, email: String) -> Result<String, Status>;
    fn sanitize_password_input(&self, password: String) -> Result<String, Status>;
}

pub struct SanitizeUserImpl;

impl SanitizeUser for SanitizeUserImpl {
    fn sanitize_username_input(&self, username: String) -> Result<String, Status> {
        if username.is_empty() {
            return Err(Status::new(
                tonic::Code::InvalidArgument,
                "Username is empty",
            ));
        };

        let mut instance = StringSanitizer::from(username);
        instance.trim().alphanumeric();

        let username_sanitized = instance.get();
        if username_sanitized.is_empty() {
            return Err(Status::new(
                tonic::Code::Internal,
                "Username is empty after sanitize",
            ));
        };

        return Ok(username_sanitized);
    }

    fn sanitize_email_input(&self, email: String) -> Result<String, Status> {
        if email.is_empty() {
            return Err(Status::new(tonic::Code::InvalidArgument, "Email is empty"));
        }

        let mut instance = StringSanitizer::from(email);
        instance.trim().to_lowercase();

        let email_sanitized = instance.get();

        if email_sanitized.is_empty() {
            return Err(Status::new(
                tonic::Code::InvalidArgument,
                "Email is empty after sanitize",
            ));
        };

        return Ok(email_sanitized);
    }

    fn sanitize_password_input(&self, password: String) -> Result<String, Status> {
        if password.is_empty() {
            return Err(Status::new(
                tonic::Code::InvalidArgument,
                "Password is empty",
            ));
        }

        let mut instance = StringSanitizer::from(password);
        instance.trim();

        let password_sanitized = instance.get();

        if password_sanitized.is_empty() {
            return Err(Status::new(
                tonic::Code::InvalidArgument,
                "Password is empty after sanitize",
            ));
        };

        return Ok(password_sanitized);
    }
}
