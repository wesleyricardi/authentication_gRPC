use crate::{
    mocks::{
        sanitizer_user_input_mock::*,
        user_model_mock::{get_mock_user_model, MockUserModelCreate, MockUserModelParams},
    },
    utils::builders::UserControllerBuilderForTest,
};
use authentication_gRPC::{
    controllers::authentication::authentication_controller::{
        AuthenticationController, RegisterParams,
    },
    models::authentication::authentication_model::{UserModelCreateParams, UserModelInsertReturn},
};

#[tokio::test]
async fn test_register() {
    const FAKE_USER_ID: &str = "user_id";
    const FAKE_USERNAME: &str = "username";
    const FAKE_EMAIL: &str = "test@controller.com";
    const FAKE_PASSWORD: &str = "password";
    const FAKE_JWT_TOKEN: &str = "fake_jwt_token";

    const SANITIZED_USERNAME: &str = "username_sanitized";
    const SANITIZED_EMAIL: &str = "sanitized@email.com";
    const SANITIZED_PASSWORD: &str = "password_sanitized";

    let mock_sanitizer_user = get_mock_user_input_sanitizer(MockUserInputSanitizeParams {
        username: Some(MockUserInputSanitizeUsername {
            calls: 1,
            param_username_with: FAKE_USERNAME.to_string(),
            fn_returning: |_| Ok(SANITIZED_USERNAME.to_string()),
        }),
        email: Some(MockUserInputSanitizeEmail {
            calls: 1,
            param_email_with: FAKE_EMAIL.to_string(),
            fn_returning: |_| Ok(SANITIZED_EMAIL.to_string()),
        }),
        password: Some(MockUserInputSanitizePassword {
            calls: 1,
            param_password_with: FAKE_PASSWORD.to_string(),
            fn_returning: |_| Ok(SANITIZED_PASSWORD.to_string()),
        }),
    });

    let user_model_params = UserModelCreateParams {
        username: SANITIZED_USERNAME.to_string(),
        email: SANITIZED_EMAIL.to_string(),
        password: SANITIZED_PASSWORD.to_string(),
    };

    let mock_user_model = get_mock_user_model(MockUserModelParams {
        create: Some(MockUserModelCreate {
            calls: 1,
            param_user_with: user_model_params,
            fn_returning: |user| {
                Ok(UserModelInsertReturn {
                    id: FAKE_USER_ID.to_string(),
                    username: user.username,
                    email: user.email,
                    activated: false,
                    blocked: false,
                })
            },
        }),
        ..Default::default()
    });

    let controller_user = UserControllerBuilderForTest::new()
        .mount_model(mock_user_model)
        .mount_sanitize_user(mock_sanitizer_user)
        .mount_jwt_encode(|_, _, _| Ok(FAKE_JWT_TOKEN.to_string()))
        .build();

    let response = controller_user
        .register(RegisterParams {
            username: FAKE_USERNAME.to_string(),
            email: FAKE_EMAIL.to_string(),
            password: FAKE_PASSWORD.to_string(),
        })
        .await
        .unwrap();

    assert_eq!(response.user.id, FAKE_USER_ID);
    assert_eq!(response.user.username, SANITIZED_USERNAME);
    assert_eq!(response.user.email, SANITIZED_EMAIL);
    assert_eq!(response.user.activated, false);
    assert_eq!(response.user.blocked, false);
    assert_eq!(response.token, FAKE_JWT_TOKEN);
}
