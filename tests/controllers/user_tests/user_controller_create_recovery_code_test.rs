use authentication_gRPC::controllers::authentication_controller::AuthenticationController;

use crate::{
    mocks::{
        sanitizer_user_input_mock::{
            get_mock_user_input_sanitizer, MockUserInputSanitizeEmail, MockUserInputSanitizeParams,
        },
        user_model_mock::{
            get_mock_user_model, MockUserModelCreateCodeByEmail, MockUserModelParams,
        },
    },
    utils::builders::UserControllerBuilderForTest,
};

const FAKE_EMAIL: &str = "test@controller.com";
const FAKE_USER_CODE: &str = "000001";
const SANITIZED_EMAIL: &str = "sanitized@controller.com";

#[tokio::test]
async fn test_create_recovery_code() {
    let mock_sanitizer_user = get_mock_user_input_sanitizer(MockUserInputSanitizeParams {
        email: Some(MockUserInputSanitizeEmail {
            calls: 1,
            param_email_with: FAKE_EMAIL.to_string(),
            fn_returning: |_| Ok(SANITIZED_EMAIL.to_string()),
        }),
        ..Default::default()
    });

    let mock_user_model = get_mock_user_model(MockUserModelParams {
        create_code_by_email: Some(MockUserModelCreateCodeByEmail {
            calls: 1,
            param_user_email_with: SANITIZED_EMAIL.to_string(),
            fn_returning: |_| Ok(FAKE_USER_CODE.to_string()),
        }),
        ..Default::default()
    });

    let controller_user = UserControllerBuilderForTest::new()
        .mount_model(mock_user_model)
        .mount_sanitize_user(mock_sanitizer_user)
        .build();

    let response = controller_user
        .create_recovery_code(FAKE_EMAIL.to_string())
        .await
        .unwrap();

    assert_eq!(response, FAKE_USER_CODE);
}
