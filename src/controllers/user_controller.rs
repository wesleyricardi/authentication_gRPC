use crate::{
    models::user_model::{get_default_user_model, DefaultUserModel, InsertUser, UserModel},
    rpc::authentication::authentication::ReqRegister,
    security::jwt::{JwtEncode, UserToken, JWT_ENCODE},
    services::sanitizer::user_input::{RegisterInputDirty, SanitizeUser, SanitizeUserImpl},
    views::user_view::UserViewArg,
};
use tonic::Status;

pub trait UserController {
    fn register<T>(
        &self,
        req: ReqRegister,
        view: fn(user: UserViewArg, token: String) -> T,
    ) -> Result<T, Status>;
}

pub struct UserControllerImpl<M, S> {
    model: M,
    sanitize_user: S,
    jwt_encode: JwtEncode,
}

pub type DefaultUserController = UserControllerImpl<DefaultUserModel, SanitizeUserImpl>;
pub fn get_default_user_controller() -> DefaultUserController {
    UserControllerImpl {
        model: get_default_user_model(),
        sanitize_user: SanitizeUserImpl,
        jwt_encode: JWT_ENCODE,
    }
}

impl<M: UserModel, S: SanitizeUser> UserController for UserControllerImpl<M, S> {
    fn register<T>(
        &self,
        req: ReqRegister,
        view: fn(user: UserViewArg, token: String) -> T,
    ) -> Result<T, Status> {
        let req_sanitized = self.sanitize_user.register_sanitize(RegisterInputDirty {
            username: req.username,
            email: req.email,
            password: req.password,
        })?;

        let user = match self.model.create(InsertUser {
            username: req_sanitized.username,
            email: req_sanitized.email,
            password: req_sanitized.password,
        }) {
            Ok(user) => user,
            Err(error) => return Err(error),
        };

        let token = (self.jwt_encode)(UserToken {
            id: user.id.clone(),
            username: user.username.clone(),
            email: user.email.clone(),
        })?;

        Ok(view(
            UserViewArg {
                id: user.id,
                username: user.username,
                email: user.email,
            },
            token,
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        models::user_model::UserModelMock, security::jwt::JWT_ENCODE_STUB,
        services::sanitizer::user_input::SanitizeUserMock,
    };

    use super::*;

    fn get_controller_with_mocks_arg() -> UserControllerImpl<UserModelMock, SanitizeUserMock> {
        UserControllerImpl {
            model: UserModelMock,
            sanitize_user: SanitizeUserMock,
            jwt_encode: JWT_ENCODE_STUB,
        }
    }

    #[test]
    fn test_register() {
        let req = ReqRegister {
            username: "username".to_string(),
            email: "test@email.com".to_string(),
            password: "password".to_string(),
        };

        struct ViewStupReturn {
            user: UserViewArg,
            token: String,
        }

        fn view_stup(user: UserViewArg, token: String) -> ViewStupReturn {
            ViewStupReturn { user, token }
        }

        let controller = get_controller_with_mocks_arg();
        let ViewStupReturn { user, token } = controller.register(req, view_stup).unwrap();

        assert_eq!(user.id.is_empty(), false);
        assert_eq!(user.username, "username".to_string());
        assert_eq!(user.email, "test@email.com".to_string());
        assert_eq!(token.is_empty(), false);
    }
}
