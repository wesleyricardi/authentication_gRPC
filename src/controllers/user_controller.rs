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
        let user_model = UserModelImpl;
        let user = match user_model.insert(InsertUser {
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

        let user_view = UserViewImpl;
        Ok(user_view.render_res_register(
            UserViewArg {
                id: user.id,
                username: user.username,
                email: user.email
            },
            token
        ))
    }
}