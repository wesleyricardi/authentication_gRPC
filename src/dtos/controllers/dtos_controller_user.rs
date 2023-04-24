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

pub type UserControllerUpdateReturn = String;

pub type UserControllerSendCodeReturn = String;

pub type UserControllerActivateReturn = String;
