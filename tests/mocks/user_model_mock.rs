use authentication_gRPC::{
    error::*,
    models::authentication::authentication_model::{
        MockAuthenticationModel, UserModelCreateParams, UserModelInsertReturn,
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

pub struct MockUserModelUpdatePassword {
    pub calls: usize,
    pub param_id_with: String,
    pub param_new_password_with: String,
    pub param_old_password_with: String,
    pub fn_returning:
        fn(user_id: String, new_password: String, old_password: String) -> Result<String, AppError>,
}

pub struct MockUserModelCreateUserActivationCode {
    pub calls: usize,
    pub param_user_id_with: String,
    pub fn_returning: fn(user_id: String) -> Result<String, AppError>,
}

pub struct MockUserModelCreateRecoverCode {
    pub calls: usize,
    pub param_user_email_with: String,
    pub fn_returning: fn(email: String) -> Result<String, AppError>,
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
    pub create_user_activation_code: Option<MockUserModelCreateUserActivationCode>,
    pub create_recover_code: Option<MockUserModelCreateRecoverCode>,
    pub activate_user: Option<MockUserModelActivateUser>,
    pub update_password: Option<MockUserModelUpdatePassword>,
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

    if let Some(MockUserModelCreateUserActivationCode {
        calls,
        fn_returning,
        param_user_id_with,
    }) = expectations.create_user_activation_code
    {
        mock_user_model
            .expect_create_user_activation_code()
            .with(predicate::eq(param_user_id_with))
            .times(calls)
            .returning(move |user_id| Box::pin(async move { fn_returning(user_id) }));
    }

    if let Some(MockUserModelCreateRecoverCode {
        calls,
        fn_returning,
        param_user_email_with,
    }) = expectations.create_recover_code
    {
        mock_user_model
            .expect_create_user_recover_code()
            .with(predicate::eq(param_user_email_with))
            .times(calls)
            .returning(move |user_email| Box::pin(async move { fn_returning(user_email) }));
    }

    if let Some(MockUserModelActivateUser {
        calls,
        param_user_id_with,
        param_code_key_with,
        fn_returning,
    }) = expectations.activate_user
    {
        mock_user_model
            .expect_activate_user()
            .with(
                predicate::eq(param_user_id_with),
                predicate::eq(param_code_key_with),
            )
            .times(calls)
            .returning(move |user_id, code_key| {
                Box::pin(async move { fn_returning(user_id, code_key) })
            });
    }

    if let Some(MockUserModelUpdatePassword {
        calls,
        param_id_with,
        param_new_password_with,
        param_old_password_with,
        fn_returning,
    }) = expectations.update_password
    {
        mock_user_model
            .expect_update_password()
            .with(
                predicate::eq(param_id_with),
                predicate::eq(param_new_password_with),
                predicate::eq(param_old_password_with),
            )
            .times(calls)
            .returning(move |user_id, new_password, old_password| {
                Box::pin(async move { fn_returning(user_id, new_password, old_password) })
            });
    }

    mock_user_model
}
