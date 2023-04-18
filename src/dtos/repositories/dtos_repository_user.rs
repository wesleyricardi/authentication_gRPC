#[derive(Debug, PartialEq)]
pub struct UserRepositoryStoreParams {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password: String,
}

pub struct UserRepositoryStoreReturn {
    pub id: String,
    pub username: String,
    pub email: String,
    pub activated: bool,
    pub blocked: bool
}

pub struct UserRepositoryConsultReturn {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub activated: bool,
    pub blocked: bool
}

#[derive(Debug, PartialEq)]
pub struct UserRepositoryUpdateParams {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}