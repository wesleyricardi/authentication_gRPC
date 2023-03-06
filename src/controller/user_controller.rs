use tonic::{Response, Status};
use crate::rpc::authentication::authentication::{ReqRegister, ResRegister, User as UserResponse};

use uuid::Uuid;

pub trait UserController {
    fn register(&self, req: ReqRegister) -> Result<Response<ResRegister>, Status>;
}

pub struct UserControllerImpl;


impl UserController for UserControllerImpl {
    fn register(&self, req: ReqRegister) -> Result<Response<ResRegister>, Status> {
        let  id =  Uuid::new_v4().to_string();

        let user = Some(UserResponse {
            id,
            username: req.username,
            email: req.email,  
        });
    
         let token = "lshfncyqpo548sh6xkf4hwçlfh3xm9itkd8lw0hs".to_string(); //random token

        Ok(Response::new(ResRegister {
            user,
            token,
        }))
    }
}