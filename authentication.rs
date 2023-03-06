pub mod authentication {
    tonic::include_proto!("authentication");
}
use crate::controllers::user_controller::{UserController, get_default_user_controller};
use authentication::authentication_server::{Authentication};
use authentication::{ReqRegister, ResRegister, ReqLogin, ResLogin, ReqAuthenticate, ResAuthenticate};

use tonic::{ Request, Response, Status};

#[derive(Debug, Default)]
pub struct AuthenticationService;

#[tonic::async_trait]
impl Authentication for AuthenticationService {
    async fn register(
        &self,
        request: Request<ReqRegister>,
    ) -> Result<Response<ResRegister>, Status> {
        
     tokio::task::spawn_blocking(|| {
        let req = request.into_inner();
        let controller = get_default_user_controller();
        controller.register(req)
     }).await.expect("Task panicked")
    }

    async fn login(
        &self,
        request: Request<ReqLogin>,
    ) -> Result<Response<ResLogin>, Status> {
        tokio::task::spawn_blocking(|| {
            let req = request.into_inner();
            let controller = get_default_user_controller();
            controller.login(req)
         }).await.expect("Task panicked")
    }

    async fn authenticate(
        &self, 
        request: Request<ReqAuthenticate>,
    ) -> Result<Response<ResAuthenticate>, Status> {

        tokio::task::spawn_blocking(|| {
            let req = request.into_inner();
            let controller = get_default_user_controller();
            controller.authenticate(req)
        }).await.expect("Task panicked")
    }
}