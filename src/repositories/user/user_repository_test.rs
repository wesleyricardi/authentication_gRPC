#[cfg(test)]
mod tests {
    use crate::repositories::user::user_repository::*;
    use crate::repositories::user::user_repository_mock::UserRepositoryMock;

    #[test]
    fn test_store() {
        let user_repository = UserRepositoryMock;

        let response = user_repository.store(UserRepositoryStoreParams {
            id: "repository_user_id".to_string(),
            username: "repository_username".to_string(),
            email: "test@repository.com".to_string(),
            password: "password".to_string(),
        });

        assert_eq!(response.id, "repository_user_id".to_string());
        assert_eq!(response.username, "repository_username".to_string());
        assert_eq!(response.email, "test@repository.com".to_string());
    }

    #[test]
    fn test_consult_by_username() {
        let user_repository = UserRepositoryMock;

        user_repository.store(UserRepositoryStoreParams {
            id: "repository_user_id2".to_string(),
            username: "repository_username2".to_string(),
            email: "test2@repository.com".to_string(),
            password: "password".to_string(),
        });

        let response = user_repository
            .consult_by_username("repository_username2".to_string())
            .unwrap();

        assert_eq!(response.id, "repository_user_id2".to_string());
        assert_eq!(response.username, "repository_username2".to_string());
        assert_eq!(response.email, "test2@repository.com".to_string());
        assert_eq!(response.password, "password".to_string());
    }

    #[test]
    fn test_store_update() {
        let user_repository = UserRepositoryMock;

        user_repository.store(UserRepositoryStoreParams {
            id: "repository_user_id3".to_string(),
            username: "repository_username3".to_string(),
            email: "test3@repository.com".to_string(),
            password: "password".to_string(),
        });

        user_repository
            .store_update(
                "repository_user_id3".to_string(),
                UserRepositoryUpdateParams {
                    username: Some("updateduser".to_string()),
                    email: Some("updated@repository.com".to_string()),
                    password: Some("changedpassword".to_string()),
                },
            )
            .unwrap();

        let response = user_repository
            .consult_by_username("updateduser".to_string())
            .unwrap();

        assert_eq!(response.id, "repository_user_id3".to_string());
        assert_eq!(response.username, "updateduser".to_string());
        assert_eq!(response.email, "updated@repository.com".to_string());
        assert_eq!(response.password, "changedpassword".to_string());
    }
}
