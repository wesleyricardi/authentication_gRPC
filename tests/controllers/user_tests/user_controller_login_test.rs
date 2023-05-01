use authentication_gRPC::dtos::{
    controllers::dtos_controller_user::LoginParams,
    models::dtos_model_user::UserModelLoginVerificationReturn,
};

use crate::{
    mocks::{
        sanitizer_user_input_mock::{
            get_mock_user_input_sanitizer, MockUserInputSanitizeParams,
            MockUserInputSanitizePassword, MockUserInputSanitizeUsername,
        },
        user_model_mock::{
            get_mock_user_model, MockUserModelLoginVerification, MockUserModelParams,
        },
    },
    utils::builders::UserControllerBuilderForTest,
};
use authentication_gRPC::controllers::authentication_controller::AuthenticationController;

#[tokio::test]
async fn test_login() {
    const FAKE_USER_ID: &str = "user_id";
    const FAKE_USERNAME: &str = "username";
    const FAKE_EMAIL: &str = "test@controller.com";
    const FAKE_PASSWORD: &str = "password";
    const FAKE_JWT_TOKEN: &str = "fake_jwt_token";

    const SANITIZED_USERNAME: &str = "username_sanitized";
    const SANITIZED_PASSWORD: &str = "password_sanitized";

    let mock_user_model = get_mock_user_model(MockUserModelParams {
        login_verification: Some(MockUserModelLoginVerification {
            calls: 1,
            param_username_with: SANITIZED_USERNAME.to_string(),
            param_password_with: SANITIZED_PASSWORD.to_string(),
            fn_returning: |username, _| {
                Ok(UserModelLoginVerificationReturn {
                    id: FAKE_USER_ID.to_string(),
                    username,
                    email: FAKE_EMAIL.to_string(),
                    activated: false,
                    blocked: false,
                })
            },
        }),
        ..Default::default()
    });

    let mock_sanitize_user = get_mock_user_input_sanitizer(MockUserInputSanitizeParams {
        username: Some(MockUserInputSanitizeUsername {
            calls: 1,
            param_username_with: FAKE_USERNAME.to_string(),
            fn_returning: |_| Ok(SANITIZED_USERNAME.to_string()),
        }),
        password: Some(MockUserInputSanitizePassword {
            calls: 1,
            param_password_with: FAKE_PASSWORD.to_string(),
            fn_returning: |_| Ok(SANITIZED_PASSWORD.to_string()),
        }),
        ..Default::default()
    });

    let controller_user = UserControllerBuilderForTest::new()
        .mount_model(mock_user_model)
        .mount_sanitize_user(mock_sanitize_user)
        .mount_jwt_encode(|_, _, _| Ok(FAKE_JWT_TOKEN.to_string()))
        .build();

    let response = controller_user
        .login(LoginParams {
            username: FAKE_USERNAME.to_string(),
            password: FAKE_PASSWORD.to_string(),
        })
        .await
        .unwrap();

    assert_eq!(response.user.id, FAKE_USER_ID);
    assert_eq!(response.user.username, SANITIZED_USERNAME);
    assert_eq!(response.user.email, FAKE_EMAIL);
    assert_eq!(response.user.activated, false);
    assert_eq!(response.user.blocked, false);
    assert_eq!(response.token, FAKE_JWT_TOKEN);
}
