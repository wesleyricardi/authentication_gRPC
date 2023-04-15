use authentication_gRPC::{
    error::*,
    models::user::user_model::{UserModel, UserModelCreateParams, UserModelImpl},
    repositories::user::user_repository::{
        UserRepositoryStoreParams, UserRepositoryStoreReturn,
    },
};

use crate::repositories::user_repository_mock::{
    get_mock_user_repository, MockUserRepositoryParams, MockUserRepositoryStore,
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
    let expectations_of_the_methods_that_will_be_used = MockUserRepositoryParams {
        store: Some(MockUserRepositoryStore {
            calls: 1,
            param_user_with: user_store_params,
            fn_returning: mock_user_repository_store,
        }),
        ..Default::default()
    };
    let model = UserModelImpl {
        user_repository: get_mock_user_repository(expectations_of_the_methods_that_will_be_used),
        password_hasher: |_| Ok(FAKE_HASH_PASSWORD.to_string()),
        password_verify: mock_password_verify_with_returning_error_if_called,
        new_id: || FAKE_ID.to_string(),
    };

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
    })
}

fn mock_password_verify_with_returning_error_if_called(
    _: String,
    _: String,
) -> Result<bool, AppError> {
    Err(AppError::new(
        Code::Internal,
        "cannot be called on this test",
    ))
}
