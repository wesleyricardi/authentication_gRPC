pub mod authentication {
    tonic::include_proto!("authentication");
}
use authentication::authentication_server::{Authentication};
use authentication::{ReqRegister, ResRegister};
use tonic::{ Request, Response, Status};

use crate::controller::user_controller::{UserControllerImpl, UserController};




#[derive(Debug, Default)]
pub struct AuthenticationService;

#[tonic::async_trait]
impl Authentication for AuthenticationService {
    async fn register(
        &self,
        request: Request<ReqRegister>,
    ) -> Result<Response<ResRegister>, Status> {
        let req = request.into_inner();

        let controller = UserControllerImpl;
        controller.register(req)
    }
}
