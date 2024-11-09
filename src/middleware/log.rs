use actix_http::h1::Payload;
use actix_web::web::BytesMut;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, HttpResponseBuilder,
};
use std::cell::RefCell;

use crate::common::http_msg;
use actix_http::body;
use actix_http::body::{EitherBody, MessageBody};
use futures_util::future::LocalBoxFuture;
use futures_util::StreamExt;
use std::future::{ready, Ready};
use std::rc::Rc;
use std::time::Instant;
use tracing::{info, warn};
pub struct LogEvent;

impl<S, B> Transform<S, ServiceRequest> for LogEvent
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = LogEventMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(LogEventMiddleware {
            service: Rc::new(RefCell::new(service)),
        }))
    }
}

pub struct LogEventMiddleware<S> {
    service: Rc<RefCell<S>>,
}

impl<S, B> Service<ServiceRequest> for LogEventMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let svc = self.service.to_owned();
        let start_time = Instant::now();
        let path = req.path().to_owned();
        let param = req.query_string().to_owned();
        let method = req.method().to_owned();
        let remote_addr = req
            .connection_info()
            .peer_addr()
            .unwrap_or("unknown")
            .to_owned();
        let version = format!("{:?}", req.version());
        let headers = req.headers().to_owned();
        let user_agent = headers
            .get(actix_web::http::header::USER_AGENT)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("-")
            .to_owned();

        Box::pin(async move {
            let mut body_payload = BytesMut::new();
            let mut payload = req.take_payload();
            while let Some(chunk) = payload.next().await {
                body_payload.extend_from_slice(&chunk?);
            }
            let (mut payload_sender, orig_payload) = Payload::create(true);
            payload_sender.feed_data(body_payload.to_owned().freeze());
            req.set_payload(orig_payload.into());
            let req_body = String::from_utf8(body_payload.to_vec()).unwrap_or_default();
            let res: Result<ServiceResponse<B>, Error> = svc.call(req).await;
            match res {
                Ok(res) => {
                    let user_context = res
                        .request()
                        .extensions()
                        .get::<http_msg::UserContext>()
                        .cloned();
                    let elapsed = start_time.elapsed();
                    let status = res.status();
                    let content_length = res
                        .headers()
                        .get(actix_web::http::header::CONTENT_LENGTH)
                        .and_then(|v| v.to_str().ok())
                        .unwrap_or("-")
                        .to_owned();

                    let res = res.map_into_boxed_body();
                    let res_status = res.status().to_owned();
                    let res_headers = res.headers().to_owned();
                    let new_request = res.request().to_owned();
                    let body_bytes = body::to_bytes(res.into_body()).await?;
                    let resp_body = std::str::from_utf8(&body_bytes).unwrap_or("-");
                    info!(
                        "{} {} {} {} {} {} {} user=[{:?}] param=[{}] req_body=[{}] resp_body=[{}] cost=[{:.6}]ms",
                        remote_addr,
                        method,
                        path,
                        version,
                        status.as_u16(),
                        content_length,
                        user_agent,
                        user_context,
                        param,
                        req_body,
                        resp_body,
                        elapsed.as_millis()
                    );
                    let mut new_response = HttpResponseBuilder::new(res_status);
                    for (header_name, header_value) in res_headers {
                        new_response.insert_header((header_name.as_str(), header_value));
                    }
                    Ok(ServiceResponse::new(
                        new_request,
                        new_response.body(body_bytes.to_vec()).map_into_right_body(),
                    ))
                }
                Err(err) => {
                    warn!(
                        "[ERROR]{} {} {} {} {} param=[{}] req_body=[{}] error={} cost=[{:.6}]ms",
                        remote_addr,
                        method,
                        path,
                        version,
                        user_agent,
                        param,
                        req_body,
                        err,
                        start_time.elapsed().as_millis()
                    );
                    Err(err)
                }
            }
        })
    }
}
