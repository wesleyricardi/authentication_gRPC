pub use crate::{
    database::connection::get_postgres_pool, dtos::repositories::dtos_repository_user::*,
};
use crate::{error::*, utils::adapters::sqlx_error_to_app_error::sqlx_error_to_app_error};
use async_trait::async_trait;
use mockall::automock;
use sqlx::{Pool, Postgres};

#[async_trait]
#[automock]
pub trait UserRepository: Sync + Send + 'static {
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
pub struct UserRepositoryPostgres {
    pub pool: Pool<Postgres>,
}

struct User {
    id: String,
    username: String,
    email: String,
}

#[async_trait]
impl UserRepository for UserRepositoryPostgres {
    async fn store(
        &self,
        user: UserRepositoryStoreParams,
    ) -> Result<UserRepositoryStoreReturn, AppError> {
        match sqlx::query!(
            "INSERT INTO users (id, username, email, password) VALUES ($1, $2, $3, $4)",
            user.id,
            user.username,
            user.email,
            user.password
        )
        .execute(&self.pool)
        .await
        {
            Ok(_) => Ok(UserRepositoryStoreReturn {
                id: user.id,
                username: user.username,
                email: user.email,
            }),
            Err(error) => Err(sqlx_error_to_app_error(error)),
        }
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

#[cfg(test)]
mod tests {
    use std::process::Command;

    use super::*;

    async fn drop_database(db_url: String, db_name: String) {
        let pool = get_postgres_pool(Some(db_url)).await;

        sqlx::query(
            "SELECT pg_terminate_backend(pg_stat_activity.pid)
                FROM pg_stat_activity
                WHERE pg_stat_activity.datname = $1
                  AND pid <> pg_backend_pid()",
        )
        .bind(db_name.clone())
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(&(format!("DROP DATABASE IF EXISTS {db_name}")))
            .execute(&pool)
            .await
            .unwrap();
    }

    async fn setup_database(db_url: String, db_name: String) {
        let pool = get_postgres_pool(Some(db_url.clone())).await;

        let database_exists: bool =
            match sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM pg_database WHERE datname = $1)")
                .bind(&db_name)
                .fetch_one(&pool)
                .await
            {
                Ok(result) => result,
                Err(_) => false,
            };

        if !database_exists {
            sqlx::query(&("CREATE DATABASE ".to_string() + &db_name))
                .execute(&pool)
                .await
                .unwrap();
        }

        let output = Command::new("sqlx")
            .args(&[
                "migrate",
                "run",
                "--database-url",
                &(format!("{db_url}/{db_name}")),
            ])
            .output()
            .expect("execute command failed");

        println!("{}", String::from_utf8_lossy(&output.stdout));
    }

    #[tokio::test]
    async fn test() {
        dotenv::from_filename(".env.test").ok();
        let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");
        let db_name = std::env::var("DATABASE_NAME").expect("Unable to read DATABASE_NAME env var");
        let db_name = format!("{db_name}_test_store_user");

        setup_database(db_url.clone(), db_name.clone()).await;

        let repository = UserRepositoryPostgres {
            pool: get_postgres_pool(Some(format!(
                "{db_url}/{db_name}",
                db_url = db_url.clone(),
                db_name = db_name.clone()
            )))
            .await,
        };

        let response = repository
            .store(UserRepositoryStoreParams {
                id: "id".to_string(),
                username: "username".to_string(),
                email: "email".to_string(),
                password: "password".to_string(),
            })
            .await
            .unwrap();

        drop_database(db_url, db_name).await;

        assert_eq!(response.id, "id");
        assert_eq!(response.username, "username");
        assert_eq!(response.email, "email");
    }
}
