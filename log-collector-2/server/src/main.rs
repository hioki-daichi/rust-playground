#[macro_use]
extern crate diesel;

use actix_web;

mod handlers;
mod model;
mod schema;

#[derive(Debug, Clone)]
pub struct Server {}

impl Server {
    pub fn new() -> Self {
        Server {}
    }
}

fn main() {
    env_logger::init();

    actix_web::server::new(|| {
        actix_web::App::with_state(Server::new())
            .route(
                "/csv",
                actix_web::http::Method::POST,
                handlers::handle_post_csv,
            )
            .route(
                "/logs",
                actix_web::http::Method::POST,
                handlers::handle_post_logs,
            )
            .route(
                "/csv",
                actix_web::http::Method::GET,
                handlers::handle_get_csv,
            )
            .route(
                "/logs",
                actix_web::http::Method::GET,
                handlers::handle_get_logs,
            )
    })
    .bind("localhost:3000")
    .unwrap()
    .run()
}
