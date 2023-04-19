use authentication_gRPC::{repositories::users_code::users_code_repository::{UsersCode, MockUsersCodeRepository}, error::AppError};

pub struct MockUsersCodeRepositoryStore {
    pub calls: usize,
    pub param_code_withf: fn(&UsersCode) -> bool,
    pub fn_returning:
        fn(code: UsersCode) -> Result<String, AppError>,
}

#[derive(Default)]
pub struct MockUsersCodeRepositoryParams {
    pub store: Option<MockUsersCodeRepositoryStore>
}

pub fn get_mock_users_code_repository(expectations: MockUsersCodeRepositoryParams) -> MockUsersCodeRepository {
    let mut mock_users_code_repository = MockUsersCodeRepository::new();
    
    if let Some(store) = expectations.store {
        let MockUsersCodeRepositoryStore {calls, param_code_withf, fn_returning} = store;

        mock_users_code_repository
            .expect_store()
            .withf(param_code_withf)
            .times(calls)
            .returning(move |code| Box::pin(async move {fn_returning(code)}));
    }

    mock_users_code_repository
}