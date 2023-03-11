#[cfg(test)]
mod tests {
    use crate::services::sanitizer::user_input::sanitize_user_input::*;

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

    #[test]
    fn test_login_sanitize() {
        let sanitize = SanitizeUserImpl;
        let username = "user@$#@name".to_string();
        let password = "  password  ".to_string();

        let LoginInputSanitized { username, password } =
            sanitize.login_sanitize(username, password).unwrap();

        assert_eq!(username, "username");
        assert_eq!(password, "password");
    }

    #[test]
    fn test_login_sanitize_error() {
        let sanitize = SanitizeUserImpl;

        match sanitize.login_sanitize("".to_string(), "password".to_string()) {
            Ok(_) => panic!("Should have failed"),
            Err(error) => assert_eq!(error.message(), "Username is empty"),
        }

        match sanitize.login_sanitize("username".to_string(), "".to_string()) {
            Ok(_) => panic!("Should have failed"),
            Err(error) => assert_eq!(error.message(), "Password is empty"),
        }

        match sanitize.login_sanitize("@#$%".to_string(), "password".to_string()) {
            Ok(_) => panic!("Should have failed"),
            Err(error) => assert_eq!(error.message(), "Username is empty after sanitize"),
        }

        match sanitize.login_sanitize("username".to_string(), "   ".to_string()) {
            Ok(_) => panic!("Should have failed"),
            Err(error) => assert_eq!(error.message(), "Password is empty after sanitize"),
        }
    }
}
