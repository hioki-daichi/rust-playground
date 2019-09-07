use crate::model;
use crate::schema;
use diesel::prelude::*;

pub fn insert_log(
    connection: &diesel::pg::PgConnection,
    new_log: &model::NewLog,
) -> diesel::result::QueryResult<i64> {
    diesel::insert_into(crate::schema::logs::dsl::logs)
        .values(new_log)
        .returning(crate::schema::logs::dsl::id)
        .get_result(connection)
}

pub fn insert_logs(
    connection: &diesel::pg::PgConnection,
    new_logs: &[model::NewLog],
) -> diesel::result::QueryResult<Vec<i64>> {
    diesel::insert_into(crate::schema::logs::dsl::logs)
        .values(new_logs)
        .returning(crate::schema::logs::dsl::id)
        .load(connection)
}

pub fn logs(
    connection: &diesel::pg::PgConnection,
    from: Option<chrono::DateTime<chrono::Utc>>,
    until: Option<chrono::DateTime<chrono::Utc>>,
) -> diesel::result::QueryResult<Vec<model::Log>> {
    let mut query = schema::logs::dsl::logs.into_boxed();

    if let Some(from) = from {
        query = query.filter(schema::logs::dsl::timestamp.ge(from.naive_utc()))
    }

    if let Some(until) = until {
        query = query.filter(schema::logs::dsl::timestamp.lt(until.naive_utc()))
    }

    query
        .order(schema::logs::dsl::timestamp.asc())
        .load(connection)
}
