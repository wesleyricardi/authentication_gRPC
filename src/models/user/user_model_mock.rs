use crate::repositories::user::user_repository_mock::*;

use super::user_model::*;

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
