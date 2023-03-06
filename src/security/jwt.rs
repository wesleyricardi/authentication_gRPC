use serde::{Serialize, Deserialize};
use jsonwebtoken::{Header, Validation, EncodingKey, DecodingKey, get_current_timestamp};
use tonic::{Status, Code};


#[derive(Debug, Serialize, Deserialize)]
pub struct UserToken {
    pub id: String,
    pub username: String,
    pub email: String
}


#[derive(Debug, Serialize, Deserialize)]
pub struct JWTAuthenticateToken {
    sub: String,
    pub user: UserToken,
    exp: usize,
}

pub type JwtEncode = fn(user: UserToken) -> Result<String, Status>;
pub type JwtDecode = fn(token: &str) -> Result<JWTAuthenticateToken, Status>;


pub const JWT_ENCODE: JwtEncode = |user| {
        let user_token = JWTAuthenticateToken {
            sub: user.id.to_string(),
            user: UserToken {
                id: user.id.to_string(),
                username: user.username,
                email: user.email
            },
            exp: (get_current_timestamp() + 1000 * 60 * 60 * 2) as usize, // 2 hours
        };
        
        match jsonwebtoken::encode(&Header::default(), &user_token, &EncodingKey::from_secret("JWT_SECRET".as_ref())) {
            Ok(token) => return Ok(token),
            Err(error) => return Err(Status::new(Code::InvalidArgument, format!("failed to encode token :{}", error))),

        };
};

pub const JWT_DECODE: JwtDecode = |token| {
        match jsonwebtoken::decode::<JWTAuthenticateToken>(token, &DecodingKey::from_secret("JWT_SECRET".as_ref()), &Validation::default()) {
            Ok(user_token) => return Ok(user_token.claims),
            Err(error) => {    
                return Err(Status::new(Code::InvalidArgument, format!("failed to decode token :{}", error)))
            },
        };
};