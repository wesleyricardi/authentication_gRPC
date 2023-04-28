use authentication_gRPC::{
    models::authentication::authentication_model::AuthenticationModel,
    repositories::{
        user::user_repository::UserRepositoryConsultReturn,
        users_code::users_code_repository::UsersCode,
    },
};
use chrono::Utc;

use crate::{
    mocks::{
        user_repository_mock::{
            get_mock_user_repository, MockUserRepositoryConsultByEmail, MockUserRepositoryParams,
        },
        users_code_repository_mock::{
            get_mock_users_code_repository, MockUsersCodeRepositoryParams,
            MockUsersCodeRepositoryStore,
        },
    },
    utils::builders::UserModelBuilderForTest,
};

#[tokio::test]
async fn test_user_model_code_by_email() {
    const FAKE_NEW_PASSWORD: &str = "newPassword";
    const FAKE_ID: &str = "userFakeId";
    const FAKE_USERNAME: &str = "userFakeUsername";
    const FAKE_EMAIL: &str = "test@email.com";
    const FAKE_CODE: &str = "000001";

    fn param_code_withf(code: &UsersCode) -> bool {
        code.code == FAKE_CODE.to_string()
            && code.user_id == FAKE_ID.to_string()
            && code.expire_at >= Utc::now().naive_utc()
    }

    let mock_users_code_repository =
        get_mock_users_code_repository(MockUsersCodeRepositoryParams {
            store: Some(MockUsersCodeRepositoryStore {
                calls: 1,
                param_code_withf: param_code_withf,
                fn_returning: |_| Ok(String::from("Code store successfully")),
            }),
            ..Default::default()
        });

    let mock_repository = get_mock_user_repository(MockUserRepositoryParams {
        consult_by_email: Some(MockUserRepositoryConsultByEmail {
            calls: 1,
            param_email_with: FAKE_EMAIL.to_string(),
            fn_returning: |_| {
                Ok(UserRepositoryConsultReturn {
                    id: FAKE_ID.to_string(),
                    username: FAKE_USERNAME.to_string(),
                    email: FAKE_EMAIL.to_string(),
                    password: FAKE_NEW_PASSWORD.to_string(),
                    activated: true,
                    blocked: false,
                })
            },
        }),
        ..Default::default()
    });

    let model_user = UserModelBuilderForTest::new()
        .mount_user_repository(mock_repository)
        .mount_generate_code(|| FAKE_CODE.to_string())
        .mount_code_repository(mock_users_code_repository)
        .build();

    let code = model_user
        .create_code_by_email(FAKE_EMAIL.to_string())
        .await
        .unwrap();

    assert_eq!(code, FAKE_CODE);
}
