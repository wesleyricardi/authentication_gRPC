use tonic::Response;

use crate::{
    dtos::controllers::dtos_controller_user::{
        UserControllerAuthenticationReturn, UserControllerLoginReturn, UserControllerRegisterReturn,
    },
    rpc::authentication::authentication::{
        ResActivateUser, ResAuthentication, ResLogin, ResRecoverUserPassword, ResRegister,
        ResSendActivationCode, ResSendRecoveryCode, ResUpdatePassword, ResUpdateUser,
        User as UserResponse,
    },
};

pub fn map_user_register_to_grpc_response(
    response: UserControllerRegisterReturn,
) -> Response<ResRegister> {
    Response::new(ResRegister {
        user: Some(UserResponse {
            id: response.user.id,
            username: response.user.username,
            email: response.user.email,
            activated: response.user.activated,
            blocked: response.user.blocked,
        }),
        token: response.token,
    })
}

pub fn map_user_login_to_grpc_response(response: UserControllerLoginReturn) -> Response<ResLogin> {
    Response::new(ResLogin {
        user: Some(UserResponse {
            id: response.user.id,
            username: response.user.username,
            email: response.user.email,
            activated: response.user.activated,
            blocked: response.user.blocked,
        }),
        token: response.token,
    })
}

pub fn map_user_auth_to_grpc_response(
    response: UserControllerAuthenticationReturn,
) -> Response<ResAuthentication> {
    Response::new(ResAuthentication {
        user: Some(UserResponse {
            id: response.user.id,
            username: response.user.username,
            email: response.user.email,
            activated: response.user.activated,
            blocked: response.user.blocked,
        }),
    })
}

pub fn map_user_update_to_grpc_response(response: String) -> Response<ResUpdateUser> {
    Response::new(ResUpdateUser { message: response })
}

pub fn map_user_update_password_to_grpc_response(response: String) -> Response<ResUpdatePassword> {
    Response::new(ResUpdatePassword { message: response })
}

pub fn map_user_send_activation_code_to_grpc_response(
    response: String,
) -> Response<ResSendActivationCode> {
    Response::new(ResSendActivationCode { message: response })
}

pub fn map_user_activate_to_grpc_response(response: String) -> Response<ResActivateUser> {
    Response::new(ResActivateUser { message: response })
}

pub fn map_send_recovery_code_to_grpc_response(response: String) -> Response<ResSendRecoveryCode> {
    Response::new(ResSendRecoveryCode { message: response })
}

pub fn map_recover_password_to_grpc_response(response: String) -> Response<ResRecoverUserPassword> {
    Response::new(ResRecoverUserPassword { message: response })
}
