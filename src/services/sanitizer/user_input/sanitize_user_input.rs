use sanitizer::prelude::*;
pub use tonic::Status;

pub struct RegisterInputDirty {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Sanitize, Debug)]
pub struct RegisterInputSanitized {
    #[sanitize(trim, alphanumeric)]
    pub username: String,
    #[sanitize(trim, lower_case)]
    pub email: String,
    #[sanitize(trim)]
    pub password: String,
}

#[derive(Sanitize, Debug)]
pub struct LoginInputSanitized {
    #[sanitize(trim, alphanumeric)]
    pub username: String,
    #[sanitize(trim)]
    pub password: String,
}

pub trait SanitizeUser {
    fn register_sanitize(
        &self,
        user_input: RegisterInputDirty,
    ) -> Result<RegisterInputSanitized, Status>;
    fn login_sanitize(
        &self,
        username: String,
        password: String,
    ) -> Result<LoginInputSanitized, Status>;
    fn sanitize_username_input(&self, username: String) -> Result<String, Status>;
    fn sanitize_email_input(&self, email: String) -> Result<String, Status>;
    fn sanitize_password_input(&self, password: String) -> Result<String, Status>;
}

pub struct SanitizeUserImpl;

impl SanitizeUser for SanitizeUserImpl {
    fn register_sanitize(
        &self,
        user_input: RegisterInputDirty,
    ) -> Result<RegisterInputSanitized, Status> {
        if user_input.username.is_empty() {
            return Err(Status::new(tonic::Code::Internal, "Username is empty"));
        };
        if user_input.email.is_empty() {
            return Err(Status::new(tonic::Code::Internal, "Email is empty"));
        };
        if user_input.password.is_empty() {
            return Err(Status::new(tonic::Code::Internal, "Password is empty"));
        };

        let mut instance = RegisterInputSanitized {
            username: user_input.username,
            email: user_input.email,
            password: user_input.password,
        };
        instance.sanitize();

        if instance.username.is_empty() {
            return Err(Status::new(
                tonic::Code::Internal,
                "Username is empty after sanitize",
            ));
        };
        if instance.email.is_empty() {
            return Err(Status::new(
                tonic::Code::Internal,
                "Email is empty after sanitize",
            ));
        };
        if instance.password.is_empty() {
            return Err(Status::new(
                tonic::Code::Internal,
                "Password is empty after sanitize",
            ));
        };

        return Ok(instance);
    }

    fn login_sanitize(
        &self,
        username: String,
        password: String,
    ) -> Result<LoginInputSanitized, Status> {
        if username.is_empty() {
            return Err(Status::new(tonic::Code::Internal, "Username is empty"));
        };

        if password.is_empty() {
            return Err(Status::new(tonic::Code::Internal, "Password is empty"));
        };

        let mut instance = LoginInputSanitized { username, password };
        instance.sanitize();

        if instance.username.is_empty() {
            return Err(Status::new(
                tonic::Code::Internal,
                "Username is empty after sanitize",
            ));
        };
        if instance.password.is_empty() {
            return Err(Status::new(
                tonic::Code::Internal,
                "Password is empty after sanitize",
            ));
        };

        return Ok(instance);
    }

    fn sanitize_username_input(&self, username: String) -> Result<String, Status> {
        if username.is_empty() {
            return Err(Status::new(
                tonic::Code::InvalidArgument,
                "Username is empty",
            ));
        };

        let mut instance = StringSanitizer::from(username);
        instance.trim().alphanumeric();

        let username_sanitized = instance.get();
        if username_sanitized.is_empty() {
            return Err(Status::new(
                tonic::Code::Internal,
                "Username is empty after sanitize",
            ));
        };

        return Ok(username_sanitized);
    }

    fn sanitize_email_input(&self, email: String) -> Result<String, Status> {
        if email.is_empty() {
            return Err(Status::new(tonic::Code::InvalidArgument, "Email is empty"));
        }

        let mut instance = StringSanitizer::from(email);
        instance.trim().to_lowercase();

        let email_sanitized = instance.get();

        if email_sanitized.is_empty() {
            return Err(Status::new(
                tonic::Code::InvalidArgument,
                "Email is empty after sanitize",
            ));
        };

        return Ok(email_sanitized);
    }

    fn sanitize_password_input(&self, password: String) -> Result<String, Status> {
        if password.is_empty() {
            return Err(Status::new(tonic::Code::InvalidArgument, "Email is empty"));
        }

        let mut instance = StringSanitizer::from(password);
        instance.trim();

        let password_sanitized = instance.get();

        if password_sanitized.is_empty() {
            return Err(Status::new(
                tonic::Code::InvalidArgument,
                "Pasword is empty after sanitize",
            ));
        };

        return Ok(password_sanitized);
    }
}
