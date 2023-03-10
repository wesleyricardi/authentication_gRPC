use crate::{
    rpc::authentication::authentication::{ResLogin, ResRegister, User as UserResponse},
    views::user_view::UserViewArg,
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
