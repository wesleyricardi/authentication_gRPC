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
    async fn store_update(
        &self,
        id: String,
        user_to_be_updated: UserRepositoryUpdateParams,
    ) -> Result<UserRepositoryUpdateReturn, AppError>;
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

    async fn store_update(
        &self,
        id: String,
        user_to_be_updated: UserRepositoryUpdateParams,
    ) -> Result<UserRepositoryUpdateReturn, AppError> {
        match sqlx::query!(
            "UPDATE users
            SET username = $1, email = $2, password = $3
            WHERE id = $4;",
            user_to_be_updated.username,
            user_to_be_updated.email,
            user_to_be_updated.password, 
            id,
        )
        .execute(self.pool)
        .await {
            Ok(_) => {
                let user = self.consult_by_id(id).await?;
                Ok(UserRepositoryUpdateReturn {
                    id: user.id,
                    username: user.username,
                    email: user.email
                })
            },
            Err(error) => Err(sqlx_error_to_app_error(error)), 
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{future::Future, process::Command};

    use super::*;

    async fn test_with_database<T, F>(
        pg_url: String,
        db_name: String,
        callback: fn(Pool<Postgres>) -> F,
    ) -> Result<T, AppError>
    where
    F: Future<Output = Result<T, AppError>>,
{
        let pool = &get_postgres_pool(Some(pg_url.clone())).await;

        setup_database(pool, &pg_url, &db_name).await;

        let db_url = format!("{pg_url}/{db_name}");
        let result = callback(get_postgres_pool(Some(db_url)).await).await;

        drop_database(pool, &db_name).await;

        return result;
    }

    async fn drop_database(pool: &Pool<Postgres>, db_name: &str) {
        sqlx::query(
            "SELECT pg_terminate_backend(pg_stat_activity.pid)
                FROM pg_stat_activity
                WHERE pg_stat_activity.datname = $1
                  AND pid <> pg_backend_pid()",
        )
        .bind(db_name)
        .execute(pool)
        .await
        .unwrap();

        sqlx::query(&(format!("DROP DATABASE IF EXISTS {db_name}")))
            .execute(pool)
            .await
            .unwrap();
    }

    async fn setup_database(pool: &Pool<Postgres>, pg_url: &str, db_name: &str) {
        let database_exists: bool =
            sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM pg_database WHERE datname = $1)")
                .bind(&db_name)
                .fetch_one(pool)
                .await
                .unwrap();

        if !database_exists {
            sqlx::query(&("CREATE DATABASE ".to_string() + db_name))
                .execute(pool)
                .await
                .unwrap();
        }

        let output = Command::new("sqlx")
            .args(&[
                "migrate",
                "run",
                "--database-url",
                &format!("{pg_url}/{db_name}"),
            ])
            .output()
            .expect("execute command failed");

        println!("{}", String::from_utf8_lossy(&output.stdout));
    }

    const FAKE_ID: &str = "userFakeId";
    const FAKE_USERNAME: &str = "username";
    const FAKE_EMAIL: &str = "test@model.com";
    const FAKE_PASSWORD: &str = "password";

    #[tokio::test]
    async fn test_store_user() {
        dotenv::from_filename(".env.test").ok();
        let pg_url = std::env::var("POSTGRES_URL").expect("Unable to read POSTGRES_URL env var");
        let mut db_name =
            std::env::var("DATABASE_NAME").expect("Unable to read DATABASE_NAME env var");
        db_name = format!("{db_name}_test_store_user");

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

        let response = test_with_database(pg_url, db_name, repository_store)
            .await
            .unwrap();

        assert_eq!(response.id, FAKE_ID);
        assert_eq!(response.username, FAKE_USERNAME);
        assert_eq!(response.email, FAKE_EMAIL);
    }

    #[tokio::test]
    async fn test_consult_user_by_username() {
        dotenv::from_filename(".env.test").ok();
        let pg_url = std::env::var("POSTGRES_URL").expect("Unable to read POSTGRES_URL env var");
        let mut db_name =
            std::env::var("DATABASE_NAME").expect("Unable to read DATABASE_NAME env var");
        db_name = format!("{db_name}_test_consult_by_username");

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

        let response = test_with_database(pg_url, db_name, repository_consult_by_username)
            .await
            .unwrap();

        assert_eq!(response.id, FAKE_ID);
        assert_eq!(response.username, FAKE_USERNAME);
        assert_eq!(response.email, FAKE_EMAIL);
        assert_eq!(response.password, FAKE_PASSWORD);

    }

    #[tokio::test]
    async fn test_consult_user_by_id() {
        dotenv::from_filename(".env.test").ok();
        let pg_url = std::env::var("POSTGRES_URL").expect("Unable to read POSTGRES_URL env var");
        let mut db_name =
            std::env::var("DATABASE_NAME").expect("Unable to read DATABASE_NAME env var");
        db_name = format!("{db_name}_test_consult_by_id");

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

        let response = test_with_database(pg_url, db_name, repository_consult_by_id)
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

        dotenv::from_filename(".env.test").ok();
        let pg_url = std::env::var("POSTGRES_URL").expect("Unable to read POSTGRES_URL env var");
        let mut db_name =
            std::env::var("DATABASE_NAME").expect("Unable to read DATABASE_NAME env var");
        db_name = format!("{db_name}_test_store_update");

        async fn repository_store_update(
            pool: Pool<Postgres>,
        ) -> Result<UserRepositoryUpdateReturn, AppError> {
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
                .store_update(FAKE_ID.to_string(), UserRepositoryUpdateParams { 
                    username: Some(FAKE_USERNAME_UPDATED.to_string()), 
                    email: Some(FAKE_EMAIL_UPDATED.to_string()), 
                    password: Some(FAKE_PASSWORD_UPDATED.to_string()) })
                .await
        }

        let response = test_with_database(pg_url, db_name, repository_store_update)
            .await
            .unwrap();

        assert_eq!(response.id, FAKE_ID);
        assert_eq!(response.username, FAKE_USERNAME_UPDATED);
        assert_eq!(response.email, FAKE_EMAIL_UPDATED);
    }

}
