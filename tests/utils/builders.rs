use authentication_gRPC::{
    repositories::{
        user::user_repository::MockUserRepository, 
        users_code::users_code_repository::MockUsersCodeRepository}, 
        models::authentication::authentication_model::UserModel, utils::hash::password::{
            PasswordHasher, 
            PasswordVerify
        }
};

use crate::mocks::{
    user_repository_mock::{
        get_mock_user_repository, 
        MockUserRepositoryParams
    }, 
    users_code_repository_mock::{
        get_mock_users_code_repository, 
        MockUsersCodeRepositoryParams
    }
};


pub struct UserModelBuilder {
    user_repository: MockUserRepository,
    user_code_repository: MockUsersCodeRepository,
    password_hasher: PasswordHasher,
    password_verify: PasswordVerify,
    new_id: fn() -> String,
    generate_code: fn() -> String,
}


impl UserModelBuilder {
    pub fn new() -> Self {
        Self {
            user_repository: get_mock_user_repository(MockUserRepositoryParams{..Default::default()}),
            user_code_repository: get_mock_users_code_repository(MockUsersCodeRepositoryParams{..Default::default()}),
            password_hasher: |_| panic!("password_hasher could not be called by method under test or was forgotten to be assembled in UserModelBuilder"),
            password_verify: |_,_| panic!("password_verify could not be called by method under test or was forgotten to be assembled in UserModelBuilder"),
            new_id:  || panic!("new_id could not be called by method under test or was forgotten to be assembled in UserModelBuilder"),
            generate_code: || panic!("generate code could not be called by method under test or was forgotten to be assembled in UserModelBuilder"), 
        }
    }

    pub fn mount_user_repository(mut self, user_repository: MockUserRepository) -> Self {
        self.user_repository = user_repository;
        self
    }

    pub fn mount_code_repository(mut self, user_code_repository: MockUsersCodeRepository) -> Self {
        self.user_code_repository = user_code_repository;
        self
    }

    pub fn mount_password_hasher(mut self, password_hasher: PasswordHasher) -> Self {
        self.password_hasher = password_hasher;
        self
    }

    pub fn mount_password_verify(mut self, password_verify: PasswordVerify) -> Self {
        self.password_verify = password_verify;
        self
    }

    pub fn mount_new_id(mut self, new_id: fn() -> String) -> Self {
        self.new_id = new_id;
        self
    }

    pub fn mount_generate_code(mut self, generate_code: fn() -> String) -> Self {
        self.generate_code = generate_code;
        self
    }

    pub fn build(self) -> UserModel<MockUserRepository, MockUsersCodeRepository> {
        UserModel {
            user_repository: self.user_repository,
            password_hasher: self.password_hasher,
            password_verify: self.password_verify,
            new_id: self.new_id,
            user_code_repository: self.user_code_repository,
            generate_code: self.generate_code,
        }
    }
}