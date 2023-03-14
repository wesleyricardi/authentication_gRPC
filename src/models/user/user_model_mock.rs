use crate::{error::AppError, repositories::user::user_repository_mock::*};

use super::user_model::*;

pub struct UserModelMock;
impl UserModel for UserModelMock {
    fn create(&self, user: UserModelCreateParams) -> Result<UserModelInsertReturn, AppError> {
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
    ) -> Result<UserModelLoginVerificationReturn, AppError> {
        let repository = UserRepositoryMock;
        let user = repository.consult_by_username(username)?;

        Ok(UserModelLoginVerificationReturn {
            id: user.id,
            username: user.username,
            email: user.email,
        })
    }

    fn recover_user_data(&self, id: String) -> Result<UserModelRecoverUserDataReturn, AppError> {
        let repository = UserRepositoryMock;
        let user = repository.consult_by_id(id)?;

        Ok(UserModelRecoverUserDataReturn {
            username: user.username,
            email: user.email,
        })
    }

    fn update(
        &self,
        id: String,
        user: UserModelUpdateParams,
    ) -> Result<UserModelUpdateReturn, AppError> {
        let user_to_be_updated = UserRepositoryUpdateParams {
            username: user.username,
            email: user.email,
            password: user.password,
        };

        let repository = UserRepositoryMock;
        let user = repository.store_update(id, user_to_be_updated)?;

        Ok(UserModelUpdateReturn {
            id: user.id,
            username: user.username,
            email: user.email,
        })
    }
}
