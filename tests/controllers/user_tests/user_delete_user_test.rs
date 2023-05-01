use authentication_gRPC::{
    controllers::authentication_controller::AuthenticationController,
    security::jwt::JWTAuthenticateToken,
};

use crate::{
    mocks::user_model_mock::{get_mock_user_model, MockUserDeleteUser, MockUserModelParams},
    utils::builders::UserControllerBuilderForTest,
};

#[tokio::test]
async fn test_delete_user() {
    const FAKE_USER_ID: &str = "fake_user_id";
    const FAKE_JWT_TOKEN: &str = "fake_jwt_token";

    let mock_user_model = get_mock_user_model(MockUserModelParams {
        delete_user: Some(MockUserDeleteUser {
            calls: 1,
            param_id_with: FAKE_USER_ID.to_string(),
            fn_returning: |_| Ok(String::from("User deleted sucessfully")),
        }),
        ..Default::default()
    });

    let controller_user = UserControllerBuilderForTest::new()
        .mount_model(mock_user_model)
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
        .delete_user(FAKE_JWT_TOKEN.to_string())
        .await
        .unwrap();

    assert_eq!(response, "User deleted sucessfully");
}
