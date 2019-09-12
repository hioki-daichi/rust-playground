use crate::db;
use crate::Server;
use itertools::Itertools;

use futures::prelude::*;

// DB に挿入した Log の件数を Result<usize, Error> 型で返す。
fn load_file(
    connection: &diesel::PgConnection,
    file: impl std::io::Read,
) -> Result<usize, failure::Error> {
    use crate::model::NewLog;

    let mut ret = 0;

    let in_csv = std::io::BufReader::new(file);

    // CSV ファイルをデコードしている
    // Serde によるデシリアライズサポートがあるため簡潔に書ける
    let in_log = csv::Reader::from_reader(in_csv).into_deserialize::<::api::Log>();

    for logs in &in_log.chunks(1000) {
        let logs = logs
            // パースに失敗した行は取り除いている
            // Result<T, E> を Option<T> に変換するメソッド
            .filter_map(Result::ok)
            .map(|log| NewLog {
                user_agent: log.user_agent,
                response_time: log.response_time,
                timestamp: log.timestamp.naive_utc(),
            })
            .collect_vec();

        let inserted = db::insert_logs(connection, &logs)?;
        ret += inserted.len();
    }

    Ok(ret)
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

pub fn handle_post_csv(
    server: actix_web::State<Server>,
    // Multiparts で来たファイルデータを一時ファイルに保存する。
    // Futures の Stream を実装しているためそのまま Stream のメソッドを呼べる。
    // Stream は非同期版イテレータのようなもので、ファイルの保存が終わり次第順に取り出されるイメージ。
    multiparts: actix_web_multipart_file::Multiparts,
) -> actix_web::FutureResponse<actix_web::HttpResponse> {
    let fut = multiparts
        .from_err() // map_err(From::from) と同じ挙動をする Stream のメソッド。Future/Stream では ? 演算子によるエラー型の自動変換ができないためこのようなメソッドが用意されている。
        .filter(|field| field.content_type == "text/csv") // Iterator にもあるメソッドの Stream 版。
        .filter_map(|field| match field.form_data {
            // Stream..
            actix_web_multipart_file::FormData::File { file, .. } => Some(file),
            actix_web_multipart_file::FormData::Data { .. } => None,
        })
        .and_then(move |file| load_file(&*server.pool.get()?, file)) // load_file の部分が本 handler の本体。与えられたファイルを CSV としてパースし、DB に保存する関数。
        .fold(0, |acc, x| Ok::<_, failure::Error>(acc + x)) // Iterator にもあるメソッドの Stream 版。
        .map(|sum| actix_web::HttpResponse::Ok().json(api::csv::post::Response(sum)))
        .from_err();

    Box::new(fut)
}

pub fn handle_get_csv(
    server: actix_web::State<Server>,
    range: actix_web::Query<api::csv::get::Query>,
) -> Result<actix_web::HttpResponse, failure::Error> {
    use chrono::{DateTime, Utc};

    let pooled_connection = server.pool.get()?;

    let logs: Vec<crate::model::Log> = db::logs(&pooled_connection, range.from, range.until)?;

    let v = Vec::new();

    let mut w = csv::Writer::from_writer(v);

    for log in logs.into_iter().map(|log| api::Log {
        user_agent: log.user_agent,
        response_time: log.response_time,
        timestamp: DateTime::from_utc(log.timestamp, Utc),
    }) {
        w.serialize(log)?;
    }

    let csv = w.into_inner()?;

    Ok(actix_web::HttpResponse::Ok()
        .header("Content-Type", "text/csv")
        .body(csv))
}
