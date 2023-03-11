pub use crate::{
    models::user::user_model::{UserModel, UserModelCreateParams},
    rpc::authentication::authentication::{ReqLogin, ReqRegister},
    security::jwt::{JwtEncode, UserToken, JWT_ENCODE},
    services::sanitizer::user_input::sanitize_user_input::{
        RegisterInputDirty, SanitizeUser, SanitizeUserImpl,
    },
    views::user_view::UserViewArg,
};
pub use tonic::Status;

pub trait UserController {
    fn register<T>(
        &self,
        req: ReqRegister,
        view: fn(user: UserViewArg, token: String) -> T,
    ) -> Result<T, Status>;
    fn login<T>(
        &self,
        req: ReqLogin,
        view: fn(user: UserViewArg, token: String) -> T,
    ) -> Result<T, Status>;
}

pub struct UserControllerImpl<M, S> {
    pub model: M,
    pub sanitize_user: S,
    pub jwt_encode: JwtEncode,
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

        let user = match self.model.create(UserModelCreateParams {
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

    fn login<T>(
        &self,
        req: ReqLogin,
        view: fn(user: UserViewArg, token: String) -> T,
    ) -> Result<T, Status> {
        let req_sanitized = self
            .sanitize_user
            .login_sanitize(req.username, req.password)?;

        let user = match self
            .model
            .login_verification(req_sanitized.username, req_sanitized.password)
        {
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
