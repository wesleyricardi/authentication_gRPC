#[cfg(test)]
mod tests {
    use crate::repositories::user::user_repository::*;
    use crate::repositories::user::user_repository_mock::UserRepositoryMock;

    #[test]
    fn test_store() {
        let user_repository = UserRepositoryMock;

        let response = user_repository.store(UserRepositoryStoreParams {
            id: "UNIQUE_ID".to_string(),
            username: "username".to_string(),
            email: "test@email.com".to_string(),
            password: "password".to_string(),
        });

        assert_eq!(response.id, "UNIQUE_ID".to_string());
        assert_eq!(response.username, "username".to_string());
        assert_eq!(response.email, "test@email.com".to_string());
    }

    #[test]
    fn test_consult_by_username() {
        let user_repository = UserRepositoryMock;

        user_repository.store(UserRepositoryStoreParams {
            id: "UNIQUE_ID2".to_string(),
            username: "username2".to_string(),
            email: "test2@email.com".to_string(),
            password: "password".to_string(),
        });

        let response = user_repository
            .consult_by_username("username2".to_string())
            .unwrap();

        assert_eq!(response.id, "UNIQUE_ID2".to_string());
        assert_eq!(response.username, "username2".to_string());
        assert_eq!(response.email, "test2@email.com".to_string());
        assert_eq!(response.password, "password".to_string());
    }
}
