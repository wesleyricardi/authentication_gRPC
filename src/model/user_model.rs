use tonic::Status;
use uuid::Uuid;
pub trait UserModel {
    fn insert(&self, user: InsertUser) -> Result<User, Status>;
}

pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
}

pub struct InsertUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub struct UserModelImpl;

impl  UserModel for UserModelImpl {
    fn insert(&self, user: InsertUser) -> Result<User, Status> {
        let  id =  Uuid::new_v4().to_string();

        let user = User {
            id,
            username: user.username,
            email: user.email,
        };
        Ok(user)
    }  
}