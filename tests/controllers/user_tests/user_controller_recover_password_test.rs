use authentication_gRPC::{
    controllers::authentication::authentication_controller::AuthenticationController,
    dtos::controllers::dtos_controller_user::UserControllerRecoverPasswordReq,
};

use crate::{
    mocks::{
        sanitizer_user_input_mock::{
            get_mock_user_input_sanitizer, MockUserInputSanitizeEmail, MockUserInputSanitizeParams,
            MockUserInputSanitizePassword,
        },
        user_model_mock::{get_mock_user_model, MockUserModelParams, MockUserModelRecoverPassword},
    },
    utils::builders::UserControllerBuilderForTest,
};

const FAKE_EMAIL: &str = "test@controller.com";
const FAKE_PASSWORD: &str = "new_password";
const SANITIZED_EMAIL: &str = "email@sanitized.com";
const SANITIZED_PASSWORD: &str = "password_sanitized";
const FAKE_CODE_KEY: &str = "000001";

#[tokio::test]
async fn test_authentication() {
    let mock_sanitize_user = get_mock_user_input_sanitizer(MockUserInputSanitizeParams {
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
        ..Default::default()
    });

    let mock_user_model = get_mock_user_model(MockUserModelParams {
        recover_password: Some(MockUserModelRecoverPassword {
            calls: 1,
            param_user_email_with: SANITIZED_EMAIL.to_string(),
            param_new_password_with: SANITIZED_PASSWORD.to_string(),
            param_code_key_with: FAKE_CODE_KEY.to_string(),
            fn_returning: |_, _, _| Ok(String::from("Password recovered successfully")),
        }),
        ..Default::default()
    });

    let controller_user = UserControllerBuilderForTest::new()
        .mount_model(mock_user_model)
        .mount_sanitize_user(mock_sanitize_user)
        .build();

    let response = controller_user
        .recover_user_password(UserControllerRecoverPasswordReq {
            email: FAKE_EMAIL.to_string(),
            new_password: FAKE_PASSWORD.to_string(),
            code_key: FAKE_CODE_KEY.to_string(),
        })
        .await
        .unwrap();

    assert_eq!(response, "Password recovered successfully")
}
