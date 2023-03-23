use authentication_gRPC::{
    error::*,
    models::user::user_model::{
        MockUserModel, UserModelCreateParams, UserModelInsertReturn,
        UserModelLoginVerificationReturn, UserModelRecoverUserDataReturn, UserModelUpdateParams,
        UserModelUpdateReturn,
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
    pub fn_returning: fn(String, UserModelUpdateParams) -> Result<UserModelUpdateReturn, AppError>,
}

#[derive(Default)]
pub struct MockUserModelParams {
    pub create: Option<MockUserModelCreate>,
    pub login_verification: Option<MockUserModelLoginVerification>,
    pub recover_user_data: Option<MockUserModelRecoverUserData>,
    pub update: Option<MockUserModelUpdate>,
}

pub fn get_mock_user_model(expectations: MockUserModelParams) -> MockUserModel {
    let mut mock_user_model = MockUserModel::new();

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
            .returning(fn_returning);
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
            .returning(fn_returning);
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
            .returning(fn_returning);
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
            .returning(fn_returning);
    }
    mock_user_model
}
