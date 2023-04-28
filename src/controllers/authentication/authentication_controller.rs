use crate::dtos::controllers::dtos_controller_user::{
    UserControllerAuthenticationReturn, UserControllerLoginReturn, UserControllerRegisterReturn,
    UserControllerUpdatePasswordReq,
};
use async_trait::async_trait;

pub use crate::{
    dtos::controllers::dtos_controller_user::*,
    models::authentication::authentication_model::{AuthenticationModel, UserModelCreateParams},
    security::jwt::JwtEncode,
    services::sanitizer::authentication_input::sanitize_authentication_input::{
        SanitizeAuthentication, SanitizeUser,
    },
};
use crate::{
    error::{AppError, Code},
    models::authentication::authentication_model::{
        UserModelRecoverUserDataReturn, UserModelUpdateParams,
    },
    security::jwt::{JWTAuthenticateToken, JwtDecode},
};

#[async_trait]
pub trait AuthenticationController: Sync + Send {
    async fn register(&self, req: RegisterParams)
        -> Result<UserControllerRegisterReturn, AppError>;
    async fn login(&self, req: LoginParams) -> Result<UserControllerLoginReturn, AppError>;
    async fn authenticate(
        &self,
        token: String,
    ) -> Result<UserControllerAuthenticationReturn, AppError>;
    async fn update(&self, token: String, req: UpdateParams) -> Result<String, AppError>;
    async fn update_password(
        &self,
        token: String,
        req: UserControllerUpdatePasswordReq,
    ) -> Result<String, AppError>;
    async fn send_activation_code(&self, token: String) -> Result<String, AppError>;
    async fn activate_user(&self, token: String, code_key: String) -> Result<String, AppError>;
    async fn send_recover_code(&self, email: String) -> Result<String, AppError>;
    async fn recover_user_password(
        &self,
        req: UserControllerRecoverPasswordReq,
    ) -> Result<String, AppError>;
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
    async fn register(
        &self,
        req: RegisterParams,
    ) -> Result<UserControllerRegisterReturn, AppError> {
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

        let token = (self.jwt_encode)(user.id.clone(), user.activated, user.blocked)?;

        Ok(UserControllerRegisterReturn {
            user: UserResponse {
                id: user.id,
                username: user.username,
                email: user.email,
                activated: user.activated,
                blocked: user.blocked,
            },
            token,
        })
    }

    async fn login(&self, req: LoginParams) -> Result<UserControllerLoginReturn, AppError> {
        let username_sanitized = self.sanitize_user.sanitize_username_input(req.username)?;
        let password_sanitized = self.sanitize_user.sanitize_password_input(req.password)?;

        let user = self
            .model
            .login_verification(username_sanitized, password_sanitized)
            .await?;

        let token = (self.jwt_encode)(user.id.clone(), user.activated, user.blocked)?;

        Ok(UserControllerLoginReturn {
            user: UserResponse {
                id: user.id,
                username: user.username,
                email: user.email,
                activated: user.activated,
                blocked: user.blocked,
            },
            token,
        })
    }

    async fn authenticate(
        &self,
        token: String,
    ) -> Result<UserControllerAuthenticationReturn, AppError> {
        let JWTAuthenticateToken { sub: user_id, .. } = (self.jwt_decode)(&token)?;

        let user = self.model.recover_user_data(user_id.clone()).await?;

        Ok(UserControllerAuthenticationReturn {
            user: UserResponse {
                id: user_id,
                username: user.username,
                email: user.email,
                activated: user.activated,
                blocked: user.blocked,
            },
        })
    }

    async fn update(&self, token: String, req: UpdateParams) -> Result<String, AppError> {
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

        let JWTAuthenticateToken {
            sub: user_id,
            activated,
            blocked,
            ..
        } = (self.jwt_decode)(&token)?;

        if blocked {
            return Err(AppError::new(Code::PermissionDenied, "User are blocked"));
        }

        if !activated {
            return Err(AppError::new(Code::PermissionDenied, "User not activated"));
        }

        let message = self
            .model
            .update(
                user_id,
                UserModelUpdateParams {
                    username: username_sanitized,
                    email: email_sanitized,
                },
            )
            .await?;

        Ok(message)
    }

    async fn update_password(
        &self,
        token: String,
        req: UserControllerUpdatePasswordReq,
    ) -> Result<String, AppError> {
        let password_sanitized = self
            .sanitize_user
            .sanitize_password_input(req.new_password)?;
        let old_password_sanitized = self
            .sanitize_user
            .sanitize_password_input(req.old_password)?;

        let JWTAuthenticateToken {
            sub: user_id,
            activated,
            blocked,
            ..
        } = (self.jwt_decode)(&token)?;

        if blocked {
            return Err(AppError::new(Code::PermissionDenied, "User are blocked"));
        }

        if !activated {
            return Err(AppError::new(Code::PermissionDenied, "User not activated"));
        }

        let message = self
            .model
            .update_password(user_id, password_sanitized, old_password_sanitized)
            .await?;

        Ok(message)
    }

    async fn send_activation_code(&self, token: String) -> Result<String, AppError> {
        let JWTAuthenticateToken {
            sub: user_id,
            activated,
            ..
        } = (self.jwt_decode)(&token)?;

        if activated {
            return Err(AppError::new(
                Code::PermissionDenied,
                "User already activated",
            ));
        }

        let UserModelRecoverUserDataReturn { email, .. } =
            self.model.recover_user_data(user_id.clone()).await?;

        let code = self.model.create_user_activation_code(user_id).await?;

        let body = format!("<div>The activation code is {}</div>", code);

        match (self.send_email)(email, String::from("activation code"), body) {
            Ok(_) => Ok(String::from("Code send successufully")),
            Err(_) => Err(AppError {
                code: Code::Internal,
                message: String::from("send email failed"),
            }),
        }
    }

    async fn activate_user(&self, token: String, code_key: String) -> Result<String, AppError> {
        let JWTAuthenticateToken {
            sub: user_id,
            activated,
            ..
        } = (self.jwt_decode)(&token)?;

        if activated {
            return Err(AppError::new(
                Code::PermissionDenied,
                "User already activated",
            ));
        }

        self.model.activate_user(user_id, code_key).await?;

        Ok(String::from("User activated successfully"))
    }

    async fn send_recover_code(&self, email: String) -> Result<String, AppError> {
        let email_sanitized = self.sanitize_user.sanitize_email_input(email)?;

        let code = self
            .model
            .create_user_recover_code(email_sanitized.clone())
            .await?;

        let body = format!("<div>The recover code is {}</div>", code);

        match (self.send_email)(email_sanitized, String::from("recover code"), body) {
            Ok(_) => Ok(String::from("Code send successufully")),
            Err(_) => Err(AppError {
                code: Code::Internal,
                message: String::from("send email failed"),
            }),
        }
    }

    async fn recover_user_password(
        &self,
        req: UserControllerRecoverPasswordReq,
    ) -> Result<String, AppError> {
        let email_sanitized = self.sanitize_user.sanitize_email_input(req.email)?;
        let password_sanitized = self
            .sanitize_user
            .sanitize_password_input(req.new_password)?;

        self.model
            .recover_user_password(email_sanitized, password_sanitized, req.code_key)
            .await?;

        Ok(String::from("Password recovered successfully"))
    }
}
