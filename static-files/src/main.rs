use actix_web::{fs, server, App};

fn main() {
    server::new(|| App::new().handler("/", fs::StaticFiles::new(".").unwrap()))
        .bind("localhost:3000")
        .unwrap()
        .run()
}
