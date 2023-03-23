use mockall::automock;

pub use crate::dtos::repositories::dtos_repository_user::*;
use crate::error::*;

#[automock]
pub trait UserRepository {
    fn store(&self, user: UserRepositoryStoreParams) -> UserRepositoryStoreReturn;
    fn consult_by_username(
        &self,
        username: String,
    ) -> Result<UserRepositoryConsultReturn, AppError>;
    fn consult_by_id(&self, id: String) -> Result<UserRepositoryConsultReturn, AppError>;
    fn store_update(
        &self,
        id: String,
        user_to_be_updated: UserRepositoryUpdateParams,
    ) -> Result<UserRepositoryUpdateReturn, AppError>;
}
