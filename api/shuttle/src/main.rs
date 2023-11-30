use actix_web::{get, post, web::{self, ServiceConfig}, HttpResponse, Responder, Result};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use sqlx::Executor;
use shuttle_secrets::SecretStore;

use sqlx::PgPool;


#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_shared_db::Postgres()] pool: PgPool,
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    // let database_url = secret_store.get("DATABASE_URL").unwrap();
    // let pool = PgPool::connect(&database_url).await.unwrap();
    pool.execute(include_str!("../../db/schema.sql")).await.map_err(CustomError::new);

    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world);
    };

    Ok(config.into())
}