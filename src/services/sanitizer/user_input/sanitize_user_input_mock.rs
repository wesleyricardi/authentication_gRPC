use crate::services::sanitizer::user_input::sanitize_user_input::*;
pub struct SanitizeUserMock;

impl SanitizeUser for SanitizeUserMock {
    fn register_sanitize(
        &self,
        user_input: RegisterInputDirty,
    ) -> Result<RegisterInputSanitized, Status> {
        assert!(!user_input.username.is_empty());
        assert!(!user_input.email.is_empty());
        assert!(!user_input.password.is_empty());

        Ok(RegisterInputSanitized {
            username: user_input.username,
            email: user_input.email,
            password: user_input.password,
        })
    }

    fn login_sanitize(
        &self,
        username: String,
        password: String,
    ) -> Result<LoginInputSanitized, Status> {
        assert!(!username.is_empty());
        assert!(!password.is_empty());

        Ok(LoginInputSanitized { username, password })
    }

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
