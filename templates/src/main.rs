use actix_web::{error, http, server, App, HttpResponse, Path, State};
use serde_derive::*;
use tera::compile_templates;
use tera::Context;
use tera::Tera;

#[derive(Deserialize)]
struct HelloPath {
    name: String,
}

struct AppState {
    template: Tera,
}

fn hello_template(
    app: State<AppState>,
    path: Path<HelloPath>,
) -> Result<HttpResponse, error::Error> {
    let mut context = Context::new();
    context.insert("name", &path.name);
    let body = app
        .template
        .render("index.html.tera", &context)
        .map_err(|e| error::ErrorInternalServerError(format!("{}", e)))?;
    Ok(HttpResponse::Ok().body(body))
}

fn main() {
    server::new(|| {
        let app = AppState {
            template: compile_templates!("templates/**/*"),
        };
        App::with_state(app).route("/{name}", http::Method::GET, hello_template)
    })
    .bind("localhost:3000")
    .unwrap()
    .run()
}
