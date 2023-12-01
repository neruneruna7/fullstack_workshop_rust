use actix_web::{
    get,
    HttpResponse, web,
};



pub async fn hello_world() -> &'static str {
    "Hello World!"
}

pub async fn health() -> HttpResponse {
    HttpResponse::Ok()
        .append_header(("version", "0.0.1"))
        .finish()
}

pub fn service(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.route("/health", web::get().to(health))
        .route("/", web::get().to(hello_world));
}
