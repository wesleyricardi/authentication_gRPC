use crate::mocks::{
    sanitizer_user_input_mock::*,
    user_model_mock::{get_mock_user_model, MockUserModelCreate, MockUserModelParams},
};
use authentication_gRPC::{
    controllers::authentication::authentication_controller::{
        AuthenticationController, RegisterParams, UserController,
    },
    error::*,
    models::authentication::authentication_model::{UserModelCreateParams, UserModelInsertReturn},
    security::jwt::JWTAuthenticateToken,
};

const FAKE_USER_ID: &str = "user_id";
const FAKE_USERNAME: &str = "username";
const FAKE_EMAIL: &str = "test@controller.com";
const FAKE_PASSWORD: &str = "password";
const FAKE_JWT_TOKEN: &str = "fake_jwt_token";

const SANITIZED_USERNAME: &str = "username_sanitized";
const SANITIZED_EMAIL: &str = "sanitized@email.com";
const SANITIZED_PASSWORD: &str = "password_sanitized";

#[tokio::test]
async fn test_register() {
    let expectation_of_sanitize_username = MockUserInputSanitizeUsername {
        calls: 1,
        param_username_with: FAKE_USERNAME.to_string(),
        fn_returning: |_| Ok(SANITIZED_USERNAME.to_string()),
    };

    let expectation_of_sanitize_email = MockUserInputSanitizeEmail {
        calls: 1,
        param_email_with: FAKE_EMAIL.to_string(),
        fn_returning: |_| Ok(SANITIZED_EMAIL.to_string()),
    };

    let expectation_of_sanitize_password = MockUserInputSanitizePassword {
        calls: 1,
        param_password_with: FAKE_PASSWORD.to_string(),
        fn_returning: |_| Ok(SANITIZED_PASSWORD.to_string()),
    };

    let user_model_params = UserModelCreateParams {
        username: SANITIZED_USERNAME.to_string(),
        email: SANITIZED_EMAIL.to_string(),
        password: SANITIZED_PASSWORD.to_string(),
    };

    let expectations_of_the_methods_that_will_be_used = MockUserModelParams {
        create: Some(MockUserModelCreate {
            calls: 1,
            param_user_with: user_model_params,
            fn_returning: mock_user_model_create,
        }),
        ..Default::default()
    };

    let controller = UserController {
        model: get_mock_user_model(expectations_of_the_methods_that_will_be_used),
        sanitize_user: get_mock_user_input_sanitizer(MockUserInputSanitizeParams {
            username: Some(expectation_of_sanitize_username),
            email: Some(expectation_of_sanitize_email),
            password: Some(expectation_of_sanitize_password),
        }),
        send_email: mock_send_email_with_returning_error_if_called,
        jwt_encode: mock_jwt_encode,
        jwt_decode: mock_jwt_decode_with_returning_error_if_called,
    };

    let req = RegisterParams {
        username: FAKE_USERNAME.to_string(),
        email: FAKE_EMAIL.to_string(),
        password: FAKE_PASSWORD.to_string(),
    };

    let response = controller.register(req).await.unwrap();

    assert_eq!(response.user.id, FAKE_USER_ID);
    assert_eq!(response.user.username, SANITIZED_USERNAME);
    assert_eq!(response.user.email, SANITIZED_EMAIL);
    assert_eq!(response.user.activated, false);
    assert_eq!(response.user.blocked, false);
    assert_eq!(response.token, FAKE_JWT_TOKEN);
}

fn mock_user_model_create(user: UserModelCreateParams) -> Result<UserModelInsertReturn, AppError> {
    Ok(UserModelInsertReturn {
        id: FAKE_USER_ID.to_string(),
        username: user.username,
        email: user.email,
        activated: false,
        blocked: false,
    })
}

fn mock_jwt_encode(_: String, _: bool, _: bool) -> Result<String, AppError> {
    Ok(FAKE_JWT_TOKEN.to_string())
}

fn mock_jwt_decode_with_returning_error_if_called(
    _: &str,
) -> Result<JWTAuthenticateToken, AppError> {
    Err(AppError::new(
        Code::Internal,
        "cannot be called on this test",
    ))
}

fn mock_send_email_with_returning_error_if_called(
    _: String,
    _: String,
    _: String,
) -> Result<String, AppError> {
    Err(AppError::new(
        Code::Internal,
        "cannot be called on this test",
    ))
}
