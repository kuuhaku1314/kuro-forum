use crate::common::http_msg::Validate;
use crate::common::Result;
use crate::constant::http_error::HTTP_ERR_PARAM;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub nickname: String,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    #[serde(skip_deserializing)]
    pub registration_time: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserTokenPayload {
    pub userid: i64,
    pub username: String,
    pub secret: String,
    pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLogin {
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
}

impl Validate for UserLogin {
    fn validate(&mut self) -> Result<()> {
        if self.username.is_empty() || self.password.is_empty() {
            Err(HTTP_ERR_PARAM.into())
        } else {
            Ok(())
        }
    }
}
