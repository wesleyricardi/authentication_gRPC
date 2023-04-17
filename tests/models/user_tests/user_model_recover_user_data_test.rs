use authentication_gRPC::{
    error::*,
    models::authentication::authentication_model::{AuthenticationModel, UserModel},
    repositories::user::user_repository::UserRepositoryConsultReturn,
};

use crate::mocks::user_repository_mock::{
    get_mock_user_repository, MockUserRepositoryConsultById, MockUserRepositoryParams,
};

const FAKE_ID: &str = "userFakeId";
const FAKE_USERNAME: &str = "username";
const FAKE_EMAIL: &str = "test@model.com";
const FAKE_PASSWORD: &str = "password";

#[tokio::test]
async fn test_user_model_recover_user_data() {
    let expectations_of_the_methods_that_will_be_used = MockUserRepositoryParams {
        consult_by_id: Some(MockUserRepositoryConsultById {
            calls: 1,
            param_id_with: FAKE_ID.to_string(),
            fn_returning: mock_user_reposory_consult_by_id,
        }),
        ..Default::default()
    };

    let model = UserModel {
        user_repository: get_mock_user_repository(expectations_of_the_methods_that_will_be_used),
        password_hasher: mock_password_hasher_with_returning_error_if_called,
        password_verify: mock_password_verify_with_returning_error_if_called,
        new_id: mock_new_id_with_panic_if_called,
    };

    let user = model.recover_user_data(FAKE_ID.to_string()).await.unwrap();

    assert_eq!(user.username, FAKE_USERNAME);
    assert_eq!(user.email, FAKE_EMAIL);
}

fn mock_user_reposory_consult_by_id(id: String) -> Result<UserRepositoryConsultReturn, AppError> {
    if FAKE_ID != id {
        return Err(AppError::new(
            Code::NotFound,
            "not found the user with the given id",
        ));
    }
    Ok(UserRepositoryConsultReturn {
        id,
        username: FAKE_USERNAME.to_string(),
        email: FAKE_EMAIL.to_string(),
        password: FAKE_PASSWORD.to_string(),
    })
}

fn mock_password_hasher_with_returning_error_if_called(_: String) -> Result<String, AppError> {
    Err(AppError::new(
        Code::Internal,
        "cannot be called on this test",
    ))
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

fn mock_new_id_with_panic_if_called() -> String {
    panic!("cannot be called on this test")
}
