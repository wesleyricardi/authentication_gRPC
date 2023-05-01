use authentication_gRPC::models::authentication_model::AuthenticationModel;

use crate::{
    mocks::user_repository_mock::{
        get_mock_user_repository, MockUserRepositoryDelete, MockUserRepositoryParams,
    },
    utils::builders::UserModelBuilderForTest,
};

#[tokio::test]
async fn test_delete_user() {
    const FAKE_ID: &str = "userFakeId";

    let mock_user_repository = get_mock_user_repository(MockUserRepositoryParams {
        delete: Some(MockUserRepositoryDelete {
            calls: 1,
            param_id_with: FAKE_ID.to_string(),
            fn_returning: |_| Ok(String::from("")),
        }),
        ..Default::default()
    });

    let model_user = UserModelBuilderForTest::new()
        .mount_user_repository(mock_user_repository)
        .build();

    let response = model_user.delete_user(FAKE_ID.to_string()).await.unwrap();

    assert_eq!(response, "User deleted successfully");
}
