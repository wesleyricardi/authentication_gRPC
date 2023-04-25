use authentication_gRPC::{
    models::authentication::authentication_model::{AuthenticationModel, UserModelUpdateParams},
    repositories::user::user_repository::{
        UserRepositoryConsultReturn, UserRepositoryUpdateParams,
    },
};

use crate::{
    mocks::user_repository_mock::{
        get_mock_user_repository, MockUserRepositoryConsultById, MockUserRepositoryParams,
        MockUserRepositoryStoreUpdate,
    },
    utils::builders::UserModelBuilderForTest,
};

const FAKE_ID: &str = "userFakeId";
const FAKE_USERNAME: &str = "username";
const FAKE_EMAIL: &str = "test@model.com";
const FAKE_PASSWORD: &str = "old password";
const FAKE_HASH_PASSWORD: &str = "old hash password";
const FAKE_UPDATE_PASSWORD: &str = "update password";
const FAKE_HASH_UPDATE_PASSWORD: &str = "update hash password";

#[tokio::test]
async fn test_update_password() {
    let mock_user_repository = get_mock_user_repository(MockUserRepositoryParams {
        consult_by_id: Some(MockUserRepositoryConsultById {
            calls: 1,
            param_id_with: FAKE_ID.to_string(),
            fn_returning: |id| {
                Ok(UserRepositoryConsultReturn {
                    id,
                    username: FAKE_USERNAME.to_string(),
                    email: FAKE_EMAIL.to_string(),
                    password: FAKE_HASH_PASSWORD.to_string(),
                    activated: true,
                    blocked: false,
                })
            },
        }),
        store_update: Some(MockUserRepositoryStoreUpdate {
            calls: 1,
            param_id_with: FAKE_ID.to_string(),
            param_user_with: UserRepositoryUpdateParams {
                password: Some(FAKE_HASH_UPDATE_PASSWORD.to_string()),
                ..Default::default()
            },
            fn_returning: |_, _| Ok(String::from("User updated successfully")),
        }),
        ..Default::default()
    });

    let model_user = UserModelBuilderForTest::new()
        .mount_user_repository(mock_user_repository)
        .mount_password_verify(|_, _| Ok(true))
        .mount_password_hasher(|_| Ok(FAKE_HASH_UPDATE_PASSWORD.to_string()))
        .build();

    let response = model_user
        .update_password(
            FAKE_ID.to_string(),
            FAKE_UPDATE_PASSWORD.to_string(),
            FAKE_PASSWORD.to_string(),
        )
        .await
        .unwrap();

    assert_eq!(response, "User updated successfully");
}

#[tokio::test]
async fn test_update_password_with_wrong_old_password() {
    let mock_user_repository = get_mock_user_repository(MockUserRepositoryParams {
        consult_by_id: Some(MockUserRepositoryConsultById {
            calls: 1,
            param_id_with: FAKE_ID.to_string(),
            fn_returning: |id| {
                Ok(UserRepositoryConsultReturn {
                    id,
                    username: FAKE_USERNAME.to_string(),
                    email: FAKE_EMAIL.to_string(),
                    password: FAKE_HASH_PASSWORD.to_string(),
                    activated: true,
                    blocked: false,
                })
            },
        }),
        ..Default::default()
    });

    let model_user = UserModelBuilderForTest::new()
        .mount_user_repository(mock_user_repository)
        .mount_password_verify(|_, _| Ok(false)) //simulate return wrong password
        .build();

    match model_user
        .update_password(
            FAKE_ID.to_string(),
            FAKE_UPDATE_PASSWORD.to_string(),
            "wrong old password".to_string(),
        )
        .await
    {
        Ok(_) => panic!("Expected error"),
        Err(error) => assert_eq!(error.message, "Old password is invalid"),
    }
}
