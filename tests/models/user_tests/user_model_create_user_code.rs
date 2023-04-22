use authentication_gRPC::{repositories::users_code::users_code_repository::UsersCode, models::authentication::authentication_model::{AuthenticationModel, CodeType}, error::AppError};
use chrono::Utc;

use crate::{mocks::{users_code_repository_mock::{MockUsersCodeRepositoryStore, MockUsersCodeRepositoryParams, get_mock_users_code_repository}}, utils::builders::UserModelBuilder};

#[tokio::test]
async fn test_user_model_create_user_code() {
    const FAKE_ID: &str = "userFakeId";
    const FAKE_CODE: &str = "0000001";

    fn param_code_withf(code: &UsersCode) -> bool {
        code.code == FAKE_CODE.to_string()
        && code.user_id == FAKE_ID.to_string()
        && code.expire_at >= Utc::now().naive_utc()
    }

    let mock_users_code_repository = get_mock_users_code_repository(MockUsersCodeRepositoryParams {
        store: Some(MockUsersCodeRepositoryStore {
            calls: 1,
            param_code_withf: param_code_withf,
            fn_returning: mock_code_repository_store
        }),
        ..Default::default()
    });

    let model = UserModelBuilder::new()
    .mount_generate_code(|| FAKE_CODE.to_string())
    .mount_code_repository(mock_users_code_repository)
    .build();

    let code = model.create_user_code(FAKE_ID.to_string(), CodeType::Activation).await.unwrap();

   assert_eq!(code, FAKE_CODE);
}

fn mock_code_repository_store(
    _: UsersCode,
) -> Result<String, AppError> {
    Ok(String::from("Code store successfully"))
}