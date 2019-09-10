use crate::Server;

pub fn handle_post_csv(
    _server: actix_web::State<Server>,
) -> Result<actix_web::HttpResponse, failure::Error> {
    // TODO: POST されたファイルを扱う

    // ダミーデータ
    let logs = Default::default();

    let resp = api::csv::post::Response(logs);

    Ok(actix_web::HttpResponse::Ok().json(resp))
}

pub fn handle_post_logs(
    _server: actix_web::State<Server>,
    log: actix_web::Json<api::logs::post::Request>,
) -> Result<actix_web::HttpResponse, failure::Error> {
    log::debug!("{:?}", log);
    Ok(actix_web::HttpResponse::Accepted().finish())
}

pub fn handle_get_logs(
    _server: actix_web::State<Server>,
    range: actix_web::Query<api::logs::get::Query>,
) -> Result<actix_web::HttpResponse, failure::Error> {
    log::debug!("{:?}", range);

    let logs = Default::default();

    let resp = api::logs::get::Response(logs);

    Ok(actix_web::HttpResponse::Ok().json(resp))
}

pub fn handle_get_csv(
    _server: actix_web::State<Server>,
    range: actix_web::Query<api::csv::get::Query>,
) -> Result<actix_web::HttpResponse, failure::Error> {
    log::debug!("{:?}", range);

    let csv: Vec<u8> = vec![];

    Ok(actix_web::HttpResponse::Ok()
        .header("Content-Type", "text/csv")
        .body(csv))
}
