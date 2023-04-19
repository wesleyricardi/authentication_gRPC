use crate::error::{AppError, Code};
use std::env;

pub fn load_env_var(name: &str) -> Result<String, AppError> {
    match env::var(name) {
        Ok(env_var) => Ok(env_var),
        Err(_) => Err(AppError::new(Code::Internal, format!("Unable to read {} env var", name)))
    }
       
}