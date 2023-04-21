use async_trait::async_trait;
use chrono::NaiveDateTime;
use mockall::automock;
use sqlx::{Pool, Postgres};
use crate::{error::AppError, utils::adapters::sqlx_error_to_app_error::sqlx_error_to_app_error};

#[async_trait]
#[automock]
pub trait UsersCodeRepository: Send + Sync{
    async fn store(&self, code: UsersCode) -> Result<String, AppError>;
    async fn get(&self, user_id: String, code: String) -> Result<UsersCode, AppError>;
    async fn delete(&self, user_id: String) -> Result<String, AppError>;
}

pub struct UsersCode {
    pub code: String, 
    pub expire_at: NaiveDateTime, 
    pub user_id: String
}

pub struct UsersCodeRepositoryPostgres<'a> {
    pub pool: &'a Pool<Postgres>,
}

#[async_trait]
impl UsersCodeRepository for UsersCodeRepositoryPostgres<'_> {
    async fn store(&self, code: UsersCode) -> Result<String, AppError> {
        match sqlx::query!("INSERT INTO users_code (code, expire_at, user_id) VALUES ($1, $2, $3)", code.code, code.expire_at, code.user_id).execute(self.pool).await {
            Ok(_) => Ok(String::from("Code store successfully")),
            Err(error) => Err(sqlx_error_to_app_error(error)),
        }
    }
    async fn get(&self, user_id: String, code_key: String) -> Result<UsersCode, AppError> {
        match sqlx::query_as!(UsersCode, "SELECT code, expire_at, user_id FROM users_code WHERE code = $1 and user_id = $2", code_key, user_id).fetch_one(self.pool).await {
            Ok(code) => Ok(code),
            Err(error) => Err(sqlx_error_to_app_error(error)),
        }
    }
    async fn delete(&self, user_id: String) -> Result<String, AppError> {
        match sqlx::query!("DELETE FROM users_code WHERE user_id = $1", user_id).execute(self.pool).await {
            Ok(_) => Ok(String::from("codes from the given user id deleted")),
            Err(error) => Err(sqlx_error_to_app_error(error)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::database::utils::integration_test::test_with_database;

    use super::*;
    use chrono::Duration;
    use sqlx::types::chrono::{Utc};

    #[tokio::test]
    async fn test_store_code() {
        const FAKE_USER_ID: &str = "UserFakeID";
        const FAKE_USER_USERNAME: &str = "usernames";
        const FAKE_USER_EMAIL: &str = "test@model.com";
        const FAKE_USER_PASSWORD: &str = "password";

        const FAKE_CODE: &str = "0000001";
      
        dotenv::from_filename(".env.test").ok();
        let pg_url = std::env::var("POSTGRES_URL").expect("Unable to read POSTGRES_URL env var");
        let mut db_name =
            std::env::var("DATABASE_NAME").expect("Unable to read DATABASE_NAME env var");
        db_name = format!("{db_name}_test_store_code");

        async fn repository_store_code(pool: Pool<Postgres>) -> Result<String, AppError> {
            let expire: NaiveDateTime = Utc::now().naive_utc() + Duration::minutes(30);
            sqlx::query!(
                "INSERT INTO users (id, username, email, password) VALUES ($1, $2, $3, $4)",
                FAKE_USER_ID,
                FAKE_USER_USERNAME,
                FAKE_USER_EMAIL,
                FAKE_USER_PASSWORD, 
            )
            .execute(&pool)
            .await.unwrap();
            
            let repository = UsersCodeRepositoryPostgres {
                pool: &pool
            };
    
           repository.store(UsersCode { code: FAKE_CODE.to_string(), expire_at: expire, user_id: FAKE_USER_ID.to_string() }).await
        }


        let response = test_with_database(pg_url, db_name, repository_store_code)
        .await
        .unwrap();

        assert_eq!(response, "Code store successfully")
    }

    #[tokio::test]
    async fn test_get_code() {
        const FAKE_USER_ID: &str = "UserFakeID";
        const FAKE_USER_USERNAME: &str = "usernames";
        const FAKE_USER_EMAIL: &str = "test@model.com";
        const FAKE_USER_PASSWORD: &str = "password";

        const FAKE_CODE: &str = "0000001";
      
        dotenv::from_filename(".env.test").ok();
        let pg_url = std::env::var("POSTGRES_URL").expect("Unable to read POSTGRES_URL env var");
        let mut db_name =
            std::env::var("DATABASE_NAME").expect("Unable to read DATABASE_NAME env var");
        db_name = format!("{db_name}_test_get_code");

        async fn repository_get_code(pool: Pool<Postgres>) -> Result<UsersCode, AppError> {
            let expire: NaiveDateTime = Utc::now().naive_utc() + Duration::minutes(30);

            sqlx::query!(
                "INSERT INTO users (id, username, email, password) VALUES ($1, $2, $3, $4)",
                FAKE_USER_ID,
                FAKE_USER_USERNAME,
                FAKE_USER_EMAIL,
                FAKE_USER_PASSWORD, 
            )
            .execute(&pool)
            .await.unwrap();

            sqlx::query!("INSERT INTO users_code (code, expire_at, user_id) VALUES ($1, $2, $3)", FAKE_CODE, expire, FAKE_USER_ID).execute(&pool).await.unwrap();
            
            let repository = UsersCodeRepositoryPostgres {
                pool: &pool
            };
    
          repository.get(FAKE_USER_ID.to_string(), FAKE_CODE.to_string()).await
        }

        let response = test_with_database(pg_url, db_name, repository_get_code)
        .await
        .unwrap();

        assert!(response.expire_at > Utc::now().naive_utc());
        assert_eq!(response.user_id, FAKE_USER_ID);
        assert_eq!(response.code, FAKE_CODE);

    }


    #[tokio::test]
    async fn test_delete_code() {
        const FAKE_USER_ID: &str = "UserFakeID";
        const FAKE_USER_USERNAME: &str = "usernames";
        const FAKE_USER_EMAIL: &str = "test@model.com";
        const FAKE_USER_PASSWORD: &str = "password";

        const FAKE_CODE: &str = "0000001";
      
        dotenv::from_filename(".env.test").ok();
        let pg_url = std::env::var("POSTGRES_URL").expect("Unable to read POSTGRES_URL env var");
        let mut db_name =
            std::env::var("DATABASE_NAME").expect("Unable to read DATABASE_NAME env var");
        db_name = format!("{db_name}_test_delete_code");

        async fn repository_delete_code(pool: Pool<Postgres>) -> Result<String, AppError> {
            let expire: NaiveDateTime = Utc::now().naive_utc() + Duration::minutes(30);

            sqlx::query!(
                "INSERT INTO users (id, username, email, password) VALUES ($1, $2, $3, $4)",
                FAKE_USER_ID,
                FAKE_USER_USERNAME,
                FAKE_USER_EMAIL,
                FAKE_USER_PASSWORD, 
            )
            .execute(&pool)
            .await.unwrap();

            sqlx::query!("INSERT INTO users_code (code, expire_at, user_id) VALUES ($1, $2, $3)", FAKE_CODE, expire, FAKE_USER_ID).execute(&pool).await.unwrap();
            
            let repository = UsersCodeRepositoryPostgres {
                pool: &pool
            };
    
           repository.delete(FAKE_USER_ID.to_string()).await
        }

        let response = test_with_database(pg_url, db_name, repository_delete_code)
        .await
        .unwrap();


        assert_eq!(response, "codes from the given user id deleted");

    }
}
