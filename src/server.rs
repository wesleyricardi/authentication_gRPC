use crate::database::connection::get_postgres_pool;
use sqlx::{Pool, Postgres};
use std::env;
use tonic::transport::Server;

mod controllers;
mod database;
mod dtos;
mod error;
mod models;
mod repositories;
mod rpc;
mod security;
mod services;
mod utils;

use crate::rpc::authentication::{
    authentication::authentication_server::AuthenticationServer, AuthenticationService,
};

pub struct AppState {
    db_pg_pool: Pool<Postgres>,
    redis_client: redis::Client,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::from_filename(".env.development").ok();

    let app_state = AppState {
        db_pg_pool: get_postgres_pool(None).await,
        redis_client: redis::Client::open(env::var("REDIS_CLIENT").unwrap()).unwrap(),
    };

    let addr = "0.0.0.0:50051".parse()?;
    let authentication_service = AuthenticationService::new(app_state);

    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(AuthenticationServer::new(authentication_service))
        .serve(addr)
        .await?;
    Ok(())
}
