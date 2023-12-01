use actix_web::{get, post, web::{self, ServiceConfig}, HttpResponse, Responder, Result};
use sqlx::PgPool;

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[get("/version")]
async fn version(db: web::Data<PgPool>) -> Result<HttpResponse> {
    tracing::info!("Getting version");
    let result: Result<String, sqlx::Error> = sqlx::query_scalar("SELECT version()")
        .fetch_one(db.get_ref())
        .await;

    let res = match result {
        Ok(version) => version,
        Err(e) => format!("Error: {:?}", e),
    };

    Ok(HttpResponse::Ok().body(res))
}

#[get("ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}
