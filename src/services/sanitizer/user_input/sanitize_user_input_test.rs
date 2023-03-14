#[cfg(test)]
mod tests {
    use crate::services::sanitizer::user_input::sanitize_user_input::*;

    #[test]
    fn test_santinize_username_input() {
        let sanitize = SanitizeUserImpl;
        let username_input_dirty = "use&$%rname".to_string();

        let sanitized_username = sanitize
            .sanitize_username_input(username_input_dirty)
            .unwrap();

        assert_eq!(sanitized_username, "username");
    }

    #[test]
    fn test_error_sanitize_username_input() {
        let sanitize = SanitizeUserImpl;
        let username_input_dirty = "".to_string();
        let username_input_dirty2 = "&$%".to_string();

        match sanitize.sanitize_username_input(username_input_dirty) {
            Ok(_) => panic!("Should have failed"),
            Err(error) => assert_eq!(error.message, "Username is empty"),
        }

        match sanitize.sanitize_username_input(username_input_dirty2) {
            Ok(_) => panic!("Should have failed"),
            Err(error) => assert_eq!(error.message, "Username is empty after sanitize"),
        }
    }

    #[test]
    fn test_santinize_email_input() {
        let sanitize = SanitizeUserImpl;
        let email_input_dirty = "  test@email.com  ".to_string();

        let sanitized_email = sanitize.sanitize_email_input(email_input_dirty).unwrap();

        assert_eq!(sanitized_email, "test@email.com");
    }

    #[test]
    fn test_error_sanitize_email_input() {
        let sanitize = SanitizeUserImpl;
        let email_input_dirty = "".to_string();
        let email_input_dirty2 = "    ".to_string();

        match sanitize.sanitize_email_input(email_input_dirty) {
            Ok(_) => panic!("Should have failed"),
            Err(error) => assert_eq!(error.message, "Email is empty"),
        }

        match sanitize.sanitize_email_input(email_input_dirty2) {
            Ok(_) => panic!("Should have failed"),
            Err(error) => assert_eq!(error.message, "Email is empty after sanitize"),
        }
    }

    #[test]
    fn test_santinize_password_input() {
        let sanitize = SanitizeUserImpl;
        let password_input_dirty = "  password  ".to_string();

        let sanitized_password = sanitize
            .sanitize_password_input(password_input_dirty)
            .unwrap();

        assert_eq!(sanitized_password, "password");
    }

    #[test]
    fn test_error_sanitize_password_input() {
        let sanitize = SanitizeUserImpl;
        let password_input_dirty = "".to_string();
        let password_input_dirty2 = "   ".to_string();

        match sanitize.sanitize_password_input(password_input_dirty) {
            Ok(_) => panic!("Should have failed"),
            Err(error) => assert_eq!(error.message, "Password is empty"),
        }

        match sanitize.sanitize_password_input(password_input_dirty2) {
            Ok(_) => panic!("Should have failed"),
            Err(error) => assert_eq!(error.message, "Password is empty after sanitize"),
        }
    }
}
