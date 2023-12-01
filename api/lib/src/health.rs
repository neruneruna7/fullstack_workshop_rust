use actix_web::{web, HttpResponse};

pub const API_VERSION: &str = "v0.0.1";

pub async fn hello_world() -> &'static str {
    "Hello World!"
}

pub async fn health() -> HttpResponse {
    HttpResponse::Ok()
        .append_header(("version", API_VERSION))
        .finish()
}

pub fn service(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.route("/health", web::get().to(health))
        .route("/", web::get().to(hello_world));
}

#[cfg(test)]
mod tests {
    use actix_web::http::StatusCode;

    use super::*;

    #[actix_rt::test]
    async fn health_check_work() {
        let res = health().await;

        assert!(res.status().is_success());
        assert_eq!(res.status(), StatusCode::OK);

        let data = res.headers().get("version").and_then(|h| h.to_str().ok());

        assert_eq!(data, Some(API_VERSION));
    }
}
