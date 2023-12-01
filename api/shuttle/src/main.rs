use actix_web::web::{self, ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;

use sqlx::Executor;

use sqlx::PgPool;

#[shuttle_runtime::main]
async fn actix_web(
    // Dockerを起動している場合は，以下のDB接続のマクロによって自動でDBコンテナが起動される
    // それによってローカルで起動可能
    // Dockerの起動を好まない場合（電力消費など）は，ConnectionStringでShuttleの共有DBにアクセスする
    #[shuttle_shared_db::Postgres()] pool: PgPool,
    // #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    // let database_url = secret_store.get("DATABASE_URL").unwrap();
    // let pool = PgPool::connect(&database_url).await.unwrap();
    let _ = pool
        .execute(include_str!("../../db/schema.sql"))
        .await
        .map_err(CustomError::new)
        .unwrap();
    let pool = web::Data::new(pool);

    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(pool)
            .configure(api_lib::health::service)
            .configure(api_lib::films::service);
    };

    Ok(config.into())
}
