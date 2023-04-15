use authentication_gRPC::{
    error::*,
    models::user::user_model::{UserModel, UserModelImpl},
    repositories::user::user_repository::UserRepositoryConsultReturn,
};

use crate::mocks::user_repository_mock::{
    get_mock_user_repository, MockUserRepositoryConsultByUsername, MockUserRepositoryParams,
};

const FAKE_ID: &str = "userFakeId";
const FAKE_USERNAME: &str = "username";
const FAKE_EMAIL: &str = "test@model.com";
const FAKE_PASSWORD: &str = "password";
const FAKE_HASH_PASSWORD: &str = "hash_password";

#[tokio::test]
async fn test_login_verification() {
    let expectations_of_the_methods_that_will_be_used = MockUserRepositoryParams {
        consult_by_username: Some(MockUserRepositoryConsultByUsername {
            calls: 1,
            param_username_with: FAKE_USERNAME.to_string(),
            fn_returning: mock_user_reposory_consult_by_username,
        }),
        ..Default::default()
    };

    let model = UserModelImpl {
        user_repository: get_mock_user_repository(expectations_of_the_methods_that_will_be_used),
        password_hasher: mock_password_hasher_with_returning_error_if_called,
        password_verify: mock_verify_password,
        new_id: mock_new_id_with_panic_if_called,
    };

    let user = model
        .login_verification(FAKE_USERNAME.to_string(), FAKE_PASSWORD.to_string())
        .await
        .unwrap();

    assert_eq!(user.id, FAKE_ID);
    assert_eq!(user.email, FAKE_EMAIL);
}

#[tokio::test]
async fn test_login_verification_givin_wrong_password() {
    const WRONG_PASSWORD: &str = "Wrong password";

    let expectations_of_the_methods_that_will_be_used = MockUserRepositoryParams {
        consult_by_username: Some(MockUserRepositoryConsultByUsername {
            calls: 1,
            param_username_with: FAKE_USERNAME.to_string(),
            fn_returning: mock_user_reposory_consult_by_username,
        }),
        ..Default::default()
    };

    let model = UserModelImpl {
        user_repository: get_mock_user_repository(expectations_of_the_methods_that_will_be_used),
        password_hasher: mock_password_hasher_with_returning_error_if_called,
        password_verify: mock_verify_password,
        new_id: mock_new_id_with_panic_if_called,
    };

    match model
        .login_verification(FAKE_USERNAME.to_string(), WRONG_PASSWORD.to_string())
        .await
    {
        Ok(_) => panic!("verification should fail"),
        Err(error) => assert_eq!(error.message, "Incorrect password"),
    };
}

fn mock_user_reposory_consult_by_username(
    username: String,
) -> Result<UserRepositoryConsultReturn, AppError> {
    if FAKE_USERNAME != username {
        return Err(AppError::new(
            Code::NotFound,
            "not found the user with the given username",
        ));
    }
    Ok(UserRepositoryConsultReturn {
        id: FAKE_ID.to_string(),
        username,
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

fn mock_new_id_with_panic_if_called() -> String {
    panic!("cannot be called on this test")
}

fn mock_verify_password(hash_password: String, password: String) -> Result<bool, AppError> {
    if FAKE_HASH_PASSWORD != hash_password && FAKE_PASSWORD != password {
        return Ok(false);
    }
    return Ok(true);
}
