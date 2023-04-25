use authentication_gRPC::{
    controllers::authentication::authentication_controller::AuthenticationController,
    security::jwt::JWTAuthenticateToken,
};

use crate::{
    mocks::{
        sanitizer_user_input_mock::{
            get_mock_user_input_sanitizer, MockUserInputSanitizeEmail, MockUserInputSanitizeParams,
        },
        user_model_mock::{
            get_mock_user_model, MockUserModelCreateRecoverCode, MockUserModelParams,
        },
    },
    utils::builders::UserControllerBuilderForTest,
};

const FAKE_USER_ID: &str = "user_id";
const FAKE_EMAIL: &str = "test@controller.com";
const FAKE_USER_CODE: &str = "000001";
const SANITIZED_EMAIL: &str = "sanitized@controller.com";

#[tokio::test]
async fn test_send_recover_code() {
    let mock_sanitizer_user = get_mock_user_input_sanitizer(MockUserInputSanitizeParams {
        email: Some(MockUserInputSanitizeEmail {
            calls: 1,
            param_email_with: FAKE_EMAIL.to_string(),
            fn_returning: |_| Ok(SANITIZED_EMAIL.to_string()),
        }),
        ..Default::default()
    });

    let mock_user_model = get_mock_user_model(MockUserModelParams {
        create_recover_code: Some(MockUserModelCreateRecoverCode {
            calls: 1,
            param_user_email_with: SANITIZED_EMAIL.to_string(),
            fn_returning: |_| Ok(FAKE_USER_CODE.to_string()),
        }),
        ..Default::default()
    });

    let controller_user = UserControllerBuilderForTest::new()
        .mount_model(mock_user_model)
        .mount_sanitize_user(mock_sanitizer_user)
        .mount_send_email(|to, subject, body| {
            assert_eq!(to, SANITIZED_EMAIL);
            assert_eq!(subject, "recover code");
            assert_eq!(
                body,
                format!("<div>The recover code is {}</div>", FAKE_USER_CODE)
            );

            Ok(String::from("Send email successfully"))
        })
        .mount_jwt_decode(|_| {
            Ok(JWTAuthenticateToken {
                sub: FAKE_USER_ID.to_string(),
                activated: false,
                blocked: false,
                exp: 99999999,
            })
        })
        .build();

    let response = controller_user
        .send_recover_code(FAKE_EMAIL.to_string())
        .await
        .unwrap();

    assert_eq!(response, "Code send successufully");
}
