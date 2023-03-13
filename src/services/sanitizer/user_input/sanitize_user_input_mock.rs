use crate::services::sanitizer::user_input::sanitize_user_input::*;
pub struct SanitizeUserMock;

impl SanitizeUser for SanitizeUserMock {
    fn sanitize_username_input(&self, username: String) -> Result<String, Status> {
        Ok(username)
    }

    fn sanitize_email_input(&self, email: String) -> Result<String, Status> {
        Ok(email)
    }

    fn sanitize_password_input(&self, password: String) -> Result<String, Status> {
        Ok(password)
    }
}
