use tonic::Response;
use crate::{rpc::authentication::{ authentication::{ResRegister, User as UserResponse}}};

pub trait UserView {
    fn render_res_register(&self, user: UserViewArg, token: String) -> Response<ResRegister>;
}
pub struct UserViewImpl;

pub struct UserViewArg {
    pub id: String,
    pub username: String,
    pub email: String,
}

impl UserView for UserViewImpl {
    fn render_res_register(&self, user: UserViewArg, token: String) -> Response<ResRegister> {
       Response::new(ResRegister {
            user: Some(UserResponse {
                id: user.id,
                username: user.username,
                email: user.email,
            }),
            token,
        })
    }
}

pub struct UserViewMock;
impl UserView for UserViewMock {
    fn render_res_register(&self, user: UserViewArg, token: String) -> Response<ResRegister> {
       Response::new(ResRegister {
            user: Some(UserResponse {
                id: user.id,
                username: user.username,
                email: user.email,
            }),
            token,
        })
    }
}