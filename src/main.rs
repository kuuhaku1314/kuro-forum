#![allow(dead_code)]
#![allow(unused_variables)]

mod cache;
mod common;
mod config;
mod constant;
mod controller;
mod dao;
mod datasource;
mod entity;
mod middleware;
mod model;
mod schema;
mod service;
mod util;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init();
    server::start().await
}

#[deny(dead_code)]
fn init() {
    config::init();
    log::init();
    util::init();
    datasource::init();
    cache::init();
    service::init();
}

mod server {
    use actix_cors::Cors;
    use actix_web::http::KeepAlive;
    use actix_web::{web, App, HttpServer};

    use crate::middleware;
    use crate::{config, controller as ctl};

    pub(super) async fn start() -> std::io::Result<()> {
        HttpServer::new(|| {
            App::new()
                .wrap(middleware::Auth)
                .wrap(middleware::LogEvent)
                .wrap(
                    Cors::default()
                        .send_wildcard()
                        .allow_any_origin()
                        .allow_any_header()
                        .allow_any_method()
                        .max_age(3600),
                )
                .configure(config)
        })
        .keep_alive(KeepAlive::Timeout(std::time::Duration::from_secs(90)))
        .bind(config::config().get_string("http_web_addr").unwrap())?
        .run()
        .await
    }

    fn config(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/api/v1")
                .service(ctl::forum::module())
                .service(ctl::user::module()),
        );
    }
}

mod log {
    use std::env;
    use tracing::Level;

    use tracing_appender::rolling;
    use tracing_subscriber::{fmt, fmt::time::ChronoLocal, layer::SubscriberExt, Registry};

    use tracing_subscriber::fmt::writer::MakeWriterExt;

    #[deny(dead_code)]
    pub(super) fn init() {
        let env: String = env::var("env").unwrap_or("dev".to_owned());
        let debug_file = rolling::daily("log", "debug.log").with_min_level(Level::DEBUG);
        let info_file = rolling::daily("log", "info.log")
            .with_max_level(Level::INFO)
            .with_min_level(Level::WARN);
        let error_file = rolling::daily("log", "error.log").with_max_level(Level::ERROR);
        let all_files = debug_file
            .and(info_file)
            .and(error_file)
            .with_max_level(Level::DEBUG);
        let file_layer = fmt::layer()
            .with_ansi(false)
            .with_timer(ChronoLocal::new("%Y-%m-%d %H:%M:%S%.3f".to_owned()))
            .with_file(true)
            .with_line_number(true)
            .with_writer(all_files);

        let subscriber = Registry::default().with(file_layer);

        if env.to_lowercase() != "live" {
            let console_layer = fmt::layer()
                .pretty()
                .with_timer(ChronoLocal::new("%Y-%m-%d %H:%M:%S%.3f".to_owned()))
                .with_writer(std::io::stdout.with_max_level(Level::INFO));
            tracing::subscriber::set_global_default(subscriber.with(console_layer)).unwrap();
            color_eyre::install().expect("set color console failed")
        } else {
            tracing::subscriber::set_global_default(subscriber).unwrap();
        }
    }
}
