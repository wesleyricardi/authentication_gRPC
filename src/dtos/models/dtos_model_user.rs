#[derive(Debug, PartialEq)]
pub struct UserModelCreateParams {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub struct UserModelInsertReturn {
    pub id: String,
    pub username: String,
    pub email: String,
    pub activated: bool,
    pub blocked: bool,
}

#[derive(Debug)]
pub struct UserModelLoginVerificationReturn {
    pub id: String,
    pub username: String,
    pub email: String,
    pub activated: bool,
    pub blocked: bool,
}

pub struct UserModelRecoverUserDataReturn {
    pub username: String,
    pub email: String,
    pub activated: bool,
    pub blocked: bool,
}

#[derive(Debug, PartialEq)]
pub struct UserModelUpdateParams {
    pub username: Option<String>,
    pub email: Option<String>,
}
