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

pub struct UserRepositoryUpdateParams {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

pub struct UserRepositoryUpdateReturn {
    pub id: String,
    pub username: String,
    pub email: String,
}

pub trait UserRepository {
    fn store(&self, user: UserRepositoryStoreParams) -> UserRepositoryStoreReturn;
    fn consult_by_username(&self, username: String) -> Result<UserRepositoryConsultReturn, Status>;
    fn store_update(
        &self,
        id: String,
        user_to_be_updated: UserRepositoryUpdateParams,
    ) -> Result<UserRepositoryUpdateReturn, Status>;
}
