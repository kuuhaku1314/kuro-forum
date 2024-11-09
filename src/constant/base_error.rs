use crate::common;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub struct BaseError(pub &'static str);

impl std::error::Error for BaseError {}

impl Display for BaseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl BaseError {
    pub fn eq(&self, err: &common::Error) -> bool {
        match err.downcast_ref::<BaseError>() {
            None => false,
            Some(err) => err == self,
        }
    }
}
