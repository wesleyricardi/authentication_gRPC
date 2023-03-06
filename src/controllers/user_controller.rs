use tonic::{Response, Status};
use crate::{
    rpc::authentication::authentication::{ReqRegister, ResRegister}, 
    models::user_model::{UserModelImpl, UserModel, InsertUser},
    views::user_view::{UserViewImpl, UserView, UserViewArg}, security::jwt::{JwtEncode, JWT_ENCODE, UserToken}
};

pub trait UserController {
    fn register(&self, req: ReqRegister) -> Result<Response<ResRegister>, Status>;
}

pub struct UserControllerImpl<M, V> {
    model: M,
    view: V,
    jwt_encode: JwtEncode,
}

pub type DefaultUserController = UserControllerImpl<UserModelImpl, UserViewImpl>;
pub fn get_default_user_controller() -> DefaultUserController {
    UserControllerImpl { 
        model: UserModelImpl,
        view: UserViewImpl,
        jwt_encode: JWT_ENCODE
    }
}


impl <M: UserModel, V: UserView> UserController for UserControllerImpl<M, V> {
    fn register(&self, req: ReqRegister) -> Result<Response<ResRegister>, Status> {
        let user = match self.model.insert(InsertUser {
            username: req.username,
            email: req.email,
            password: req.password
        }) {
            Ok(user) => user,
            Err(error) => return Err(error)
        };
    
        let token = (self.jwt_encode)(UserToken {
            id: user.id.clone(),
            username: user.username.clone(),
            email: user.email.clone()
        })?;

        Ok(self.view.render_res_register(
            UserViewArg {
                id: user.id,
                username: user.username,
                email: user.email
            },
            token
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::{models::user_model::UserModelMock, security::jwt::JWT_ENCODE_STUB, views::user_view::UserViewMock};

    use super::*;

    fn get_controller_with_mocks_arg() -> UserControllerImpl<UserModelMock, UserViewMock> {
        UserControllerImpl { 
            model: UserModelMock,
            view: UserViewMock,
            jwt_encode: JWT_ENCODE_STUB
        }
    }

    #[test]
    fn test_register() {
        let req = ReqRegister {
            username: "username".to_string(),
            email: "test@email.com".to_string(),
            password: "password".to_string(),
        };

        let controller = get_controller_with_mocks_arg();
        let ResRegister {user, token} = controller.register(req).unwrap().into_inner();

        assert_eq!(user.clone().unwrap().id.is_empty(), false);
        assert_eq!(user.clone().unwrap().username, "username".to_string());
        assert_eq!(user.clone().unwrap().email, "test@email.com".to_string());
        assert_eq!(token.is_empty(), false);
    }
}
