use crate::{
    mocks::{
        sanitizer_user_input_mock::*,
        user_model_mock::{get_mock_user_model, MockUserModelParams, MockUserModelUpdate},
    },
    utils::builders::UserControllerBuilderForTest,
};
use authentication_gRPC::{
    controllers::authentication_controller::AuthenticationController,
    dtos::models::dtos_model_user::UserModelUpdateParams, security::jwt::JWTAuthenticateToken,
};

const FAKE_USER_ID: &str = "user_id";
const FAKE_EMAIL: &str = "test@controller.com";
const FAKE_JWT_TOKEN: &str = "fake_jwt_token";

const SANITIZED_EMAIL: &str = "sanitized@email.com";

#[tokio::test]
async fn test_update_email() {
    let mock_sanitize_user = get_mock_user_input_sanitizer(MockUserInputSanitizeParams {
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
                username: None,
                email: Some(SANITIZED_EMAIL.to_string()),
            },
            fn_returning: |_, _| Ok(String::from("Email updated successfully")),
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
        .update_email(FAKE_JWT_TOKEN.to_string(), FAKE_EMAIL.to_string())
        .await
        .unwrap();

    assert_eq!(response, "Email updated successfully");
}
