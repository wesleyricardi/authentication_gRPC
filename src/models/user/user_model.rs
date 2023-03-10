use crate::repositories::user::user_repository_mock::UserRepositoryUpdateParams;
pub use crate::{
    repositories::user::user_repository::{UserRepository, UserRepositoryStoreParams},
    utils::hash::password::{PasswordHasher, PasswordVerify, PASSWORD_HASHER, PASSWORD_VERIFY},
};
pub use tonic::Status;
use uuid::Uuid;

pub struct UserModelCreateParams {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub struct UserModelInsertReturn {
    pub id: String,
    pub username: String,
    pub email: String,
}

pub struct UserModelLoginVerificationReturn {
    pub id: String,
    pub username: String,
    pub email: String,
}

pub struct UserModelUpdateParams {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

pub struct UserModelUpdateReturn {
    pub id: String,
    pub username: String,
    pub email: String,
}

pub trait UserModel {
    fn create(&self, user: UserModelCreateParams) -> Result<UserModelInsertReturn, Status>;
    fn login_verification(
        &self,
        username: String,
        password: String,
    ) -> Result<UserModelLoginVerificationReturn, Status>;
    fn update(
        &self,
        id: String,
        user: UserModelUpdateParams,
    ) -> Result<UserModelUpdateReturn, Status>;
}

pub struct UserModelImpl<R> {
    pub user_repository: R,
    pub password_hasher: PasswordHasher,
    pub password_verify: PasswordVerify,
}

impl<R: UserRepository> UserModel for UserModelImpl<R> {
    fn create(&self, user: UserModelCreateParams) -> Result<UserModelInsertReturn, Status> {
        let id = Uuid::new_v4().to_string();
        let hashed_password = (self.password_hasher)(user.password)?;

        let user = self.user_repository.store(UserRepositoryStoreParams {
            id,
            username: user.username,
            email: user.email,
            password: hashed_password,
        });

        Ok(UserModelInsertReturn {
            id: user.id,
            username: user.username,
            email: user.email,
        })
    }
    fn login_verification(
        &self,
        username: String,
        password: String,
    ) -> Result<UserModelLoginVerificationReturn, Status> {
        let user = self.user_repository.consult_by_username(username)?;

        if !(self.password_verify)(user.password, password)? {
            return Err(Status::new(
                tonic::Code::Unauthenticated,
                "Incorrect password",
            ));
        }

        Ok(UserModelLoginVerificationReturn {
            id: user.id,
            username: user.username,
            email: user.email,
        })
    }

    fn update(
        &self,
        id: String,
        user: UserModelUpdateParams,
    ) -> Result<UserModelUpdateReturn, Status> {
        let password = match user.password {
            Some(password) => Some((self.password_hasher)(password)?),
            None => None,
        };

        let user_to_be_updated = UserRepositoryUpdateParams {
            username: user.username,
            email: user.email,
            password,
        };

        let user = self.user_repository.store_update(id, user_to_be_updated)?;

        Ok(UserModelUpdateReturn {
            id: user.id,
            username: user.username,
            email: user.email,
        })
    }
}
