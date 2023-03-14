pub struct UserModelCreateParams {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub struct UserModelInsertReturn {
    pub id: String,
    pub username: String,
    pub email: String,
}

pub struct UserModelLoginVerificationReturn {
    pub id: String,
    pub username: String,
    pub email: String,
}

pub struct UserModelRecoverUserDataReturn {
    pub username: String,
    pub email: String,
}

pub struct UserModelUpdateParams {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

pub struct UserModelUpdateReturn {
    pub id: String,
    pub username: String,
    pub email: String,
}
