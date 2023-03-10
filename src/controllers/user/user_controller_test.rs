#[cfg(test)]
mod tests {
    use crate::{
        controllers::user::user_controller::{
            LoginParams, RegisterParams, UpdateParams, UserController, UserControllerImpl,
            UserViewArg,
        },
        models::user::user_model_mock::UserModelMock,
        security::jwt::{JWT_DECODE_STUB, JWT_ENCODE_STUB},
        services::sanitizer::user_input::sanitize_user_input_mock::SanitizeUserMock,
    };

    fn get_controller_with_mocks_arg() -> UserControllerImpl<UserModelMock, SanitizeUserMock> {
        UserControllerImpl {
            model: UserModelMock,
            sanitize_user: SanitizeUserMock,
            jwt_encode: JWT_ENCODE_STUB,
            jwt_decode: JWT_DECODE_STUB,
        }
    }

    struct ViewStupReturn {
        user: UserViewArg,
        token: String,
    }

    fn view_stup(user: UserViewArg, token: String) -> ViewStupReturn {
        ViewStupReturn { user, token }
    }

    struct ViewStupUpdateReturn {
        user: UserViewArg,
    }

    fn view_update_stup(user: UserViewArg) -> ViewStupUpdateReturn {
        ViewStupUpdateReturn { user }
    }

    #[test]
    fn test_register() {
        let req = RegisterParams {
            username: "username".to_string(),
            email: "test@email.com".to_string(),
            password: "password".to_string(),
        };

        let controller = get_controller_with_mocks_arg();
        let ViewStupReturn { user, token } = controller.register(req, view_stup).unwrap();

        assert_eq!(user.id.is_empty(), false);
        assert_eq!(user.username, "username".to_string());
        assert_eq!(user.email, "test@email.com".to_string());
        assert_eq!(token.is_empty(), false);
    }

    #[test]
    fn test_login() {
        let req = RegisterParams {
            username: "username2".to_string(),
            email: "test2@email.com".to_string(),
            password: "password".to_string(),
        };

        let controller = get_controller_with_mocks_arg();
        let ViewStupReturn { user, token: _ } = controller.register(req, view_stup).unwrap();

        let req = LoginParams {
            username: user.username.clone(),
            password: "password".to_string(),
        };

        let response = controller.login(req, view_stup).unwrap();

        assert_eq!(response.user.id, user.id);
        assert_eq!(response.user.username, user.username);
        assert_eq!(response.user.email, user.email);
        assert_eq!(response.token.is_empty(), false);
    }

    #[test]
    fn test_update() {
        let req = RegisterParams {
            username: "username3".to_string(),
            email: "test3@email.com".to_string(),
            password: "password".to_string(),
        };

        let controller = get_controller_with_mocks_arg();
        let ViewStupReturn { user, token: _ } = controller.register(req, view_stup).unwrap();

        let req = UpdateParams {
            username: Some("username_update".to_string()),
            email: Some("test_update@email.com".to_string()),
            password: Some("password_update".to_string()),
        };

        controller
            .update(user.id.clone(), req, view_update_stup)
            .unwrap();

        let req = LoginParams {
            username: "username_update".to_string(),
            password: "password_update".to_string(),
        };

        let response = controller.login(req, view_stup).unwrap();

        assert_eq!(response.user.id, user.id);
        assert_eq!(response.user.username, "username_update".to_string());
        assert_eq!(response.user.email, "test_update@email.com".to_string());
        assert_eq!(response.token.is_empty(), false);
    }
}
