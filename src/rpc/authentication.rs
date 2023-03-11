pub mod authentication {
    tonic::include_proto!("authentication");
}
use authentication::authentication_server::Authentication;
use authentication::{ReqLogin, ReqRegister, ResLogin, ResRegister};
use tonic::{Request, Response, Status};

use crate::controllers::default_controllers::{get_default_user_controller, UserController};
use crate::views::rpc;

#[derive(Debug, Default)]
pub struct AuthenticationService;

#[tonic::async_trait]
impl Authentication for AuthenticationService {
    async fn register(
        &self,
        request: Request<ReqRegister>,
    ) -> Result<Response<ResRegister>, Status> {
        let req = request.into_inner();
        let view = rpc::user_view::render_res_register;
        let controller = get_default_user_controller();
        controller.register(req, view)
    }

    async fn login(&self, request: Request<ReqLogin>) -> Result<Response<ResLogin>, Status> {
        let req = request.into_inner();
        let view = rpc::user_view::render_res_login;
        let controller = get_default_user_controller();
        controller.login(req, view)
    }
}
