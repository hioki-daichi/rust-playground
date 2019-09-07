use serde_derive::*;

struct AppState {
    template: tera::Tera,
}

#[derive(Deserialize)]
struct HelloPath {
    name: String,
}

fn handler(
    app: actix_web::State<AppState>,
    path: actix_web::Path<HelloPath>,
) -> Result<actix_web::HttpResponse, actix_web::error::Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("name", &path.name);
    let body = app
        .template
        .render("index.html.tera", &ctx)
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("{}", e)))?;
    Ok(actix_web::HttpResponse::Ok().body(body))
}

fn main() {
    actix_web::server::new(|| {
        let app = AppState {
            template: tera::compile_templates!("templates/**/*"),
        };
        actix_web::App::with_state(app).route("/{name}", actix_web::http::Method::GET, handler)
    })
    .bind("localhost:3000")
    .unwrap()
    .run()
}
