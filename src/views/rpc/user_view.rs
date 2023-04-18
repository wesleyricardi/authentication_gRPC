use crate::{
    dtos::views::dtos_view_user::*,
    rpc::authentication::authentication::{
        ResAuthentication, ResLogin, ResRegister, ResUpdateUser, User as UserResponse,
    },
};
use tonic::Response;

pub fn render_res_register(user: UserViewArg, token: String) -> Response<ResRegister> {
    Response::new(ResRegister {
        user: Some(UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
            activated: user.activated,
            blocked: user.blocked
        }),
        token,
    })
}

pub fn render_res_login(user: UserViewArg, token: String) -> Response<ResLogin> {
    Response::new(ResLogin {
        user: Some(UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
            activated: user.activated,
            blocked: user.blocked
        }),
        token,
    })
}

pub fn render_res_authentication(user: UserViewArg) -> Response<ResAuthentication> {
    Response::new(ResAuthentication {
        user: Some(UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
            activated: user.activated,
            blocked: user.blocked
        }),
    })
}

pub fn render_res_update(message: String) -> Response<ResUpdateUser> {
    Response::new(ResUpdateUser {
        message
    })
}
