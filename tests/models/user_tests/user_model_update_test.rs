use authentication_gRPC::{
    models::authentication::authentication_model::{AuthenticationModel, UserModelUpdateParams},
    repositories::user::user_repository::UserRepositoryUpdateParams,
};

use crate::{
    mocks::user_repository_mock::{
        get_mock_user_repository, MockUserRepositoryParams, MockUserRepositoryStoreUpdate,
    },
    utils::builders::UserModelBuilderForTest,
};

#[tokio::test]
async fn test_update() {
    const FAKE_ID: &str = "userFakeId";
    const FAKE_UPDATE_USERNAME: &str = "updatedUsername";
    const FAKE_UPDATE_EMAIL: &str = "updated_email@model.com";

    let user_store_update_params = UserRepositoryUpdateParams {
        username: Some(FAKE_UPDATE_USERNAME.to_string()),
        email: Some(FAKE_UPDATE_EMAIL.to_string()),
        ..Default::default()
    };

    let mock_user_repository = get_mock_user_repository(MockUserRepositoryParams {
        store_update: Some(MockUserRepositoryStoreUpdate {
            calls: 1,
            param_id_with: FAKE_ID.to_string(),
            param_user_with: user_store_update_params,
            fn_returning: |_, _| Ok(String::from("User updated successfully")),
        }),
        ..Default::default()
    });

    let model_user = UserModelBuilderForTest::new()
        .mount_user_repository(mock_user_repository)
        .build();

    let response = model_user
        .update(
            FAKE_ID.to_string(),
            UserModelUpdateParams {
                username: Some(FAKE_UPDATE_USERNAME.to_string()),
                email: Some(FAKE_UPDATE_EMAIL.to_string()),
            },
        )
        .await
        .unwrap();

    assert_eq!(response, "User updated successfully");
}
