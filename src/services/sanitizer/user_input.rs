use sanitizer::prelude::*;
use tonic::Status;

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

pub trait SanitizeUser {
    fn register_sanitize(
        &self,
        user_input: RegisterInputDirty,
    ) -> Result<RegisterInputSanitized, Status>;
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
            username: user_input.username.clone(),
            email: user_input.email.clone(),
            password: user_input.password.clone(),
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
}

pub struct SanitizeUserMock;

impl SanitizeUser for SanitizeUserMock {
    fn register_sanitize(
        &self,
        user_input: RegisterInputDirty,
    ) -> Result<RegisterInputSanitized, Status> {
        assert!(!user_input.username.is_empty());
        assert!(!user_input.email.is_empty());
        assert!(!user_input.password.is_empty());

        Ok(RegisterInputSanitized {
            username: user_input.username.clone(),
            email: user_input.email.clone(),
            password: user_input.password.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_sanitize() {
        let sanitize = SanitizeUserImpl;
        let user_input = RegisterInputDirty {
            username: "use&$%rname".to_string(),
            email: "  test@email.com  ".to_string(),
            password: "   password  ".to_string(),
        };

        let RegisterInputSanitized {
            username,
            email,
            password,
        } = sanitize.register_sanitize(user_input).unwrap();

        assert_eq!(username, "username");
        assert_eq!(email, "test@email.com");
        assert_eq!(password, "password");
    }

    #[test]
    fn test_register_sanitize_error() {
        let sanitize = SanitizeUserImpl;

        match sanitize.register_sanitize(RegisterInputDirty {
            username: "".to_string(),
            email: "test@email.com".to_string(),
            password: "password".to_string(),
        }) {
            Ok(_) => panic!("Should have failed"),
            Err(error) => assert_eq!(error.message(), "Username is empty"),
        }

        match sanitize.register_sanitize(RegisterInputDirty {
            username: "username".to_string(),
            email: "".to_string(),
            password: "password".to_string(),
        }) {
            Ok(_) => panic!("Should have failed"),
            Err(error) => assert_eq!(error.message(), "Email is empty"),
        }

        match sanitize.register_sanitize(RegisterInputDirty {
            username: "username".to_string(),
            email: "test@email.com".to_string(),
            password: "".to_string(),
        }) {
            Ok(_) => panic!("Should have failed"),
            Err(error) => assert_eq!(error.message(), "Password is empty"),
        }

        match sanitize.register_sanitize(RegisterInputDirty {
            username: "%$@*".to_string(),
            email: "test@email.com".to_string(),
            password: "password".to_string(),
        }) {
            Ok(_) => panic!("Should have failed"),
            Err(error) => assert_eq!(error.message(), "Username is empty after sanitize"),
        }

        match sanitize.register_sanitize(RegisterInputDirty {
            username: "username".to_string(),
            email: "   ".to_string(),
            password: "password".to_string(),
        }) {
            Ok(_) => panic!("Should have failed"),
            Err(error) => assert_eq!(error.message(), "Email is empty after sanitize"),
        }

        match sanitize.register_sanitize(RegisterInputDirty {
            username: "username".to_string(),
            email: "test@email.com".to_string(),
            password: "   ".to_string(),
        }) {
            Ok(_) => panic!("Should have failed"),
            Err(error) => assert_eq!(error.message(), "Password is empty after sanitize"),
        }
    }
}
