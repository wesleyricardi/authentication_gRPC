pub use crate::{
    dtos::models::dtos_model_user::*,
    repositories::user::user_repository::{UserRepository, UserRepositoryStoreParams},
    utils::hash::password::{PasswordHasher, PasswordVerify, PASSWORD_HASHER, PASSWORD_VERIFY},
};
use crate::{
    error::*,
    repositories::{
        user::user_repository::UserRepositoryUpdateParams,
        users_code::users_code_repository::{UsersCode, UsersCodeRepository},
    },
};
use async_trait::async_trait;
use chrono::{Duration, Utc};
use mockall::automock;

#[derive(Debug, PartialEq)]
pub enum CodeType {
    Activation,
    Recovery,
}

#[async_trait]
#[automock]
pub trait AuthenticationModel: Sync + Send {
    async fn create(&self, user: UserModelCreateParams) -> Result<UserModelInsertReturn, AppError>;
    async fn login_verification(
        &self,
        username: String,
        password: String,
    ) -> Result<UserModelLoginVerificationReturn, AppError>;
    async fn recover_user_data(
        &self,
        id: String,
    ) -> Result<UserModelRecoverUserDataReturn, AppError>;
    async fn update(&self, id: String, user: UserModelUpdateParams) -> Result<String, AppError>;
    async fn create_user_code(
        &self,
        user_id: String,
        code_type: CodeType,
    ) -> Result<String, AppError>;
    async fn active_user(&self, user_id: String, code_key: String) -> Result<String, AppError>;
}

pub struct UserModel<R, C> {
    pub user_repository: R,
    pub user_code_repository: C,
    pub password_hasher: PasswordHasher,
    pub password_verify: PasswordVerify,
    pub new_id: fn() -> String,
    pub generate_code: fn() -> String,
}

#[async_trait]
impl<R: UserRepository, C: UsersCodeRepository> AuthenticationModel for UserModel<R, C> {
    async fn create(&self, user: UserModelCreateParams) -> Result<UserModelInsertReturn, AppError> {
        let id = (self.new_id)();
        let hashed_password = (self.password_hasher)(user.password)?;

        let user = self
            .user_repository
            .store(UserRepositoryStoreParams {
                id,
                username: user.username,
                email: user.email,
                password: hashed_password,
            })
            .await?;

        Ok(UserModelInsertReturn {
            id: user.id,
            username: user.username,
            email: user.email,
            activated: user.activated,
            blocked: user.blocked,
        })
    }
    async fn login_verification(
        &self,
        username: String,
        password: String,
    ) -> Result<UserModelLoginVerificationReturn, AppError> {
        let user = self.user_repository.consult_by_username(username).await?;

        if !(self.password_verify)(user.password, password)? {
            return Err(AppError::new(Code::Unauthenticated, "Incorrect password"));
        }

        Ok(UserModelLoginVerificationReturn {
            id: user.id,
            username: user.username,
            email: user.email,
            activated: user.activated,
            blocked: user.blocked,
        })
    }

    async fn recover_user_data(
        &self,
        id: String,
    ) -> Result<UserModelRecoverUserDataReturn, AppError> {
        let user = self.user_repository.consult_by_id(id).await?;

        Ok(UserModelRecoverUserDataReturn {
            username: user.username,
            email: user.email,
            activated: user.activated,
            blocked: user.blocked,
        })
    }

    async fn update(&self, id: String, user: UserModelUpdateParams) -> Result<String, AppError> {
        let password = match user.password {
            Some(password) => Some((self.password_hasher)(password)?),
            None => None,
        };

        let user_to_be_updated = UserRepositoryUpdateParams {
            username: user.username,
            email: user.email,
            password,
            activated: None,
            blocked: None,
        };

        self.user_repository
            .store_update(id, user_to_be_updated)
            .await
    }
    async fn create_user_code(
        &self,
        user_id: String,
        code_type: CodeType,
    ) -> Result<String, AppError> {
        let expire_minutes = match code_type {
            CodeType::Activation => 30, //minutes
            CodeType::Recovery => 30,   //minutes
        };

        let expire_at = Utc::now().naive_utc() + Duration::minutes(expire_minutes.into());

        let code_key = (self.generate_code)();

        let code = UsersCode {
            code: code_key.clone(),
            expire_at,
            user_id,
        };

        match self.user_code_repository.store(code).await {
            Ok(_) => Ok(String::from(code_key)),
            Err(error) => Err(error),
        }
    }

    async fn active_user(&self, user_id: String, code_key: String) -> Result<String, AppError> {
        let code = self
            .user_code_repository
            .get(user_id.clone(), code_key)
            .await
            .map_err(|error| match error.code {
                Code::NotFound => AppError::new(Code::NotFound, "Code not found"),
                _ => AppError::new(Code::Internal, "internal error"),
            })?;

        if code.expire_at < Utc::now().naive_utc() {
            return Err(AppError::new(Code::InvalidArgument, "Code expired"));
        }

        let user_to_be_updated = UserRepositoryUpdateParams {
            username: None,
            email: None,
            password: None,
            activated: Some(true),
            blocked: None,
        };

        self.user_repository
            .store_update(user_id, user_to_be_updated)
            .await?;

        Ok(String::from("User activated"))
    }
}
