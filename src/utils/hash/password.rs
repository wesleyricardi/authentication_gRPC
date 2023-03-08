use tonic::Status;

pub type PasswordHasher = fn(password: String) -> Result<String, Status>;
pub type PasswordVerify = fn(hash_string: String, password: String) -> Result<bool, Status>;

pub const PASSWORD_HASHER: PasswordHasher = |password| {
    let hash = match bcrypt::hash(password, 8) {
        Ok(hash) => hash,
        Err(error) => return Err(Status::new(tonic::Code::Internal, error.to_string())),
    };
    return Ok(hash);
};

pub const PASSWORD_VERIFY: PasswordVerify = |hash_string, password| {
    let result = match bcrypt::verify(password, &hash_string) {
        Ok(result) => result,
        Err(error) => return Err(Status::new(tonic::Code::Internal, error.to_string())),
    };
    return Ok(result);
};

pub const PASSWORD_HASHER_STUP: PasswordHasher = |password| {
    assert!(!password.is_empty());

    Ok("hashpassword".to_string())
};

pub const PASSWORD_VERIFY_STUP: PasswordVerify = |hash_string, password| {
    assert!(!hash_string.is_empty());
    assert!(!password.is_empty());
    assert!(hash_string != password);

    Ok(true)
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hasher() {
        let password = "password";
        let hash = PASSWORD_HASHER(password.to_string()).unwrap();

        assert!(PASSWORD_VERIFY(hash.clone(), password.to_string()).unwrap());
        assert!(!PASSWORD_VERIFY(hash, "wrong_password".to_string()).unwrap());
    }
}
