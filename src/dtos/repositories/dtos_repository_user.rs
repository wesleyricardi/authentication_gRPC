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
}

pub struct UserRepositoryConsultReturn {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password: String,
}

pub struct UserRepositoryUpdateParams {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

pub struct UserRepositoryUpdateReturn {
    pub id: String,
    pub username: String,
    pub email: String,
}
