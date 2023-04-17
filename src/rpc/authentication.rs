pub mod authentication {
    tonic::include_proto!("authentication");
}

use authentication::authentication_server::Authentication;
use authentication::{
    ReqAuthentication, ReqLogin, ReqRegister, ReqUpdateUser, ResAuthentication, ResLogin,
    ResRegister, ResUpdateUser,
};
use tonic::{Request, Response, Status};

use crate::controllers::authentication::authentication_controller::{
    LoginParams, RegisterParams, SanitizeUser, UpdateParams, AuthenticationController, UserController,
};

use crate::models::authentication::authentication_model::UserModel;
use crate::repositories::user::user_repository::{UserRepositoryPostgres};
use crate::security::jwt::{JWT_DECODE, JWT_ENCODE};
use crate::utils::adapters::app_error_to_grpc_error::app_error_to_grpc_error;
use crate::utils::generate_id::uuidv4::new_uuidv4;
use crate::utils::hash::password::{PASSWORD_HASHER, PASSWORD_VERIFY};
use crate::views::rpc;
use crate::AppState;

pub struct AuthenticationService {
    app_state: AppState,
}

impl AuthenticationService {
    pub fn new(app_state: AppState) -> Self {
        AuthenticationService { app_state }
    }
}

pub type DefaultAuthenticationModel<'a> = UserModel<UserRepositoryPostgres<'a>>;
pub fn create_user_model(app_state: &AppState) -> DefaultAuthenticationModel {
    let pool = &app_state.db_pg_pool;
    UserModel {
        user_repository: UserRepositoryPostgres {
            pool,
        },
        password_hasher: PASSWORD_HASHER,
        password_verify: PASSWORD_VERIFY,
        new_id: new_uuidv4,
    }
}

type DefaultAuthenticationController<'a> = UserController<DefaultAuthenticationModel<'a>, SanitizeUser>;
pub fn create_user_controller(app_state: &AppState) -> DefaultAuthenticationController {
    UserController {
        model: create_user_model(app_state),
        sanitize_user: SanitizeUser,
        jwt_encode: JWT_ENCODE,
        jwt_decode: JWT_DECODE,
    }
}

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
        let app_state = &self.app_state;

        let view = rpc::user_view::render_res_register;
        let controller = create_user_controller(app_state);

        match controller
            .register(
                RegisterParams {
                    username,
                    email,
                    password,
                },
                view,
            )
            .await
        {
            Ok(response) => Ok(response),
            Err(error) => Err(app_error_to_grpc_error(error)),
        }
    }

    async fn login(&self, request: Request<ReqLogin>) -> Result<Response<ResLogin>, Status> {
        let ReqLogin { username, password } = request.into_inner();
        let app_state = &self.app_state;

        let view = rpc::user_view::render_res_login;
        let controller = create_user_controller(app_state);

        match controller
            .login(LoginParams { username, password }, view)
            .await
        {
            Ok(response) => Ok(response),
            Err(error) => Err(app_error_to_grpc_error(error)),
        }
    }

    async fn authentication(
        &self,
        request: Request<ReqAuthentication>,
    ) -> Result<Response<ResAuthentication>, Status> {
        let app_state = &self.app_state;
        let metadata = request.metadata();
        let token = match metadata.get("authorization") {
            Some(t) => t.to_str().unwrap(),
            None => return Err(Status::unauthenticated("Token JWT not found")),
        };

        let view = rpc::user_view::render_res_authentication;
        let controller = create_user_controller(app_state);

        match controller.authenticate(token.to_string(), view).await {
            Ok(response) => Ok(response),
            Err(error) => Err(app_error_to_grpc_error(error)),
        }
    }

    async fn update(
        &self,
        request: Request<ReqUpdateUser>,
    ) -> Result<Response<ResUpdateUser>, Status> {
        let app_state = &self.app_state;
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
        let controller = create_user_controller(app_state);

        match controller
            .update(
                token.to_string(),
                UpdateParams {
                    username,
                    email,
                    password,
                },
                view,
            )
            .await
        {
            Ok(response) => Ok(response),
            Err(error) => Err(app_error_to_grpc_error(error)),
        }
    }
}
