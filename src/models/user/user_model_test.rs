#[cfg(test)]
mod tests {
    use crate::{
        models::user::user_model::*,
        repositories::user::user_repository_mock::*,
        utils::hash::password::{PASSWORD_HASHER_STUP, PASSWORD_VERIFY_STUP},
    };

    fn get_mocked_model() -> UserModelImpl<UserRepositoryMock> {
        UserModelImpl {
            user_repository: UserRepositoryMock,
            password_hasher: PASSWORD_HASHER_STUP,
            password_verify: PASSWORD_VERIFY_STUP,
        }
    }

    #[test]
    fn test_insert() {
        let model = get_mocked_model();

        let response = model
            .create(UserModelCreateParams {
                username: "model_username".to_string(),
                email: "test@model.com".to_string(),
                password: "password".to_string(),
            })
            .unwrap();

        assert_eq!(response.id.is_empty(), false);
        assert_eq!(response.username, "model_username".to_string());
        assert_eq!(response.email, "test@model.com".to_string())
    }

    #[test]
    fn test_login_verification() {
        let model = get_mocked_model();

        let response = model
            .create(UserModelCreateParams {
                username: "model_username2".to_string(),
                email: "test2@model.com".to_string(),
                password: "password".to_string(),
            })
            .unwrap();

        let user = model
            .login_verification("model_username2".to_string(), "password".to_string())
            .unwrap();

        assert_eq!(user.id, response.id);
    }

    #[test]
    fn test_update() {
        let model = get_mocked_model();

        let created_user = model
            .create(UserModelCreateParams {
                username: "model_username".to_string(),
                email: "test@model.com".to_string(),
                password: "password".to_string(),
            })
            .unwrap();

        model
            .update(
                created_user.id.clone(),
                UserModelUpdateParams {
                    username: Some("model_usernameupdated".to_string()),
                    email: Some("updated@model.com".to_string()),
                    password: Some("changedpassword".to_string()),
                },
            )
            .unwrap();

        let response = model
            .login_verification(
                "model_usernameupdated".to_string(),
                "changedpassword".to_string(),
            )
            .unwrap();

        assert_eq!(response.id, created_user.id);
        assert_eq!(response.username, "model_usernameupdated".to_string());
        assert_eq!(response.email, "updated@model.com".to_string());
    }
}
