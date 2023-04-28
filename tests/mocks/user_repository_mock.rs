use authentication_gRPC::{
    error::*,
    repositories::user::user_repository::{
        MockUserRepository, UserRepositoryConsultReturn, UserRepositoryStoreParams,
        UserRepositoryStoreReturn, UserRepositoryUpdateParams,
    },
};
use mockall::predicate;

#[derive(Debug, PartialEq)]
pub struct MockUserRepositoryStore {
    pub calls: usize,
    pub param_user_with: UserRepositoryStoreParams,
    pub fn_returning: fn(UserRepositoryStoreParams) -> Result<UserRepositoryStoreReturn, AppError>,
}
pub struct MockUserRepositoryConsultByUsername {
    pub calls: usize,
    pub param_username_with: String,
    pub fn_returning: fn(id: String) -> Result<UserRepositoryConsultReturn, AppError>,
}

pub struct MockUserRepositoryConsultById {
    pub calls: usize,
    pub param_id_with: String,
    pub fn_returning: fn(id: String) -> Result<UserRepositoryConsultReturn, AppError>,
}

pub struct MockUserRepositoryConsultByEmail {
    pub calls: usize,
    pub param_email_with: String,
    pub fn_returning: fn(id: String) -> Result<UserRepositoryConsultReturn, AppError>,
}

pub struct MockUserRepositoryStoreUpdate {
    pub calls: usize,
    pub param_id_with: String,
    pub param_user_with: UserRepositoryUpdateParams,
    pub fn_returning: fn(id: String, UserRepositoryUpdateParams) -> Result<String, AppError>,
}

pub struct MockUserRepositoryDelete {
    pub calls: usize,
    pub param_id_with: String,
    pub fn_returning: fn(id: String) -> Result<String, AppError>,
}

#[derive(Default)]
pub struct MockUserRepositoryParams {
    pub store: Option<MockUserRepositoryStore>,
    pub consult_by_username: Option<MockUserRepositoryConsultByUsername>,
    pub consult_by_id: Option<MockUserRepositoryConsultById>,
    pub consult_by_email: Option<MockUserRepositoryConsultByEmail>,
    pub store_update: Option<MockUserRepositoryStoreUpdate>,
    pub delete: Option<MockUserRepositoryDelete>,
}

pub fn get_mock_user_repository(expectations: MockUserRepositoryParams) -> MockUserRepository {
    let mut mock_user_repository = MockUserRepository::new();

    if expectations.store.is_some() {
        let MockUserRepositoryStore {
            calls,
            fn_returning,
            param_user_with,
        } = expectations.store.unwrap();

        mock_user_repository
            .expect_store()
            .with(predicate::eq(param_user_with))
            .times(calls)
            .returning(move |user| Box::pin(async move { fn_returning(user) }));
    }

    if expectations.consult_by_username.is_some() {
        let MockUserRepositoryConsultByUsername {
            calls,
            fn_returning,
            param_username_with,
        } = expectations.consult_by_username.unwrap();

        mock_user_repository
            .expect_consult_by_username()
            .with(predicate::eq(param_username_with))
            .times(calls)
            .returning(move |username| Box::pin(async move { fn_returning(username) }));
    }

    if expectations.consult_by_id.is_some() {
        let MockUserRepositoryConsultById {
            calls,
            fn_returning,
            param_id_with,
        } = expectations.consult_by_id.unwrap();

        mock_user_repository
            .expect_consult_by_id()
            .with(predicate::eq(param_id_with))
            .times(calls)
            .returning(move |id| Box::pin(async move { fn_returning(id) }));
    }

    if let Some(MockUserRepositoryConsultByEmail {
        calls,
        param_email_with,
        fn_returning,
    }) = expectations.consult_by_email
    {
        mock_user_repository
            .expect_consult_by_email()
            .with(predicate::eq(param_email_with))
            .times(calls)
            .returning(move |email| Box::pin(async move { fn_returning(email) }));
    }

    if expectations.store_update.is_some() {
        let MockUserRepositoryStoreUpdate {
            calls,
            fn_returning,
            param_id_with,
            param_user_with,
        } = expectations.store_update.unwrap();

        mock_user_repository
            .expect_store_update()
            .with(predicate::eq(param_id_with), predicate::eq(param_user_with))
            .times(calls)
            .returning(move |id, user| Box::pin(async move { fn_returning(id, user) }));
    }

    if let Some(MockUserRepositoryDelete { calls, param_id_with, fn_returning}) = expectations.delete {
        mock_user_repository
            .expect_delete()
            .with(predicate::eq(param_id_with))
            .times(calls)
            .returning(move |id| Box::pin(async move { fn_returning(id) }));
    }

    mock_user_repository
}
