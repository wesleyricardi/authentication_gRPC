pub use crate::dtos::repositories::dtos_repository_user::*;
pub use tonic::Status;

pub trait UserRepository {
    fn store(&self, user: UserRepositoryStoreParams) -> UserRepositoryStoreReturn;
    fn consult_by_username(&self, username: String) -> Result<UserRepositoryConsultReturn, Status>;
    fn store_update(
        &self,
        id: String,
        user_to_be_updated: UserRepositoryUpdateParams,
    ) -> Result<UserRepositoryUpdateReturn, Status>;
}
