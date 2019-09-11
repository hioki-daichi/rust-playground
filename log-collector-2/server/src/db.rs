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
