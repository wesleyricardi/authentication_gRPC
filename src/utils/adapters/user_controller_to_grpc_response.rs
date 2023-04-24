use tonic::Response;

use crate::{
    dtos::controllers::dtos_controller_user::{
        UserControllerAuthenticationReturn, UserControllerLoginReturn,
        UserControllerRegisterReturn, UserControllerUpdateReturn,
    },
    rpc::authentication::authentication::{
        ResAuthentication, ResLogin, ResRegister, ResUpdateUser, User as UserResponse,
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

pub fn map_user_update_to_grpc_response(
    response: UserControllerUpdateReturn,
) -> Response<ResUpdateUser> {
    Response::new(ResUpdateUser { message: response })
}
