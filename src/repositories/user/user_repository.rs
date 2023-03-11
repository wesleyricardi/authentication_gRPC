pub use tonic::Status;

pub struct UserRepositoryStoreParams {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password: String,
}

pub struct UserRepositoryStoreReturn {
    pub id: String,
    pub username: String,
    pub email: String,
}

pub struct UserRepositoryConsultReturn {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password: String,
}

pub trait UserRepository {
    fn store(&self, user: UserRepositoryStoreParams) -> UserRepositoryStoreReturn;
    fn consult_by_username(&self, username: String) -> Result<UserRepositoryConsultReturn, Status>;
}
