use crate::{error::*, utils::env_var::load_env_var::load_env_var};
use jsonwebtoken::{get_current_timestamp, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JWTAuthenticateToken {
    pub sub: String,
    pub activated: bool,
    pub blocked: bool,
    pub exp: usize,
}

pub type JwtEncode = fn(id: String, activated: bool, blocked: bool) -> Result<String, AppError>;
pub type JwtDecode = fn(token: &str) -> Result<JWTAuthenticateToken, AppError>;

pub fn jwt_encode(id: String, activated: bool, blocked: bool) -> Result<String, AppError> {
    let user_token = JWTAuthenticateToken {
        sub: id,
        activated,
        blocked,
        exp: (get_current_timestamp() + 1000 * 60 * 60 * 2) as usize, // 2 hours
    };

    match jsonwebtoken::encode(
        &Header::default(),
        &user_token,
        &EncodingKey::from_secret(load_env_var("JWT_SECRET")?.as_ref()),
    ) {
        Ok(token) => Ok(token),
        Err(error) => Err(AppError::new(
            Code::InvalidArgument,
            format!("failed to encode token :{}", error),
        )),
    }
}

pub fn jwt_decode(token: &str) -> Result<JWTAuthenticateToken, AppError> {
    match jsonwebtoken::decode::<JWTAuthenticateToken>(
        token,
        &DecodingKey::from_secret(load_env_var("JWT_SECRET")?.as_ref()),
        &Validation::default(),
    ) {
        Ok(user_token) => Ok(user_token.claims),
        Err(error) => Err(AppError::new(
            Code::InvalidArgument,
            format!("failed to decode token :{}", error),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let user_token = jwt_encode("uuidv4".to_string(), true, false).unwrap();

        assert!(!user_token.is_empty())
    }

    #[test]
    fn test_decode() {
        let jwt_token = jwt_encode("uuidv4".to_string(), true, false).unwrap();
        let JWTAuthenticateToken {
            sub,
            activated,
            blocked,
            ..
        } = jwt_decode(&jwt_token).unwrap();

        assert_eq!("uuidv4", sub);
        assert_eq!(true, activated);
        assert_eq!(false, blocked);
    }
}
