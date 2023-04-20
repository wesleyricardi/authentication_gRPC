use authentication_gRPC::{repositories::users_code::users_code_repository::UsersCode, models::authentication::authentication_model::{UserModel, AuthenticationModel, CodeType}, error::{AppError, Code}};
use chrono::{Utc};

use crate::mocks::{users_code_repository_mock::{MockUsersCodeRepositoryStore, MockUsersCodeRepositoryParams, get_mock_users_code_repository}, user_repository_mock::{get_mock_user_repository, MockUserRepositoryParams}};

#[tokio::test]
async fn test_user_model_create_user_code() {
    const FAKE_ID: &str = "userFakeId";
    const FAKE_CODE: &str = "0000001";

    fn param_code_withf(code: &UsersCode) -> bool {
        code.code == FAKE_CODE.to_string()
        && code.user_id == FAKE_ID.to_string()
        && code.expire_at >= Utc::now().naive_utc()
    }

    let mock_code_repository_expectations = MockUsersCodeRepositoryParams {
        store: Some(MockUsersCodeRepositoryStore {
            calls: 1,
            param_code_withf: param_code_withf,
            fn_returning: mock_code_repository_store
        }),
        ..Default::default()
    };
    
    let model = UserModel {
        user_code_repository: get_mock_users_code_repository(mock_code_repository_expectations),
        generate_code: || FAKE_CODE.to_string(),
        user_repository: get_mock_user_repository(MockUserRepositoryParams {
            ..Default::default()
        }),
        password_hasher: mock_password_hasher_with_returning_error_if_called,
        password_verify: mock_password_verify_with_returning_error_if_called,
        new_id: || panic!("cannot be called on this test"),
    };

    let code = model.create_user_code(FAKE_ID.to_string(), CodeType::Activation).await.unwrap();

   assert_eq!(code, FAKE_CODE);
}

fn mock_code_repository_store(
    _: UsersCode,
) -> Result<String, AppError> {
    Ok(String::from("Code store successfully"))
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

fn mock_password_hasher_with_returning_error_if_called(_: String) -> Result<String, AppError> {
    Err(AppError::new(
        Code::Internal,
        "cannot be called on this test",
    ))
}