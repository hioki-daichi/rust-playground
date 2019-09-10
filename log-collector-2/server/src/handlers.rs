use crate::Server;

pub fn handle_post_csv(
    _server: actix_web::State<Server>,
) -> Result<actix_web::HttpResponse, failure::Error> {
    unimplemented!()
}

pub fn handle_post_logs(
    _server: actix_web::State<Server>,
) -> Result<actix_web::HttpResponse, failure::Error> {
    unimplemented!()
}

pub fn handle_get_csv(
    _server: actix_web::State<Server>,
) -> Result<actix_web::HttpResponse, failure::Error> {
    unimplemented!()
}

pub fn handle_get_logs(
    _server: actix_web::State<Server>,
) -> Result<actix_web::HttpResponse, failure::Error> {
    unimplemented!()
}
