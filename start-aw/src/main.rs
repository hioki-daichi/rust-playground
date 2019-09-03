use actix_web::{server, App, Error, State};

struct MyApp {
    started_at: u64,
}

fn handler(app: State<MyApp>) -> Result<String, Error> {
    Ok(format!(
        "This server was started at {} and current unixtime is {}",
        &app.started_at,
        unixtime()
    ))
}

fn main() {
    let addr = "127.0.0.1:59090";
    server::new(|| {
        App::with_state(MyApp {
            started_at: unixtime(),
        })
        .resource("/info", |r| r.with(handler))
    })
    .bind(addr)
    .unwrap()
    .run();
}

fn unixtime() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
