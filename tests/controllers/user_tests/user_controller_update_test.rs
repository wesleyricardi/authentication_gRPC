use crate::{
    mocks::{
        sanitizer_user_input_mock::*,
        user_model_mock::{get_mock_user_model, MockUserModelParams, MockUserModelUpdate},
    },
    utils::builders::UserControllerBuilderForTest,
};
use authentication_gRPC::{
    controllers::authentication::authentication_controller::{
        AuthenticationController, UpdateParams,
    },
    models::authentication::authentication_model::UserModelUpdateParams,
    security::jwt::JWTAuthenticateToken,
};

#[tokio::test]
async fn test_update() {
    const FAKE_USER_ID: &str = "user_id";
    const FAKE_USERNAME: &str = "username";
    const FAKE_EMAIL: &str = "test@controller.com";
    const FAKE_JWT_TOKEN: &str = "fake_jwt_token";

    const SANITIZED_USERNAME: &str = "username_sanitized";
    const SANITIZED_EMAIL: &str = "sanitized@email.com";

    let mock_sanitize_user = get_mock_user_input_sanitizer(MockUserInputSanitizeParams {
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
        ..Default::default()
    });

    let mock_user_model = get_mock_user_model(MockUserModelParams {
        update: Some(MockUserModelUpdate {
            calls: 1,
            param_id_with: FAKE_USER_ID.to_string(),
            param_user_with: UserModelUpdateParams {
                username: Some(SANITIZED_USERNAME.to_string()),
                email: Some(SANITIZED_EMAIL.to_string()),
            },
            fn_returning: |_, _| Ok(String::from("User updated successfully")),
        }),
        ..Default::default()
    });

    let controller_user = UserControllerBuilderForTest::new()
        .mount_model(mock_user_model)
        .mount_sanitize_user(mock_sanitize_user)
        .mount_jwt_decode(|_| {
            Ok(JWTAuthenticateToken {
                sub: FAKE_USER_ID.to_string(),
                activated: true,
                blocked: false,
                exp: 99999999,
            })
        })
        .build();

    let response = controller_user
        .update(
            FAKE_JWT_TOKEN.to_string(),
            UpdateParams {
                username: Some(FAKE_USERNAME.to_string()),
                email: Some(FAKE_EMAIL.to_string()),
            },
        )
        .await
        .unwrap();

    assert_eq!(response, "User updated successfully");
}
