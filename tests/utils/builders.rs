use authentication_gRPC::{
    controllers::authentication::authentication_controller::UserController,
    error::AppError,
    models::authentication::authentication_model::{MockAuthenticationModel, UserModel},
    repositories::{
        user::user_repository::MockUserRepository,
        users_code::users_code_repository::MockUsersCodeRepository,
    },
    security::jwt::{JwtDecode, JwtEncode},
    services::sanitizer::authentication_input::sanitize_authentication_input::MockSanitizeAuthentication,
    utils::hash::password::{PasswordHasher, PasswordVerify},
};

pub struct UserModelBuilderForTest {
    user_repository: MockUserRepository,
    user_code_repository: MockUsersCodeRepository,
    password_hasher: PasswordHasher,
    password_verify: PasswordVerify,
    new_id: fn() -> String,
    generate_code: fn() -> String,
}

impl UserModelBuilderForTest {
    pub fn new() -> Self {
        Self {
            user_repository: MockUserRepository::new(),
            user_code_repository: MockUsersCodeRepository::new(),
            password_hasher: |_| {
                panic!("password_hasher could not be called by method under test or was forgotten to be assembled in UserModelBuilderForTest")
            },
            password_verify: |_, _| {
                panic!("password_verify could not be called by method under test or was forgotten to be assembled in UserModelBuilderForTest")
            },
            new_id: || {
                panic!("new_id could not be called by method under test or was forgotten to be assembled in UserModelBuilderForTest")
            },
            generate_code: || {
                panic!("generate code could not be called by method under test or was forgotten to be assembled in UserModelBuilderForTest")
            },
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

pub struct UserControllerBuilderForTest {
    jwt_decode: JwtDecode,
    jwt_encode: JwtEncode,
    model: MockAuthenticationModel,
    sanitize_user: MockSanitizeAuthentication,
    send_email: fn(to: String, subject: String, body: String) -> Result<String, AppError>,
}

impl UserControllerBuilderForTest {
    pub fn new() -> Self {
        Self {
            model: MockAuthenticationModel::new(),
            sanitize_user: MockSanitizeAuthentication::new(),
            jwt_decode: |_| {
                panic!("jwt_decode could not be called by method under test or was forgotten to be assembled in UserControllerBuilderForTest")
            },
            jwt_encode: |_, _, _| {
                panic!("jwt_encode could not be called by method under test or was forgotten to be assembled in UserControllerBuilderForTest")
            },
            send_email: |_, _, _| {
                panic!("send_email could not be called by method under test or was forgotten to be assembled in UserControllerBuilderForTest")
            },
        }
    }

    pub fn mount_model(mut self, model: MockAuthenticationModel) -> Self {
        self.model = model;
        self
    }

    pub fn mount_sanitize_user(mut self, sanitize_user: MockSanitizeAuthentication) -> Self {
        self.sanitize_user = sanitize_user;
        self
    }

    pub fn mount_jwt_decode(mut self, jwt_decode: JwtDecode) -> Self {
        self.jwt_decode = jwt_decode;
        self
    }

    pub fn mount_jwt_encode(mut self, jwt_encode: JwtEncode) -> Self {
        self.jwt_encode = jwt_encode;
        self
    }

    pub fn build(self) -> UserController<MockAuthenticationModel, MockSanitizeAuthentication> {
        UserController {
            model: self.model,
            sanitize_user: self.sanitize_user,
            jwt_decode: self.jwt_decode,
            jwt_encode: self.jwt_encode,
            send_email: self.send_email,
        }
    }
}
