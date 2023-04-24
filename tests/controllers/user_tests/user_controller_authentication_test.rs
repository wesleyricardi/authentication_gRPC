use authentication_gRPC::{
    controllers::authentication::authentication_controller::AuthenticationController,
    models::authentication::authentication_model::UserModelRecoverUserDataReturn,
    security::jwt::JWTAuthenticateToken,
};

use crate::{
    mocks::user_model_mock::{
        get_mock_user_model, MockUserModelParams, MockUserModelRecoverUserData,
    },
    utils::builders::UserControllerBuilderForTest,
};

#[tokio::test]
async fn test_authentication() {
    const FAKE_USER_ID: &str = "user_id";
    const FAKE_USERNAME: &str = "username";
    const FAKE_EMAIL: &str = "test@controller.com";
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
        ..Default::default()
    });

    let controller_user = UserControllerBuilderForTest::new()
        .mount_model(mock_user_model)
        .mount_jwt_decode(|_| {
            Ok(JWTAuthenticateToken {
                sub: FAKE_USER_ID.to_string(),
                activated: false,
                blocked: false,
                exp: 999999,
            })
        })
        .build();

    let response = controller_user
        .authenticate(FAKE_JWT_TOKEN.to_string())
        .await
        .unwrap();

    assert_eq!(response.user.username, FAKE_USERNAME);
    assert_eq!(response.user.email, FAKE_EMAIL);
    assert_eq!(response.user.activated, false);
    assert_eq!(response.user.blocked, false);
}
