use std::sync::Mutex;

pub use super::user_repository::*;

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

    fn consult_by_username(&self, username: String) -> Result<UserRepositoryConsultReturn, Status> {
        let stored_user = unsafe { &mut STORED_USERS.lock().unwrap() };
        let result = match stored_user.iter().find(|user| user.username == username) {
            Some(user) => user,
            None => return Err(Status::not_found("User not found")),
        };

        Ok(UserRepositoryConsultReturn {
            id: result.id.clone(),
            username: result.username.clone(),
            email: result.email.clone(),
            password: result.password.clone(),
        })
    }
}
