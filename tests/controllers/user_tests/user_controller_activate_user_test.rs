use authentication_gRPC::{
    controllers::authentication::authentication_controller::AuthenticationController,
    security::jwt::JWTAuthenticateToken,
};

use crate::{
    mocks::user_model_mock::{get_mock_user_model, MockUserModelActivateUser, MockUserModelParams},
    utils::builders::UserControllerBuilderForTest,
};

const TOKEN_FAKE: &str = "this.is.a.fake.jtw.token";
const USER_ID_FAKE: &str = "UserID";
const CODE_FAKE: &str = "000001";

#[tokio::test]
async fn test_activate_user() {
    let mock_user_model = get_mock_user_model(MockUserModelParams {
        activate_user: Some(MockUserModelActivateUser {
            calls: 1,
            param_user_id_with: USER_ID_FAKE.to_string(),
            param_code_key_with: CODE_FAKE.to_string(),
            fn_returning: |_, _| Ok(String::from("User activated successfully")),
        }),
        ..Default::default()
    });

    let controller_user = UserControllerBuilderForTest::new()
        .mount_jwt_decode(|_| {
            Ok(JWTAuthenticateToken {
                sub: USER_ID_FAKE.to_string(),
                activated: false,
                blocked: false,
                exp: 999999,
            })
        })
        .mount_model(mock_user_model)
        .build();

    let response = controller_user
        .activate_user(TOKEN_FAKE.to_string(), CODE_FAKE.to_string())
        .await
        .unwrap();

    assert_eq!(response, "User activated successfully")
}

#[tokio::test]
async fn test_activate_user_already_active() {
    let controller_user = UserControllerBuilderForTest::new()
        .mount_jwt_decode(|_| {
            Ok(JWTAuthenticateToken {
                sub: USER_ID_FAKE.to_string(),
                activated: true,
                blocked: false,
                exp: 999999,
            })
        })
        .build();

    match controller_user
        .activate_user(TOKEN_FAKE.to_string(), CODE_FAKE.to_string())
        .await
    {
        Ok(_) => panic!("Expected error"),
        Err(error) => assert_eq!(error.message, "User already activated"),
    }
}
