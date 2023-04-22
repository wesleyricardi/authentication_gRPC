use crate::mocks::{
    sanitizer_user_input_mock::{
        get_mock_user_input_sanitizer, MockUserInputSanitizeParams, MockUserInputSanitizePassword,
        MockUserInputSanitizeUsername,
    },
    user_model_mock::{get_mock_user_model, MockUserModelLoginVerification, MockUserModelParams},
};
use authentication_gRPC::{
    controllers::authentication::authentication_controller::{
        AuthenticationController, LoginParams, UserController, UserViewArg,
    },
    error::*,
    models::authentication::authentication_model::UserModelLoginVerificationReturn,
    security::jwt::{JWTAuthenticateToken, UserToken},
};

const FAKE_USER_ID: &str = "user_id";
const FAKE_USERNAME: &str = "username";
const FAKE_EMAIL: &str = "test@controller.com";
const FAKE_PASSWORD: &str = "password";
const FAKE_JWT_TOKEN: &str = "fake_jwt_token";

const SANITIZED_USERNAME: &str = "username_sanitized";
const SANITIZED_PASSWORD: &str = "password_sanitized";

#[tokio::test]
async fn test_login() {
    let expectation_of_sanitize_username = MockUserInputSanitizeUsername {
        calls: 1,
        param_username_with: FAKE_USERNAME.to_string(),
        fn_returning: |_| Ok(SANITIZED_USERNAME.to_string()),
    };

    let expectation_of_sanitize_password = MockUserInputSanitizePassword {
        calls: 1,
        param_password_with: FAKE_PASSWORD.to_string(),
        fn_returning: |_| Ok(SANITIZED_PASSWORD.to_string()),
    };

    let expectations_of_the_methods_that_will_be_used = MockUserModelParams {
        login_verification: Some(MockUserModelLoginVerification {
            calls: 1,
            param_username_with: SANITIZED_USERNAME.to_string(),
            param_password_with: SANITIZED_PASSWORD.to_string(),
            fn_returning: mock_user_model_login_verification,
        }),
        ..Default::default()
    };

    let controller = UserController {
        model: get_mock_user_model(expectations_of_the_methods_that_will_be_used),
        sanitize_user: get_mock_user_input_sanitizer(MockUserInputSanitizeParams {
            username: Some(expectation_of_sanitize_username),
            password: Some(expectation_of_sanitize_password),
            ..Default::default()
        }),
        send_email: mock_send_email_with_returning_error_if_called,
        jwt_encode: mock_jwt_encode,
        jwt_decode: mock_jwt_decode_with_returning_error_if_called,
    };

    let req = LoginParams {
        username: FAKE_USERNAME.to_string(),
        password: FAKE_PASSWORD.to_string(),
    };

    let MockViewReturn { user, token } = controller.login(req, view_mock).await.unwrap();

    assert_eq!(user.id, FAKE_USER_ID);
    assert_eq!(user.username, SANITIZED_USERNAME);
    assert_eq!(user.email, FAKE_EMAIL);
    assert_eq!(user.activated, false);
    assert_eq!(user.blocked, false);
    assert_eq!(token, FAKE_JWT_TOKEN);
}
struct MockViewReturn {
    user: UserViewArg,
    token: String,
}

fn view_mock(user: UserViewArg, token: String) -> MockViewReturn {
    MockViewReturn { user, token }
}

fn mock_user_model_login_verification(
    username: String,
    password: String,
) -> Result<UserModelLoginVerificationReturn, AppError> {
    if username != SANITIZED_USERNAME {
        return Err(AppError::new(
            Code::NotFound,
            "not found the user with the given username",
        ));
    }

    if password != SANITIZED_PASSWORD {
        return Err(AppError::new(Code::PermissionDenied, "Invalid password"));
    }

    Ok(UserModelLoginVerificationReturn {
        id: FAKE_USER_ID.to_string(),
        username,
        email: FAKE_EMAIL.to_string(),
        activated: false,
        blocked: false,
    })
}

fn mock_jwt_encode(user: UserToken) -> Result<String, AppError> {
    let UserToken {
        id,
        username,
        email,
    } = user;
    if id != FAKE_USER_ID || username != SANITIZED_USERNAME || email != FAKE_EMAIL {
        panic!("received invalid wrong expected params")
    }

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
