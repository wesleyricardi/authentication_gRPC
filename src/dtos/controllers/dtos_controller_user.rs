pub struct RegisterParams {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub struct UserResponse {
    pub id: String,
    pub username: String,
    pub email: String,
    pub activated: bool,
    pub blocked: bool,
}

pub struct UserControllerRegisterReturn {
    pub user: UserResponse,
    pub token: String,
}

pub struct LoginParams {
    pub username: String,
    pub password: String,
}

pub struct UserControllerLoginReturn {
    pub user: UserResponse,
    pub token: String,
}

pub struct UserControllerAuthenticationReturn {
    pub user: UserResponse,
}

pub struct UpdateParams {
    pub username: Option<String>,
    pub email: Option<String>,
}

pub struct UserControllerUpdatePasswordReq {
    pub new_password: String,
    pub old_password: String,
}

pub struct UserControllerRecoverPasswordReq {
    pub new_password: String,
    pub email: String,
    pub code_key: String,
}
