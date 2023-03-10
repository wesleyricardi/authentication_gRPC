use tonic::Status;
use uuid::Uuid;

use crate::{
    repositories::user_repository::{
        UserRepository, UserRepositoryMock, UserRepositoryStoreParams,
    },
    utils::hash::password::{PasswordHasher, PasswordVerify, PASSWORD_HASHER, PASSWORD_VERIFY},
};

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

pub trait UserModel {
    fn create(&self, user: UserModelCreateParams) -> Result<UserModelInsertReturn, Status>;
    fn login_verification(
        &self,
        username: String,
        password: String,
    ) -> Result<UserModelLoginVerificationReturn, Status>;
}

pub struct UserModelImpl<R> {
    user_repository: R,
    password_hasher: PasswordHasher,
    password_verify: PasswordVerify,
}

pub type DefaultUserModel = UserModelImpl<UserRepositoryMock>;
pub fn get_default_user_model() -> DefaultUserModel {
    UserModelImpl {
        user_repository: UserRepositoryMock,
        password_hasher: PASSWORD_HASHER,
        password_verify: PASSWORD_VERIFY,
    }
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
}

pub struct UserModelMock;
impl UserModel for UserModelMock {
    fn create(&self, user: UserModelCreateParams) -> Result<UserModelInsertReturn, Status> {
        assert!(!user.username.is_empty());
        assert!(!user.email.is_empty());
        assert!(!user.password.is_empty());

        let id = "UUIDV4".to_string();

        let repository = UserRepositoryMock;
        let user = repository.store(UserRepositoryStoreParams {
            id,
            username: user.username,
            email: user.email,
            password: user.password,
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
        _password: String,
    ) -> Result<UserModelLoginVerificationReturn, Status> {
        let repository = UserRepositoryMock;
        let user = repository.consult_by_username(username)?;

        Ok(UserModelLoginVerificationReturn {
            id: user.id,
            username: user.username,
            email: user.email,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        repositories::user_repository::UserRepositoryMock,
        utils::hash::password::{PASSWORD_HASHER_STUP, PASSWORD_VERIFY_STUP},
    };

    use super::*;

    fn get_mocked_model() -> UserModelImpl<UserRepositoryMock> {
        UserModelImpl {
            user_repository: UserRepositoryMock,
            password_hasher: PASSWORD_HASHER_STUP,
            password_verify: PASSWORD_VERIFY_STUP,
        }
    }

    #[test]
    fn test_insert() {
        let model = get_mocked_model();

        let response = model
            .create(UserModelCreateParams {
                username: "username".to_string(),
                email: "test@email.com".to_string(),
                password: "password".to_string(),
            })
            .unwrap();

        assert_eq!(response.id.is_empty(), false);
        assert_eq!(response.username, "username".to_string());
        assert_eq!(response.email, "email".to_string())
    }

    #[test]
    fn test_login_verification() {
        let model = get_mocked_model();

        let response = model
            .create(UserModelCreateParams {
                username: "username2".to_string(),
                email: "test2@email.com".to_string(),
                password: "password".to_string(),
            })
            .unwrap();

        let user = model
            .user_repository
            .consult_by_username(response.username)
            .unwrap();

        assert_eq!(user.id, response.id);
    }
}
