use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn get_postgres_pool() -> Pool<Postgres> {
    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

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
        dotenv::dotenv().unwrap();
        let pool = get_postgres_pool().await;

        let res = sqlx::query!("SELECT 1 + 1 as sum")
            .fetch_one(&pool)
            .await
            .unwrap();

        assert_eq!(res.sum, Some(2));
    }
}
