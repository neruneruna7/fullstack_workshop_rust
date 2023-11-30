use actix_web::{get, post, web::{self, ServiceConfig}, HttpResponse, Responder, Result};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;

use sqlx::PgPool;


#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_shared_db::Postgres()] pool: PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world);
    };

    Ok(config.into())
}