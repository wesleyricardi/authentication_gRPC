pub use crate::{
    database::connection::get_postgres_pool, dtos::repositories::dtos_repository_user::*,
};
use crate::{error::*, utils::adapters::sqlx_error_to_app_error::sqlx_error_to_app_error};
use async_trait::async_trait;
use mockall::automock;
use sqlx::{Pool, Postgres};

#[async_trait]
#[automock]
pub trait UserRepository: Sync + Send {
    async fn store(
        &self,
        user: UserRepositoryStoreParams,
    ) -> Result<UserRepositoryStoreReturn, AppError>;
    async fn consult_by_username(
        &self,
        username: String,
    ) -> Result<UserRepositoryConsultReturn, AppError>;
    async fn consult_by_id(&self, id: String) -> Result<UserRepositoryConsultReturn, AppError>;
    async fn consult_by_email(&self, email: String) -> Result<UserRepositoryConsultReturn, AppError>;
    async fn store_update(
        &self,
        id: String,
        user_to_be_updated: UserRepositoryUpdateParams,
    ) -> Result<String, AppError>;
    async fn delete(&self, id: String) -> Result<String, AppError>;
}
pub struct UserRepositoryPostgres<'a> {
    pub pool: &'a Pool<Postgres>,
}

#[async_trait]
impl UserRepository for UserRepositoryPostgres<'_> {
    async fn store(
        &self,
        user: UserRepositoryStoreParams,
    ) -> Result<UserRepositoryStoreReturn, AppError> {
        match sqlx::query!(
            "INSERT INTO users (id, username, email, password) VALUES ($1, $2, $3, $4)",
            user.id,
            user.username,
            user.email,
            user.password, 
        )
        .execute(self.pool)
        .await
        {
            Ok(_) => Ok(UserRepositoryStoreReturn {
                id: user.id,
                username: user.username,
                email: user.email,
                activated: false,
                blocked: false,
            }),
            Err(error) => Err(sqlx_error_to_app_error(error)),
        }
    }

    async fn consult_by_username(
        &self,
        username: String,
    ) -> Result<UserRepositoryConsultReturn, AppError> {
        match sqlx::query_as!(UserRepositoryConsultReturn, "SELECT id, username, email, password, activated, blocked FROM users WHERE username = $1", username).fetch_one(self.pool).await {
            Ok(user) => Ok(user),
            Err(error) => Err(sqlx_error_to_app_error(error)), 
        }
    }

    async fn consult_by_id(&self, id: String) -> Result<UserRepositoryConsultReturn, AppError> {
        match sqlx::query_as!(UserRepositoryConsultReturn, "SELECT id, username, email, password, activated, blocked FROM users WHERE id = $1", id).fetch_one(self.pool).await {
            Ok(user) => Ok(user),
            Err(error) => Err(sqlx_error_to_app_error(error)), 
        }
    }

    async fn consult_by_email(&self, email: String) -> Result<UserRepositoryConsultReturn, AppError> {
        match sqlx::query_as!(UserRepositoryConsultReturn, "SELECT id, username, email, password, activated, blocked FROM users WHERE email = $1", email).fetch_one(self.pool).await {
            Ok(user) => Ok(user),
            Err(error) => Err(sqlx_error_to_app_error(error)),
        }
    }

    async fn store_update(
        &self,
        id: String,
        user_to_be_updated: UserRepositoryUpdateParams,
    ) -> Result<String, AppError> {
        let mut set_clauses = Vec::new();
    
        if let Some(username) = user_to_be_updated.username {
            set_clauses.push(format!("username = '{}'", username));
        }
    
        if let Some(email) = user_to_be_updated.email {
            set_clauses.push(format!("email = '{}'", email));
        }
    
        if let Some(password) = user_to_be_updated.password {
            set_clauses.push(format!("password = '{}'", password));
        }
    
        if let Some(activated) = user_to_be_updated.activated {
            set_clauses.push(format!("activated = '{}'", activated));
        }
    
        if let Some(blocked) = user_to_be_updated.blocked {
            set_clauses.push(format!("blocked = '{}'", blocked));
        }
    
        if set_clauses.is_empty() {
            return Err(AppError::new(Code::InvalidArgument, "No fields to update"));
        }
    
        let set_clause = set_clauses.join(", ");
        
        let query = format!(
            "UPDATE users SET {} WHERE id = $1",
            set_clause,
        );
        
        match sqlx::query(&query)
            .bind(id)
            .execute(self.pool)
            .await
        {
            Ok(_) => Ok(String::from("User updated successfully")),
            Err(error) => Err(sqlx_error_to_app_error(error)),
        }
    }

    async fn delete(&self, id: String) -> Result<String, AppError> {
        match sqlx::query!("DELETE FROM users WHERE id = $1", id)
            .execute(self.pool)
            .await
        {
            Ok(_) => Ok(String::from("User deleted successfully")),
            Err(error) => Err(sqlx_error_to_app_error(error)),
        }
    }

}

#[cfg(test)]
mod tests {
    use crate::database::utils::integration_test::test_with_database;

    use super::*;

    const FAKE_ID: &str = "userFakeId";
    const FAKE_USERNAME: &str = "username";
    const FAKE_EMAIL: &str = "test@model.com";
    const FAKE_PASSWORD: &str = "password";

    #[tokio::test]
    async fn test_store_user() {
        async fn repository_store(
            pool: Pool<Postgres>,
        ) -> Result<UserRepositoryStoreReturn, AppError> {
            let repository = UserRepositoryPostgres { pool: &pool };

            repository
                .store(UserRepositoryStoreParams {
                    id: FAKE_ID.to_string(),
                    username: FAKE_USERNAME.to_string(),
                    email: FAKE_EMAIL.to_string(),
                    password: FAKE_PASSWORD.to_string(),
                })
                .await
        }

        let response = test_with_database("test_store_user", repository_store)
            .await
            .unwrap();

        assert_eq!(response.id, FAKE_ID);
        assert_eq!(response.username, FAKE_USERNAME);
        assert_eq!(response.email, FAKE_EMAIL);
    }

    #[tokio::test]
    async fn test_consult_user_by_username() {
        async fn repository_consult_by_username(
            pool: Pool<Postgres>,
        ) -> Result<UserRepositoryConsultReturn, AppError> {
            sqlx::query_as!(
                User,
                "INSERT INTO users (id, username, email, password) VALUES ($1, $2, $3, $4)",
                FAKE_ID,
                FAKE_USERNAME,
                FAKE_EMAIL,
                FAKE_PASSWORD, 
            )
            .execute(&pool)
            .await.unwrap();

            let repository = UserRepositoryPostgres { pool: &pool };

            repository
                .consult_by_username(FAKE_USERNAME.to_string())
                .await
        }

        let response = test_with_database("test_consult_by_username", repository_consult_by_username)
            .await
            .unwrap();

        assert_eq!(response.id, FAKE_ID);
        assert_eq!(response.username, FAKE_USERNAME);
        assert_eq!(response.email, FAKE_EMAIL);
        assert_eq!(response.password, FAKE_PASSWORD);

    }

    #[tokio::test]
    async fn test_consult_user_by_id() {
        async fn repository_consult_by_id(
            pool: Pool<Postgres>,
        ) -> Result<UserRepositoryConsultReturn, AppError> {
            sqlx::query_as!(
                User,
                "INSERT INTO users (id, username, email, password) VALUES ($1, $2, $3, $4)",
                FAKE_ID,
                FAKE_USERNAME,
                FAKE_EMAIL,
                FAKE_PASSWORD, 
            )
            .execute(&pool)
            .await.unwrap();

            let repository = UserRepositoryPostgres { pool: &pool };

            repository
                .consult_by_id(FAKE_ID.to_string())
                .await
        }

        let response = test_with_database("test_consult_by_id", repository_consult_by_id)
            .await
            .unwrap();

        assert_eq!(response.id, FAKE_ID);
        assert_eq!(response.username, FAKE_USERNAME);
        assert_eq!(response.email, FAKE_EMAIL);
        assert_eq!(response.password, FAKE_PASSWORD);
    }


    #[tokio::test]
    async fn test_consult_user_by_email() {
        async fn repository_consult_by_email(
            pool: Pool<Postgres>,
        ) -> Result<UserRepositoryConsultReturn, AppError> {
            sqlx::query_as!(
                User,
                "INSERT INTO users (id, username, email, password) VALUES ($1, $2, $3, $4)",
                FAKE_ID,
                FAKE_USERNAME,
                FAKE_EMAIL,
                FAKE_PASSWORD,
            )
            .execute(&pool)
            .await.unwrap();

            let repository = UserRepositoryPostgres { pool: &pool };

            repository
                .consult_by_email(FAKE_EMAIL.to_string())
                .await
        }

        let response = test_with_database("test_consult_by_email", repository_consult_by_email)
            .await
            .unwrap();

        assert_eq!(response.id, FAKE_ID);
        assert_eq!(response.username, FAKE_USERNAME);
        assert_eq!(response.email, FAKE_EMAIL);
        assert_eq!(response.password, FAKE_PASSWORD);
    }

    #[tokio::test]
    async fn test_store_update() {
        const FAKE_USERNAME_UPDATED: &str = "username_uptadated";
        const FAKE_EMAIL_UPDATED: &str = "email@updated.com";
        const FAKE_PASSWORD_UPDATED: &str = "updated_password";

        async fn repository_store_update(
            pool: Pool<Postgres>,
        ) -> Result<String, AppError> {
            sqlx::query_as!(
                User,
                "INSERT INTO users (id, username, email, password) VALUES ($1, $2, $3, $4)",
                FAKE_ID,
                FAKE_USERNAME,
                FAKE_EMAIL,
                FAKE_PASSWORD, 
            )
            .execute(&pool)
            .await.unwrap();

            let repository = UserRepositoryPostgres { pool: &pool };

            let response = repository
                .store_update(FAKE_ID.to_string(), UserRepositoryUpdateParams { 
                    username: Some(FAKE_USERNAME_UPDATED.to_string()), 
                    email: None, 
                    password: Some(FAKE_PASSWORD_UPDATED.to_string()),
                    activated: None,
                    blocked: None
                 })
                .await?;

            let result = sqlx::query_as!(UserRepositoryConsultReturn, 
                "SELECT id, username, email, password, activated, 
                blocked FROM users WHERE id = $1", FAKE_ID)
                .fetch_one(&pool)
                .await.unwrap();
            
            assert_eq!(result.username, FAKE_USERNAME_UPDATED);
            assert_eq!(result.email, FAKE_EMAIL);
            assert_eq!(result.password, FAKE_PASSWORD_UPDATED);
            assert_eq!(result.activated, false);
            assert_eq!(result.blocked, false);
 
            Ok(response)
        }

        let response = test_with_database("test_store_update", repository_store_update)
            .await
            .unwrap();

        assert_eq!(response, "User updated successfully");
    }

    #[tokio::test]
    async fn test_delete() {
        async fn repository_delete_user(
            pool: Pool<Postgres>,
        ) -> Result<String, AppError> {
            sqlx::query_as!(
                User,
                "INSERT INTO users (id, username, email, password) VALUES ($1, $2, $3, $4)",
                FAKE_ID,
                FAKE_USERNAME,
                FAKE_EMAIL,
                FAKE_PASSWORD,
            )
            .execute(&pool)
            .await.unwrap();

            let repository = UserRepositoryPostgres { pool: &pool };

            let response = repository
                .delete(FAKE_ID.to_string())
                .await?;

            match sqlx::query!(
                "SELECT username FROM users WHERE id = $1", FAKE_ID)
                .fetch_one(&pool)
                .await {
                    Ok(_) => panic!("User should not exist"),
                    Err(_) => (),
            }

            Ok(response)
        }

        let response = test_with_database("test_delete_user", repository_delete_user)
            .await
            .unwrap();

        assert_eq!(response, "User deleted successfully");
    }
}