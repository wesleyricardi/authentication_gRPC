pub use super::user_repository::*;
use crate::error::*;
use std::sync::Mutex;

#[derive(Debug, Clone)]
pub struct UserStored {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password: String,
}
type UsersStored = Mutex<Vec<UserStored>>;
static mut STORED_USERS: UsersStored = Mutex::new(Vec::new());
pub struct UserRepositoryMock;

impl UserRepository for UserRepositoryMock {
    fn store(&self, user: UserRepositoryStoreParams) -> UserRepositoryStoreReturn {
        let stored_users = unsafe { &mut STORED_USERS.lock().unwrap() };

        stored_users.push(UserStored {
            id: user.id,
            username: user.username.clone(),
            email: user.email,
            password: user.password,
        });

        let result = stored_users
            .iter()
            .find(|result| result.username == user.username)
            .unwrap();

        UserRepositoryStoreReturn {
            id: result.id.clone(),
            username: result.username.clone(),
            email: result.email.clone(),
        }
    }

    fn consult_by_username(
        &self,
        username: String,
    ) -> Result<UserRepositoryConsultReturn, AppError> {
        let stored_user = unsafe { &mut STORED_USERS.lock().unwrap() };
        let result = match stored_user.iter().find(|user| user.username == username) {
            Some(user) => user,
            None => return Err(AppError::new(Code::NotFound, "User not found")),
        };

        Ok(UserRepositoryConsultReturn {
            id: result.id.clone(),
            username: result.username.clone(),
            email: result.email.clone(),
            password: result.password.clone(),
        })
    }

    fn store_update(
        &self,
        id: String,
        user_to_be_updated: UserRepositoryUpdateParams,
    ) -> Result<UserRepositoryUpdateReturn, AppError> {
        let stored_user = unsafe { &mut STORED_USERS.lock().unwrap() };

        let user = match stored_user.iter().find(|user| user.id == id) {
            Some(user) => user,
            None => return Err(AppError::new(Code::NotFound, "User not found")),
        };

        let user_prepared_to_be_updated = UserStored {
            id: user.id.clone(),
            username: user_to_be_updated
                .username
                .unwrap_or_else(|| user.username.clone()),
            email: user_to_be_updated
                .email
                .unwrap_or_else(|| user.email.clone()),
            password: user_to_be_updated
                .password
                .unwrap_or_else(|| user.password.clone()),
        };

        for user in stored_user.iter_mut() {
            if user.id == id {
                *user = user_prepared_to_be_updated.clone();
                break;
            }
        }

        Ok(UserRepositoryUpdateReturn {
            id: user_prepared_to_be_updated.id,
            username: user_prepared_to_be_updated.username,
            email: user_prepared_to_be_updated.email,
        })
    }
}
