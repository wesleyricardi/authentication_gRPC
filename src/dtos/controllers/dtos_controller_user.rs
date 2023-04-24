pub struct RegisterParams {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub struct LoginParams {
    pub username: String,
    pub password: String,
}

pub struct UpdateParams {
    pub username: Option<String>,
    pub email: Option<String>,
}
