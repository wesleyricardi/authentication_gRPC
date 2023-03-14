use crate::{error::*, services::sanitizer::user_input::sanitize_user_input::*};
pub struct SanitizeUserMock;

impl SanitizeUser for SanitizeUserMock {
    fn sanitize_username_input(&self, username: String) -> Result<String, AppError> {
        Ok(username)
    }

    fn sanitize_email_input(&self, email: String) -> Result<String, AppError> {
        Ok(email)
    }

    fn sanitize_password_input(&self, password: String) -> Result<String, AppError> {
        Ok(password)
    }
}
