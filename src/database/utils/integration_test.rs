use std::{future::Future, process::Command};

use sqlx::{Pool, Postgres};

use crate::{database::connection::get_postgres_pool, error::AppError};

pub async fn test_with_database<T, F>(
    test_name: &str,
    callback: fn(Pool<Postgres>) -> F,
) -> Result<T, AppError>
where
    F: Future<Output = Result<T, AppError>>,
{
    dotenv::from_filename(".env.test").ok();
    let pg_url = std::env::var("POSTGRES_URL").expect("Unable to read POSTGRES_URL env var");
    let mut db_name = std::env::var("DATABASE_NAME").expect("Unable to read DATABASE_NAME env var");
    db_name = format!("{db_name}_{test_name}");

    let pool = &get_postgres_pool(Some(pg_url.clone())).await;

    drop_database(pool, &db_name).await;
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
