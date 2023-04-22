use authentication_gRPC::{
    controllers::authentication::authentication_controller::{
        AuthenticationController, UserController, UserViewArg,
    },
    error::*,
    models::authentication::authentication_model::UserModelRecoverUserDataReturn,
    security::jwt::{JWTAuthenticateToken, UserToken},
};

use crate::mocks::{
    sanitizer_user_input_mock::{get_mock_user_input_sanitizer, MockUserInputSanitizeParams},
    user_model_mock::{get_mock_user_model, MockUserModelParams, MockUserModelRecoverUserData},
};

const FAKE_USER_ID: &str = "user_id";
const FAKE_USERNAME: &str = "username";
const FAKE_EMAIL: &str = "test@controller.com";
const FAKE_JWT_TOKEN: &str = "fake_jwt_token";

#[tokio::test]
async fn test_authentication() {
    let expectations_of_the_methods_that_will_be_used = MockUserModelParams {
        recover_user_data: Some(MockUserModelRecoverUserData {
            calls: 1,
            param_id_with: FAKE_USER_ID.to_string(),
            fn_returning: mock_user_model_recover,
        }),
        ..Default::default()
    };

    let controller = UserController {
        model: get_mock_user_model(expectations_of_the_methods_that_will_be_used),
        sanitize_user: get_mock_user_input_sanitizer(MockUserInputSanitizeParams {
            ..Default::default()
        }),
        send_email: mock_send_email_with_returning_error_if_called,
        jwt_encode: mock_jwt_encode_with_returning_error_if_called,
        jwt_decode: mock_jwt_decode,
    };

    let MockViewReturn { user, .. } = controller
        .authenticate(FAKE_JWT_TOKEN.to_string(), view_mock)
        .await
        .unwrap();

    assert_eq!(user.username, FAKE_USERNAME);
    assert_eq!(user.email, FAKE_EMAIL);
    assert_eq!(user.activated, false);
    assert_eq!(user.blocked, false);
}

struct MockViewReturn {
    user: UserViewArg,
}

fn view_mock(user: UserViewArg) -> MockViewReturn {
    MockViewReturn { user }
}

fn mock_user_model_recover(id: String) -> Result<UserModelRecoverUserDataReturn, AppError> {
    if id != FAKE_USER_ID {
        return Err(AppError::new(
            Code::NotFound,
            "not found the user with the given ID",
        ));
    }

    Ok(UserModelRecoverUserDataReturn {
        username: FAKE_USERNAME.to_string(),
        email: FAKE_EMAIL.to_string(),
        activated: false,
        blocked: false,
    })
}
fn mock_jwt_decode(token: &str) -> Result<JWTAuthenticateToken, AppError> {
    if token != FAKE_JWT_TOKEN {
        return Err(AppError::new(Code::PermissionDenied, "Invalid token"));
    }

    Ok(JWTAuthenticateToken {
        sub: FAKE_USER_ID.to_string(),
        user: UserToken {
            id: FAKE_USER_ID.to_string(),
            username: FAKE_USERNAME.to_string(),
            email: FAKE_EMAIL.to_string(),
        },
        exp: 99999999,
    })
}

fn mock_jwt_encode_with_returning_error_if_called(_: UserToken) -> Result<String, AppError> {
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
