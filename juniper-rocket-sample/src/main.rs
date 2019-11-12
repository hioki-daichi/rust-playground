fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

fn get_graphql_handler(
    context: State<Database>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

fn main() {
    rocket::ignite()
        .manage(Database::new())
        .manage(Schema::new(Query, EmptyMutation::<Database>::new()))
        .mount("/", rocket::routes![graphiql, get_graphql_handler])
        .launch();
}
