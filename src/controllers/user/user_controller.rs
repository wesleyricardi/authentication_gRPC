pub use crate::{
    dtos::controllers::dtos_controller_user::*,
    dtos::views::dtos_view_user::*,
    models::user::user_model::{UserModel, UserModelCreateParams},
    security::jwt::{JwtEncode, UserToken, JWT_ENCODE},
    services::sanitizer::user_input::sanitize_user_input::{SanitizeUser, SanitizeUserImpl},
};
use crate::{
    error::AppError, models::user::user_model::UserModelUpdateParams, security::jwt::JwtDecode,
};

pub trait UserController {
    fn register<T>(
        &self,
        req: RegisterParams,
        view: fn(user: UserViewArg, token: String) -> T,
    ) -> Result<T, AppError>;
    fn login<T>(
        &self,
        req: LoginParams,
        view: fn(user: UserViewArg, token: String) -> T,
    ) -> Result<T, AppError>;
    fn update<T>(
        &self,
        token: String,
        req: UpdateParams,
        view: fn(user: UserViewArg) -> T,
    ) -> Result<T, AppError>;
}

pub struct UserControllerImpl<M, S> {
    pub model: M,
    pub sanitize_user: S,
    pub jwt_encode: JwtEncode,
    pub jwt_decode: JwtDecode,
}

impl<M: UserModel, S: SanitizeUser> UserController for UserControllerImpl<M, S> {
    fn register<T>(
        &self,
        req: RegisterParams,
        view: fn(user: UserViewArg, token: String) -> T,
    ) -> Result<T, AppError> {
        let username_sanitized = self.sanitize_user.sanitize_username_input(req.username)?;
        let email_sanitized = self.sanitize_user.sanitize_email_input(req.email)?;
        let password_sanitized = self.sanitize_user.sanitize_password_input(req.password)?;

        let user = self.model.create(UserModelCreateParams {
            username: username_sanitized,
            email: email_sanitized,
            password: password_sanitized,
        })?;

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
        req: LoginParams,
        view: fn(user: UserViewArg, token: String) -> T,
    ) -> Result<T, AppError> {
        let username_sanitized = self.sanitize_user.sanitize_username_input(req.username)?;
        let password_sanitized = self.sanitize_user.sanitize_password_input(req.password)?;

        let user = self
            .model
            .login_verification(username_sanitized, password_sanitized)?;

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

    fn update<T>(
        &self,
        token: String,
        req: UpdateParams,
        view: fn(user: UserViewArg) -> T,
    ) -> Result<T, AppError> {
        let username_sanitized = match req.username {
            Some(username) => match self.sanitize_user.sanitize_username_input(username) {
                Ok(username) => Some(username),
                Err(_) => None,
            },
            None => None,
        };

        let email_sanitized = match req.email {
            Some(email) => match self.sanitize_user.sanitize_email_input(email) {
                Ok(email) => Some(email),
                Err(_) => None,
            },
            None => None,
        };

        let password_sanitized = match req.password {
            Some(password) => match self.sanitize_user.sanitize_password_input(password) {
                Ok(password) => Some(password),
                Err(_) => None,
            },
            None => None,
        };

        let jwt_decoded = (self.jwt_decode)(&token)?;

        let UserToken {
            id,
            username: _,
            email: _,
        } = jwt_decoded.user;

        let user = self.model.update(
            id,
            UserModelUpdateParams {
                username: username_sanitized,
                email: email_sanitized,
                password: password_sanitized,
            },
        )?;

        Ok(view(UserViewArg {
            id: user.id,
            username: user.username,
            email: user.email,
        }))
    }
}
