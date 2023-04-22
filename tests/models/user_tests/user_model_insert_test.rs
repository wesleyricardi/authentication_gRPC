use authentication_gRPC::{
    error::*,
    models::authentication::authentication_model::{AuthenticationModel, UserModelCreateParams},
    repositories::user::user_repository::{
        UserRepositoryStoreParams, UserRepositoryStoreReturn,
    },
};

use crate::{
    mocks::user_repository_mock::{
        get_mock_user_repository, 
        MockUserRepositoryParams, 
        MockUserRepositoryStore,
    }, 
    utils::builders::UserModelBuilder
};

#[tokio::test]
async fn test_user_model_create() {
    const FAKE_ID: &str = "userFakeId";
    const FAKE_USERNAME: &str = "usernames";
    const FAKE_EMAIL: &str = "test@model.com";
    const FAKE_PASSWORD: &str = "password";
    const FAKE_HASH_PASSWORD: &str = "hash_password";

    let user_store_params = UserRepositoryStoreParams {
        id: FAKE_ID.to_string(),
        username: FAKE_USERNAME.to_string(),
        email: FAKE_EMAIL.to_string(),
        password: FAKE_HASH_PASSWORD.to_string(),
    };

    let mock_user_repository = get_mock_user_repository(MockUserRepositoryParams {
        store: Some(MockUserRepositoryStore {
            calls: 1,
            param_user_with: user_store_params,
            fn_returning: mock_user_repository_store,
        }),
        ..Default::default()
    });

    let model = UserModelBuilder::new()
    .mount_password_hasher(|_| Ok(FAKE_HASH_PASSWORD.to_string()))
    .mount_new_id(|| FAKE_ID.to_string())
    .mount_user_repository(mock_user_repository)
    .build();

    let response = model
        .create(UserModelCreateParams {
            username: FAKE_USERNAME.to_string(),
            email: FAKE_EMAIL.to_string(),
            password: FAKE_PASSWORD.to_string(),
        })
        .await
        .unwrap();

    assert_eq!(response.id, FAKE_ID);
    assert_eq!(response.username, FAKE_USERNAME);
    assert_eq!(response.email, FAKE_EMAIL)
}

fn mock_user_repository_store(
    user: UserRepositoryStoreParams,
) -> Result<UserRepositoryStoreReturn, AppError> {
    Ok(UserRepositoryStoreReturn {
        id: user.id,
        username: user.username,
        email: user.email,
        activated: false,
        blocked: false
    })
}
