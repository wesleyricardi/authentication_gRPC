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
mod views;

use crate::rpc::authentication::{
    authentication::authentication_server::AuthenticationServer, AuthenticationService,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::from_filename(".env.development").ok();

    let addr = "[::1]:50051".parse()?;
    let authentication_service = AuthenticationService::default();

    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(AuthenticationServer::new(authentication_service))
        .serve(addr)
        .await?;
    Ok(())
}
