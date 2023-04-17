use authentication_gRPC::{
    error::*,
    models::authentication::authentication_model::{AuthenticationModel, UserModel, UserModelUpdateParams},
    repositories::user::user_repository::{
        UserRepositoryUpdateParams, UserRepositoryUpdateReturn,
    },
};

use crate::mocks::user_repository_mock::{
    get_mock_user_repository, MockUserRepositoryParams, MockUserRepositoryStoreUpdate,
};

const FAKE_ID: &str = "userFakeId";
const FAKE_USERNAME: &str = "username";
const FAKE_EMAIL: &str = "test@model.com";
const FAKE_HASH_PASSWORD: &str = "hash_password";

#[tokio::test]
async fn test_update() {
    const FAKE_UPDATE_USERNAME: &str = "updatedUsername";
    const FAKE_UPDATE_PASSWORD: &str = "password";
    const FAKE_UPDATE_EMAIL: &str = "updated_email@model.com";

    let user_store_update_params = UserRepositoryUpdateParams {
        username: Some(FAKE_UPDATE_USERNAME.to_string()),
        email: Some(FAKE_UPDATE_EMAIL.to_string()),
        password: Some(FAKE_HASH_PASSWORD.to_string()),
    };

    let mock_repository = get_mock_user_repository(MockUserRepositoryParams {
        store_update: Some(MockUserRepositoryStoreUpdate {
            calls: 1,
            param_id_with: FAKE_ID.to_string(),
            param_user_with: user_store_update_params,
            fn_returning: mock_user_repository_store_update,
        }),
        ..Default::default()
    });
    let model = UserModel {
        user_repository: mock_repository,
        password_hasher: |_| Ok(FAKE_HASH_PASSWORD.to_string()),
        password_verify: mock_password_verify_with_returning_error_if_called,
        new_id: mock_new_id_with_panic_if_called,
    };

    let response = model
        .update(
            FAKE_ID.to_string(),
            UserModelUpdateParams {
                username: Some(FAKE_UPDATE_USERNAME.to_string()),
                email: Some(FAKE_UPDATE_EMAIL.to_string()),
                password: Some(FAKE_UPDATE_PASSWORD.to_string()),
            },
        )
        .await
        .unwrap();

    assert_eq!(response.id, FAKE_ID);
    assert_eq!(response.username, FAKE_UPDATE_USERNAME);
    assert_eq!(response.email, FAKE_UPDATE_EMAIL);
}

fn mock_user_repository_store_update(
    id: String,
    user: UserRepositoryUpdateParams,
) -> Result<UserRepositoryUpdateReturn, AppError> {
    if FAKE_ID != id {
        return Err(AppError::new(
            Code::NotFound,
            "not found the user with the given id",
        ));
    }
    Ok(UserRepositoryUpdateReturn {
        id,
        username: user.username.unwrap_or(FAKE_USERNAME.to_string()),
        email: user.email.unwrap_or(FAKE_EMAIL.to_string()),
    })
}

fn mock_new_id_with_panic_if_called() -> String {
    panic!("cannot be called on this test")
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
