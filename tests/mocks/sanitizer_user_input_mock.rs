use authentication_gRPC::{
    error::AppError,
    services::sanitizer::authentication_input::sanitize_authentication_input::MockSanitizeAuthentication,
};
use mockall::predicate;

pub struct MockUserInputSanitizeUsername {
    pub calls: usize,
    pub param_username_with: String,
    pub fn_returning: fn(String) -> Result<String, AppError>,
}

pub struct MockUserInputSanitizeEmail {
    pub calls: usize,
    pub param_email_with: String,
    pub fn_returning: fn(String) -> Result<String, AppError>,
}

pub struct MockUserInputSanitizePassword {
    pub calls: usize,
    pub param_password_with: String,
    pub fn_returning: fn(String) -> Result<String, AppError>,
}

#[derive(Default)]
pub struct MockUserInputSanitizeParams {
    pub username: Option<MockUserInputSanitizeUsername>,
    pub email: Option<MockUserInputSanitizeEmail>,
    pub password: Option<MockUserInputSanitizePassword>,
}

pub fn get_mock_user_input_sanitizer(
    expectations: MockUserInputSanitizeParams,
) -> MockSanitizeAuthentication {
    let mut mock_user_input_sanitize = MockSanitizeAuthentication::new();

    if expectations.username.is_some() {
        let MockUserInputSanitizeUsername {
            calls,
            param_username_with,
            fn_returning,
        } = expectations.username.unwrap();

        mock_user_input_sanitize
            .expect_sanitize_username_input()
            .with(predicate::eq(param_username_with))
            .times(calls)
            .returning(fn_returning);
    }

    if expectations.email.is_some() {
        let MockUserInputSanitizeEmail {
            calls,
            param_email_with,
            fn_returning,
        } = expectations.email.unwrap();

        mock_user_input_sanitize
            .expect_sanitize_email_input()
            .with(predicate::eq(param_email_with))
            .times(calls)
            .returning(fn_returning);
    }

    if expectations.password.is_some() {
        let MockUserInputSanitizePassword {
            calls,
            param_password_with,
            fn_returning,
        } = expectations.password.unwrap();

        mock_user_input_sanitize
            .expect_sanitize_password_input()
            .with(predicate::eq(param_password_with))
            .times(calls)
            .returning(fn_returning);
    }

    mock_user_input_sanitize
}
