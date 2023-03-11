use crate::{
    repositories::user::user_repository_mock::UserRepositoryMock,
    utils::hash::password::{PASSWORD_HASHER, PASSWORD_VERIFY},
};

pub use super::user::user_model::UserModel;
use super::user::user_model::UserModelImpl;

pub type DefaultUserModel = UserModelImpl<UserRepositoryMock>;
pub fn get_default_user_model() -> DefaultUserModel {
    UserModelImpl {
        user_repository: UserRepositoryMock,
        password_hasher: PASSWORD_HASHER,
        password_verify: PASSWORD_VERIFY,
    }
}
