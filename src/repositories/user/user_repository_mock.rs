pub use super::user_repository::*;

pub struct UserStored {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password: String,
}
type UsersStored = Vec<UserStored>;
static mut STORE: UsersStored = Vec::new();
pub struct UserRepositoryMock;

impl UserRepository for UserRepositoryMock {
    fn store(&self, user: UserRepositoryStoreParams) -> UserRepositoryStoreReturn {
        unsafe {
            STORE.push(UserStored {
                id: user.id,
                username: user.username,
                email: user.email,
                password: user.password,
            });

            let result = STORE.get(0).unwrap();

            UserRepositoryStoreReturn {
                id: result.id.clone(),
                username: result.username.clone(),
                email: result.email.clone(),
            }
        }
    }

    fn consult_by_username(&self, username: String) -> Result<UserRepositoryConsultReturn, Status> {
        let result = match unsafe { STORE.iter().find(|user| user.username == username) } {
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
