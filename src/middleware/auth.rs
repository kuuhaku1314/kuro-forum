use crate::common::http_msg;
use crate::constant::http_error::HttpError::{BuiltInErr, CustomizedErr};
use crate::constant::http_error::{
    HTTP_ERR_CODE_INVALID_TOKEN, HTTP_ERR_INNER, HTTP_ERR_INVALID_TOKEN,
};
use crate::service::user;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    error, Error, HttpMessage,
};

use crate::{common, service};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};

pub const ALLOW_NO_LOGIN_URL: [&str; 2] = ["/user/login", "/user/signup"];
pub struct Auth;

impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { service }))
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let result = has_permission(&req);
        match result {
            Ok(_) => {
                let fut = self.service.call(req);
                Box::pin(async move {
                    let res = fut.await?;
                    Ok(res)
                })
            }
            Err(err) => Box::pin(async move {
                let err = err.into();
                Err(match err {
                    BuiltInErr(HTTP_ERR_CODE_INVALID_TOKEN, _) => error::ErrorUnauthorized(
                        serde_json::json!(http_msg::failure_from(HTTP_ERR_INVALID_TOKEN)),
                    ),
                    CustomizedErr(HTTP_ERR_CODE_INVALID_TOKEN, _) => {
                        error::ErrorUnauthorized(serde_json::json!(http_msg::failure_from(err)))
                    }
                    _ => error::ErrorInternalServerError(serde_json::json!(
                        http_msg::failure_from(HTTP_ERR_INNER)
                    )),
                })
            }),
        }
    }
}

fn has_permission(req: &ServiceRequest) -> common::Result<()> {
    let path = req.path();
    if ALLOW_NO_LOGIN_URL.contains(&path.strip_prefix("/api/v1").unwrap_or(path)) {
        return Ok(());
    }
    let token = req.cookie("user_session").ok_or(HTTP_ERR_INVALID_TOKEN)?;
    match user::decrypt_token(token.value()) {
        Ok(result) => {
            req.extensions_mut().insert(http_msg::UserContext {
                uid: result.userid,
                username: result.username,
            });
            Ok(())
        }
        Err(result) => {
            if service::error::ERR_INVALID_TOKEN.eq(&result) {
                Err(HTTP_ERR_INVALID_TOKEN.into())
            } else {
                Err(result)
            }
        }
    }
}
