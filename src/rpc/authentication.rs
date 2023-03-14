pub mod authentication {
    tonic::include_proto!("authentication");
}
use authentication::authentication_server::Authentication;
use authentication::{ReqLogin, ReqRegister, ReqUpdateUser, ResLogin, ResRegister, ResUpdateUser};
use tonic::{Request, Response, Status};

use crate::controllers::default_controllers::{get_default_user_controller, UserController};
use crate::controllers::user::user_controller::{LoginParams, RegisterParams, UpdateParams};
use crate::views::rpc;

#[derive(Debug, Default)]
pub struct AuthenticationService;

#[tonic::async_trait]
impl Authentication for AuthenticationService {
    async fn register(
        &self,
        request: Request<ReqRegister>,
    ) -> Result<Response<ResRegister>, Status> {
        let ReqRegister {
            username,
            email,
            password,
        } = request.into_inner();

        let view = rpc::user_view::render_res_register;
        let controller = get_default_user_controller();

        controller.register(
            RegisterParams {
                username,
                email,
                password,
            },
            view,
        )
    }

    async fn login(&self, request: Request<ReqLogin>) -> Result<Response<ResLogin>, Status> {
        let ReqLogin { username, password } = request.into_inner();

        let view = rpc::user_view::render_res_login;
        let controller = get_default_user_controller();

        controller.login(LoginParams { username, password }, view)
    }

    async fn update(
        &self,
        request: Request<ReqUpdateUser>,
    ) -> Result<Response<ResUpdateUser>, Status> {
        let metadata = request.metadata().to_owned();
        let token = match metadata.get("authorization") {
            Some(t) => t.to_str().unwrap(),
            None => return Err(Status::unauthenticated("Token JWT not found")),
        };

        let ReqUpdateUser {
            username,
            email,
            password,
        } = request.into_inner();

        let view = rpc::user_view::render_res_update;
        let controller = get_default_user_controller();

        controller.update(
            token.to_string(),
            UpdateParams {
                username,
                email,
                password,
            },
            view,
        )
    }
}
