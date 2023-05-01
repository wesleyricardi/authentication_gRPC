use authentication_gRPC::{
    error::AppError,
    repositories::users_code_repository::{MockUsersCodeRepository, UsersCode},
};
use mockall::predicate;

pub struct MockUsersCodeRepositoryStore {
    pub calls: usize,
    pub param_code_withf: fn(&UsersCode) -> bool,
    pub fn_returning: fn(code: UsersCode) -> Result<String, AppError>,
}

pub struct MockUsersCodeRepositoryGet {
    pub calls: usize,
    pub param_user_id_with: String,
    pub param_code_with: String,
    pub fn_returning: fn(user_id: String, code: String) -> Result<UsersCode, AppError>,
}

#[derive(Default)]
pub struct MockUsersCodeRepositoryParams {
    pub store: Option<MockUsersCodeRepositoryStore>,
    pub get: Option<MockUsersCodeRepositoryGet>,
}

pub fn get_mock_users_code_repository(
    expectations: MockUsersCodeRepositoryParams,
) -> MockUsersCodeRepository {
    let mut mock_users_code_repository = MockUsersCodeRepository::new();

    if let Some(store) = expectations.store {
        let MockUsersCodeRepositoryStore {
            calls,
            param_code_withf,
            fn_returning,
        } = store;

        mock_users_code_repository
            .expect_store()
            .withf(param_code_withf)
            .times(calls)
            .returning(move |code| Box::pin(async move { fn_returning(code) }));
    }

    if let Some(MockUsersCodeRepositoryGet {
        calls,
        param_user_id_with,
        param_code_with,
        fn_returning,
    }) = expectations.get
    {
        mock_users_code_repository
            .expect_get()
            .with(
                predicate::eq(param_user_id_with),
                predicate::eq(param_code_with),
            )
            .times(calls)
            .returning(move |user_id, code| Box::pin(async move { fn_returning(user_id, code) }));
    }

    mock_users_code_repository
}
