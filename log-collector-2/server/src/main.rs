use actix_web;

#[derive(Debug, Clone)]
struct Server {}

impl Server {
    pub fn new() -> Self {
        Server {}
    }
}

fn main() {
    env_logger::init();

    actix_web::server::new(|| actix_web::App::with_state(Server::new()))
        .bind("localhost:3000")
        .unwrap()
        .run()
}
