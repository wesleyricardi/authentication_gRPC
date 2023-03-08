use crate::{
    models::user_model::{InsertUser, UserModel, UserModelImpl},
    rpc::authentication::authentication::ReqRegister,
    security::jwt::{JwtEncode, UserToken, JWT_ENCODE},
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

pub struct UserControllerImpl<M> {
    model: M,
    jwt_encode: JwtEncode,
}

pub type DefaultUserController = UserControllerImpl<UserModelImpl>;
pub fn get_default_user_controller() -> DefaultUserController {
    UserControllerImpl {
        model: UserModelImpl,
        jwt_encode: JWT_ENCODE,
    }
}

impl<M: UserModel> UserController for UserControllerImpl<M> {
    fn register<T>(
        &self,
        req: ReqRegister,
        view: fn(user: UserViewArg, token: String) -> T,
    ) -> Result<T, Status> {
        let user = match self.model.insert(InsertUser {
            username: req.username,
            email: req.email,
            password: req.password,
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
    use crate::{models::user_model::UserModelMock, security::jwt::JWT_ENCODE_STUB};

    use super::*;

    fn get_controller_with_mocks_arg() -> UserControllerImpl<UserModelMock> {
        UserControllerImpl {
            model: UserModelMock,
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
