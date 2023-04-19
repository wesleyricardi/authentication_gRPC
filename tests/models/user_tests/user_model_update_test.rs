use authentication_gRPC::{
    error::*,
    models::authentication::authentication_model::{AuthenticationModel, UserModel, UserModelUpdateParams},
    repositories::user::user_repository::{
        UserRepositoryUpdateParams,
    },
};

use crate::mocks::{user_repository_mock::{
    get_mock_user_repository, MockUserRepositoryParams, MockUserRepositoryStoreUpdate,
}, users_code_repository_mock::{get_mock_users_code_repository, MockUsersCodeRepositoryParams}};

const FAKE_ID: &str = "userFakeId";
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
        activated: None,
        blocked: None,
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
        user_code_repository: get_mock_users_code_repository(MockUsersCodeRepositoryParams {
            ..Default::default()
        }),
        generate_code: || panic!("cannot be called on this test")
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

    assert_eq!(response, "User updated successfully");
}

fn mock_user_repository_store_update(
    id: String,
    _user: UserRepositoryUpdateParams,
) -> Result<String, AppError> {
    if FAKE_ID != id {
        return Err(AppError::new(
            Code::NotFound,
            "not found the user with the given id",
        ));
    }
    Ok(String::from("User updated successfully"))
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
