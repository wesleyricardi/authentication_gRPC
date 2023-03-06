use tonic::{Response, Status};
use crate::{
    rpc::authentication::authentication::{ReqRegister, ResRegister}, 
    models::user_model::{UserModelImpl, UserModel, InsertUser},
    views::user_view::{UserViewImpl, UserView, UserViewArg}
};

pub trait UserController {
    fn register(&self, req: ReqRegister) -> Result<Response<ResRegister>, Status>;
}

pub struct UserControllerImpl;


impl UserController for UserControllerImpl {
    fn register(&self, req: ReqRegister) -> Result<Response<ResRegister>, Status> {
        
        let user_model = UserModelImpl;
        let user = match user_model.insert(InsertUser {
            username: req.username,
            email: req.email,
            password: req.password
        }) {
            Ok(user) => user,
            Err(error) => return Err(error)
        };
    
        let token = "lshfncyqpo548sh6xkf4hwçlfh3xm9itkd8lw0hs".to_string(); //random token


        let user_view = UserViewImpl;
        Ok(user_view.render_res_register(
            UserViewArg {
                id: user.id,
                username: user.username,
                email: user.email
            },
            token
        ))
    }
}