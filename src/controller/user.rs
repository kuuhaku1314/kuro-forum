use crate::common::http_msg::*;
use crate::common::Error;
use crate::constant::http_error;
use crate::model::user::{User, UserLogin};
use crate::service::{error, user as user_service};
use actix_web::cookie::time::Duration;
use actix_web::cookie::Cookie;
use actix_web::{post, web, HttpResponse, Responder, Scope};
use tracing::{error, info};

pub fn module() -> Scope {
    web::scope("/user").service(login).service(signup)
}

#[post("/login")]
async fn login(mut user: web::Json<UserLogin>) -> impl Responder {
    if let Err(err) = user.validate() {
        return failure_response_from(err);
    }
    match user_service::login(
        user.username.as_str(),
        user.password.as_str(),
        Duration::days(30),
    ) {
        Ok(token) => HttpResponse::Ok()
            .cookie(
                Cookie::build("user_session", token)
                    .max_age(Duration::days(30))
                    .path("/")
                    .finish(),
            )
            .json(success("")),
        Err(err) => handle_error(err),
    }
}

#[post("/signup")]
async fn signup(_user: web::Json<User>) -> impl Responder {
    let user = _user.into_inner();
    let result = user_service::signup(&user);
    match result {
        Ok(id) => {
            info!("signup success request={:?}", user);
            success_response(format!(
                "signup successful, your uid is {}, welcome {}",
                id, user.nickname
            ))
        }
        Err(err) => {
            error!("signup failed {}", err);
            handle_error(err)
        }
    }
}

fn handle_error(err: Error) -> HttpResponse {
    if error::ERR_USER_NOT_FOUND.eq(&err) {
        failure_response_from(http_error::HTTP_ERR_RECORD_NOT_FOUND)
    } else if error::ERR_USER_EXISTED.eq(&err) {
        failure_response_from(http_error::HTTP_ERR_DUPLICATE)
    } else if error::ERR_INVALID_PARAM.eq(&err) {
        failure_response_from(http_error::HTTP_ERR_PARAM)
    } else {
        failure_response_from(err)
    }
}
