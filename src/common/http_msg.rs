use crate::constant::http_error::{HttpError, IntoErr};
use actix_web::{HttpMessage, HttpRequest, HttpResponse};

use serde::Serialize;

#[derive(Serialize)]
pub struct HttpResult<T> {
    pub code: u32,
    pub message: String,
    pub data: T,
}

pub fn success<T: Serialize>(data: T) -> HttpResult<T> {
    HttpResult {
        code: 0,
        message: "".to_owned(),
        data,
    }
}

pub fn success_response<T: Serialize>(data: T) -> HttpResponse {
    HttpResponse::Ok().json(success(data))
}

pub fn failure_response(code: u32, message: String) -> HttpResponse {
    HttpResponse::Ok().json(failure(code, message))
}

pub fn failure_response_from<E: IntoErr>(err: E) -> HttpResponse {
    HttpResponse::Ok().json(failure_from::<E>(err))
}

pub fn failure(code: u32, message: String) -> HttpResult<Option<()>> {
    HttpResult {
        code,
        message,
        data: None,
    }
}

pub fn failure_from<E: IntoErr>(err: E) -> HttpResult<Option<()>> {
    match err.into_err() {
        HttpError::BuiltInErr(code, msg) => HttpResult {
            code,
            message: msg.to_owned(),
            data: None,
        },
        HttpError::CustomizedErr(code, msg) => HttpResult {
            code,
            message: msg.to_owned(),
            data: None,
        },
    }
}

#[derive(Clone, Debug, Default)]
pub struct UserContext {
    pub uid: i64,
    pub username: String,
}

pub fn user_info(req: &HttpRequest) -> UserContext {
    req.extensions().get::<UserContext>().unwrap().to_owned()
}

pub trait Validate {
    fn validate(&mut self) -> crate::common::Result<()>;
}
