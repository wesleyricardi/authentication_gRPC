pub mod authentication {
    tonic::include_proto!("authentication");
}

use authentication::authentication_server::Authentication;
use authentication::{
    ReqActivateUser, ReqCreateActivationCode, ReqCreateRecoveryCode, ReqLogin, ReqRecoverUserData,
    ReqRecoverUserPassword, ReqRegister, ReqUpdateEmail, ReqUpdatePassword, ReqUpdateUser,
    ResActivateUser, ResCreateActivationCode, ResCreateRecoveryCode, ResLogin, ResRecoverUserData,
    ResRecoverUserPassword, ResRegister, ResUpdateEmail, ResUpdatePassword, ResUpdateUser,
};
use tonic::{Request, Response, Status};

use crate::controllers::authentication_controller::{AuthenticationController, UserController};
use crate::dtos::controllers::dtos_controller_user::{
    LoginParams, RegisterParams, UpdateParams, UserControllerRecoverPasswordReq,
    UserControllerUpdatePasswordReq,
};
use crate::models::authentication_model::UserModel;
use crate::repositories::user_repository::UserRepositoryPostgres;
use crate::repositories::users_code_repository::UsersCodeRepositoryRedis;
use crate::security::jwt::{jwt_decode, jwt_encode};
use crate::services::sanitizer::sanitize_authentication_input::SanitizeUser;
use crate::utils::adapters::app_error_to_grpc_error::app_error_to_grpc_error;
use crate::utils::adapters::user_controller_to_grpc_response::{
    map_create_recovery_code_to_grpc_response, map_delete_user_to_grpc_response,
    map_recovery_password_to_grpc_response, map_user_activate_to_grpc_response,
    map_user_auth_to_grpc_response, map_user_create_activation_code_to_grpc_response,
    map_user_login_to_grpc_response, map_user_register_to_grpc_response,
    map_user_update_email_to_grpc_response, map_user_update_password_to_grpc_response,
    map_user_update_to_grpc_response,
};
use crate::utils::generate_code::six_number_code_generator::six_number_code_generator;
use crate::utils::generate_id::uuidv4::new_uuidv4;
use crate::utils::hash::password::{PASSWORD_HASHER, PASSWORD_VERIFY};
use crate::AppState;

use self::authentication::{ReqDeleteUser, ResDeleteUser};

pub struct AuthenticationService {
    app_state: AppState,
}

impl AuthenticationService {
    pub fn new(app_state: AppState) -> Self {
        AuthenticationService { app_state }
    }
}

pub type DefaultAuthenticationModel<'a> =
    UserModel<UserRepositoryPostgres<'a>, UsersCodeRepositoryRedis<'a>>;
pub fn create_user_model(app_state: &AppState) -> DefaultAuthenticationModel {
    let pool = &app_state.db_pg_pool;
    let redis_client = &app_state.redis_client;
    UserModel {
        user_repository: UserRepositoryPostgres { pool },
        user_code_repository: UsersCodeRepositoryRedis {
            client: redis_client,
        },
        password_hasher: PASSWORD_HASHER,
        password_verify: PASSWORD_VERIFY,
        new_id: new_uuidv4,
        generate_code: six_number_code_generator,
    }
}

type DefaultAuthenticationController<'a> =
    UserController<DefaultAuthenticationModel<'a>, SanitizeUser>;
pub fn create_user_controller(app_state: &AppState) -> DefaultAuthenticationController {
    UserController {
        model: create_user_model(app_state),
        sanitize_user: SanitizeUser,
        jwt_encode,
        jwt_decode,
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

        let controller = create_user_controller(app_state);

        match controller
            .register(RegisterParams {
                username,
                email,
                password,
            })
            .await
        {
            Ok(response) => Ok(map_user_register_to_grpc_response(response)),
            Err(error) => Err(app_error_to_grpc_error(error)),
        }
    }

    async fn login(&self, request: Request<ReqLogin>) -> Result<Response<ResLogin>, Status> {
        let ReqLogin { username, password } = request.into_inner();
        let app_state = &self.app_state;

        let controller = create_user_controller(app_state);

        match controller.login(LoginParams { username, password }).await {
            Ok(response) => Ok(map_user_login_to_grpc_response(response)),
            Err(error) => Err(app_error_to_grpc_error(error)),
        }
    }

    async fn recover_user_data(
        &self,
        request: Request<ReqRecoverUserData>,
    ) -> Result<Response<ResRecoverUserData>, Status> {
        let app_state = &self.app_state;
        let metadata = request.metadata();
        let token = match metadata.get("authorization") {
            Some(t) => t.to_str().unwrap(),
            None => return Err(Status::unauthenticated("Token JWT not found")),
        };

        let controller = create_user_controller(app_state);

        match controller.recover_user_data(token.to_string()).await {
            Ok(response) => Ok(map_user_auth_to_grpc_response(response)),
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

        let ReqUpdateUser { username, email } = request.into_inner();

        let controller = create_user_controller(app_state);

        match controller
            .update(token.to_string(), UpdateParams { username, email })
            .await
        {
            Ok(response) => Ok(map_user_update_to_grpc_response(response)),
            Err(error) => Err(app_error_to_grpc_error(error)),
        }
    }

    async fn update_email(
        &self,
        request: Request<ReqUpdateEmail>,
    ) -> Result<Response<ResUpdateEmail>, Status> {
        let app_state = &self.app_state;
        let metadata = request.metadata().to_owned();
        let token = match metadata.get("authorization") {
            Some(t) => t.to_str().unwrap(),
            None => return Err(Status::unauthenticated("Token JWT not found")),
        };

        let ReqUpdateEmail { email } = request.into_inner();

        let controller = create_user_controller(app_state);

        match controller.update_email(token.to_string(), email).await {
            Ok(response) => Ok(map_user_update_email_to_grpc_response(response)),
            Err(error) => Err(app_error_to_grpc_error(error)),
        }
    }

    async fn update_password(
        &self,
        request: Request<ReqUpdatePassword>,
    ) -> Result<Response<ResUpdatePassword>, Status> {
        let app_state = &self.app_state;
        let metadata = request.metadata().to_owned();
        let token = match metadata.get("authorization") {
            Some(t) => t.to_str().unwrap(),
            None => return Err(Status::unauthenticated("Token JWT not found")),
        };

        let ReqUpdatePassword {
            new_password,
            old_password,
        } = request.into_inner();

        let controller = create_user_controller(app_state);

        match controller
            .update_password(
                token.to_string(),
                UserControllerUpdatePasswordReq {
                    new_password,
                    old_password,
                },
            )
            .await
        {
            Ok(response) => Ok(map_user_update_password_to_grpc_response(response)),
            Err(error) => Err(app_error_to_grpc_error(error)),
        }
    }

    async fn create_activation_code(
        &self,
        request: Request<ReqCreateActivationCode>,
    ) -> Result<Response<ResCreateActivationCode>, Status> {
        let app_state = &self.app_state;
        let metadata = request.metadata().to_owned();
        let token = match metadata.get("authorization") {
            Some(t) => t.to_str().unwrap(),
            None => return Err(Status::unauthenticated("Token JWT not found")),
        };

        let controller = create_user_controller(app_state);

        match controller.create_activation_code(token.to_string()).await {
            Ok(response) => Ok(map_user_create_activation_code_to_grpc_response(response)),
            Err(error) => Err(app_error_to_grpc_error(error)),
        }
    }

    async fn activate_user(
        &self,
        request: Request<ReqActivateUser>,
    ) -> Result<Response<ResActivateUser>, Status> {
        let app_state = &self.app_state;
        let metadata = request.metadata().to_owned();
        let token = match metadata.get("authorization") {
            Some(t) => t.to_str().unwrap(),
            None => return Err(Status::unauthenticated("Token JWT not found")),
        };
        let ReqActivateUser { code_key } = request.into_inner();

        let controller = create_user_controller(app_state);

        match controller.activate_user(token.to_string(), code_key).await {
            Ok(response) => Ok(map_user_activate_to_grpc_response(response)),
            Err(error) => Err(app_error_to_grpc_error(error)),
        }
    }

    async fn create_recovery_code(
        &self,
        request: Request<ReqCreateRecoveryCode>,
    ) -> Result<Response<ResCreateRecoveryCode>, Status> {
        let app_state = &self.app_state;
        let ReqCreateRecoveryCode { email } = request.into_inner();

        let controller = create_user_controller(app_state);

        match controller.create_recovery_code(email.to_string()).await {
            Ok(response) => Ok(map_create_recovery_code_to_grpc_response(response)),
            Err(error) => Err(app_error_to_grpc_error(error)),
        }
    }

    async fn recover_user_password(
        &self,
        request: Request<ReqRecoverUserPassword>,
    ) -> Result<Response<ResRecoverUserPassword>, Status> {
        let app_state = &self.app_state;
        let ReqRecoverUserPassword {
            email,
            new_password,
            code_key,
        } = request.into_inner();

        let controller = create_user_controller(app_state);

        match controller
            .recover_user_password(UserControllerRecoverPasswordReq {
                email,
                new_password,
                code_key,
            })
            .await
        {
            Ok(response) => Ok(map_recovery_password_to_grpc_response(response)),
            Err(error) => Err(app_error_to_grpc_error(error)),
        }
    }

    async fn delete_user(
        &self,
        request: Request<ReqDeleteUser>,
    ) -> Result<Response<ResDeleteUser>, Status> {
        let app_state = &self.app_state;
        let metadata = request.metadata().to_owned();
        let token = match metadata.get("authorization") {
            Some(t) => t.to_str().unwrap(),
            None => return Err(Status::unauthenticated("Token JWT not found")),
        };

        let controller = create_user_controller(app_state);

        match controller.delete_user(token.to_string()).await {
            Ok(response) => Ok(map_delete_user_to_grpc_response(response)),
            Err(error) => Err(app_error_to_grpc_error(error)),
        }
    }
}
