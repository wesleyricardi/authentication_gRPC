use tonic::Response;

use crate::{
    dtos::controllers::dtos_controller_user::{
        UserControllerAuthenticationReturn, UserControllerLoginReturn, UserControllerRegisterReturn,
    },
    rpc::authentication::authentication::{
        ResActivateUser, ResCreateActivationCode, ResCreateRecoveryCode, ResLogin,
        ResRecoverUserData, ResRecoverUserPassword, ResRegister, ResUpdateEmail, ResUpdatePassword,
        ResUpdateUser, User as UserResponse, ResDeleteUser,
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
) -> Response<ResRecoverUserData> {
    Response::new(ResRecoverUserData {
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

pub fn map_user_update_email_to_grpc_response(response: String) -> Response<ResUpdateEmail> {
    Response::new(ResUpdateEmail { message: response })
}

pub fn map_user_update_password_to_grpc_response(response: String) -> Response<ResUpdatePassword> {
    Response::new(ResUpdatePassword { message: response })
}

pub fn map_user_create_activation_code_to_grpc_response(
    response: String,
) -> Response<ResCreateActivationCode> {
    Response::new(ResCreateActivationCode { code: response })
}

pub fn map_user_activate_to_grpc_response(response: String) -> Response<ResActivateUser> {
    Response::new(ResActivateUser { message: response })
}

pub fn map_create_recovery_code_to_grpc_response(
    response: String,
) -> Response<ResCreateRecoveryCode> {
    Response::new(ResCreateRecoveryCode { code: response })
}

pub fn map_recovery_password_to_grpc_response(
    response: String,
) -> Response<ResRecoverUserPassword> {
    Response::new(ResRecoverUserPassword { message: response })
}

pub fn map_delete_user_to_grpc_response(
    response: String
) -> Response<ResDeleteUser> {
        Response::new(ResDeleteUser { message: response })
}