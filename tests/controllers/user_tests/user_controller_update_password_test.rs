use crate::{
    mocks::{
        sanitizer_user_input_mock::*,
        user_model_mock::{get_mock_user_model, MockUserModelParams, MockUserModelUpdatePassword},
    },
    utils::builders::UserControllerBuilderForTest,
};
use authentication_gRPC::{
    controllers::authentication::authentication_controller::AuthenticationController,
    dtos::controllers::dtos_controller_user::UserControllerUpdatePasswordReq,
    security::jwt::JWTAuthenticateToken,
};

const FAKE_USER_ID: &str = "user_id";
const FAKE_JWT_TOKEN: &str = "fake_jwt_token";
const FAKE_PASSWORD: &str = "fake_password";
const SANITIZED_PASSWORD: &str = "fake_password_sanitized";

#[tokio::test]
async fn test_update_password() {
    let mock_sanitize_user = get_mock_user_input_sanitizer(MockUserInputSanitizeParams {
        password: Some(MockUserInputSanitizePassword {
            calls: 2,
            param_password_with: FAKE_PASSWORD.to_string(),
            fn_returning: |_| Ok(SANITIZED_PASSWORD.to_string()),
        }),
        ..Default::default()
    });

    let mock_user_model = get_mock_user_model(MockUserModelParams {
        update_password: Some(MockUserModelUpdatePassword {
            calls: 1,
            param_id_with: FAKE_USER_ID.to_string(),
            param_new_password_with: SANITIZED_PASSWORD.to_string(),
            param_old_password_with: SANITIZED_PASSWORD.to_string(),
            fn_returning: |_, _, _| Ok(String::from("User password updated successfully")),
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
        .update_password(
            FAKE_JWT_TOKEN.to_string(),
            UserControllerUpdatePasswordReq {
                new_password: FAKE_PASSWORD.to_string(), //i`m using the same password, because mock_sanitize_user expected 2 calls with param_password_with equals
                old_password: FAKE_PASSWORD.to_string(), //to change that, must refactor the factory get_mock_user_input_sanitizer
            },
        )
        .await
        .unwrap();

    assert_eq!(response, "User password updated successfully");
}

#[tokio::test]
async fn test_update_password_with_user_blocked() {
    let mock_sanitize_user = get_mock_user_input_sanitizer(MockUserInputSanitizeParams {
        password: Some(MockUserInputSanitizePassword {
            calls: 2,
            param_password_with: FAKE_PASSWORD.to_string(),
            fn_returning: |_| Ok(SANITIZED_PASSWORD.to_string()),
        }),
        ..Default::default()
    });

    let controller_user = UserControllerBuilderForTest::new()
        .mount_sanitize_user(mock_sanitize_user)
        .mount_jwt_decode(|_| {
            Ok(JWTAuthenticateToken {
                sub: FAKE_USER_ID.to_string(),
                activated: true,
                blocked: true,
                exp: 99999999,
            })
        })
        .build();

    match controller_user
        .update_password(
            FAKE_JWT_TOKEN.to_string(),
            UserControllerUpdatePasswordReq {
                new_password: FAKE_PASSWORD.to_string(), //i`m using the same password, because mock_sanitize_user expected 2 calls with param_password_with equals
                old_password: FAKE_PASSWORD.to_string(), //to change that, must refactor the factory get_mock_user_input_sanitizer
            },
        )
        .await
    {
        Ok(_) => panic!("Expected error"),
        Err(error) => assert_eq!(error.message, "User are blocked"),
    }
}

#[tokio::test]
async fn test_update_password_with_user_not_activated() {
    let mock_sanitize_user = get_mock_user_input_sanitizer(MockUserInputSanitizeParams {
        password: Some(MockUserInputSanitizePassword {
            calls: 2,
            param_password_with: FAKE_PASSWORD.to_string(),
            fn_returning: |_| Ok(SANITIZED_PASSWORD.to_string()),
        }),
        ..Default::default()
    });

    let controller_user = UserControllerBuilderForTest::new()
        .mount_sanitize_user(mock_sanitize_user)
        .mount_jwt_decode(|_| {
            Ok(JWTAuthenticateToken {
                sub: FAKE_USER_ID.to_string(),
                activated: false,
                blocked: false,
                exp: 99999999,
            })
        })
        .build();

    match controller_user
        .update_password(
            FAKE_JWT_TOKEN.to_string(),
            UserControllerUpdatePasswordReq {
                new_password: FAKE_PASSWORD.to_string(), //i`m using the same password, because mock_sanitize_user expected 2 calls with param_password_with equals
                old_password: FAKE_PASSWORD.to_string(), //to change that, must refactor the factory get_mock_user_input_sanitizer
            },
        )
        .await
    {
        Ok(_) => panic!("Expected error"),
        Err(error) => assert_eq!(error.message, "User not activated"),
    }
}
