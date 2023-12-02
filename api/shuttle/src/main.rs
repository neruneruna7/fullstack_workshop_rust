use actix_web::web;
use actix_web::web::ServiceConfig;

use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;

use sqlx::Executor;

use sqlx::PgPool;

#[shuttle_runtime::main]
async fn actix_web(
    // Dockerを起動している場合は，以下のDB接続のマクロによって自動でDBコンテナが起動される
    // それによってローカルで起動可能
    // Dockerの起動を好まない場合（電力消費など）は，ConnectionStringでShuttleの共有DBにアクセスする
    // static_folderは廃止されました
    // ignoreしたファイルをデプロイに含めるには，Shuttle.tomlにassets = ["file_path",] と記述する
    // あとはハードコーディングでファイルパスを指定できます
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
    let film_repository = api_lib::film_repository::PostgresFilmRepository::new(pool);
    let film_repository = actix_web::web::Data::new(film_repository);

    println!("file_path: {:?}", std::env::current_dir());

    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(
            web::scope("/api")
                .app_data(film_repository)
                .configure(api_lib::health::service)
                .configure(
                    api_lib::films::service::<api_lib::film_repository::PostgresFilmRepository>,
                ),
        )
        .service(
            actix_files::Files::new("/", "api/shuttle/static/")
                .show_files_listing()
                .index_file("index.html"),
        );
    };

    Ok(config.into())
}
