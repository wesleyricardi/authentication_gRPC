use crate::mocks::{
    sanitizer_user_input_mock::*,
    user_model_mock::{get_mock_user_model, MockUserModelParams, MockUserModelUpdate},
};
use authentication_gRPC::{
    controllers::authentication::authentication_controller::{
        AuthenticationController, UpdateParams, UserController,
    },
    error::*,
    models::authentication::authentication_model::UserModelUpdateParams,
    security::jwt::JWTAuthenticateToken,
};

const FAKE_USER_ID: &str = "user_id";
const FAKE_USERNAME: &str = "username";
const FAKE_EMAIL: &str = "test@controller.com";
const FAKE_JWT_TOKEN: &str = "fake_jwt_token";

const SANITIZED_USERNAME: &str = "username_sanitized";
const SANITIZED_EMAIL: &str = "sanitized@email.com";

#[tokio::test]
async fn test_update() {
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

    let user_model_params = UserModelUpdateParams {
        username: Some(SANITIZED_USERNAME.to_string()),
        email: Some(SANITIZED_EMAIL.to_string()),
    };

    let expectations_of_the_methods_that_will_be_used = MockUserModelParams {
        update: Some(MockUserModelUpdate {
            calls: 1,
            param_id_with: FAKE_USER_ID.to_string(),
            param_user_with: user_model_params,
            fn_returning: mock_user_model_update,
        }),
        ..Default::default()
    };

    let controller = UserController {
        model: get_mock_user_model(expectations_of_the_methods_that_will_be_used),
        sanitize_user: get_mock_user_input_sanitizer(MockUserInputSanitizeParams {
            username: Some(expectation_of_sanitize_username),
            email: Some(expectation_of_sanitize_email),
            ..Default::default()
        }),
        send_email: mock_send_email_with_returning_error_if_called,
        jwt_encode: |_, _, _| panic!("should not be called"),
        jwt_decode: mock_jwt_decode,
    };
    let req = UpdateParams {
        username: Some(FAKE_USERNAME.to_string()),
        email: Some(FAKE_EMAIL.to_string()),
    };

    let response = controller
        .update(FAKE_JWT_TOKEN.to_string(), req, view_mock)
        .await
        .unwrap();

    assert_eq!(response, "User updated successfully");
}

fn view_mock(message: String) -> String {
    message
}

fn mock_user_model_update(id: String, _user: UserModelUpdateParams) -> Result<String, AppError> {
    if id != FAKE_USER_ID {
        return Err(AppError::new(
            Code::NotFound,
            "not found the user with the given ID",
        ));
    }

    Ok(String::from("User updated successfully"))
}

fn mock_jwt_decode(token: &str) -> Result<JWTAuthenticateToken, AppError> {
    if token != FAKE_JWT_TOKEN {
        return Err(AppError::new(Code::PermissionDenied, "Invalid token"));
    }

    Ok(JWTAuthenticateToken {
        sub: FAKE_USER_ID.to_string(),
        activated: true,
        blocked: false,
        exp: 99999999,
    })
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
