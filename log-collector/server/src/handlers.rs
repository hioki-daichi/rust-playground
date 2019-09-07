use futures::prelude::*;
use itertools::Itertools;

pub fn handle_get_logs(
    server: actix_web::State<crate::Server>,
    range: actix_web::Query<api::logs::get::Query>,
) -> Result<actix_web::HttpResponse, failure::Error> {
    let pooled_connection = server.pool.get()?;

    Ok(actix_web::HttpResponse::Ok().json(api::logs::get::Response(
        crate::db::logs(&pooled_connection, range.from, range.until)?
            .into_iter()
            .map(|log| {
                let user_agent = log.user_agent;
                let response_time = log.response_time;
                let timestamp = chrono::DateTime::from_utc(log.timestamp, chrono::Utc);

                api::Log {
                    user_agent,
                    response_time,
                    timestamp,
                }
            })
            .collect(),
    )))
}

pub fn handle_get_csv(
    server: actix_web::State<crate::Server>,
    range: actix_web::Query<::api::csv::get::Query>,
) -> Result<actix_web::HttpResponse, failure::Error> {
    let pooled_connection = server.pool.get()?;

    let mut writer = ::csv::Writer::from_writer(Vec::new());

    for log in crate::db::logs(&pooled_connection, range.from, range.until)?
        .into_iter()
        .map(|log| {
            let user_agent = log.user_agent;
            let response_time = log.response_time;
            let timestamp = chrono::DateTime::from_utc(log.timestamp, chrono::Utc);

            ::api::Log {
                user_agent,
                response_time,
                timestamp,
            }
        })
    {
        writer.serialize(log)?;
    }

    Ok(actix_web::HttpResponse::Ok()
        .header("Content-Type", "text/csv")
        .body(writer.into_inner()?))
}

pub fn handle_post_csv(
    server: actix_web::State<crate::Server>,
    multiparts: actix_web_multipart_file::Multiparts,
) -> actix_web::FutureResponse<actix_web::HttpResponse> {
    Box::new(
        multiparts
            .from_err()
            .filter(|field| field.content_type == "text/csv")
            .filter_map(|field| match field.form_data {
                actix_web_multipart_file::FormData::File { file, .. } => Some(file),
                actix_web_multipart_file::FormData::Data { .. } => None,
            })
            .and_then(move |file| -> Result<usize, failure::Error> {
                let pooled_connection = server.pool.get()?;

                let mut ret = 0;

                for logs in &csv::Reader::from_reader(std::io::BufReader::new(file))
                    .into_deserialize::<::api::Log>()
                    .chunks(1000)
                {
                    let inserted = crate::db::insert_logs(
                        &pooled_connection,
                        &logs
                            .filter_map(Result::ok)
                            .map(|log| {
                                let user_agent = log.user_agent;
                                let response_time = log.response_time;
                                let timestamp = log.timestamp.naive_utc();

                                crate::model::NewLog {
                                    user_agent,
                                    response_time,
                                    timestamp,
                                }
                            })
                            .collect_vec(),
                    )?;

                    ret += inserted.len();
                }

                Ok(ret)
            })
            .fold(0, |acc, x| Ok::<_, failure::Error>(acc + x))
            .map(|sum| actix_web::HttpResponse::Ok().json(api::csv::post::Response(sum)))
            .from_err(),
    )
}

pub fn handle_post_logs(
    server: actix_web::State<crate::Server>,
    log: actix_web::Json<api::logs::post::Request>,
) -> Result<actix_web::HttpResponse, failure::Error> {
    let new_log = crate::model::NewLog {
        user_agent: log.user_agent.clone(),
        response_time: log.response_time,
        timestamp: log
            .timestamp
            .unwrap_or_else(|| chrono::Utc::now())
            .naive_utc(),
    };

    let pooled_connection = server.pool.get()?;

    crate::db::insert_log(&pooled_connection, &new_log)?;

    log::debug!("received log: {:?}", new_log);

    Ok(actix_web::HttpResponse::Accepted().finish())
}
