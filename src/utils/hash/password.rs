use crate::error::*;

pub type PasswordHasher = fn(password: String) -> Result<String, AppError>;
pub type PasswordVerify = fn(hash_string: String, password: String) -> Result<bool, AppError>;

pub const PASSWORD_HASHER: PasswordHasher = |password| {
    let hash = match bcrypt::hash(password, 8) {
        Ok(hash) => hash,
        Err(error) => return Err(AppError::new(Code::Internal, error.to_string())),
    };
    Ok(hash)
};

pub const PASSWORD_VERIFY: PasswordVerify = |hash_string, password| {
    let result = match bcrypt::verify(password, &hash_string) {
        Ok(result) => result,
        Err(error) => return Err(AppError::new(Code::Internal, error.to_string())),
    };
    Ok(result)
};

#[cfg(test)]
mod tests {
    use super::*;

    const PASSWORD: &str = "password";

    #[test]
    fn test_password_hasher() {
        let hash = PASSWORD_HASHER(PASSWORD.to_string()).unwrap();

        assert_ne!(hash, PASSWORD);
    }

    #[test]
    fn test_password_verify() {
        let hash = PASSWORD_HASHER(PASSWORD.to_string()).unwrap();

        let result = PASSWORD_VERIFY(hash, PASSWORD.to_string()).unwrap();
        assert_eq!(result, true);
    }

    #[test]
    fn test_password_with_wrong_password() {
        let hash = PASSWORD_HASHER(PASSWORD.to_string()).unwrap();

        let result = PASSWORD_VERIFY(hash, "wrong password".to_string()).unwrap();

        assert_eq!(result, false);
    }
}
