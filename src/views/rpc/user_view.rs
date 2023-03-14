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
        }),
    })
}

pub fn render_res_update(user: UserViewArg) -> Response<ResUpdateUser> {
    Response::new(ResUpdateUser {
        user: Some(UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
        }),
    })
}
