use crate::error::*;
pub use crate::{
    database::connection::get_postgres_pool, dtos::repositories::dtos_repository_user::*,
};
use async_trait::async_trait;
use mockall::automock;

#[async_trait]
#[automock]
pub trait UserRepository: Sync + Send + 'static {
    async fn store(&self, user: UserRepositoryStoreParams) -> UserRepositoryStoreReturn;
    async fn consult_by_username(
        &self,
        username: String,
    ) -> Result<UserRepositoryConsultReturn, AppError>;
    async fn consult_by_id(&self, id: String) -> Result<UserRepositoryConsultReturn, AppError>;
    async fn store_update(
        &self,
        id: String,
        user_to_be_updated: UserRepositoryUpdateParams,
    ) -> Result<UserRepositoryUpdateReturn, AppError>;
}
pub struct UserRepositoryPostgres;

#[async_trait]
impl UserRepository for UserRepositoryPostgres {
    async fn store(&self, user: UserRepositoryStoreParams) -> UserRepositoryStoreReturn {
        let pool = get_postgres_pool().await;

        todo!();
    }

    async fn consult_by_username(
        &self,
        username: String,
    ) -> Result<UserRepositoryConsultReturn, AppError> {
        todo!();
    }

    async fn consult_by_id(&self, id: String) -> Result<UserRepositoryConsultReturn, AppError> {
        todo!();
    }

    async fn store_update(
        &self,
        id: String,
        user_to_be_updated: UserRepositoryUpdateParams,
    ) -> Result<UserRepositoryUpdateReturn, AppError> {
        todo!();
    }
}
