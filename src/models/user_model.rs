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

impl UserModel for UserModelImpl {
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


pub struct UserModelMock; 
    impl UserModel for UserModelMock {
        fn insert(&self, user: InsertUser) -> Result<User, Status> {
            let  id =  "UUIDV4".to_string();

            let user = User {
                id,
                username: user.username,
                email: user.email,
            };
            Ok(user)
        }
    }


#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn test_insert() {
        let model = UserModelImpl;

        let response = model.insert(InsertUser {
            username: "username".to_string(),
            email: "email".to_string(),
            password: "password".to_string(),
        }).unwrap();

        assert_eq!(response.id.is_empty(), false);
        assert_eq!(response.username, "username".to_string());
        assert_eq!(response.email, "email".to_string())
    }
}