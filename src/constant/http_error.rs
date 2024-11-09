use crate::constant::base_error::BaseError;
use paste::paste;
use std::fmt::{self, Debug, Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum HttpError {
    BuiltInErr(ErrCode, &'static str),
    CustomizedErr(ErrCode, String),
}

impl std::error::Error for HttpError {}

impl Display for HttpError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            HttpError::BuiltInErr(code, msg) => write!(f, "err_code={}, message={}", code, msg),
            HttpError::CustomizedErr(code, msg) => write!(f, "err_code={}, message={}", code, msg),
        }
    }
}

pub trait IntoErr {
    fn into_err(self) -> HttpError;
}

impl IntoErr for String {
    fn into_err(self) -> HttpError {
        new_customized_error(HTTP_ERR_CODE_INNER, self)
    }
}

impl IntoErr for &'static str {
    fn into_err(self) -> HttpError {
        new_builtin_error(HTTP_ERR_CODE_INNER, self)
    }
}

impl IntoErr for HttpError {
    fn into_err(self) -> HttpError {
        self
    }
}

impl IntoErr for Box<dyn std::error::Error> {
    fn into_err(self) -> HttpError {
        self.into()
    }
}

impl IntoErr for BaseError {
    fn into_err(self) -> HttpError {
        new_builtin_error(HTTP_ERR_CODE_INNER, self.0)
    }
}

impl From<Box<dyn std::error::Error>> for HttpError {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        let result = error.downcast_ref::<HttpError>();
        if result.is_none() {
            return HttpError::CustomizedErr(HTTP_ERR_CODE_INNER, error.to_string());
        }
        result.unwrap().to_owned()
    }
}
pub type ErrCode = u32;

macro_rules! make_http_error_description {
    ($error_name:ident, $code:expr, $msg:expr) => {
        paste! {
            pub const [<HTTP_ERR_CODE_ $error_name>]: ErrCode = $code;
            pub const [<HTTP_ERR_ $error_name>]: HttpError = HttpError::BuiltInErr([<HTTP_ERR_CODE_ $error_name>], $msg);
        }
    };
}

make_http_error_description!(INNER, 100000, "inner error");
make_http_error_description!(DB, 100001, "database error");
make_http_error_description!(PARAM, 100002, "param error");
make_http_error_description!(INVALID_TOKEN, 100003, "invalid token");
make_http_error_description!(RECORD_NOT_FOUND, 100004, "record not found");
make_http_error_description!(DUPLICATE, 100005, "duplicate record");

pub fn new_customized_error(code: ErrCode, msg: String) -> HttpError {
    HttpError::CustomizedErr(code, msg)
}

fn new_builtin_error(code: ErrCode, msg: &'static str) -> HttpError {
    HttpError::BuiltInErr(code, msg)
}
