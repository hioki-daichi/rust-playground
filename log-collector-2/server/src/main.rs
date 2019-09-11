#[macro_use]
extern crate diesel;

use actix_web;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use std::env;

mod db;
mod handlers;
mod model;
mod schema;

#[derive(Clone)]
pub struct Server {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl Server {
    pub fn new() -> Self {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");

        let manager = ConnectionManager::<PgConnection>::new(database_url);

        let pool = Pool::builder()
            .build(manager)
            .expect("Failed to connection pool");

        Server { pool }
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
