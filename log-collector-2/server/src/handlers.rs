use crate::db;
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
    server: actix_web::State<Server>,
    log: actix_web::Json<api::logs::post::Request>,
) -> Result<actix_web::HttpResponse, failure::Error> {
    use crate::model::NewLog;
    use chrono::Utc;

    let user_agent = log.user_agent.clone();
    let response_time = log.response_time;
    let timestamp = log.timestamp.unwrap_or_else(|| Utc::now()).naive_utc();

    let log = NewLog {
        user_agent,
        response_time,
        timestamp,
    };
    let pooled_connection = server.pool.get()?;
    db::insert_log(&pooled_connection, &log)?;
    log::debug!("received log: {:?}", log);
    Ok(actix_web::HttpResponse::Accepted().finish())
}

pub fn handle_get_logs(
    server: actix_web::State<Server>,
    range: actix_web::Query<api::logs::get::Query>,
) -> Result<actix_web::HttpResponse, failure::Error> {
    log::debug!("{:?}", range);

    let pooled_connection = server.pool.get()?;
    let logs = db::logs(&pooled_connection, range.from, range.until)?;
    let logs = logs
        .into_iter()
        .map(|log| api::Log {
            user_agent: log.user_agent,
            response_time: log.response_time,
            timestamp: chrono::DateTime::from_utc(log.timestamp, chrono::Utc),
        })
        .collect();

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
