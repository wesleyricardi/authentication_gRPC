use authentication_gRPC::{
    models::authentication_model::AuthenticationModel,
    repositories::user_repository::UserRepositoryConsultReturn,
};

use crate::{
    mocks::user_repository_mock::{
        get_mock_user_repository, MockUserRepositoryConsultById, MockUserRepositoryParams,
    },
    utils::builders::UserModelBuilderForTest,
};

#[tokio::test]
async fn test_user_model_recover_user_data() {
    const FAKE_ID: &str = "userFakeId";
    const FAKE_USERNAME: &str = "username";
    const FAKE_EMAIL: &str = "test@model.com";
    const FAKE_PASSWORD: &str = "password";

    let mock_user_repository = get_mock_user_repository(MockUserRepositoryParams {
        consult_by_id: Some(MockUserRepositoryConsultById {
            calls: 1,
            param_id_with: FAKE_ID.to_string(),
            fn_returning: |id| {
                Ok(UserRepositoryConsultReturn {
                    id,
                    username: FAKE_USERNAME.to_string(),
                    email: FAKE_EMAIL.to_string(),
                    password: FAKE_PASSWORD.to_string(),
                    activated: false,
                    blocked: false,
                })
            },
        }),
        ..Default::default()
    });

    let model_user = UserModelBuilderForTest::new()
        .mount_user_repository(mock_user_repository)
        .build();

    let user = model_user
        .recover_user_data(FAKE_ID.to_string())
        .await
        .unwrap();

    assert_eq!(user.username, FAKE_USERNAME);
    assert_eq!(user.email, FAKE_EMAIL);
}
