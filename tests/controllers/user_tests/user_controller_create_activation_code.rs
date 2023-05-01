use authentication_gRPC::{
    controllers::authentication_controller::AuthenticationController,
    security::jwt::JWTAuthenticateToken,
};

use crate::{
    mocks::user_model_mock::{
        get_mock_user_model, MockUserModelCreateCodeByUserID, MockUserModelParams,
    },
    utils::builders::UserControllerBuilderForTest,
};

const FAKE_USER_ID: &str = "user_id";
const FAKE_USER_CODE: &str = "000001";
const FAKE_JWT_TOKEN: &str = "fake_jwt_token";

#[tokio::test]
async fn test_create_activation_code() {
    let mock_user_model = get_mock_user_model(MockUserModelParams {
        create_code_by_user_id: Some(MockUserModelCreateCodeByUserID {
            calls: 1,
            param_user_id_with: FAKE_USER_ID.to_string(),
            fn_returning: |_| Ok(FAKE_USER_CODE.to_string()),
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
                exp: 99999999,
            })
        })
        .build();

    let response = controller_user
        .create_activation_code(FAKE_JWT_TOKEN.to_string())
        .await
        .unwrap();

    assert_eq!(response, FAKE_USER_CODE);
}

#[tokio::test]
async fn test_create_activation_code_for_user_already_active() {
    let controller_user = UserControllerBuilderForTest::new()
        .mount_jwt_decode(|_| {
            Ok(JWTAuthenticateToken {
                sub: FAKE_USER_ID.to_string(),
                activated: true,
                blocked: false,
                exp: 99999999,
            })
        })
        .build();

    match controller_user
        .create_activation_code(FAKE_JWT_TOKEN.to_string())
        .await
    {
        Ok(_) => panic!("Expected error"),
        Err(error) => assert_eq!(error.message, "User already activated"),
    }
}
