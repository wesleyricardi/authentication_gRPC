use tonic::{transport::Server};
mod rpc;
 
use crate::rpc::authentication::{AuthenticationService, authentication::authentication_server::{AuthenticationServer}};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let authentication_service = AuthenticationService::default();

    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(AuthenticationServer::new(authentication_service))
        .serve(addr).await?;
    Ok(())
}