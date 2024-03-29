use crate::{
    error::{AppError, Code},
    utils::adapters::{
        redis_error_to_app_error::redis_error_to_app_error,
        sqlx_error_to_app_error::sqlx_error_to_app_error,
    },
};
use async_trait::async_trait;
use chrono::{Duration, NaiveDateTime, Utc};
use mockall::automock;
use redis::AsyncCommands;
use sqlx::{Pool, Postgres};

#[async_trait]
#[automock]
pub trait UsersCodeRepository: Send + Sync {
    async fn store(&self, code: UsersCode) -> Result<String, AppError>;
    async fn get(&self, user_id: String, code: String) -> Result<UsersCode, AppError>;
    async fn delete(&self, user_id: String) -> Result<String, AppError>;
}

pub struct UsersCode {
    pub code: String,
    pub expire_at: NaiveDateTime,
    pub user_id: String,
}

pub struct UsersCodeRepositoryPostgres<'a> {
    pub pool: &'a Pool<Postgres>,
}

#[async_trait]
impl UsersCodeRepository for UsersCodeRepositoryPostgres<'_> {
    async fn store(&self, code: UsersCode) -> Result<String, AppError> {
        match sqlx::query!(
            "INSERT INTO users_code (code, expire_at, user_id) VALUES ($1, $2, $3)",
            code.code,
            code.expire_at,
            code.user_id
        )
        .execute(self.pool)
        .await
        {
            Ok(_) => Ok(String::from("Code store successfully")),
            Err(error) => Err(sqlx_error_to_app_error(error)),
        }
    }
    async fn get(&self, user_id: String, code_key: String) -> Result<UsersCode, AppError> {
        match sqlx::query_as!(
            UsersCode,
            "SELECT code, expire_at, user_id FROM users_code WHERE code = $1 and user_id = $2",
            code_key,
            user_id
        )
        .fetch_one(self.pool)
        .await
        {
            Ok(code) => Ok(code),
            Err(error) => Err(sqlx_error_to_app_error(error)),
        }
    }
    async fn delete(&self, user_id: String) -> Result<String, AppError> {
        match sqlx::query!("DELETE FROM users_code WHERE user_id = $1", user_id)
            .execute(self.pool)
            .await
        {
            Ok(_) => Ok(String::from("codes from the given user id deleted")),
            Err(error) => Err(sqlx_error_to_app_error(error)),
        }
    }
}

pub struct UsersCodeRepositoryRedis<'a> {
    pub client: &'a redis::Client,
}

#[async_trait]
impl UsersCodeRepository for UsersCodeRepositoryRedis<'_> {
    async fn store(&self, code: UsersCode) -> Result<String, AppError> {
        let mut connection = self
            .client
            .get_async_connection()
            .await
            .map_err(redis_error_to_app_error)?;
        let key = code.user_id;
        let value = code.code;

        redis::pipe()
            .atomic()
            .set(&key, &value)
            .ignore()
            .cmd("EXPIREAT")
            .arg(&key)
            .arg(code.expire_at.timestamp())
            .query_async(&mut connection)
            .await
            .map_err(redis_error_to_app_error)?;

        Ok(String::from("Code stored successfully"))
    }

    async fn get(&self, user_id: String, code: String) -> Result<UsersCode, AppError> {
        let mut connection = self
            .client
            .get_async_connection()
            .await
            .map_err(redis_error_to_app_error)?;

        let value: Option<String> = connection
            .get(user_id.clone())
            .await
            .map_err(redis_error_to_app_error)?;

        if let Some(stored_code) = value {
            if stored_code == code {
                let expire_at_seconds: i64 = connection
                    .ttl(user_id.clone())
                    .await
                    .map_err(redis_error_to_app_error)?;

                let expire_at = Utc::now().naive_utc() + Duration::seconds(expire_at_seconds);

                return Ok(UsersCode {
                    user_id,
                    code,
                    expire_at,
                });
            }
        }

        Err(AppError::new(Code::NotFound, "Code not found"))
    }

    async fn delete(&self, user_id: String) -> Result<String, AppError> {
        let mut connection = self
            .client
            .get_async_connection()
            .await
            .map_err(redis_error_to_app_error)?;

        connection
            .del(user_id)
            .await
            .map_err(redis_error_to_app_error)?;

        Ok(String::from("Code deleted successfully"))
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::{database::utils::integration_test::test_with_database, error::Code};

    use super::*;
    use chrono::Duration;
    use redis::RedisError;
    use sqlx::types::chrono::Utc;

    const FAKE_USER_ID: &str = "UserFakeID";
    const FAKE_USER_USERNAME: &str = "usernames";
    const FAKE_USER_EMAIL: &str = "test@model.com";
    const FAKE_USER_PASSWORD: &str = "password";

    const FAKE_CODE: &str = "0000001";

    async fn store_fake_user_for_test(pool: &Pool<Postgres>) {
        sqlx::query!(
            "INSERT INTO users (id, username, email, password) VALUES ($1, $2, $3, $4)",
            FAKE_USER_ID,
            FAKE_USER_USERNAME,
            FAKE_USER_EMAIL,
            FAKE_USER_PASSWORD,
        )
        .execute(pool)
        .await
        .unwrap();
    }

    async fn store_fake_code_for_test(pool: &Pool<Postgres>) {
        let expire: NaiveDateTime = Utc::now().naive_utc() + Duration::minutes(30);
        sqlx::query!(
            "INSERT INTO users_code (code, expire_at, user_id) VALUES ($1, $2, $3)",
            FAKE_CODE,
            expire,
            FAKE_USER_ID
        )
        .execute(pool)
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn test_store_code() {
        async fn repository_store_code(pool: Pool<Postgres>) -> Result<String, AppError> {
            store_fake_user_for_test(&pool).await;

            let repository = UsersCodeRepositoryPostgres { pool: &pool };

            let expire: NaiveDateTime = Utc::now().naive_utc() + Duration::minutes(30);
            repository
                .store(UsersCode {
                    code: FAKE_CODE.to_string(),
                    expire_at: expire,
                    user_id: FAKE_USER_ID.to_string(),
                })
                .await
        }

        let response = test_with_database("test_store_code", repository_store_code)
            .await
            .unwrap();

        assert_eq!(response, "Code store successfully")
    }

    #[tokio::test]
    async fn test_store_code_without_user() {
        async fn repository_store_code_without_user(
            pool: Pool<Postgres>,
        ) -> Result<String, AppError> {
            let repository = UsersCodeRepositoryPostgres { pool: &pool };

            let expire: NaiveDateTime = Utc::now().naive_utc() + Duration::minutes(30);
            repository
                .store(UsersCode {
                    code: FAKE_CODE.to_string(),
                    expire_at: expire,
                    user_id: FAKE_USER_ID.to_string(),
                })
                .await
        }

        let error = match test_with_database(
            "test_store_code_without_user",
            repository_store_code_without_user,
        )
        .await
        {
            Ok(_) => panic!("test should fail"),
            Err(error) => error,
        };

        assert_eq!(
            error.message,
            "insert or update on table violates foreign key constraint"
        )
    }

    #[tokio::test]
    async fn test_get_code() {
        async fn repository_get_code(pool: Pool<Postgres>) -> Result<UsersCode, AppError> {
            store_fake_user_for_test(&pool).await;
            store_fake_code_for_test(&pool).await;

            let repository = UsersCodeRepositoryPostgres { pool: &pool };

            repository
                .get(FAKE_USER_ID.to_string(), FAKE_CODE.to_string())
                .await
        }

        let response = test_with_database("test_get_code", repository_get_code)
            .await
            .unwrap();

        assert!(response.expire_at > Utc::now().naive_utc());
        assert_eq!(response.user_id, FAKE_USER_ID);
        assert_eq!(response.code, FAKE_CODE);
    }

    #[tokio::test]
    async fn test_get_nonexistent_code() {
        async fn repository_test_get_nonexistent_code(
            pool: Pool<Postgres>,
        ) -> Result<UsersCode, AppError> {
            let repository = UsersCodeRepositoryPostgres { pool: &pool };

            repository
                .get(FAKE_USER_ID.to_string(), FAKE_CODE.to_string())
                .await
        }

        let error = match test_with_database(
            "test_get_nonexistent_code",
            repository_test_get_nonexistent_code,
        )
        .await
        {
            Ok(_) => panic!("test should fail"),
            Err(error) => error,
        };

        assert_eq!(error.code, Code::NotFound);
    }

    #[tokio::test]
    async fn test_delete_code() {
        async fn repository_delete_code(pool: Pool<Postgres>) -> Result<String, AppError> {
            store_fake_user_for_test(&pool).await;

            store_fake_code_for_test(&pool).await;

            let repository = UsersCodeRepositoryPostgres { pool: &pool };

            repository.delete(FAKE_USER_ID.to_string()).await
        }

        let response = test_with_database("test_delete_code", repository_delete_code)
            .await
            .unwrap();

        assert_eq!(response, "codes from the given user id deleted");
    }

    #[tokio::test]
    async fn test_redis_store_code() {
        dotenv::from_filename(".env.test").ok();
        let repository = UsersCodeRepositoryRedis {
            client: &redis::Client::open(env::var("REDIS_CLIENT").unwrap()).unwrap(),
        };
        let expire: NaiveDateTime = Utc::now().naive_utc() + Duration::minutes(30);
        let response = repository
            .store(UsersCode {
                code: FAKE_CODE.to_string(),
                expire_at: expire,
                user_id: FAKE_USER_ID.to_string(),
            })
            .await
            .unwrap();

        assert_eq!(response, "Code stored successfully");
    }

    #[tokio::test]
    async fn test_redis_get_code() {
        dotenv::from_filename(".env.test").ok();
        let repository = UsersCodeRepositoryRedis {
            client: &redis::Client::open(env::var("REDIS_CLIENT").unwrap()).unwrap(),
        };
        let mut connection = repository.client.get_async_connection().await.unwrap();
        let expire: NaiveDateTime = Utc::now().naive_utc() + Duration::minutes(30);

        let result: Result<(), RedisError> = redis::pipe()
            .atomic()
            .set("FAKE_USER_ID", "FAKE_CODE")
            .ignore()
            .cmd("EXPIREAT")
            .arg("FAKE_USER_ID")
            .arg(expire.timestamp())
            .query_async(&mut connection)
            .await;
        let _ = result.unwrap();

        let response = repository
            .get("FAKE_USER_ID".to_string(), "FAKE_CODE".to_string())
            .await
            .unwrap();

        assert!(response.expire_at > Utc::now().naive_utc());
        assert_eq!(response.user_id, "FAKE_USER_ID");
        assert_eq!(response.code, "FAKE_CODE");
    }

    #[tokio::test]
    async fn test_redis_get_nonexistent_code() {
        dotenv::from_filename(".env.test").ok();
        let repository = UsersCodeRepositoryRedis {
            client: &redis::Client::open(env::var("REDIS_CLIENT").unwrap()).unwrap(),
        };

        let error = match repository
            .get("KEY_NONEXISTENT".to_string(), "CODE NOEXISTENT".to_string())
            .await
        {
            Ok(_) => panic!("test should fail"),
            Err(error) => error,
        };

        assert_eq!(error.code, Code::NotFound);
    }

    #[tokio::test]
    async fn test_redis_delete_code() {
        dotenv::from_filename(".env.test").ok();
        let repository = UsersCodeRepositoryRedis {
            client: &redis::Client::open(env::var("REDIS_CLIENT").unwrap()).unwrap(),
        };
        let mut connection = repository.client.get_async_connection().await.unwrap();
        let expire: NaiveDateTime = Utc::now().naive_utc() + Duration::minutes(30);

        let result: Result<(), RedisError> = redis::pipe()
            .atomic()
            .set(FAKE_USER_ID, FAKE_CODE)
            .ignore()
            .cmd("EXPIREAT")
            .arg(FAKE_USER_ID)
            .arg(expire.timestamp())
            .query_async(&mut connection)
            .await;
        let _ = result.unwrap();

        let response = repository.delete(FAKE_USER_ID.to_string()).await.unwrap();

        assert_eq!(response, "Code deleted successfully");
    }
}
