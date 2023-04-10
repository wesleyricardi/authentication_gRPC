use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

/// Get a pool connection using lib sqlx em postgres.
///
/// # Examples
/// ```
/// use authentication_gRPC::database::connection::get_postgres_pool;
///
/// let pool = get_postgres_pool(None);
/// ```
///
/// Passing None as the argument will use environment variable DATABASE_URL as postgres url for connection,
/// or you can set another postgres url like:
///
/// ```
/// use authentication_gRPC::database::connection::get_postgres_pool;
///
/// let pool = get_postgres_pool(Some(String::from(
///        "postgres://postgres:123456789@localhost:5432/authentication",
/// )));
/// ```
pub async fn get_postgres_pool(db_url: Option<String>) -> Pool<Postgres> {
    let db_url = db_url
        .unwrap_or(std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var"));

    PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await
        .expect("Unable to connect to Postgres")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_connection() {
        dotenv::from_filename(".env.test").ok();
        let pool = get_postgres_pool(None).await;

        let res = sqlx::query!("SELECT 1 + 1 as sum")
            .fetch_one(&pool)
            .await
            .unwrap();

        assert_eq!(res.sum, Some(2));
    }
}
