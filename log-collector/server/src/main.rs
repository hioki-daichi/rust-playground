#[macro_use]
extern crate diesel;
use actix_web::http::Method;

mod db;
mod handlers;
mod model;
mod schema;

#[derive(Clone)]
pub struct Server {
    pool: diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::pg::PgConnection>>,
}

fn main() {
    env_logger::init();

    actix_web::server::new(|| {
        dotenv::dotenv().ok();

        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");

        let connection_manager =
            diesel::r2d2::ConnectionManager::<diesel::pg::PgConnection>::new(database_url);

        let pool = diesel::r2d2::Pool::builder()
            .build(connection_manager)
            .expect("Failed to create pool.");

        actix_web::App::with_state(Server { pool })
            .route("/logs", Method::POST, crate::handlers::handle_post_logs)
            .route("/csv", Method::POST, crate::handlers::handle_post_csv)
            .route("/csv", Method::GET, crate::handlers::handle_get_csv)
            .route("/logs", Method::GET, crate::handlers::handle_get_logs)
    })
    .bind("localhost:3000")
    .expect("Can not bind to port 3000")
    .run();
}
