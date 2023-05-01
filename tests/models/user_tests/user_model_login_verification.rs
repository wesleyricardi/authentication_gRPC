use authentication_gRPC::{
    models::authentication_model::AuthenticationModel,
    repositories::user_repository::UserRepositoryConsultReturn,
};

use crate::{
    mocks::user_repository_mock::{
        get_mock_user_repository, MockUserRepositoryConsultByUsername, MockUserRepositoryParams,
    },
    utils::builders::UserModelBuilderForTest,
};

const FAKE_ID: &str = "userFakeId";
const FAKE_USERNAME: &str = "username";
const FAKE_EMAIL: &str = "test@model.com";
const FAKE_PASSWORD: &str = "password";

#[tokio::test]
async fn test_login_verification() {
    let mock_user_repository = get_mock_user_repository(MockUserRepositoryParams {
        consult_by_username: Some(MockUserRepositoryConsultByUsername {
            calls: 1,
            param_username_with: FAKE_USERNAME.to_string(),
            fn_returning: |username| {
                Ok(UserRepositoryConsultReturn {
                    id: FAKE_ID.to_string(),
                    username,
                    email: FAKE_EMAIL.to_string(),
                    password: FAKE_PASSWORD.to_string(),
                    activated: false,
                    blocked: false,
                })
            },
        }),
        ..Default::default()
    });

    let model_user = UserModelBuilderForTest::new()
        .mount_user_repository(mock_user_repository)
        .mount_password_verify(|_, _| return Ok(true))
        .build();

    let user = model_user
        .login_verification(FAKE_USERNAME.to_string(), FAKE_PASSWORD.to_string())
        .await
        .unwrap();

    assert_eq!(user.id, FAKE_ID);
    assert_eq!(user.email, FAKE_EMAIL);
}

#[tokio::test]
async fn test_login_verification_givin_wrong_password() {
    const WRONG_PASSWORD: &str = "Wrong password";

    let mock_user_repository = get_mock_user_repository(MockUserRepositoryParams {
        consult_by_username: Some(MockUserRepositoryConsultByUsername {
            calls: 1,
            param_username_with: FAKE_USERNAME.to_string(),
            fn_returning: |username| {
                Ok(UserRepositoryConsultReturn {
                    id: FAKE_ID.to_string(),
                    username,
                    email: FAKE_EMAIL.to_string(),
                    password: FAKE_PASSWORD.to_string(),
                    activated: false,
                    blocked: false,
                })
            },
        }),
        ..Default::default()
    });

    let model_user = UserModelBuilderForTest::new()
        .mount_user_repository(mock_user_repository)
        .mount_password_verify(|_, _| return Ok(false)) //return false to simulate wrong password
        .build();

    match model_user
        .login_verification(FAKE_USERNAME.to_string(), WRONG_PASSWORD.to_string())
        .await
    {
        Ok(_) => panic!("verification should fail"),
        Err(error) => assert_eq!(error.message, "Incorrect password"),
    };
}
