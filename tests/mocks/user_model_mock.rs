use authentication_gRPC::{
    error::*,
    models::authentication::authentication_model::{
        CodeType, MockAuthenticationModel, UserModelCreateParams, UserModelInsertReturn,
        UserModelLoginVerificationReturn, UserModelRecoverUserDataReturn, UserModelUpdateParams,
    },
};
use mockall::predicate;
pub struct MockUserModelCreate {
    pub calls: usize,
    pub param_user_with: UserModelCreateParams,
    pub fn_returning: fn(UserModelCreateParams) -> Result<UserModelInsertReturn, AppError>,
}

pub struct MockUserModelLoginVerification {
    pub calls: usize,
    pub param_username_with: String,
    pub param_password_with: String,
    pub fn_returning: fn(
        username: String,
        password: String,
    ) -> Result<UserModelLoginVerificationReturn, AppError>,
}

pub struct MockUserModelRecoverUserData {
    pub calls: usize,
    pub param_id_with: String,
    pub fn_returning: fn(String) -> Result<UserModelRecoverUserDataReturn, AppError>,
}

pub struct MockUserModelUpdate {
    pub calls: usize,
    pub param_id_with: String,
    pub param_user_with: UserModelUpdateParams,
    pub fn_returning: fn(String, UserModelUpdateParams) -> Result<String, AppError>,
}

pub struct MockUserModelCreateUserCode {
    pub calls: usize,
    pub param_user_id_with: String,
    pub param_code_type_with: CodeType,
    pub fn_returning: fn(user_id: String, code: CodeType) -> Result<String, AppError>,
}

pub struct MockUserModelActivateUser {
    pub calls: usize,
    pub param_user_id_with: String,
    pub param_code_key_with: String,
    pub fn_returning: fn(user_id: String, code_key: String) -> Result<String, AppError>,
}

#[derive(Default)]
pub struct MockUserModelParams {
    pub create: Option<MockUserModelCreate>,
    pub login_verification: Option<MockUserModelLoginVerification>,
    pub recover_user_data: Option<MockUserModelRecoverUserData>,
    pub update: Option<MockUserModelUpdate>,
    pub create_user_code: Option<MockUserModelCreateUserCode>,
    pub activate_user: Option<MockUserModelActivateUser>,
}

pub fn get_mock_user_model(expectations: MockUserModelParams) -> MockAuthenticationModel {
    let mut mock_user_model = MockAuthenticationModel::new();

    if expectations.create.is_some() {
        let MockUserModelCreate {
            calls,
            fn_returning,
            param_user_with,
        } = expectations.create.unwrap();

        mock_user_model
            .expect_create()
            .with(predicate::eq(param_user_with))
            .times(calls)
            .returning(move |user| Box::pin(async move { fn_returning(user) }));
    }

    if expectations.login_verification.is_some() {
        let MockUserModelLoginVerification {
            calls,
            fn_returning,
            param_username_with,
            param_password_with,
        } = expectations.login_verification.unwrap();

        mock_user_model
            .expect_login_verification()
            .with(
                predicate::eq(param_username_with),
                predicate::eq(param_password_with),
            )
            .times(calls)
            .returning(move |username, password| {
                Box::pin(async move { fn_returning(username, password) })
            });
    }

    if expectations.recover_user_data.is_some() {
        let MockUserModelRecoverUserData {
            calls,
            fn_returning,
            param_id_with,
        } = expectations.recover_user_data.unwrap();

        mock_user_model
            .expect_recover_user_data()
            .with(predicate::eq(param_id_with))
            .times(calls)
            .returning(move |id| Box::pin(async move { fn_returning(id) }));
    }

    if expectations.update.is_some() {
        let MockUserModelUpdate {
            calls,
            fn_returning,
            param_id_with,
            param_user_with,
        } = expectations.update.unwrap();

        mock_user_model
            .expect_update()
            .with(predicate::eq(param_id_with), predicate::eq(param_user_with))
            .times(calls)
            .returning(move |id, user| Box::pin(async move { fn_returning(id, user) }));
    }

    if let Some(create_user_code) = expectations.create_user_code {
        let MockUserModelCreateUserCode {
            calls,
            fn_returning,
            param_user_id_with,
            param_code_type_with,
        } = create_user_code;

        mock_user_model
            .expect_create_user_code()
            .with(
                predicate::eq(param_user_id_with),
                predicate::eq(param_code_type_with),
            )
            .times(calls)
            .returning(move |user_id, code| Box::pin(async move { fn_returning(user_id, code) }));
    }

    if let Some(MockUserModelActivateUser {
        calls,
        param_user_id_with,
        param_code_key_with,
        fn_returning,
    }) = expectations.activate_user
    {
        mock_user_model
            .expect_active_user()
            .with(
                predicate::eq(param_user_id_with),
                predicate::eq(param_code_key_with),
            )
            .times(calls)
            .returning(move |user_id, code_key| {
                Box::pin(async move { fn_returning(user_id, code_key) })
            });
    }

    mock_user_model
}
