use crate::{
    models::default_models::{get_default_user_model, DefaultUserModel},
    security::jwt::JWT_DECODE,
};

pub use super::user::user_controller::UserController;
use super::user::user_controller::{SanitizeUserImpl, UserControllerImpl, JWT_ENCODE};

type DefaultUserController = UserControllerImpl<DefaultUserModel, SanitizeUserImpl>;
pub fn get_default_user_controller() -> DefaultUserController {
    UserControllerImpl {
        model: get_default_user_model(),
        sanitize_user: SanitizeUserImpl,
        jwt_encode: JWT_ENCODE,
        jwt_decode: JWT_DECODE,
    }
}
