use async_trait::async_trait;

pub use crate::{
    dtos::controllers::dtos_controller_user::*,
    dtos::views::dtos_view_user::*,
    models::authentication::authentication_model::{AuthenticationModel, UserModelCreateParams},
    security::jwt::{JwtEncode, UserToken, JWT_ENCODE},
    services::sanitizer::authentication_input::sanitize_authentication_input::{
        SanitizeAuthentication, SanitizeUser,
    },
};
use crate::{
    error::{AppError, Code},
    models::authentication::authentication_model::{
        CodeType, UserModelRecoverUserDataReturn, UserModelUpdateParams,
    },
    security::jwt::{JWTAuthenticateToken, JwtDecode},
};

#[async_trait]
pub trait AuthenticationController: Sync + Send {
    async fn register<T>(
        &self,
        req: RegisterParams,
        view: fn(user: UserViewArg, token: String) -> T,
    ) -> Result<T, AppError>;
    async fn login<T>(
        &self,
        req: LoginParams,
        view: fn(user: UserViewArg, token: String) -> T,
    ) -> Result<T, AppError>;
    async fn authenticate<T>(
        &self,
        token: String,
        view: fn(user: UserViewArg) -> T,
    ) -> Result<T, AppError>;
    async fn update<T>(
        &self,
        token: String,
        req: UpdateParams,
        view: fn(message: String) -> T,
    ) -> Result<T, AppError>;
    async fn send_activation_code<T>(
        &self,
        token: String,
        view: fn(message: String) -> T,
    ) -> Result<T, AppError>;
    async fn activate_user<T>(
        &self,
        token: String,
        code_key: String,
        view: fn(message: String) -> T,
    ) -> Result<T, AppError>;
}

pub struct UserController<M, S> {
    pub model: M,
    pub sanitize_user: S,
    pub send_email:
        fn(to_adress: String, subject: String, body: String) -> Result<String, AppError>,
    pub jwt_encode: JwtEncode,
    pub jwt_decode: JwtDecode,
}

#[async_trait]
impl<M: AuthenticationModel, S: SanitizeAuthentication> AuthenticationController
    for UserController<M, S>
{
    async fn register<T>(
        &self,
        req: RegisterParams,
        view: fn(user: UserViewArg, token: String) -> T,
    ) -> Result<T, AppError> {
        let username_sanitized = self.sanitize_user.sanitize_username_input(req.username)?;
        let email_sanitized = self.sanitize_user.sanitize_email_input(req.email)?;
        let password_sanitized = self.sanitize_user.sanitize_password_input(req.password)?;

        let user = self
            .model
            .create(UserModelCreateParams {
                username: username_sanitized,
                email: email_sanitized,
                password: password_sanitized,
            })
            .await?;

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
                activated: user.activated,
                blocked: user.blocked,
            },
            token,
        ))
    }

    async fn login<T>(
        &self,
        req: LoginParams,
        view: fn(user: UserViewArg, token: String) -> T,
    ) -> Result<T, AppError> {
        let username_sanitized = self.sanitize_user.sanitize_username_input(req.username)?;
        let password_sanitized = self.sanitize_user.sanitize_password_input(req.password)?;

        let user = self
            .model
            .login_verification(username_sanitized, password_sanitized)
            .await?;

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
                activated: user.activated,
                blocked: user.blocked,
            },
            token,
        ))
    }

    async fn authenticate<T>(
        &self,
        token: String,
        view: fn(user: UserViewArg) -> T,
    ) -> Result<T, AppError> {
        let JWTAuthenticateToken {
            user: user_token, ..
        } = (self.jwt_decode)(&token)?;

        let user = self.model.recover_user_data(user_token.id.clone()).await?;

        Ok(view(UserViewArg {
            id: user_token.id,
            username: user.username,
            email: user.email,
            activated: user.activated,
            blocked: user.blocked,
        }))
    }

    async fn update<T>(
        &self,
        token: String,
        req: UpdateParams,
        view: fn(message: String) -> T,
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

        let UserToken { id, .. } = jwt_decoded.user;

        let message = self
            .model
            .update(
                id,
                UserModelUpdateParams {
                    username: username_sanitized,
                    email: email_sanitized,
                    password: password_sanitized,
                },
            )
            .await?;

        Ok(view(message))
    }

    async fn send_activation_code<T>(
        &self,
        token: String,
        view: fn(message: String) -> T,
    ) -> Result<T, AppError> {
        let jwt_decoded = (self.jwt_decode)(&token)?;

        let UserToken { id, .. } = jwt_decoded.user;

        let UserModelRecoverUserDataReturn { email, .. } =
            self.model.recover_user_data(id.clone()).await?;

        let code = self
            .model
            .create_user_code(id, CodeType::Activation)
            .await?;

        let body = format!("<div>The activation code is {}</div>", code);

        match (self.send_email)(email, String::from("activation code"), body) {
            Ok(_) => Ok(view(String::from("Code send successufully"))),
            Err(_) => Err(AppError {
                code: Code::Internal,
                message: String::from("send email failed"),
            }),
        }
    }

    async fn activate_user<T>(
        &self,
        token: String,
        code_key: String,
        view: fn(message: String) -> T,
    ) -> Result<T, AppError> {
        let JWTAuthenticateToken { sub: user_id, .. } = (self.jwt_decode)(&token)?;

        self.model.active_user(user_id, code_key).await?;

        Ok(view(String::from("User activated successfully")))
    }
}
