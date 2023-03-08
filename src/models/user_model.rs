use tonic::Status;
use uuid::Uuid;

use crate::utils::hash::password::{PasswordHasher, PASSWORD_HASHER};
pub trait UserModel {
    fn create(&self, user: InsertUser) -> Result<UserModelInsertReturn, Status>;
}

pub struct UserModelInsertReturn {
    pub id: String,
    pub username: String,
    pub email: String,
}

pub struct InsertUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub struct UserModelImpl {
    password_hasher: PasswordHasher,
}

pub type DefaultUserModel = UserModelImpl;
pub fn get_default_user_model() -> UserModelImpl {
    UserModelImpl {
        password_hasher: PASSWORD_HASHER,
    }
}

impl UserModel for UserModelImpl {
    fn create(&self, user: InsertUser) -> Result<UserModelInsertReturn, Status> {
        let id = Uuid::new_v4().to_string();
        let hashed_password = (self.password_hasher)(user.password)?;

        let user = UserModelInsertReturn {
            id,
            username: user.username,
            email: user.email,
        };
        Ok(user)
    }
}

pub struct UserModelMock;
impl UserModel for UserModelMock {
    fn create(&self, user: InsertUser) -> Result<UserModelInsertReturn, Status> {
        assert!(!user.username.is_empty());
        assert!(!user.email.is_empty());
        assert!(!user.password.is_empty());

        let id = "UUIDV4".to_string();

        let user = UserModelInsertReturn {
            id,
            username: user.username,
            email: user.email,
        };
        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::hash::password::PASSWORD_HASHER_STUP;

    use super::*;

    #[test]
    fn test_insert() {
        let model = UserModelImpl {
            password_hasher: PASSWORD_HASHER_STUP,
        };

        let response = model
            .create(InsertUser {
                username: "username".to_string(),
                email: "email".to_string(),
                password: "password".to_string(),
            })
            .unwrap();

        assert_eq!(response.id.is_empty(), false);
        assert_eq!(response.username, "username".to_string());
        assert_eq!(response.email, "email".to_string())
    }
}
