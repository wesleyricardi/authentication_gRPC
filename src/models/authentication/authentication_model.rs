pub use crate::{
    dtos::models::dtos_model_user::*,
    repositories::user::user_repository::{UserRepository, UserRepositoryStoreParams},
    utils::hash::password::{PasswordHasher, PasswordVerify, PASSWORD_HASHER, PASSWORD_VERIFY},
};
use crate::{error::*, repositories::user::user_repository::UserRepositoryUpdateParams};
use async_trait::async_trait;
use mockall::automock;

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
    async fn update(
        &self,
        id: String,
        user: UserModelUpdateParams,
    ) -> Result<String, AppError>;
}

pub struct UserModel<R> {
    pub user_repository: R,
    pub password_hasher: PasswordHasher,
    pub password_verify: PasswordVerify,
    pub new_id: fn() -> String,
}

#[async_trait]
impl<R: UserRepository> AuthenticationModel for UserModel<R> {
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

    async fn update(
        &self,
        id: String,
        user: UserModelUpdateParams,
    ) -> Result<String, AppError> {
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
}
