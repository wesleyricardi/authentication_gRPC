use authentication_gRPC::{
    controllers::authentication::authentication_controller::AuthenticationController,
    models::authentication::authentication_model::{CodeType, UserModelRecoverUserDataReturn},
    security::jwt::JWTAuthenticateToken,
};

use crate::{
    mocks::user_model_mock::{
        get_mock_user_model, MockUserModelCreateUserCode, MockUserModelParams,
        MockUserModelRecoverUserData,
    },
    utils::builders::UserControllerBuilderForTest,
};

#[tokio::test]
async fn test_authentication() {
    const FAKE_USER_ID: &str = "user_id";
    const FAKE_EMAIL: &str = "test@controller.com";
    const FAKE_USERNAME: &str = "username";
    const FAKE_USER_CODE: &str = "000001";
    const FAKE_JWT_TOKEN: &str = "fake_jwt_token";

    let mock_user_model = get_mock_user_model(MockUserModelParams {
        recover_user_data: Some(MockUserModelRecoverUserData {
            calls: 1,
            param_id_with: FAKE_USER_ID.to_string(),
            fn_returning: |_| {
                Ok(UserModelRecoverUserDataReturn {
                    username: FAKE_USERNAME.to_string(),
                    email: FAKE_EMAIL.to_string(),
                    activated: false,
                    blocked: false,
                })
            },
        }),
        create_user_code: Some(MockUserModelCreateUserCode {
            calls: 1,
            param_user_id_with: FAKE_USER_ID.to_string(),
            param_code_type_with: CodeType::Activation,
            fn_returning: |_, _| Ok(FAKE_USER_CODE.to_string()),
        }),
        ..Default::default()
    });

    let controller_user = UserControllerBuilderForTest::new()
        .mount_model(mock_user_model)
        .mount_send_email(|to, subject, body| {
            assert_eq!(to, FAKE_EMAIL);
            assert_eq!(subject, "activation code");
            assert_eq!(
                body,
                format!("<div>The activation code is {}</div>", FAKE_USER_CODE)
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
        .send_activation_code(FAKE_JWT_TOKEN.to_string())
        .await
        .unwrap();

    assert_eq!(response, "Code send successufully");
}
