use crate::common::http_msg;
use actix_web::{get, post, web, HttpRequest, Responder, Scope};
use tracing::info;

pub fn module() -> Scope {
    web::scope("/forum").service(hello).service(echo)
}

#[get("")]
async fn hello(ctx: HttpRequest) -> impl Responder {
    info!("invoke hello, {:?}", http_msg::user_info(&ctx));
    http_msg::success_response("Hello world!")
}

#[post("/echo")]
async fn echo(ctx: HttpRequest, req_body: String) -> impl Responder {
    http_msg::success_response(req_body)
}
