pub mod authentication {
    tonic::include_proto!("authentication");
}
use authentication::authentication_server::{Authentication};
use authentication::{ReqRegister, ResRegister, User as UserResponse};
use tonic::{ Request, Response, Status};

use uuid::Uuid;


#[derive(Debug, Default)]
pub struct AuthenticationService;

#[tonic::async_trait]
impl Authentication for AuthenticationService {
    async fn register(
        &self,
        request: Request<ReqRegister>,
    ) -> Result<Response<ResRegister>, Status> {
        let req = request.into_inner();
        
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
