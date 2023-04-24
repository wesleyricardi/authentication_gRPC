use chrono::{Duration, Utc};

use authentication_gRPC::{
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

#[tokio::test]
async fn test_active_user() {
    const FAKE_UPDATE_USERNAME: &str = "updatedUsername";
    const FAKE_UPDATE_PASSWORD: &str = "password";
    const FAKE_UPDATE_EMAIL: &str = "updated_email@model.com";
    const FAKE_ID: &str = "userFakeId";
    const FAKE_CODE: &str = "000001";

    let user_store_update_params = UserRepositoryUpdateParams {
        activated: Some(true),
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
        .activate_user(FAKE_ID.to_string(), FAKE_CODE.to_string())
        .await
        .unwrap();

    assert_eq!(response, "User activated");
}
