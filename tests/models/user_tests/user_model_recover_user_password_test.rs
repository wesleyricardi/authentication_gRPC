use chrono::{Duration, Utc};

use authentication_gRPC::{
    error::{AppError, Code},
    models::authentication::authentication_model::AuthenticationModel,
    repositories::{
        user::user_repository::UserRepositoryUpdateParams,
        users_code::users_code_repository::UsersCode,
    },
};

use crate::{
    mocks::{
        user_repository_mock::{
            get_mock_user_repository, MockUserRepositoryParams, MockUserRepositoryStoreUpdate,
        },
        users_code_repository_mock::{
            get_mock_users_code_repository, MockUsersCodeRepositoryGet,
            MockUsersCodeRepositoryParams,
        },
    },
    utils::builders::UserModelBuilderForTest,
};

const FAKE_NEW_PASSWORD: &str = "newPassword";
const FAKE_ID: &str = "userFakeId";
const FAKE_CODE: &str = "000001";

#[tokio::test]
async fn test_recover_user_password() {
    let user_store_update_params = UserRepositoryUpdateParams {
        password: Some(FAKE_NEW_PASSWORD.to_string()),
        ..Default::default()
    };

    let mock_repository = get_mock_user_repository(MockUserRepositoryParams {
        store_update: Some(MockUserRepositoryStoreUpdate {
            calls: 1,
            param_id_with: FAKE_ID.to_string(),
            param_user_with: user_store_update_params,
            fn_returning: |_, _| Ok(String::from("User updated successfully")),
        }),
        ..Default::default()
    });

    let mock_users_code_repository =
        get_mock_users_code_repository(MockUsersCodeRepositoryParams {
            get: Some(MockUsersCodeRepositoryGet {
                calls: 1,
                param_user_id_with: FAKE_ID.to_string(),
                param_code_with: FAKE_CODE.to_string(),
                fn_returning: |user_id, code| {
                    Ok(UsersCode {
                        code,
                        user_id,
                        expire_at: Utc::now().naive_utc() + Duration::minutes(30),
                    })
                },
            }),
            ..Default::default()
        });

    let model_user = UserModelBuilderForTest::new()
        .mount_user_repository(mock_repository)
        .mount_code_repository(mock_users_code_repository)
        .build();

    let response = model_user
        .recover_user_password(
            FAKE_ID.to_string(),
            FAKE_NEW_PASSWORD.to_string(),
            FAKE_CODE.to_string(),
        )
        .await
        .unwrap();

    assert_eq!(response, "Password updated");
}

#[tokio::test]
async fn test_recover_user_password_with_expire_code() {
    let mock_users_code_repository =
        get_mock_users_code_repository(MockUsersCodeRepositoryParams {
            get: Some(MockUsersCodeRepositoryGet {
                calls: 1,
                param_user_id_with: FAKE_ID.to_string(),
                param_code_with: FAKE_CODE.to_string(),
                fn_returning: |user_id, code| {
                    Ok(UsersCode {
                        code,
                        user_id,
                        expire_at: Utc::now().naive_utc() - Duration::minutes(30), //simulating code expired
                    })
                },
            }),
            ..Default::default()
        });

    let model_user = UserModelBuilderForTest::new()
        .mount_code_repository(mock_users_code_repository)
        .build();

    match model_user
        .recover_user_password(
            FAKE_ID.to_string(),
            FAKE_NEW_PASSWORD.to_string(),
            FAKE_CODE.to_string(),
        )
        .await
    {
        Ok(_) => panic!("Expected error"),
        Err(error) => assert_eq!(error.message, "Code expired"),
    }
}

#[tokio::test]
async fn test_recover_user_password_with_invalid_code() {
    let mock_users_code_repository =
        get_mock_users_code_repository(MockUsersCodeRepositoryParams {
            get: Some(MockUsersCodeRepositoryGet {
                calls: 1,
                param_user_id_with: FAKE_ID.to_string(),
                param_code_with: FAKE_CODE.to_string(),
                fn_returning: |_, _| {
                    Err(AppError::new(Code::NotFound, "code not found")) //simulating code not found
                },
            }),
            ..Default::default()
        });

    let model_user = UserModelBuilderForTest::new()
        .mount_code_repository(mock_users_code_repository)
        .build();

    match model_user
        .recover_user_password(
            FAKE_ID.to_string(),
            FAKE_NEW_PASSWORD.to_string(),
            FAKE_CODE.to_string(),
        )
        .await
    {
        Ok(_) => panic!("Expected error"),
        Err(error) => assert_eq!(error.message, "Code not found"),
    }
}
