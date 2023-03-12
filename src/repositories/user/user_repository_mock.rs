pub use super::user_repository::*;

pub struct UserStored {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password: String,
}
type UsersStored = Vec<UserStored>;
static mut STORED_USERS: UsersStored = Vec::new();
pub struct UserRepositoryMock;

impl UserRepository for UserRepositoryMock {
    fn store(&self, user: UserRepositoryStoreParams) -> UserRepositoryStoreReturn {
        unsafe {
            STORED_USERS.push(UserStored {
                id: user.id,
                username: user.username.clone(),
                email: user.email,
                password: user.password,
            });

            let result = STORED_USERS
                .iter()
                .find(|result| result.username == user.username)
                .unwrap();

            UserRepositoryStoreReturn {
                id: result.id.clone(),
                username: result.username.clone(),
                email: result.email.clone(),
            }
        }
    }

    fn consult_by_username(&self, username: String) -> Result<UserRepositoryConsultReturn, Status> {
        let result = match unsafe { STORED_USERS.iter().find(|user| user.username == username) } {
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
