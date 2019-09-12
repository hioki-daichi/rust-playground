use diesel::prelude::*; // PgConnection が使えるようになる

pub fn insert_log(
    connection: &PgConnection,
    log: &crate::model::NewLog,
) -> diesel::result::QueryResult<i64> {
    use crate::schema::logs::dsl;

    diesel::insert_into(dsl::logs) // dsl::logs は logs テーブルを表している
        .values(log)
        .returning(dsl::id) // dsl::id は id カラムを表している
        .get_result(connection)
}

#[allow(dead_code)]
pub fn insert_logs(
    connection: &PgConnection,
    logs: &[crate::model::NewLog],
) -> diesel::result::QueryResult<Vec<i64>> {
    use crate::schema::logs::dsl;

    diesel::insert_into(dsl::logs)
        .values(logs)
        .returning(dsl::id)
        .load(connection)
}

pub fn logs(
    connection: &PgConnection,
    from: Option<chrono::DateTime<chrono::Utc>>,
    until: Option<chrono::DateTime<chrono::Utc>>,
) -> diesel::result::QueryResult<Vec<crate::model::Log>> {
    use crate::schema::logs::dsl;

    let mut query = dsl::logs.into_boxed();

    if let Some(from) = from {
        query = query.filter(dsl::timestamp.ge(from.naive_utc()))
    }

    if let Some(until) = until {
        query = query.filter(dsl::timestamp.lt(until.naive_utc()))
    }

    query.order(dsl::timestamp.asc()).load(connection)
}
